mod game;
use game::Game;
mod board_parser;

fn main() {
    let game: Game = board_parser::game_from_sample_board("standard", "01");
    println!();
    println!("{}", game.format_game());
    println!();
    let game_status = game.is_valid(true);
    eprintln!("{}", game_status.format());
}
