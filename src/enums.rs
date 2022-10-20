pub enum NumberOrdering {
    TooSmall,
    TooBig,
}

pub enum GuessResult {
    Correct,
    Incorrect(NumberOrdering),
    Invalid,
}

#[derive(Clone, Copy)]
pub enum TurnResult {
    Guessed,
    NotGuessed,
}
