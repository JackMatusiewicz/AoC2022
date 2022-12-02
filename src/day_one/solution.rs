use std::result::Result;
use std::vec::Vec;

use crate::core::advent_error::AdventError;
use crate::day_one::elf::Elf;
use crate::core::file::read_lines_to_vec;

fn convert_lines_to_elves(lines: Vec<String>) -> Result<Vec<Elf>, AdventError> {
    let mut v = Vec::<i32>::new();
    let mut elves = Vec::<Elf>::new();

    for l in lines.into_iter() {
        if l.is_empty() {
            let elf = Elf::make(v);
            elves.push(elf);
            v = Vec::<i32>::new();
        } else {
            if let Ok(i) = l.parse::<i32>() {
                v.push(i);
            } else {
                return Result::Err(AdventError::OtherError(format!("Invalid number: {}", l)));
            }
        }
    }

    if v.len() > 0 {
        elves.push(Elf::make(v));
    }

    Ok(elves)
}

fn find_largest_stash(elves: &Vec<Elf>) -> i32 {
    elves
        .iter()
        .map(|c| c.sum_of_items)
        .max()
        .unwrap_or(0)
}

pub fn calculate_part_one(path: String) -> Result<i32, AdventError> {
    let lines = read_lines_to_vec(path)?;
    let elves = convert_lines_to_elves(lines)?;
    let largest = find_largest_stash(&elves);
    
    Ok(largest)
}

fn find_largest_three(elves: &Vec<Elf>) -> i32 {
    let mut sums: Vec<i32> =
        elves
            .iter()
            .map(|c| c.sum_of_items)
            .collect();
    sums.sort_by(|a,b| b.cmp(a));

    sums.iter().take(3).sum()
}

pub fn calculate_part_two(path: String) -> Result<i32, AdventError> {
    let lines = read_lines_to_vec(path)?;
    let mut elves = convert_lines_to_elves(lines)?;

    Ok(find_largest_three(&elves))

}
