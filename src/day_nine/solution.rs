use super::direction::Direction;
use super::rope::Rope;
use std::collections::HashSet;

fn find_path_taken(movements: Vec<Direction>) -> Vec<Rope> {
    let origin = Rope {
        head: (0, 0),
        tail: (0, 0),
    };

    origin.apply_movements(&movements)
}

fn find_unique_tail_positions(data: &Vec<Rope>) -> usize {
    let mut seen = HashSet::new();
    for v in data.iter() {
        seen.insert(v.tail);
    }
    seen.len()
}

fn convert_input(line: String) -> Vec<Direction> {
    let mut split = line.split(' ');
    let dir_str = split.next().unwrap();
    let count = split.next().unwrap().parse::<i32>().unwrap();
    let c = dir_str.chars().collect::<Vec<char>>()[0];
    let dir = Direction::from_char(c);

    let mut dirs = vec![];

    for _ in 0..count {
        dirs.push(dir.clone());
    }

    dirs
}

fn print_path(path: &Vec<Rope>) -> () {
    for pos in path.iter() {
        println!("Head: ({},{}) - Tail: ({}, {})", pos.head.0, pos.head.1, pos.tail.0, pos.tail.1);
    }
}

pub fn calculate_part_one(data: Vec<String>) -> usize {
    let movements: Vec<Direction> = data.into_iter().flat_map(convert_input).collect();

    let path = find_path_taken(movements);

    //print_path(&path);

    find_unique_tail_positions(&path)
}
