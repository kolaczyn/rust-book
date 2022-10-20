use rand::Rng;
use std::cmp::Ordering;
use std::io;

// FIXME there's too much functions with side effect (println)

fn gen_rand_num() -> i32 {
    rand::thread_rng().gen_range(1..=3)
}

fn read_num() -> String {
    let mut guessed = String::new();

    io::stdin()
        .read_line(&mut guessed)
        .expect("Something went wrong");

    guessed
}

enum NumberOrdering {
    TooSmall,
    TooBig,
}

enum GuessResult {
    Correct,
    Incorrect(NumberOrdering),
    Invalid,
}

fn get_message(guess_result: &GuessResult) -> String {
    match guess_result {
        GuessResult::Correct => "You did it!".to_string(),
        GuessResult::Incorrect(NumberOrdering::TooBig) => "The correct is too big".to_string(),
        GuessResult::Incorrect(NumberOrdering::TooSmall) => "The correct is too small".to_string(),
        GuessResult::Invalid => "Invalid guess".to_string(),
    }
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

#[derive(Clone, Copy)]
enum TurnResult {
    Guessed,
    NotGuessed,
}

fn one_turn(rand: i32) -> TurnResult {
    println!("Input your guess:");
    let guess = read_num();

    let result = check_numbers(guess, rand);
    let message = get_message(result);

    println!("{message}");

    match result {
        GuessResult::Correct => TurnResult::Guessed,
        _ => TurnResult::NotGuessed,
    }
}

fn main() {
    let rand = gen_rand_num();
    println!("{rand}");
    one_turn(rand);
}
