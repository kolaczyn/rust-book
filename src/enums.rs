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

impl GuessResult {
    pub fn get_message(guess_result: &GuessResult) -> &str {
        match guess_result {
            GuessResult::Correct => "You did it!",
            GuessResult::Incorrect(NumberOrdering::TooBig) => "The correct is too big",
            GuessResult::Incorrect(NumberOrdering::TooSmall) => "The correct is too small",
            GuessResult::Invalid => "Invalid guess",
        }
    }
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
