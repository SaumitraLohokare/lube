use core::fmt;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
};

use crate::{
    ir::{self, Size},
    util::RegisterAllocator,
};

pub struct Asm {
    instructions: Vec<Instruction>,
}

impl Asm {
    pub(crate) fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub(crate) fn from_module(module: &mut ir::Module) -> Self {
        let mut asm = Self::new();

        for func in module.funcs_cloned() {
            let reg_map =
                RegisterAllocator::new().allocate(func.instructions(), usable_registers());

            let offsets = func.generate_stack_slot_offsets();
            let return_label = module.next_label();

            // func prologue
            asm.generate_func_prologue(&func, &offsets);

            for inst in func.instructions() {
                asm.add_inst(func.is_leaf(), inst, &reg_map, &offsets, return_label);
            }

            // func epilogue
            asm.generate_func_epilogue(&func, return_label);

            // Spacing between functions
            asm.instructions.push(Instruction::Empty);
        }

        asm
    }

    fn generate_func_prologue(
        &mut self,
        func: &ir::Function,
        stack_slot_offsets: &HashMap<ir::StackSlot, u16>,
    ) {
        // .global func_name
        if func.is_public() {
            self.instructions
                .push(Instruction::custom(format!(".global {}", func.name())));
        }

        // .align 2
        self.instructions
            .push(Instruction::custom(".align 2".to_string()));

        // func_name:
        self.instructions
            .push(Instruction::custom(format!("{}:", func.name())));

        // sub sp, stack size
        let stack_size = func.stack_size();
        let func_stack_size = stack_size + if !func.is_leaf() { 16 } else { 0 };

        if func_stack_size != 0 {
            self.instructions.push(Instruction::sub_imm(
                Register::sp(),
                Register::sp(),
                func_stack_size,
            ));
        }

        // Store x29, x30 if needed
        if !func.is_leaf() {
            // stp x29, x30, [sp, stack size]
            let inst = Instruction::stp(Register::x29(), Register::x30(), Register::sp(), stack_size);
            self.instructions.push(inst);

            // add x29, sp, stack size
            let inst = Instruction::add_imm(Register::x29(), Register::sp(), 16);
            self.instructions.push(inst);
        }

        // Store args in stack slots
        let mut arg_num = 0;
        let mut additional_args_offset = 0;
        #[allow(clippy::explicit_counter_loop)]
        for arg in func.args() {
            let offset = stack_slot_offsets.get(arg).unwrap();
            if let Some(arg_reg) = arg_register(arg_num, arg.size()) {
                self.instructions
                    .push(Instruction::str(arg_reg, Register::sp(), *offset));
            } else {
                while (stack_size + additional_args_offset) % arg.size().in_bytes() != 0 {
                    additional_args_offset += 1;
                }

                self.instructions.push(Instruction::ldr(
                    Register::r9(arg.size()),
                    Register::sp(),
                    func_stack_size + additional_args_offset,
                    arg.is_signed(),
                ));

                additional_args_offset += arg.size().in_bytes();

                self.instructions.push(Instruction::str(
                    Register::r9(arg.size()),
                    Register::sp(),
                    *offset,
                ));
            }

            arg_num += 1;
        }
    }

    fn generate_func_epilogue(&mut self, func: &ir::Function, return_label: ir::Label) {
        let stack_size = func.stack_size();
        
        // return_label:
        self.instructions.push(Instruction::label(return_label));

        // Load x29, x30 if needed
        if !func.is_leaf() {
            // ldp x29, x30, [sp, stack size]
            let inst = Instruction::ldp(Register::x29(), Register::x30(), Register::sp(), stack_size);
            self.instructions.push(inst);
        }

        // add sp, stack size
        let stack_size = func.stack_size();
        let func_stack_size = stack_size + if !func.is_leaf() { 16 } else { 0 };
        if func_stack_size != 0 {
            self.instructions.push(Instruction::add_imm(
                Register::sp(),
                Register::sp(),
                func_stack_size,
            ));
        }

        // ret
        self.instructions.push(Instruction::ret());
    }

