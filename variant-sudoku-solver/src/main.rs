use game::Game;

pub mod game;

fn setup_game() -> Game {
  Game{
    board: [['1', '2', '3', '4', '5', '6', '7', '8', '9']; 9],
    digits: ['1', '2', '3', '4', '5', '6', '7', '8', '9']
  }
}

fn main() {
    let game: Game = setup_game();
    println!();
    println!("{}", game.format_game());
    println!();
    let game_status = game.is_valid();
    println!("{}", game_status.format());
}
