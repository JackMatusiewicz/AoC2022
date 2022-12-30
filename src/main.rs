pub mod core;
pub mod day_eight;
pub mod day_eleven;
pub mod day_five;
pub mod day_four;
pub mod day_nine;
pub mod day_one;
pub mod day_seven;
pub mod day_six;
pub mod day_ten;
pub mod day_three;
pub mod day_twelve;
pub mod day_two;

use crate::core::advent_error::AdventError;
use crate::core::file::read_lines_to_vec;
use crate::day_twelve::solution;

fn main() {
    let lines =
        read_lines_to_vec("/home/jack/Projects/Rust/advent22/resources/day_twelve.txt".to_string());
    println!("Starting up");
    match lines {
        Ok(lines) => {
            println!("Got some lines");
            let answer = solution::calculate_part_two(lines);
            println!("{}", answer);
        }
        _ => (),
    }
}
