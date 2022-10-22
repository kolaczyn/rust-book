use crate::{
    enums::{GuessResult, NumberOrdering, TurnResult},
    game::{check_numbers, one_turn},
    game_state::GameState,
};

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
    let game_state = GameState::new();
    let new_game_state = one_turn("21".to_string(), 30, game_state);
    assert_eq!(new_game_state.points, 9);
    assert_eq!(new_game_state.last_turn_result, TurnResult::NotGuessed);
}
