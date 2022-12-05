// due to laziness I have skipped writing the parser for the initial state.

use regex::Regex;
use crate::day_five::crane_command::CraneCommand;
use crate::day_five::dock::Dock;

fn get_regex() -> Regex {
    Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap()
}

fn convert_to_command(v: &str) -> Option<CraneCommand> {
    let r = get_regex();

    for cap in r.captures_iter(v) {
        let cc = CraneCommand {
            count: cap[1].parse().unwrap(),
            source_stack: cap[2].parse().unwrap(),
            sink_stack: cap[3].parse().unwrap()
        };

        return Some(cc);
    }
    return None;
}

fn make_dock() -> Dock {
    let docks =
        vec![
            vec!['F', 'C', 'J', 'P', 'H', 'T', 'W'],
            vec!['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'],
            vec!['H', 'P', 'T', 'R'],
            vec!['Z', 'S', 'N', 'P', 'H', 'T'],
            vec!['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'],
            vec!['P', 'M', 'G', 'F', 'W', 'D', 'Z'],
            vec!['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'],
            vec!['N', 'D', 'S'],
            vec!['D', 'Z', 'S', 'F', 'M'],
        ];
    return Dock(docks);
}

pub fn calculate_part_one(data: Vec<String>) -> String {
    let commands: Vec<CraneCommand> =
        data.iter()
            .filter_map(|s| convert_to_command(&s))
            .collect();

    let mut dock = make_dock();

    for c in commands.iter() {
        dock.apply_command(c);
    }

    dock.get_tops()
}