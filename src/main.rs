use crate::game::game_loop;

pub mod enums;
pub mod game;
pub mod utils;

// FIXME there's too much functions with side effect (println)
// TODO add points
// TODO add name as options
// FIXME the one_turn function can recieve a GameState where last_turn is NotGuesses, which doesn't make sense

fn main() {
    game_loop()
}
