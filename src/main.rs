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
    // i32 is the correct ans
    Incorrect(i32),
    // String is the reason
    Invalid(String),
}

fn get_message(guess_result: GuessResult) -> String {
    match guess_result {
        GuessResult::Correct => "You did it!".to_string(),
        GuessResult::Incorrect(correct) => format!("The correct answer was {correct}"),
        GuessResult::Invalid(reason) => reason,
    }
}

fn check_numbers(guess: String, rand: i32) -> GuessResult {
    if let Ok(num) = guess.trim().parse::<i32>() {
        if num == rand {
            return GuessResult::Correct;
        }
        return GuessResult::Incorrect(rand);
    }
    GuessResult::Invalid("Not a valid number".to_string())
}

fn one_turn(rand: i32) {
    println!("Input your guess:");
    let guess = read_num();

    let result = check_numbers(guess, rand);
    let message = get_message(result);

    println!("{message}");
}

fn main() {
    let rand = gen_rand_num();
    one_turn(rand);
}
