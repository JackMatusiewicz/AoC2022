use std::collections::VecDeque;

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub enum MonkeyId {
    Id(i32),
}

pub struct Monkey {
    pub id: MonkeyId,
    pub items: VecDeque<i64>,
    pub update: Box<dyn Fn(i64) -> i64>,
    pub test_divisible_by: i64,
    pub monkey_for_true: MonkeyId,
    pub monkey_for_false: MonkeyId
}

impl Monkey {
    fn inspect_next_item(&mut self, relief: &Box<dyn Fn(i64) -> i64>) -> Option<i64> {
        self.items
            .pop_front()
            .map(|v| (self.update)(v))
             .map(|v| relief(v))
    }

    fn find_monkey_to_throw_to(&self, v: i64) -> (MonkeyId, i64) {
        match v % self.test_divisible_by {
            0 => (self.monkey_for_true.clone(), v),
            _ => (self.monkey_for_false.clone(), v),
        }
    }

    pub fn take_turn(&mut self, relief: &Box<dyn Fn(i64) -> i64>) -> Option<(MonkeyId, i64)> {
        self.inspect_next_item(relief)
            .map(|v| self.find_monkey_to_throw_to(v))
    }

    pub fn add_item(&mut self, item: i64) -> () {
        self.items.push_back(item)
    }
}
