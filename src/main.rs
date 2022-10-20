use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

// FIXME there's too much functions with side effect (println)
// TODO organize into files
// TODO add points
// TODO add points

enum NumberOrdering {
    TooSmall,
    TooBig,
}

enum GuessResult {
    Correct,
    Incorrect(NumberOrdering),
    Invalid,
}

#[derive(Clone, Copy)]
enum TurnResult {
    Guessed,
    NotGuessed,
}

fn gen_rand_num(max: i32) -> i32 {
    rand::thread_rng().gen_range(1..=max)
}

fn read_num() -> String {
    let mut guessed = String::new();

    io::stdin()
        .read_line(&mut guessed)
        .expect("Something went wrong");

    guessed
}

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

fn print_without_newline(s: String) {
    print!("{s}");
    io::stdout().flush().unwrap();
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

fn game_loop(rand: i32) {
    loop {
        let turn_result = one_turn(rand);
        match turn_result {
            TurnResult::Guessed => break,
            TurnResult::NotGuessed => {}
        }
    }
}

fn main() {
    let rand = gen_rand_num(100);
    game_loop(rand)
}
