use crate::day_two::game_state::GameState;

#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn get_hand_score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn get_hand_for_result(&self, state: &GameState) -> Hand {
        if *state == GameState::Draw {
            return match self {
                Hand::Paper => Hand::Paper,
                Hand::Rock => Hand::Rock,
                Hand::Scissors => Hand::Scissors,
            };
        }

        if *state == GameState::Win {
            return match self {
                Hand::Paper => Hand::Scissors,
                Hand::Rock => Hand::Paper,
                Hand::Scissors => Hand::Rock,
            };
        }

        return match self {
            Hand::Paper => Hand::Rock,
            Hand::Rock => Hand::Scissors,
            Hand::Scissors => Hand::Paper,
        };
    }
}

impl From<char> for Hand {
    fn from(c: char) -> Self {
        match c {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!("We should never see this in a valid case"),
        }
    }
}
