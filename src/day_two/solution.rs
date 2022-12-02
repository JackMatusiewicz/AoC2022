use std::result::Result;

use crate::core::advent_error::AdventError;
use crate::core::file::read_lines_to_vec;
use crate::day_two::game::Game;
use crate::day_two::game_state::GameState;
use crate::day_two::game_two::GameTwo;
use crate::day_two::hand::Hand;

pub fn calculate_part_one(path: String) -> Result<i32, AdventError> {
    let lines = read_lines_to_vec(path)?;

    let score: i32 = lines
        .into_iter()
        .map(|a| -> Vec<char> { a.chars().collect() })
        .map(|c| Game {
            my_play: Hand::from(c[2]),
            opposition_play: Hand::from(c[0]),
        })
        .map(|game| game.get_game_score())
        .sum();
    Ok(score)
}

pub fn calculate_part_two(path: String) -> Result<i32, AdventError> {
    let lines = read_lines_to_vec(path)?;

    let score: i32 = lines
        .into_iter()
        .map(|a| -> Vec<char> { a.chars().collect() })
        .map(|c| GameTwo {
            desired_outcome: GameState::from(c[2]),
            opposition_hand: Hand::from(c[0]),
        })
        .map(|game| game.get_score())
        .sum();
    Ok(score)
}
