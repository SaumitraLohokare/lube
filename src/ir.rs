use std::collections::HashMap;

use crate::{arm64::Asm, util::Iota};

pub struct Module {
    lbl_iota: Iota,
    funcs: Vec<Function>,
}

impl Module {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            funcs: Vec::new(),
            lbl_iota: Iota::new(),
        }
    }

    pub fn add_func(&mut self, func: Function) {
        self.funcs.push(func);
    }

    pub fn generate_asm(&mut self) -> Asm {
        Asm::from_module(self)
    }

    pub(crate) fn funcs(&self) -> &[Function] {
        &self.funcs
    }
}

#[derive(Clone)]
pub struct Function {
    is_public: bool,
    is_leaf: bool,
    name: String,
    tmp_iota: Iota,
    var_iota: Iota,
    args: Vec<StackSlot>,
    stack_slots: Vec<StackSlot>,
    instructions: Vec<Instruction>,
}

impl Function {
    pub fn new(name: String) -> Self {
        Self {
            is_public: false,
            is_leaf: true,
            name,
            tmp_iota: Iota::new(),
            var_iota: Iota::new(),
            args: Vec::new(),
            stack_slots: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn make_public(&mut self) {
        self.is_public = true;
    }

    pub(crate) fn is_public(&self) -> bool {
        self.is_public
    }

    pub(crate) fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub(crate) fn args(&self) -> &Vec<StackSlot> {
        &self.args
    }

    pub fn add_arg(&mut self, size: Size, signed: bool) -> StackSlot {
        let slot = StackSlot::new(self.var_iota.next(), size, signed);
        // NOTE: We are duplicating data... but whatever
        self.args.push(slot);
        self.stack_slots.push(slot);
        slot
    }

    pub fn add_inst_set(&mut self, value: Value) -> Temporary {
        let result = Temporary::new(self.tmp_iota.next(), value.size(), value.is_signed());

        let inst = Instruction::Set {
            dest: result,
            src: value,
        };
        self.instructions.push(inst);

        result
    }

    pub fn add_inst_return(&mut self, tmp: Option<Temporary>) {
        let inst = Instruction::Return { src: tmp };
        self.instructions.push(inst);
    }

    pub fn add_inst_load(&mut self, slot: StackSlot) -> Temporary {
        let result = Temporary::new(self.tmp_iota.next(), slot.size(), slot.is_signed());

        let inst = Instruction::Load {
            dest: result,
            src: slot,
        };
        self.instructions.push(inst);

        result
    }

    pub fn add_inst_store(&mut self, tmp: Temporary) -> StackSlot {
        let slot = StackSlot::new(self.var_iota.next(), tmp.size(), tmp.is_signed());
        self.stack_slots.push(slot);

        let inst = Instruction::Store {
            dest: slot,
            src: tmp,
        };
        self.instructions.push(inst);

        slot
    }

    pub fn add_inst_add(&mut self, src_1: Temporary, src_2: Temporary) -> Temporary {
        // NOTE: For now assert their sizes are equal
        assert_eq!(src_1.size(), src_2.size());

        let result = Temporary::new(
            self.tmp_iota.next(),
            src_1.size(),
            src_1.is_signed() | src_2.is_signed(),
        );

        let inst = Instruction::Add {
            dest: result,
            src_1,
            src_2,
        };
        self.instructions.push(inst);

        result
    }

    pub fn add_inst_call(
        &mut self,
        func: String,
        args: Vec<Temporary>,
    ) {
        self.is_leaf = false;
        
        let inst = Instruction::Call { func, args };
        self.instructions.push(inst);
    }

    pub fn add_inst_call_result(&mut self, size: Size, signed: bool) -> Temporary {
        let result = Temporary::new(
            self.tmp_iota.next(),
            size,
            signed,
        );

        let inst = Instruction::CallResult { dest: result };
        self.instructions.push(inst);

        result
    }

    pub(crate) fn generate_stack_slot_offsets(&self) -> HashMap<StackSlot, u16> {
        // TODO: C gives an extra 4 byte gap before variables... why?
        let mut stack_slot_offsets = HashMap::new();
        let mut current_offset = self.stack_size();

        for slot in &self.stack_slots {
            let slot_size = slot.size().in_bytes();
            // round up current offset to nearest slot_size
            while current_offset % slot_size != 0 {
                current_offset -= 1;
            }

            current_offset -= slot_size;
            stack_slot_offsets.insert(*slot, current_offset);
        }

        stack_slot_offsets
    }

    pub(crate) fn stack_size(&self) -> u16 {
        let mut stack_size = 0;

        for slot in &self.stack_slots {
            let slot_size = slot.size().in_bytes();

            // Add alignment if needed
            while stack_size % slot_size != 0 {
                stack_size += 1;
            }

            stack_size += slot_size;
        }

        // Align stack to 16 bytes
        while stack_size % 16 != 0 {
            stack_size += 1;
        }

        stack_size
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Temporary {
    id: usize,
    size: Size,
    signed: bool,
}

impl Temporary {
    fn new(id: usize, size: Size, signed: bool) -> Self {
        Self { id, size, signed }
    }

    pub(crate) fn size(self) -> Size {
        self.size
    }

    fn is_signed(self) -> bool {
        self.signed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Size {
    Byte,
    Word,
    DoubleWord,
    QuadWord,
}

impl Size {
    pub(crate) fn in_bytes(self) -> u16 {
        match self {
            Size::Byte => 1,
            Size::Word => 2,
            Size::DoubleWord => 4,
            Size::QuadWord => 8,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl Value {
    fn size(self) -> Size {
        match self {
            Value::U8(_) => Size::Byte,
            Value::U16(_) => Size::Word,
            Value::U32(_) => Size::DoubleWord,
            Value::U64(_) => Size::QuadWord,
            Value::I8(_) => Size::Byte,
            Value::I16(_) => Size::Word,
            Value::I32(_) => Size::DoubleWord,
            Value::I64(_) => Size::QuadWord,
        }
    }

    pub(crate) fn as_u64(self) -> u64 {
        match self {
            Value::U8(x) => x as u64,
            Value::U16(x) => x as u64,
            Value::U32(x) => x as u64,
            Value::U64(x) => x,
            Value::I8(x) => x as u64,
            Value::I16(x) => x as u64,
            Value::I32(x) => x as u64,
            Value::I64(x) => x as u64,
        }
    }

    fn is_signed(self) -> bool {
        match self {
            Value::U8(_) | Value::U16(_) | Value::U32(_) | Value::U64(_) => false,
            Value::I8(_) | Value::I16(_) | Value::I32(_) | Value::I64(_) => true,
        }
    }
}

#[rustfmt::skip]
#[derive(Clone)]
pub(crate) enum Instruction {
    Set        { dest: Temporary, src: Value },
    Return     { src: Option<Temporary> },
    Load       { dest: Temporary, src: StackSlot },
    Store      { dest: StackSlot, src: Temporary },
    Add        { dest: Temporary, src_1: Temporary, src_2: Temporary },
    Call       { func: String, args: Vec<Temporary> },
    CallResult { dest: Temporary },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StackSlot {
    id: usize,
    size: Size,
    signed: bool,
}

impl StackSlot {
    fn new(id: usize, size: Size, signed: bool) -> Self {
        Self { id, size, signed }
    }

    pub(crate) fn size(self) -> Size {
        self.size
    }

    pub(crate) fn is_signed(self) -> bool {
        self.signed
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Label {
    id: usize,
}

impl Label {
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }

    pub(crate) fn id(&self) -> usize {
        self.id
    }
}
