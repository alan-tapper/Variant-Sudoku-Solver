mod game;
use game::Game;
use std::env;

mod local_board_importer;
mod variants;

fn main() {
    let args: Vec<String> = env::args().collect();
    let game: Game = local_board_importer::game_from_sample_board(&args[1], &args[2]);
    println!();
    println!("{}", game.render_game_terminal(true, false));
}
