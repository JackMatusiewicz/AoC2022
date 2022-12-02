pub mod core;
pub mod day_one;
pub mod day_two;

use crate::core::advent_error::AdventError;
use day_two::solution;

fn main() {
    let answer = solution::calculate_part_two("".to_string());

    match answer {
        Ok(v) => {
            println!("The answer is: {}", v)
        }
        Err(AdventError::IoError(ioErr)) => {
            println!("IO error: {}", ioErr);
        }
        Err(AdventError::OtherError(s)) => {
            println!("Other error: {}", s);
        }
    }
}
