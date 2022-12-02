#[derive(PartialEq, Eq, Debug)]
pub struct Elf {
    items: Vec<i32>,
    pub sum_of_items: i32,
}

impl Elf {
    pub fn make(items: Vec<i32>) -> Elf {
        let sum: i32 = items.iter().sum();

        Elf {
            items,
            sum_of_items: sum,
        }
    }
}
