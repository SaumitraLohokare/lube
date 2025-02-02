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

    let arg_0 = func.add_arg(Size::DoubleWord, true);
    let arg_1 = func.add_arg(Size::DoubleWord, true);

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

    func.add_arg(Size::DoubleWord, true);
    func.add_arg(Size::DoubleWord, true);
    func.add_arg(Size::DoubleWord, true);
    func.add_arg(Size::DoubleWord, true);
    func.add_arg(Size::DoubleWord, true);
    func.add_arg(Size::DoubleWord, true);
    func.add_arg(Size::DoubleWord, true);
    func.add_arg(Size::DoubleWord, true);
    func.add_arg(Size::DoubleWord, true);
    func.add_arg(Size::DoubleWord, true);

    func.add_inst_return(None);

    module.add_func(func);

    module.generate_asm().save_to("why_would_you_do_this.s")?;

    /*
    
        void variables() {
            char a = 33;
            unsigned short b = 69;
            int c = -666;
            unsigned long d = 9876543210;
        }

        ==========

        variables:
            %0 = 33
            $0 = %0
            %1 = 69
            $1 = %1
            %2 = -666
            $2 = %2
            %3 = 9876543210
            $3 = %3
    
    */

    let mut module = Module::new();

    let mut func = Function::new("_variables".to_string());
    func.make_public();

    let tmp_0 = func.add_inst_set(Value::I8(33));
    let _var_0 = func.add_inst_store(tmp_0);
    let tmp_1 = func.add_inst_set(Value::U16(69));
    let _var_1 = func.add_inst_store(tmp_1);
    let tmp_2 = func.add_inst_set(Value::I32(-666));
    let _var_2 = func.add_inst_store(tmp_2);
    let tmp_3 = func.add_inst_set(Value::U64(9876543210));
    let _var_3 = func.add_inst_store(tmp_3);

    module.add_func(func);

    module.generate_asm().save_to("variables.s")?;

    /*
    
        short signed_add(short a, short b) {
            short c = a + b;
            return c;
        }

        ==========

        signed_add:
            %0 = $0
            %1 = $1
            %2 = %0 + %1
            $2 = %2
            %3 = $2
            return %3
    
    */

    let mut module = Module::new();

    let mut func = Function::new("_signed_add".to_string());
    func.make_public();

    let arg_0 = func.add_arg(Size::Word, true);
    let arg_1 = func.add_arg(Size::Word, true);

    let tmp_0 = func.add_inst_load(arg_0);
    let tmp_1 = func.add_inst_load(arg_1);
    let tmp_2 = func.add_inst_add(tmp_0, tmp_1);
    let var_2 = func.add_inst_store(tmp_2);
    let tmp_3 = func.add_inst_load(var_2);
    func.add_inst_return(Some(tmp_3));

    module.add_func(func);

    module.generate_asm().save_to("signed_add.s")?;
    
    /*

        int add(int a, int b) {
            return a + b;
        }
    
        int main(void) {
            int a = 2;
            int b = -2;
            return add(a, b);
        }

        ==========

        add:
            %0 = $0
            %1 = $1
            %2 = %0 + %1
            return %2

        main:
            %0 = 2
            $0 = %0
            %1 = -2
            $1 = %1
            %2 = $0
            %3 = $1
            call add %2 %3
            %4 = call_result
            return %4
    
    */

    let mut module = Module::new();

    let mut func_1 = Function::new("_add".to_string());
    func_1.make_public();

    let arg_0 = func_1.add_arg(Size::DoubleWord, true);
    let arg_1 = func_1.add_arg(Size::DoubleWord, true);

    let tmp_0 = func_1.add_inst_load(arg_0);
    let tmp_1 = func_1.add_inst_load(arg_1);
    let tmp_2 = func_1.add_inst_add(tmp_0, tmp_1);
    func_1.add_inst_return(Some(tmp_2));

    module.add_func(func_1);

    let mut func_2 = Function::new("_main".to_string());
    func_2.make_public();

    let tmp_0 = func_2.add_inst_set(Value::I32(2));
    let var_0 = func_2.add_inst_store(tmp_0);
    let tmp_1 = func_2.add_inst_set(Value::I32(-2));
    let var_1 = func_2.add_inst_store(tmp_1);
    let tmp_2 = func_2.add_inst_load(var_0);
    let tmp_3 = func_2.add_inst_load(var_1);
    func_2.add_inst_call("_add".to_string(), vec![tmp_2, tmp_3]);
    let tmp_4 = func_2.add_inst_call_result(Size::DoubleWord, true);
    func_2.add_inst_return(Some(tmp_4));

    module.add_func(func_2);

    module.generate_asm().save_to("func_call.s")?;


    Ok(())
}
