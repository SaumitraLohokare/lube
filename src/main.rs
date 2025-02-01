use std::io;

use lube::ir::{Module, Procedure, Size, Value};

fn main() -> io::Result<()> {
    /*

        int deepThink() {
            return 42;
        }

        ==========

        deepThink:
            %0 = 42
            return %0

    */

    let mut module = Module::new();

    let mut proc = Procedure::new("_deepThink".to_string());

    let tmp_1 = proc.add_inst_set(Value::I32(42));
    proc.add_inst_return(tmp_1);

    module.add_proc(proc);

    module.generate_asm().save_to("deepThink.s")?;

    /*

        int add(int a, int b) {
            return a + b;
        }

        ==========

        add:
            %0 = $0
            %1 = $1
            %2 = %0 + %1
            return %2

    */

    let mut module = Module::new();

    let mut proc = Procedure::new("_add".to_string());

    let arg_0 = proc.add_arg(Size::DoubleWord);
    let arg_1 = proc.add_arg(Size::DoubleWord);

    let tmp_0 = proc.add_inst_load(arg_0);
    let tmp_1 = proc.add_inst_load(arg_1);
    let tmp_2 = proc.add_inst_add(tmp_0, tmp_1);
    proc.add_inst_return(tmp_2);

    module.add_proc(proc);

    module.generate_asm().save_to("add.s")?;

    Ok(())
}
