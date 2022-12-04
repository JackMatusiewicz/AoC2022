pub mod core;
pub mod day_one;
pub mod day_two;
pub mod day_three;
pub mod day_four;

use crate::core::advent_error::AdventError;
use crate::core::file::read_lines_to_vec;
use crate::day_four::solution;


fn main() {
    let lines = read_lines_to_vec("".to_string());

    match lines {
        Ok(lines) => {
            let answer = solution::calculate_part_two(lines);
            println!("{}", answer);
        },
        _ => ()
    }
}
