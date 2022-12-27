#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    Noop,
    Add(i32),
}

impl Instruction {
    pub fn parse(v: String) -> Instruction {
        match v.as_str() {
            "noop" => Instruction::Noop,
            _ => {
                let mut sp = v.split(' ');
                sp.next().unwrap();
                let value = sp.next().unwrap().parse::<i32>().unwrap();
                Instruction::Add(value)
            }
        }
    }
}
