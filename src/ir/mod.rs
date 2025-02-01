use crate::codegen::{gen_asm, Asm, Target};

/// This represents one module that contains
/// functions, data, etc.
pub struct Module {
    /// List of procedures
    procs: Vec<Procedure>,
}

impl Module {
    pub fn new() -> Self {
        Self { procs: Vec::new() }
    }

    pub fn add_proc(&mut self, proc: Procedure) {
        self.procs.push(proc);
    }

    pub fn generate_asm(&self, target: Target) -> Asm {
        gen_asm(self, target)
    }

    pub fn procs(&self) -> &Vec<Procedure> {
        &self.procs
    }
}

/// This represents one procedure
pub struct Procedure {
    name: String,
    args: Vec<(Variable, Type)>,
    var_iota: Iota,
}

impl Procedure {
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: Vec::new(),
            var_iota: Iota::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn add_arg(&mut self, data_type: Type) {
        self.args
            .push((Variable::new(self.var_iota.next()), data_type));
    }
}

pub enum Type {
    Byte,
    Word,
    DoubleWord,
    QuadWord,
}

pub struct Variable {
    id: usize,
}

impl Variable {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

struct Iota {
    counter: usize,
}

impl Iota {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn next(&mut self) -> usize {
        let count = self.counter;
        self.counter += 1;
        count
    }
}
