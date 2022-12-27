use super::instruction::Instruction;

#[derive(PartialEq, Eq, Clone)]
pub struct Cpu {
    pub register_value: i32,
}

impl Cpu {
    fn add_to_register(&self, v: &i32) -> Cpu {
        Cpu {
            register_value: self.register_value + v,
        }
    }

    pub fn apply_instruction(&self, inst: &Instruction) -> Vec<Cpu> {
        match inst {
            Instruction::Noop => vec![self.clone()],
            Instruction::Add(v) => vec![self.clone(), self.add_to_register(v)],
        }
    }
}
