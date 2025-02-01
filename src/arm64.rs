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
                asm.add_inst(*inst, &reg_map, &offsets, return_label);
            }

            // func epilogue
            asm.generate_func_epilogue(&func, return_label);
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
        if stack_size != 0 {
            self.instructions.push(Instruction::sub_imm(
                Register::sp(),
                Register::sp(),
                stack_size,
            ));
        }

        // TODO: Store x29, x30 if needed

        // Store args in stack slots
        let mut arg_num = 0;
        #[allow(clippy::explicit_counter_loop)]
        for arg in func.args() {
            let offset = stack_slot_offsets.get(arg).unwrap();
            if let Some(arg_reg) = arg_register(arg_num) {
                self.instructions
                    .push(Instruction::str(arg_reg, Register::sp(), *offset));
            } else {
                unimplemented!()
            }

            arg_num += 1;
        }
    }

    fn generate_func_epilogue(&mut self, func: &ir::Function, return_label: ir::Label) {
        // return_label:
        self.instructions.push(Instruction::label(return_label));

        // add sp, stack size
        let stack_size = func.stack_size();
        if stack_size != 0 {
            self.instructions.push(Instruction::add_imm(
                Register::sp(),
                Register::sp(),
                stack_size,
            ));
        }

        // TODO: Load x29, x30 if needed

        // ret
        self.instructions.push(Instruction::ret());
    }

    fn add_inst(
        &mut self,
        inst: ir::Instruction,
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
                let inst = Instruction::ldr(*dest_reg, Register::sp(), *offset);
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
                let src_reg = reg_map.get(&src).unwrap();

                let inst = Instruction::mov(Register::r0(src_reg.size()), *src_reg);
                self.instructions.push(inst);

                let inst = Instruction::br(return_label);
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
    },
    Str {
        src: Register,
        addr: Register,
        offset: u16,
    },
    Br {
        label: ir::Label,
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

        if value >> 16 == 0 {
            return asm;
        }

        let imm = ((value & flag_32) >> 16) as u16;
        asm.push(Instruction::MovKImm { dest, imm });

        if value >> 32 == 0 {
            return asm;
        }

        let imm = ((value & flag_48) >> 32) as u16;
        asm.push(Instruction::MovKImm { dest, imm });

        if value >> 48 == 0 {
            return asm;
        }

        let imm = ((value & flag_64) >> 48) as u16;
        asm.push(Instruction::MovKImm { dest, imm });

        asm
    }

    fn mov(dest: Register, src: Register) -> Self {
        Self::MovReg { dest, src }
    }

    fn add(dest: Register, src_1: Register, src_2: Register) -> Self {
        Self::Add { dest, src_1, src_2 }
    }

    fn ldr(dest: Register, addr: Register, offset: u16) -> Self {
        Self::Ldr { dest, addr, offset }
    }

    fn str(src: Register, addr: Register, offset: u16) -> Self {
        Self::Str { src, addr, offset }
    }

    fn br(label: ir::Label) -> Self {
        Self::Br { label }
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
            Instruction::Custom { string }              => write!(f, "{string}"),
            Instruction::Label { label }                => write!(f, "label_{}:", label.id()),
            Instruction::MovReg { dest, src }           => write!(f, "    mov {dest}, {src}"),
            Instruction::MovImm { dest, imm }           => write!(f, "    mov {dest}, #{imm}"),
            Instruction::MovKImm { dest, imm }          => write!(f, "    movk {dest}, #{imm}"),
            Instruction::Add { dest, src_1, src_2 }     => write!(f, "    add {dest}, {src_1}, {src_2}"),
            Instruction::AddImm { dest, src_1, src_2 }  => write!(f, "    add {dest}, {src_1}, #{src_2}"),
            Instruction::SubImm { dest, src_1, src_2 }  => write!(f, "    sub {dest}, {src_1}, #{src_2}"),
            Instruction::Ldr { dest, addr, offset }     => 
                if *offset != 0 {
                    write!(f, "    ldr {dest}, [{addr}, #{offset}]")
                } else {
                    write!(f, "    ldr {dest}, [{addr}]")
                }
            Instruction::Str { src, addr, offset }      => 
                if *offset != 0 {
                    write!(f, "    str {src}, [{addr}, #{offset}]")
                } else {
                    write!(f, "    str {src}, [{addr}]")
                }
            Instruction::Br { label }                   => write!(f, "    b label_{}", label.id()),
            Instruction::Ret                            => write!(f, "    ret"),
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

    fn r0(size: Size) -> Self {
        Self::new(RegisterNumber::R0, size)
    } 

    fn x0() -> Self {
        Self::new(RegisterNumber::R0, Size::QuadWord)
    }

    fn x1() -> Self {
        Self::new(RegisterNumber::R1, Size::QuadWord)
    }

    fn x2() -> Self {
        Self::new(RegisterNumber::R2, Size::QuadWord)
    }

    fn x3() -> Self {
        Self::new(RegisterNumber::R3, Size::QuadWord)
    }

    fn x4() -> Self {
        Self::new(RegisterNumber::R4, Size::QuadWord)
    }

    fn x5() -> Self {
        Self::new(RegisterNumber::R5, Size::QuadWord)
    }

    fn x6() -> Self {
        Self::new(RegisterNumber::R6, Size::QuadWord)
    }

    fn x7() -> Self {
        Self::new(RegisterNumber::R7, Size::QuadWord)
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
        R9,  R10, R11, R12, R13, R14, 
        R15, R19, R20, R21, R22, R23, 
        R24, R25, R26, R27, R28
    ]
}

fn arg_register(arg_num: u8) -> Option<Register> {
    match arg_num {
        0 => Some(Register::x0()),
        1 => Some(Register::x1()),
        2 => Some(Register::x2()),
        3 => Some(Register::x3()),
        4 => Some(Register::x4()),
        5 => Some(Register::x5()),
        6 => Some(Register::x6()),
        7 => Some(Register::x7()),
        _ => None,
    }
}