    fn add_inst(
        &mut self,
        func_is_leaf: bool,
        inst: &ir::Instruction,
        reg_map: &HashMap<ir::Temporary, Register>,
        stack_slot_offsets: &HashMap<ir::StackSlot, u16>,
        return_label: ir::Label,
    ) {
        match inst {
            ir::Instruction::Set { dest, src } => {
                let reg = reg_map.get(&dest).unwrap();
                let inst = Instruction::mov_imm(*reg, src.as_u64());
                self.instructions.extend(inst);
            }
            ir::Instruction::Load { dest, src } => {
                let dest_reg = reg_map.get(&dest).unwrap();
                let offset = stack_slot_offsets.get(&src).unwrap();
                let offset = get_correct_offset(func_is_leaf, *offset);

                let inst = Instruction::ldr(*dest_reg, Register::sp(), offset, src.is_signed());
                self.instructions.push(inst);
            }
            ir::Instruction::Add { dest, src_1, src_2 } => {
                let src_1_reg = reg_map.get(&src_1).unwrap();
                let src_2_reg = reg_map.get(&src_2).unwrap();
                let dest_reg = reg_map.get(&dest).unwrap();

                let inst = Instruction::add(*dest_reg, *src_1_reg, *src_2_reg);
                self.instructions.push(inst);
            }
            ir::Instruction::Return { src } => {
                if let Some(src) = src {
                    let src_reg = reg_map.get(&src).unwrap();

                    let inst = Instruction::mov(Register::r0(src_reg.size()), *src_reg);
                    self.instructions.push(inst);
                }

                let inst = Instruction::b(return_label);
                self.instructions.push(inst);
            }
            ir::Instruction::Store { dest, src } => {
                let src_reg = reg_map.get(&src).unwrap();
                let offset = stack_slot_offsets.get(&dest).unwrap();
                let offset = get_correct_offset(func_is_leaf, *offset);

                let inst = Instruction::str(*src_reg, Register::sp(), offset);
                self.instructions.push(inst);
            }
            ir::Instruction::Call { func, args } => {
                // Store args in correct registers/stack
                let mut arg_num = 0;
                let mut additional_args_offset = 0;
                #[allow(clippy::explicit_counter_loop)]
                for arg in args {
                    let value_reg = reg_map.get(arg).unwrap();
                    if let Some(arg_reg) = arg_register(arg_num, arg.size()) {
                        let inst = Instruction::mov(arg_reg, *value_reg);
                        self.instructions.push(inst);
                    } else {
                        let inst = Instruction::str(*value_reg, Register::sp(), additional_args_offset);
                        self.instructions.push(inst);
                        additional_args_offset += arg.size().in_bytes();
                    }

                    arg_num += 1;
                }

                // Call func
                let inst = Instruction::bl(func.clone());
                self.instructions.push(inst);
            }
            ir::Instruction::CallResult { dest } => {
                let dest_reg = reg_map.get(&dest).unwrap();

                // mov dest_reg, x0
                let inst = Instruction::mov(*dest_reg, Register::r0(dest_reg.size()));
                self.instructions.push(inst);
            }
        }
    }

    pub fn save_to(self, file: &str) -> io::Result<()> {
        let mut file = File::create(file)?;

        for inst in self.instructions {
            writeln!(file, "{inst}")?;
        }

        Ok(())
    }
}

enum Instruction {
    // Empty line in generated asm code
    Empty,
    Custom {
        string: String,
    },
    Label {
        label: ir::Label,
    },
    MovReg {
        dest: Register,
        src: Register,
    },
    MovImm {
        dest: Register,
        imm: u16,
    },
    MovKImm {
        dest: Register,
        imm: u16,
        offset: u16,
    },
    Add {
        dest: Register,
        src_1: Register,
        src_2: Register,
    },
    AddImm {
        dest: Register,
        src_1: Register,
        src_2: u16,
    },
    SubImm {
        dest: Register,
        src_1: Register,
        src_2: u16,
    },
    Ldr {
        dest: Register,
        addr: Register,
        offset: u16,
        signed: bool,
    },
    Ldp {
        dest_1: Register,
        dest_2: Register,
        addr: Register,
        offset: u16,
    },
    Str {
        src: Register,
        addr: Register,
        offset: u16,
    },
    Stp {
        src_1: Register,
        src_2: Register,
        addr: Register,
        offset: u16,
    },
    Br {
        label: ir::Label,
    },
    Bl {
        func: String,
    },
    Ret,
}

impl Instruction {
    fn custom(string: String) -> Self {
        Self::Custom { string }
    }

