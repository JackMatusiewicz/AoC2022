pub mod core;
pub mod day_one;

use day_one::solution;
use crate::core::advent_error::AdventError;

fn main() {
    let answer = solution::calculate_part_two("".to_string());

    match answer {
        Ok(v) => {
            println!("The answer is: {}", v)
        },
        Err(AdventError::IoError(ioErr)) => {
            println!("IO error: {}", ioErr);
        },
        Err(AdventError::OtherError(s)) => {
            println!("Other error: {}", s);
        }
    }
}
