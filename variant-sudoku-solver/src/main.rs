mod game;
use game::Game;
use std::env;

mod board_parser;
mod variants;

fn main() {
    let args: Vec<String> = env::args().collect();
    let game: Game = board_parser::game_from_sample_board(&args[1], &args[2]);
    println!();
    println!("{}", game.render_game_terminal(true, false));
}
