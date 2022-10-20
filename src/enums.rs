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
    pub fn get_message(guess_result: &GuessResult) -> String {
        match guess_result {
            GuessResult::Correct => "You did it!".to_string(),
            GuessResult::Incorrect(NumberOrdering::TooBig) => "The correct is too big".to_string(),
            GuessResult::Incorrect(NumberOrdering::TooSmall) => {
                "The correct is too small".to_string()
            }
            GuessResult::Invalid => "Invalid guess".to_string(),
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
