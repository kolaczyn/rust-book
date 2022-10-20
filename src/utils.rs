use std::io::{self, Write};

use rand::Rng;

pub fn gen_rand_num(max: i32) -> i32 {
    rand::thread_rng().gen_range(1..=max)
}

fn print_without_newline(s: String) {
    print!("{s}");
    io::stdout().flush().unwrap();
}

fn read_num() -> String {
    let mut guessed = String::new();

    io::stdin()
        .read_line(&mut guessed)
        .expect("Something went wrong");

    guessed
}

pub fn read_number_from_user() -> String {
    print_without_newline("Input your guess: ".to_owned());
    read_num()
}
