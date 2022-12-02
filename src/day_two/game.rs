use crate::day_two::{game_state::GameState, hand::Hand};

pub struct Game {
    pub my_play: Hand,
    pub opposition_play: Hand,
}

impl Game {
    pub fn get_result(&self) -> GameState {
        if self.my_play == self.opposition_play {
            return GameState::Draw;
        }

        match (&self.my_play, &self.opposition_play) {
            (Hand::Paper, Hand::Rock) => GameState::Win,
            (Hand::Rock, Hand::Scissors) => GameState::Win,
            (Hand::Scissors, Hand::Paper) => GameState::Win,
            _ => GameState::Loss,
        }
    }

    pub fn get_game_score(&self) -> i32 {
        self.my_play.get_hand_score() + self.get_result().get_score()
    }
}
