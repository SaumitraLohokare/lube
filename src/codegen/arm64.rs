use std::{fs::File, io::{self, Write}};

use crate::ir::{Module, Procedure};

enum Register {
    R0,
    SP,
}

pub struct Asm {
    insts: Vec<Instruction>,
}

impl Asm {
    pub fn new() -> Self {
        Self { insts: Vec::new() }
    }

    fn add_inst(&mut self, inst: Instruction) {
        self.insts.push(inst);
    }

    pub fn save_to(&self, file: &str) -> io::Result<()> {
        let mut file = File::create(file)?;

        for inst in &self.insts {
            file.write_all(inst.as_bytes())?;
        }

        Ok(())
    }
}

enum Instruction {
    Label { label: String },
    Ret,

    Sub { dest: Register, src1: Register, src2: Register}
}

impl Instruction {
    fn label(mut label: String) -> Self {
        label.push_str(":\n");
        Self::Label { label }
    }

    fn ret() -> Self {
        Self::Ret
    }

    fn as_bytes(&self) -> &[u8] {
        match self {
            Instruction::Label { label } => label.as_bytes(),
            Instruction::Ret => "    ret\n".as_bytes(),
            Instruction::Sub { dest, src1, src2 } => todo!(),
        }
    }
}

pub fn gen_asm(module: &Module) -> Asm {
    let mut asm = Asm::new();

    for proc in module.procs() {
        gen_proc_asm(proc, &mut asm);
    }

    asm
}

fn gen_proc_asm(proc: &Procedure, asm: &mut Asm) {
    gen_proc_prologue(proc, asm);
    gen_proc_epilogue(proc, asm);
}

fn proc_stack_size(proc: &Procedure) -> usize {
    todo!()
}

fn gen_proc_prologue(proc: &Procedure, asm: &mut Asm) {
    asm.add_inst(Instruction::label(proc.name()));
    // sub sp, sp, #stack_size
    // stp x29, x30, [sp, offset]
}

fn gen_proc_epilogue(_proc: &Procedure, asm: &mut Asm) {
    // ldp x29, x30, [sp, offset]
    // add sp, sp, #stack_size
    asm.add_inst(Instruction::ret());
}
