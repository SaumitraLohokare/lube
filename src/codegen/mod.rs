use std::io;

use crate::ir::Module;

mod arm64;

pub enum Target {
    AppleArm64,
}

pub enum Asm {
    Arm64(arm64::Asm),
}

impl Asm {
    pub fn save_to(&self, file: &str) -> io::Result<()> {
        match self {
            Asm::Arm64(asm) => asm.save_to(file),
        }
    }
}

pub fn gen_asm(module: &Module, target: Target) -> Asm {
    match target {
        Target::AppleArm64 => Asm::Arm64(arm64::gen_asm(module)),
    }
}