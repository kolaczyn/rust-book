use std::cmp::Ordering;

use crate::{
    enums::{GameDifficulty, GameState, GuessResult, NumberOrdering, TurnResult},
    utils::{gen_rand_num, read_number_from_user},
};

fn check_numbers(guess: String, rand: i32) -> &'static GuessResult {
    if let Ok(num) = guess.trim().parse::<i32>() {
        match num.cmp(&rand) {
            Ordering::Less => &GuessResult::Incorrect(NumberOrdering::TooSmall),
            Ordering::Equal => &GuessResult::Correct,
            Ordering::Greater => &GuessResult::Incorrect(NumberOrdering::TooBig),
        }
    } else {
        &GuessResult::Invalid
    }
}
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

fn get_message(guess_result: &GuessResult) -> String {
    match guess_result {
        GuessResult::Correct => "You did it!".to_string(),
        GuessResult::Incorrect(NumberOrdering::TooBig) => "The correct is too big".to_string(),
        GuessResult::Incorrect(NumberOrdering::TooSmall) => "The correct is too small".to_string(),
        GuessResult::Invalid => "Invalid guess".to_string(),
    }
}

fn one_turn(user_input: String, rand: i32, game_state: GameState) -> GameState {
    let result = check_numbers(user_input, rand);
    let message = get_message(result);

    println!("{message}");

    let (turn_result, points) = match result {
        // TODO add this logic to GameState traits
        GuessResult::Correct => (TurnResult::Guessed, game_state.points),
        _ => (TurnResult::NotGuessed, game_state.points - 1),
    };
    GameState {
        points,
        last_turn_result: turn_result,
        difficulty: GameDifficulty::Easy,
    }
}

#[test]
fn test_one_turn() {
    let game_state = GameState {
        points: 10,
        last_turn_result: TurnResult::NotGuessed,
        difficulty: GameDifficulty::Easy,
    };
    let new_game_state = one_turn("21".to_string(), 30, game_state);
    assert_eq!(new_game_state.points, 9);
    assert_eq!(new_game_state.last_turn_result, TurnResult::NotGuessed);
}

pub fn game_loop() {
    let rand = gen_rand_num(100);
    let initial_game_state = GameState {
        difficulty: GameDifficulty::Easy,
        // technically true, but doesn't make much sense
        last_turn_result: TurnResult::NotGuessed,
        points: 10,
    };
    // Workaround, because I'm using a loop to keep doing stuff
    let mut curr_game_state = initial_game_state.clone();
    loop {
        let user_input = read_number_from_user();
        curr_game_state = one_turn(user_input, rand, curr_game_state);
        match curr_game_state.last_turn_result {
            TurnResult::Guessed => break,
            TurnResult::NotGuessed => {}
        }
    }
    let points = curr_game_state.points;
    println!("You have {points} points")
}
