use std::io;

use lube::ir::{Module, Function, Size, Value};

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

    let mut func = Function::new("_deepThink".to_string());
    func.make_public();

    let tmp_1 = func.add_inst_set(Value::I32(42));
    func.add_inst_return(Some(tmp_1));

    module.add_func(func);

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

    let mut func = Function::new("_add".to_string());
    func.make_public();

    let arg_0 = func.add_arg(Size::DoubleWord);
    let arg_1 = func.add_arg(Size::DoubleWord);

    let tmp_0 = func.add_inst_load(arg_0);
    let tmp_1 = func.add_inst_load(arg_1);
    let tmp_2 = func.add_inst_add(tmp_0, tmp_1);
    func.add_inst_return(Some(tmp_2));

    module.add_func(func);

    module.generate_asm().save_to("add.s")?;

    /*
    
        void why_would_you_do_this(int a, int b, int c, int d, int e, int f, int g, int h, int i, int j) 
        {
        }

        ==========

        why_would_you_do_this:
            return
    
    */

    let mut module = Module::new();

    let mut func = Function::new("_why_would_you_do_this".to_string());

    func.add_arg(Size::DoubleWord);
    func.add_arg(Size::DoubleWord);
    func.add_arg(Size::DoubleWord);
    func.add_arg(Size::DoubleWord);
    func.add_arg(Size::DoubleWord);
    func.add_arg(Size::DoubleWord);
    func.add_arg(Size::DoubleWord);
    func.add_arg(Size::DoubleWord);
    func.add_arg(Size::DoubleWord);
    func.add_arg(Size::DoubleWord);

    func.add_inst_return(None);

    module.add_func(func);

    module.generate_asm().save_to("why_would_you_do_this.s")?;

    /*

        int add(int a, int b) {
            return a + b;
        }
    
        int main(void) {
            return add(2, -2);
        }

        ==========

        add:
            %0 = $0
            %1 = $1
            %2 = %0 + %1
            return %2

        main:
            %0 = 2
            %1 = -1
            %3 = call add %0 %1
            return %3
    
    */

    Ok(())
}