    fn mov_imm(dest: Register, value: u64) -> Vec<Instruction> {
        let mut asm = Vec::with_capacity(4);

        let flag_16 = 0x000000000000FFFF;
        let flag_32 = 0x00000000FFFF0000;
        let flag_48 = 0x0000FFFF00000000;
        let flag_64 = 0xFFFF000000000000;

        let imm = (value & flag_16) as u16;
        asm.push(Instruction::MovImm { dest, imm });

        if value >> 16 == 0 || dest.size() == Size::Byte || dest.size() == Size::Word {
            return asm;
        }

        let imm = ((value & flag_32) >> 16) as u16;
        asm.push(Instruction::MovKImm {
            dest,
            imm,
            offset: 16,
        });

        if value >> 32 == 0 || dest.size() == Size::DoubleWord {
            return asm;
        }

        let imm = ((value & flag_48) >> 32) as u16;
        asm.push(Instruction::MovKImm {
            dest,
            imm,
            offset: 32,
        });

        if value >> 48 == 0 {
            return asm;
        }

        let imm = ((value & flag_64) >> 48) as u16;
        asm.push(Instruction::MovKImm {
            dest,
            imm,
            offset: 48,
        });

        asm
    }

    fn mov(dest: Register, src: Register) -> Self {
        Self::MovReg { dest, src }
    }

    fn add(dest: Register, src_1: Register, src_2: Register) -> Self {
        Self::Add { dest, src_1, src_2 }
    }

    fn ldr(dest: Register, addr: Register, offset: u16, signed: bool) -> Self {
        Self::Ldr {
            dest,
            addr,
            offset,
            signed,
        }
    }

    fn ldp(dest_1: Register, dest_2: Register, addr: Register, offset: u16) -> Self {
        Self::Ldp {
            dest_1,
            dest_2,
            addr,
            offset,
        }
    }

    fn str(src: Register, addr: Register, offset: u16) -> Self {
        Self::Str { src, addr, offset }
    }

    fn stp(src_1: Register, src_2: Register, addr: Register, offset: u16) -> Self {
        Self::Stp {
            src_1,
            src_2,
            addr,
            offset,
        }
    }

    fn b(label: ir::Label) -> Self {
        Self::Br { label }
    }

    fn bl(func: String) -> Self {
        Self::Bl { func }
    }

    fn sub_imm(dest: Register, src_1: Register, src_2: u16) -> Self {
        // Imm is 12 bits wide
        assert!(src_2 >> 12 == 0);

        Self::SubImm { dest, src_1, src_2 }
    }

    fn add_imm(dest: Register, src_1: Register, src_2: u16) -> Self {
        // Imm is 12 bits wide
        assert!(src_2 >> 12 == 0);

        Self::AddImm { dest, src_1, src_2 }
    }

    fn label(label: ir::Label) -> Self {
        Self::Label { label }
    }

    fn ret() -> Self {
        Self::Ret
    }
}

