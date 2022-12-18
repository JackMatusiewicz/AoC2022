use std::collections::VecDeque;

use super::crane_command::CraneCommand;

pub struct Dock(pub Vec<Vec<char>>);

impl Dock {
    pub fn apply_command(&mut self, command: &CraneCommand) {
        let mut to_move = command.count;
        let source_id = (command.source_stack - 1) as usize;
        let sink_id = (command.sink_stack - 1) as usize;

        // This modification is what was needed for part 2, remove it and slap them
        // straight into the sink stack for part one.
        let mut crates_to_move = Vec::new();

        while to_move > 0 && !self.0[source_id].is_empty() {
            to_move -= 1;
            let v = self.0[source_id].pop().unwrap();
            crates_to_move.push(v);
        }

        while crates_to_move.len() > 0 {
            let v = crates_to_move.pop().unwrap();
            self.0[sink_id].push(v);
        }
    }

    pub fn get_tops(&mut self) -> String {
        let mut v = Vec::new();
        for stack in &mut self.0 {
            let top = stack.pop();
            top.map(|c| v.push(c));
        }

        v.into_iter().collect()
    }
}
