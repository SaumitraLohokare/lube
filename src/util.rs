use std::collections::{HashMap, HashSet};

use crate::{
    arm64::{self, Register},
    ir,
};

#[derive(Clone, Copy)]
pub(crate) struct Iota {
    counter: usize,
}

impl Iota {
    pub(crate) fn new() -> Self {
        Self { counter: 0 }
    }

    pub(crate) fn next(&mut self) -> usize {
        let count = self.counter;
        self.counter += 1;
        count
    }
}

/// Register allocator using graph coloring algorithm
pub(crate) struct RegisterAllocator {
    edges: HashMap<ir::Temporary, Vec<ir::Temporary>>,
}

impl RegisterAllocator {
    pub(crate) fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: ir::Temporary, to: &HashSet<ir::Temporary>) {
        self.edges.entry(from).or_default();

        for node in to {
            if !self.edges.contains_key(node) {
                self.edges.insert(*node, vec![from]);
            } else {
                self.edges.get_mut(node).unwrap().push(from);
            }

            self.edges.get_mut(&from).unwrap().push(*node);
        }
    }

    fn generate_edges(&mut self, ir: &[ir::Instruction]) -> HashMap<ir::Temporary, arm64::Register> {
        let mut alive_set = HashSet::new();
        let mut restricted_regs = HashMap::new();

        for inst in ir.iter().rev() {
            match inst {
                ir::Instruction::Set { dest, .. } => {
                    alive_set.remove(dest);
                    self.add_edge(*dest, &alive_set);
                }
                ir::Instruction::Return { src } => {
                    if let Some(src) = src {
                        alive_set.insert(*src);
                        restricted_regs.insert(*src, arm64::Register::r0(src.size()));
                    }
                }
                ir::Instruction::Load { dest, .. } => {
                    alive_set.remove(dest);
                    self.add_edge(*dest, &alive_set);
                }
                ir::Instruction::Add { dest, src_1, src_2 } => {
                    alive_set.remove(dest);
                    alive_set.insert(*src_1);
                    alive_set.insert(*src_2);
                    self.add_edge(*dest, &alive_set);
                }
                ir::Instruction::Store { src, .. } => {
                    alive_set.insert(*src);
                }
                ir::Instruction::Call { args, .. } => {
                    for (arg_num, arg) in args.iter().enumerate() {
                        alive_set.insert(*arg);
                        if let Some(arg_reg) = arm64::arg_register(arg_num as u8, arg.size()) {
                            restricted_regs.insert(*arg, arg_reg);
                        }
                    }
                }
                ir::Instruction::CallResult { dest } => {
                    alive_set.remove(dest);
                    self.add_edge(*dest, &alive_set);
                }
                ir::Instruction::LoadAddr { dest, .. } => {
                    alive_set.remove(dest);
                    self.add_edge(*dest, &alive_set);
                }
            }
        }

        restricted_regs
    }

    fn allocate_registers(
        self,
        regs: Vec<arm64::RegisterNumber>,
        restricted_regs: HashMap<ir::Temporary, arm64::Register>,
    ) -> HashMap<ir::Temporary, arm64::Register> {
        let mut reg_map: HashMap<ir::Temporary, arm64::Register> = HashMap::new();

        for (tmp, connected_tmps) in self.edges {
            if let Some(restricted_reg) = restricted_regs.get(&tmp) {
                // Check if we can assign the restricted register
                let mut can_assign = true;
                for connected_tmp in &connected_tmps {
                    if let Some(allocated_reg) = reg_map.get(connected_tmp) {
                        if allocated_reg.number() == restricted_reg.number() {
                            can_assign = false;
                            break;
                        }
                    }
                }

                // If we can assign, then assign and move on
                if can_assign {
                    reg_map.insert(tmp, *restricted_reg);
                    continue;
                }
            }

            'outer: for reg in &regs {
                for connected_tmp in &connected_tmps {
                    if let Some(allocated_reg) = reg_map.get(connected_tmp) {
                        if allocated_reg.number() == *reg {
                            continue 'outer;
                        }
                    }
                }

                reg_map.insert(tmp, Register::new(*reg, tmp.size()));
                break;
            }

            if !reg_map.contains_key(&tmp) {
                panic!("Failed to allocate registers");
            }
        }

        reg_map
    }

    pub(crate) fn allocate(
        mut self,
        ir: &[ir::Instruction],
        regs: Vec<arm64::RegisterNumber>,
    ) -> HashMap<ir::Temporary, arm64::Register> {
        let restricted_regs = self.generate_edges(ir);

        self.allocate_registers(regs, restricted_regs)
    }
}
