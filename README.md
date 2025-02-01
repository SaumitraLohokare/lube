# 💦 lube

A Little Useful Back End for making compilers.

## Examples

```rs
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
    func.add_inst_return(tmp_1);

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
    func.add_inst_return(tmp_2);

    module.add_func(func);

    module.generate_asm().save_to("add.s")?;

    Ok(())
}
```