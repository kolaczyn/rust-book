#[test]
fn test_check_numbers() {
    assert!(matches!(
        check_numbers("21".to_string(), 30),
        GuessResult::Incorrect(NumberOrdering::TooSmall)
    ));
    assert!(matches!(
        check_numbers("5".to_string(), 5),
        GuessResult::Correct
    ));
    assert!(matches!(
        check_numbers("50".to_string(), 1),
        GuessResult::Incorrect(NumberOrdering::TooBig)
    ));
}

#[test]
fn test_one_turn() {
    assert!(matches!(
        one_turn("21".to_string(), 30),
        TurnResult::NotGuessed
    ));
    assert!(matches!(
        one_turn("69".to_string(), 69),
        TurnResult::Guessed
    ))
}
