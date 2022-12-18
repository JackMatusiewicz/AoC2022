use std::collections::{HashMap, VecDeque};

pub fn calculate(data: String) -> Option<i32> {
    // switch between 4 and 14 for part one and two.
    let distinct_count = 14;
    let mut seen_map = HashMap::<char, i32>::new();
    let mut seen_deque = VecDeque::<char>::new();

    let mut i = 0;
    for c in data.chars() {
        i += 1;
        if seen_deque.len() >= distinct_count {
            let removed = seen_deque.pop_front()?;
            let count = seen_map.entry(removed).or_insert(0);
            *count -= 1;
            if *count == 0 {
                seen_map.remove(&removed);
            }
        }
        seen_deque.push_back(c);
        *(seen_map.entry(c).or_insert(0)) += 1;

        if seen_deque.len() == distinct_count {
            if seen_map.keys().len() == distinct_count {
                return Some(i);
            }
        }
    }

    return None;
}
