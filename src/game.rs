use std::cmp::Ordering;

use crate::enums::{GuessResult, NumberOrdering, TurnResult};
use crate::game_state::GameState;
use crate::utils::{gen_rand_num, read_number_from_user};

pub fn check_numbers(guess: String, rand: i32) -> &'static GuessResult {
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

pub fn one_turn(user_input: String, rand: i32, game_state: GameState) -> GameState {
    let result = check_numbers(user_input, rand);
    let message = GuessResult::get_message(result);

    println!("{message}");

    let turn_result = match result {
        GuessResult::Correct => TurnResult::Guessed,
        _ => TurnResult::NotGuessed,
    };
    game_state.new_turn(turn_result)
}

pub fn game_loop() {
    let rand = gen_rand_num(100);
    let initial_game_state = GameState::new();

    // Workaround, because I'm using a loop to keep doing stuff
    let mut curr_game_state = initial_game_state;
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
