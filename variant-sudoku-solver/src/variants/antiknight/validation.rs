use crate::Game;
use crate::game::status::{self, Status};
use super::super::common::utils;

pub fn validate(game: &Game, _in_progress: bool) -> status::Status {
  println!("Validating anti-knight criteria");

  //8 directions to check for:
  // up-left-left, up-up-left, up-up-right, up-right-right
  // down-left-left, down-down-left, down-down-right, down-right-right

  let mut knight_move_cells: Vec<Vec<(usize, usize)>> = Vec::new();

  for i in 0..game.board.len() {
    for j in 0..game.board.len() {
      if i >= 2 && j >= 1 {
        knight_move_cells.push(vec![(i, j), (i - 2, j - 1)]);
      }
      if i >= 1 && j >= 2 {
        knight_move_cells.push(vec![(i, j), (i - 1, j - 2)]);
      }
      if i >= 2 && j < game.board.len() - 1 {
        knight_move_cells.push(vec![(i, j), (i - 2, j + 1)]);
      }
      if i >= 1 && j < game.board.len() - 2 {
        knight_move_cells.push(vec![(i, j), (i - 1, j + 2)]);
      }
    }
  }

  let mut statuses: Vec<Status> = Vec::new();
  for adj in knight_move_cells.iter() {
    let status = utils::check_for_duplicates_in_set(game, adj, "knight move cells");
    if status.0 == false {
      statuses.push(status);
    }
  }

  return status::combine_statuses(statuses);
}