use std::collections::HashMap;

use crate::{
    arm64::Asm,
    util::Iota,
};

pub struct Module {
    lbl_iota: Iota,
    procs: Vec<Procedure>,
}

impl Module {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            procs: Vec::new(),
            lbl_iota: Iota::new(),
        }
    }

    pub fn add_proc(&mut self, proc: Procedure) {
        self.procs.push(proc);
    }

    pub fn generate_asm(&mut self) -> Asm {
        Asm::from_module(self)
    }

    pub(crate) fn procs_cloned(&self) -> Vec<Procedure> {
        self.procs.clone()
    }

    pub(crate) fn next_label(&mut self) -> Label {
        Label::new(self.lbl_iota.next())
    }
}

#[derive(Clone)]
pub struct Procedure {
    name: String,
    tmp_iota: Iota,
    var_iota: Iota,
    args: Vec<StackSlot>,
    stack_slots: Vec<StackSlot>,
    instructions: Vec<Instruction>,
}

impl Procedure {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tmp_iota: Iota::new(),
            var_iota: Iota::new(),
            args: Vec::new(),
            stack_slots: Vec::new(),
            instructions: Vec::new(),
        }
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

    pub fn add_arg(&mut self, size: Size) -> StackSlot {
        let slot = StackSlot::new(self.var_iota.next(), size);
        // NOTE: We are duplicating data... but whatever
        self.args.push(slot);
        self.stack_slots.push(slot);
        slot
    }

    pub fn add_inst_set(&mut self, value: Value) -> Temporary {
        let result = Temporary::new(self.tmp_iota.next(), value.size());

        let inst = Instruction::Set {
            dest: result,
            src: value,
        };
        self.instructions.push(inst);

        result
    }

    pub fn add_inst_return(&mut self, tmp: Temporary) {
        let inst = Instruction::Return { src: tmp };
        self.instructions.push(inst);
    }

    pub fn add_inst_load(&mut self, slot: StackSlot) -> Temporary {
        let result = Temporary::new(self.tmp_iota.next(), slot.size());

        let inst = Instruction::Load {
            dest: result,
            src: slot,
        };
        self.instructions.push(inst);

        result
    }

    pub fn add_inst_add(&mut self, src_1: Temporary, src_2: Temporary) -> Temporary {
        // NOTE: For now assert their sizes are equal
        assert_eq!(src_1.size(), src_2.size());

        let result = Temporary::new(self.tmp_iota.next(), src_1.size);

        let inst = Instruction::Add {
            dest: result,
            src_1,
            src_2,
        };
        self.instructions.push(inst);

        result
    }

    pub(crate) fn generate_stack_slot_offsets(&self) -> HashMap<StackSlot, u16> {
        let mut stack_slot_offsets = HashMap::new();
        let mut current_offset = 0;

        for slot in &self.stack_slots {
            let slot_size = slot.size().in_bytes();
            // round up current offset to nearest slot_size
            while current_offset % slot_size != 0 {
                current_offset += 1;
            }

            stack_slot_offsets.insert(*slot, current_offset);
            current_offset += slot_size;
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

        stack_size
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Temporary {
    id: usize,
    size: Size,
}

impl Temporary {
    fn new(id: usize, size: Size) -> Self {
        Self { id, size }
    }

    pub(crate) fn size(self) -> Size {
        self.size
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
    fn in_bytes(self) -> u16 {
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
}

#[rustfmt::skip]
#[derive(Clone, Copy)]
pub(crate) enum Instruction {
    Set { dest: Temporary, src: Value },
    Return { src: Temporary },
    Load { dest: Temporary, src: StackSlot },
    Add { dest: Temporary, src_1: Temporary, src_2: Temporary },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StackSlot {
    id: usize,
    size: Size,
}

impl StackSlot {
    fn new(id: usize, size: Size) -> Self {
        Self { id, size }
    }

    fn size(self) -> Size {
        self.size
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
