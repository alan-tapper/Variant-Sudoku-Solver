use crate::Game;
use crate::game::status;
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
    let row_status = check_row(&game, i);
    if row_status.0 == false {
      error_statuses.push(row_status);
    }
  }
  
  //check that all the cols are valid
  for j in 0..9 {
    let col_status = check_col(&game, j);
    if col_status.0 == false {
      error_statuses.push(col_status);
    }
  }
  
  //check that all the boxes are valid
  for box_i in 0..3 {
    for box_j in 0..3 {
      let box_status = check_box(&game, box_i, box_j);
      if box_status.0 == false {
        error_statuses.push(box_status);
      }
    }
  }
  return status::combine_statuses(error_statuses);
}



fn check_row(game: &Game, i: usize) -> status::Status {
  let mut error_statuses: Vec<status::Status> = Vec::new();
  for j1 in 0..9 {
    for j2 in 0..9 {
      if game.board[i][j1] != Game::EMPTY_DIGIT && j1 < j2 && game.board[i][j1] == game.board[i][j2] {
        let mut v: Vec<String> = Vec::new();
        v.push(format!("duplicate {}s in a row at ({}, {}), ({}, {})", game.board[i][j1], i, j1, i, j2).to_string());
        let s = status::Status(false, v);
        error_statuses.push(s);
      }
    }
  }
  return status::combine_statuses(error_statuses);
}

fn check_col(game: &Game, j: usize) -> status::Status {
  let mut error_statuses: Vec<status::Status> = Vec::new();
  for i1 in 0..9 {
    for i2 in 0..9 {
      if game.board[i1][j] != Game::EMPTY_DIGIT && i1 < i2 && game.board[i1][j] == game.board[i2][j] {
        let mut v: Vec<String> = Vec::new();
        v.push(format!("duplicate {}s in a col at ({}, {}), ({}, {})", game.board[i1][j], i1, j, i2, j).to_string());
        let s = status::Status(false, v);
        error_statuses.push(s);
      }
    }
  }
  return status::combine_statuses(error_statuses);
}

fn check_box(game: &Game, box_i: usize, box_j: usize) -> status::Status {
  let mut error_statuses: Vec<status::Status> = Vec::new();
  for i1 in (3 * box_i)..(3 * box_i + 3) {
    for j1 in (3 * box_j)..(3 * box_j + 3) {
      for i2 in (3 * box_i)..(3 * box_i + 3) {
        for j2 in (3 * box_j)..(3 * box_j + 3) {
          if game.board[i1][j1] != Game::EMPTY_DIGIT && (i1 + j1) < (i2 + j2) && game.board[i1][j1] == game.board[i2][j2] {
            let mut v: Vec<String> = Vec::new();
            v.push(format!("duplicate {}s in a box at ({}, {}), ({}, {})", game.board[i1][j1], i1, j1, i2, j2).to_string());
            let s = status::Status(false, v);
            error_statuses.push(s);
          }
        }
      }
    }
  }
  return status::combine_statuses(error_statuses);
}