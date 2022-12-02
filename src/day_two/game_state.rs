#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub enum GameState {
    Win,
    Loss,
    Draw,
}

impl GameState {
    pub fn get_score(&self) -> i32 {
        match self {
            GameState::Loss => 0,
            GameState::Draw => 3,
            GameState::Win => 6,
        }
    }
}

impl From<char> for GameState {
    fn from(a: char) -> Self {
        match a {
            'X' => GameState::Loss,
            'Y' => GameState::Draw,
            'Z' => GameState::Win,
            _ => panic!("Should not get here with valid input"),
        }
    }
}
