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