impl fmt::Display for Instruction {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Empty                          => write!(f, ""),
            Instruction::Custom { string }              => write!(f, "{string}"),
            Instruction::Label { label }                => write!(f, "label_{}:", label.id()),
            Instruction::MovReg { dest, src }           => write!(f, "    mov {dest}, {src}"),
            Instruction::MovImm { dest, imm }           => write!(f, "    mov {dest}, #{imm}"),
            Instruction::MovKImm { dest, imm, offset }  => write!(f, "    movk {dest}, #{imm}, lsl #{offset}"),
            Instruction::Add { dest, src_1, src_2 }     => write!(f, "    add {dest}, {src_1}, {src_2}"),
            Instruction::AddImm { dest, src_1, src_2 }  => write!(f, "    add {dest}, {src_1}, #{src_2}"),
            Instruction::SubImm { dest, src_1, src_2 }  => write!(f, "    sub {dest}, {src_1}, #{src_2}"),
            Instruction::Br { label }                   => write!(f, "    b label_{}", label.id()),
            Instruction::Bl { func }                    => write!(f, "    bl {func}"),
            Instruction::Ret                            => write!(f, "    ret"),
            Instruction::Ldr { dest, addr, offset, signed } => {
                match dest.size() {
                    Size::Byte       => write!(f, "    ldr{}b {dest}, ", if *signed { "s" } else { "" }),
                    Size::Word       => write!(f, "    ldr{}h {dest}, ", if *signed { "s" } else { "" }),
                    Size::DoubleWord => write!(f, "    ldr {dest}, "), // TODO: If we need to load 32 bit signed to 64 bit signed use ldrsw
                    Size::QuadWord   => write!(f, "    ldr {dest}, "),
                }?;

                if *offset != 0 {
                    write!(f, "[{addr}, #{offset}]")
                } else {
                    write!(f, "[{addr}]")
                }
            }
            Instruction::Ldp { dest_1, dest_2, addr, offset } => {
                write!(f, "    ldp {dest_1}, {dest_2}, ")?;

                if *offset != 0 {
                    write!(f, "[{addr}, #{offset}]")
                } else {
                    write!(f, "[{addr}]")
                }
            }
            Instruction::Str { src, addr, offset } => {
                match src.size() {
                    Size::Byte       => write!(f, "    strb {src}, "),
                    Size::Word       => write!(f, "    strh {src}, "),
                    Size::DoubleWord => write!(f, "    str {src}, "),
                    Size::QuadWord   => write!(f, "    str {src}, "),
                }?;

                if *offset != 0 {
                    write!(f, "[{addr}, #{offset}]")
                } else {
                    write!(f, "[{addr}]")
                }
            }
            Instruction::Stp { src_1, src_2, addr, offset } => {
                write!(f, "    stp {src_1}, {src_2}, ")?;

                if *offset != 0 {
                    write!(f, "[{addr}, #{offset}]")
                } else {
                    write!(f, "[{addr}]")
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Register {
    number: RegisterNumber,
    size: Size,
}

impl Register {
    pub(crate) fn new(number: RegisterNumber, size: Size) -> Self {
        Self { number, size }
    }

    pub(crate) fn number(self) -> RegisterNumber {
        self.number
    }

    fn size(&self) -> Size {
        self.size
    }

    fn sp() -> Self {
        Self::new(RegisterNumber::SP, Size::QuadWord)
    }

    fn x29() -> Self {
        Self::new(RegisterNumber::R29, Size::QuadWord)
    }

    fn x30() -> Self {
        Self::new(RegisterNumber::R30, Size::QuadWord)
    }

    fn r0(size: Size) -> Self {
        Self::new(RegisterNumber::R0, size)
    }

    fn r1(size: Size) -> Self {
        Self::new(RegisterNumber::R1, size)
    }

    fn r2(size: Size) -> Self {
        Self::new(RegisterNumber::R2, size)
    }

    fn r3(size: Size) -> Self {
        Self::new(RegisterNumber::R3, size)
    }

    fn r4(size: Size) -> Self {
        Self::new(RegisterNumber::R4, size)
    }

    fn r5(size: Size) -> Self {
        Self::new(RegisterNumber::R5, size)
    }

    fn r6(size: Size) -> Self {
        Self::new(RegisterNumber::R6, size)
    }

    fn r7(size: Size) -> Self {
        Self::new(RegisterNumber::R7, size)
    }

    fn r9(size: Size) -> Self {
        Self::new(RegisterNumber::R9, size)
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.number == RegisterNumber::SP {
            return write!(f, "sp");
        }

        let reg_num = self.number as u8;
        match self.size {
            Size::Byte | Size::Word | Size::DoubleWord => write!(f, "w{reg_num}"),
            Size::QuadWord => write!(f, "x{reg_num}"),
        }
    }
}

#[rustfmt::skip]
#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum RegisterNumber {
    R0,  R1,  R2,  R3,  R4,  R5,  R6,  R7,
    R8,  R9,  R10, R11, R12, R13, R14, R15,
    R16, R17, R18, R19, R20, R21, R22, R23,
    R24, R25, R26, R27, R28, R29, R30, SP,
}

#[rustfmt::skip]
pub(crate) fn usable_registers() -> Vec<RegisterNumber> {
    use RegisterNumber::*;
    vec![
        R8,  R9,  R10, R11, R12, R13, 
        R14, R15, R19, R20, R21, R22, 
        R23, R24, R25, R26, R27, R28
    ]
}

fn arg_register(arg_num: u8, size: Size) -> Option<Register> {
    match arg_num {
        0 => Some(Register::r0(size)),
        1 => Some(Register::r1(size)),
        2 => Some(Register::r2(size)),
        3 => Some(Register::r3(size)),
        4 => Some(Register::r4(size)),
        5 => Some(Register::r5(size)),
        6 => Some(Register::r6(size)),
        7 => Some(Register::r7(size)),
        _ => None,
    }
}

fn get_correct_offset(is_leaf: bool, offset: u16) -> u16 {
    // NOTE: Write propper tests to check this
    if !is_leaf {
        offset // + 16
    } else {
        offset
    }
}
