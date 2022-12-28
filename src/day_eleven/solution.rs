use std::collections::{HashMap, VecDeque};

use super::troop::Troop;
use super::monkey::{Monkey, MonkeyId};

fn make_troop() -> Troop {
    let monkeys =
        vec![
            Monkey {
                id: MonkeyId::Id(0),
                items: VecDeque::from([98, 97, 98, 55, 56, 72]),
                update: Box::new(|old| old * 13),
                test_divisible_by: 11,
                monkey_for_true: MonkeyId::Id(4),
                monkey_for_false: MonkeyId::Id(7)
            },
            Monkey {
                id: MonkeyId::Id(1),
                items: VecDeque::from([73, 99, 55, 54, 88, 50, 55]),
                update: Box::new(|old| old + 4),
                test_divisible_by: 17,
                monkey_for_true: MonkeyId::Id(2),
                monkey_for_false: MonkeyId::Id(6)
            },
            Monkey {
                id: MonkeyId::Id(2),
                items: VecDeque::from([67, 98]),
                update: Box::new(|old| old * 11),
                test_divisible_by: 5,
                monkey_for_true: MonkeyId::Id(6),
                monkey_for_false: MonkeyId::Id(5)
            },
            Monkey {
                id: MonkeyId::Id(3),
                items: VecDeque::from([82, 91, 92, 53, 99]),
                update: Box::new(|old| old + 8),
                test_divisible_by: 13,
                monkey_for_true: MonkeyId::Id(1),
                monkey_for_false: MonkeyId::Id(2)
            },
            Monkey {
                id: MonkeyId::Id(4),
                items: VecDeque::from([52, 62, 94, 96, 52, 87, 53, 60]),
                update: Box::new(|old| old * old),
                test_divisible_by: 19,
                monkey_for_true: MonkeyId::Id(3),
                monkey_for_false: MonkeyId::Id(1)
            },
            Monkey {
                id: MonkeyId::Id(5),
                items: VecDeque::from([94, 80, 84, 79]),
                update: Box::new(|old| old + 5),
                test_divisible_by: 2,
                monkey_for_true: MonkeyId::Id(7),
                monkey_for_false: MonkeyId::Id(0)
            },
            Monkey {
                id: MonkeyId::Id(6),
                items: VecDeque::from([89]),
                update: Box::new(|old| old + 1),
                test_divisible_by: 3,
                monkey_for_true: MonkeyId::Id(0),
                monkey_for_false: MonkeyId::Id(5)
            },
            Monkey {
                id: MonkeyId::Id(7),
                items: VecDeque::from([70, 59, 63]),
                update: Box::new(|old| old + 3),
                test_divisible_by: 7,
                monkey_for_true: MonkeyId::Id(4),
                monkey_for_false: MonkeyId::Id(3)
            },
            
        ];

    Troop {
        monkeys
    }
}

fn make_example_troop() -> Troop {
    let monkeys =
        vec![
            Monkey {
                id: MonkeyId::Id(0),
                items: VecDeque::from([79, 98]),
                update: Box::new(|old| old * 19),
                test_divisible_by: 23,
                monkey_for_true: MonkeyId::Id(2),
                monkey_for_false: MonkeyId::Id(3)
            },
            Monkey {
                id: MonkeyId::Id(1),
                items: VecDeque::from([54, 65, 75, 74]),
                update: Box::new(|old| old + 6),
                test_divisible_by: 19,
                monkey_for_true: MonkeyId::Id(2),
                monkey_for_false: MonkeyId::Id(0)
            },
            Monkey {
                id: MonkeyId::Id(2),
                items: VecDeque::from([79, 60, 97]),
                update: Box::new(|old| old * old),
                test_divisible_by: 13,
                monkey_for_true: MonkeyId::Id(1),
                monkey_for_false: MonkeyId::Id(3)
            },
            Monkey {
                id: MonkeyId::Id(3),
                items: VecDeque::from([74]),
                update: Box::new(|old| old + 3),
                test_divisible_by: 17,
                monkey_for_true: MonkeyId::Id(0),
                monkey_for_false: MonkeyId::Id(1)
            }
        ];

    Troop {
        monkeys
    }
}

pub fn calculate_part_one(data: Vec<String>) -> i64 {
    let mut troop = make_troop();
    let mut monkey_inspection_count: HashMap<MonkeyId, i64> = HashMap::new();
    let relief: Box<dyn Fn(i64) -> i64> = Box::new(|v| v / 3);
    for _ in 0 .. 20 {
        let turn_changes = troop.take_turn(&relief);
        for (monkey_id, count) in turn_changes {
            let v = monkey_inspection_count.entry(monkey_id).or_insert(0);
            *v += count;
        }
    }
    let mut counts: Vec<i64> = monkey_inspection_count.into_values().collect();
    counts.sort_by(|a,b| b.cmp(a));
    counts.iter().take(2).fold(1 as i64, |s,a| a * s)
}

pub fn calculate_part_two(data: Vec<String>) -> i64 {
    let mut troop = make_troop();
    let mut monkey_inspection_count: HashMap<MonkeyId, i64> = HashMap::new();

    /*
    We want to cap the values (as we want to avoid overflow) but we also don't want to affect the calculations
    of the monkey throw. In order to do that we can calculate the product of all of the divisors to ensure
    that the modulo arithmetic calculations still hold.
     */
    let divisble_product =
        troop.monkeys.iter().map(|m| m.test_divisible_by).fold(1, |a,s| a * s);
    
    let relief: Box<dyn Fn(i64) -> i64> = Box::new(move |v| v % divisble_product);

    for _ in 0 .. 10000 {
        let turn_changes = troop.take_turn(&relief);
        for (monkey_id, count) in turn_changes {
            let v = monkey_inspection_count.entry(monkey_id).or_insert(0);
            *v += count;
        }
    }
    let mut counts: Vec<i64> = monkey_inspection_count.into_values().collect();
    counts.sort_by(|a,b| b.cmp(a));
    for v in counts.iter() {
        println!("{v}");
    }
    counts.iter().take(2).fold(1 as i64, |s,a| (*a as i64) * s)
}