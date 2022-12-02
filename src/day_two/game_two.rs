use crate::day_two::game_state::GameState;
use crate::day_two::hand::Hand;

pub struct GameTwo {
    pub opposition_hand: Hand,
    pub desired_outcome: GameState,
}

impl GameTwo {
    pub fn get_score(&self) -> i32 {
        let my_hand_score = self
            .opposition_hand
            .get_hand_for_result(&self.desired_outcome)
            .get_hand_score();

        my_hand_score + self.desired_outcome.get_score()
    }
}
