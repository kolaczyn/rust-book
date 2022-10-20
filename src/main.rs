use crate::game::game_loop;

pub mod enums;
pub mod game;
pub mod utils;

// FIXME there's too much functions with side effect (println)
// TODO add tests
// TODO add points

fn main() {
    game_loop()
}
