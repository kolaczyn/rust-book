use std::io;

use rand::Rng;

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

enum GuessResult {
    Correct,
    Incorrect,
    Invalid,
}

fn check_numbers(guess: String, rand: i32) -> GuessResult {
    if let Ok(num) = guess.trim().parse::<i32>() {
        if num == rand {
            return GuessResult::Correct;
        }
        return GuessResult::Incorrect;
    }
    GuessResult::Invalid
}

fn main() {
    let rand = gen_rand_num();
    println!("Input your guess:");
    let guess = read_num();

    let message: String = match check_numbers(guess, rand) {
        GuessResult::Correct => "Bravo".to_string(),
        _ => format!("Unbravo, the answer was {rand}"),
    };

    println!("{message}")
}
