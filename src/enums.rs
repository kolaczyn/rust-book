#[derive(PartialEq, Eq)]
pub enum NumberOrdering {
    TooSmall,
    TooBig,
}

#[derive(PartialEq, Eq)]
pub enum GuessResult {
    Correct,
    Incorrect(NumberOrdering),
    Invalid,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TurnResult {
    Guessed,
    NotGuessed,
}

#[derive(PartialEq, Eq, Clone)]
pub enum GameDifficulty {
    Easy,
    Normal,
    Hard,
}

// Not really an enum, but whatever, I'll fix it later
#[derive(Clone)]
pub struct GameState {
    pub points: i32,
    pub last_turn_result: TurnResult,
    pub difficulty: GameDifficulty,
}
