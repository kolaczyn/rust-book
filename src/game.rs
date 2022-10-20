use std::cmp::Ordering;

use crate::{
    enums::{GuessResult, NumberOrdering, TurnResult},
    utils::{gen_rand_num, print_without_newline, read_num},
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

fn get_message(guess_result: &GuessResult) -> String {
    match guess_result {
        GuessResult::Correct => "You did it!".to_string(),
        GuessResult::Incorrect(NumberOrdering::TooBig) => "The correct is too big".to_string(),
        GuessResult::Incorrect(NumberOrdering::TooSmall) => "The correct is too small".to_string(),
        GuessResult::Invalid => "Invalid guess".to_string(),
    }
}

fn one_turn(rand: i32) -> TurnResult {
    print_without_newline("Input your guess: ".to_owned());

    let guess = read_num();

    let result = check_numbers(guess, rand);
    let message = get_message(result);

    println!("{message}");

    match result {
        GuessResult::Correct => TurnResult::Guessed,
        _ => TurnResult::NotGuessed,
    }
}

pub fn game_loop() {
    let rand = gen_rand_num(100);
    loop {
        let turn_result = one_turn(rand);
        match turn_result {
            TurnResult::Guessed => break,
            TurnResult::NotGuessed => {}
        }
    }
}
