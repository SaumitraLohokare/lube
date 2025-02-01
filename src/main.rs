use std::io;

use lube::{
    codegen::Target,
    ir::{Module, Procedure, Type},
};

fn main() -> io::Result<()> {
    println!("Hello, Lube!");

    let mut module = Module::new();
    let mut proc = Procedure::new("_main".into());
    proc.add_arg(Type::DoubleWord);
    proc.add_arg(Type::QuadWord);
    module.add_proc(proc);

    module.generate_asm(Target::AppleArm64).save_to("out.s")
}
