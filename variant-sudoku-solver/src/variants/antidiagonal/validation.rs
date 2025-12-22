use crate::Game;
use crate::game::status::{self};
use super::super::common::utils;

pub fn validate(game: &Game, _in_progress: bool) -> status::Status {
  println!("Validating anti-diagonal criteria");

  //positive diagonal
  let mut positive_diagonal: Vec<(usize, usize)> = Vec::new();
  let mut negative_diagonal: Vec<(usize, usize)> = Vec::new();
  for i in 0..game.board.len() {
    positive_diagonal.push((i, i));
    negative_diagonal.push((i, game.board.len() - i - 1));
  }

  let positive_diagonal_status = utils::check_for_duplicates(game, &positive_diagonal, "the positive diagonal");
  let negative_diagonal_status = utils::check_for_duplicates(game, &negative_diagonal, "the negative diagonal");

  return status::combine_statuses(vec![positive_diagonal_status, negative_diagonal_status]);
}