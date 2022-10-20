use std::cmp::Ordering;

use crate::{
    enums::{GuessResult, NumberOrdering, TurnResult},
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

fn one_turn(user_input: String, rand: i32) -> TurnResult {
    let result = check_numbers(user_input, rand);
    let message = get_message(result);

    println!("{message}");

    match result {
        GuessResult::Correct => TurnResult::Guessed,
        _ => TurnResult::NotGuessed,
    }
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

pub fn game_loop() {
    let rand = gen_rand_num(100);
    loop {
        let user_input = read_number_from_user();
        let turn_result = one_turn(user_input, rand);
        match turn_result {
            TurnResult::Guessed => break,
            TurnResult::NotGuessed => {}
        }
    }
}
