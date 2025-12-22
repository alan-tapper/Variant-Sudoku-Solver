use crate::Game;
use crate::game::status;
use super::super::common::utils;

pub fn validate(game: &Game, in_progress: bool) -> status::Status {
  let mut error_statuses: Vec<status::Status> = Vec::new();
  //check that each individual cell is valid and that all the rows are valid
  for i in 0..9 {
    for j in 0..9 {
      let mut digit_is_valid = false;
      for k in 0..9 {
        if game.board[i][j] == Game::DIGITS[k] {
          digit_is_valid = true;
          break;
        }
        if in_progress && game.board[i][j] == Game::EMPTY_DIGIT {
          digit_is_valid = true;
        }
      }
      if digit_is_valid == false {
        let mut vec: Vec<String> = Vec::new();
        vec.push(format!("invalid digit: {} at ({}, {})", game.board[i][j], i, j));
        error_statuses.push(status::Status(false, vec));
      }
    }
    let row_status = validate_row(&game, i);
    if row_status.0 == false {
      error_statuses.push(row_status);
    }
  }
  
  //check that all the cols are valid
  for j in 0..9 {
    let col_status = validate_column(&game, j);
    if col_status.0 == false {
      error_statuses.push(col_status);
    }
  }
  
  //check that all the boxes are valid
  for box_i in 0..3 {
    for box_j in 0..3 {
      let box_status = validate_box(&game, box_i, box_j);
      if box_status.0 == false {
        error_statuses.push(box_status);
      }
    }
  }
  return status::combine_statuses(error_statuses);
}

fn validate_row(game: &Game, i: usize) -> status::Status {
  let mut statuses: Vec<status::Status> = Vec::new();
  let mut row: Vec<(usize, usize)> = Vec::new();
  for j in 0..9 {
    row.push((i, j));
  }
  statuses.push(utils::check_for_duplicates(game, &row, &format!("row {}", i)));
  return status::combine_statuses(statuses);
}

fn validate_column(game: &Game, j: usize) -> status::Status {
  let mut statuses: Vec<status::Status> = Vec::new();
  let mut col: Vec<(usize, usize)> = Vec::new();
  for i in 0..9 {
    col.push((i, j));
  }
  statuses.push(utils::check_for_duplicates(game, &col, &format!("column {}", j)));
  return status::combine_statuses(statuses);
}

fn validate_box(game: &Game, box_i: usize, box_j: usize) -> status::Status {
  let mut statuses: Vec<status::Status> = Vec::new();
  let mut board_box: Vec<(usize, usize)> = Vec::new();
  for i1 in (3 * box_i)..(3 * box_i + 3) {
    for j1 in (3 * box_j)..(3 * box_j + 3) {
      for i2 in (3 * box_i)..(3 * box_i + 3) {
        for j2 in (3 * box_j)..(3 * box_j + 3) {
          if i1 == i2 && j1 == j2 {
            board_box.push((i1, j1));
          }
        }
      }
    }
  }
  statuses.push(utils::check_for_duplicates(game, &board_box, &format!("box ({}, {})", box_i, box_j)));
  return status::combine_statuses(statuses);
}