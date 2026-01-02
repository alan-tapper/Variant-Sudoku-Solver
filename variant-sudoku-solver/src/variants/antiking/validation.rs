use crate::Game;
use crate::game::status::{self, Status};
use super::super::common::utils;

pub fn validate(game: &Game, _in_progress: bool) -> status::Status {
  println!("Validating anti-king criteria");

  //4 directions to check for: down-left, down, down-right, right

  let mut adjacent_cells: Vec<Vec<(usize, usize)>> = Vec::new();

  for i in 0..game.board.len() {
    for j in 0..game.board.len() {
      if i != game.board.len() - 1 {
        if j != 0 {
          adjacent_cells.push(vec![(i, j), (i + 1, j - 1)]);
        }

        adjacent_cells.push(vec![(i, j), (i + 1, j)]);

        if j != game.board.len() - 1 {
          adjacent_cells.push(vec![(i, j), (i + 1, j + 1)]);
        }
      }
      if j != game.board.len() - 1 {
        adjacent_cells.push(vec![(i, j), (i, j + 1)]);
      }
    }
  }

  let mut statuses: Vec<Status> = Vec::new();
  for adj in adjacent_cells.iter() {
    let status = utils::check_for_duplicates_in_set(game, adj, "adjacent cells");
    if status.0 == false {
      statuses.push(status);
    }
  }

  return status::combine_statuses(statuses);
}