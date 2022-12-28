use super::monkey::{Monkey, MonkeyId};
use std::collections::HashMap;

pub struct Troop {
    // Encoded the MonkeyId as the index of the vector.
    pub monkeys: Vec<Monkey>,
}

impl Troop {

    fn update_monkey(&mut self, monkey_id: usize, relief: &Box<dyn Fn(i64) -> i64>) -> Option<(MonkeyId, i64)> {
        let monkey =  self.monkeys.get_mut(monkey_id)?;
        monkey.take_turn(relief)
    }

    pub fn add_item_to_monkey(&mut self, id: i32, item: i64) -> Option<()> {
        let monkey = self.monkeys.get_mut(id as usize)?;
        monkey.add_item(item);
        Option::None
    }

    fn take_turn_for(&mut self, monkey_id: usize, relief: &Box<dyn Fn(i64) -> i64>) -> Option<i64> {
        let mut items_thrown = 0;
        while let Some((MonkeyId::Id(id), item)) = Troop::update_monkey(self, monkey_id, relief) {
            items_thrown += 1;
            Troop::add_item_to_monkey(self, id, item);
        }
        Option::Some(items_thrown)
    }

    pub fn take_turn(&mut self, relief: &Box<dyn Fn(i64) -> i64>) -> HashMap<MonkeyId, i64> {
        let total_monkeys = self.monkeys.len();
        let mut throws_per_monkey = HashMap::new();

        for i in 0 .. total_monkeys {
            let items_thrown = self.take_turn_for(i, relief).unwrap_or(0);
            throws_per_monkey.insert(MonkeyId::Id(i as i32), items_thrown);
        }

        throws_per_monkey
    }
}
