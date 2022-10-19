use std::io;

use rand::Rng;

fn main() {
    let mut guessed = String::new();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guessed)
        .expect("Something went wrong");

    match guessed.trim().parse::<i32>() {
        Ok(num) => {
            println!("You entered a vaild number {num}");
            if num == secret_number {
                println!("You guessed correctly");
                return;
            }
            println!("You didn't guess the number, it was {secret_number}")
        }
        Err(_) => {
            println!("You didn't enter a valid number")
        }
    }
}
