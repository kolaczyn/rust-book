use crate::enums::{GameDifficulty, TurnResult};

#[derive(Clone)]
pub struct GameState {
    pub points: i32,
    pub last_turn_result: TurnResult,
    pub difficulty: GameDifficulty,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            points: 10,
            last_turn_result: TurnResult::NotGuessed,
            difficulty: GameDifficulty::Easy,
        }
    }
    pub fn new_turn(&self, turn_result: TurnResult) -> Self {
        match turn_result {
            TurnResult::Guessed => GameState {
                points: self.points,
                last_turn_result: TurnResult::Guessed,
                difficulty: self.difficulty.clone(),
            },
            TurnResult::NotGuessed => GameState {
                points: self.points - 1,
                last_turn_result: TurnResult::NotGuessed,
                difficulty: self.difficulty.clone(),
            },
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
