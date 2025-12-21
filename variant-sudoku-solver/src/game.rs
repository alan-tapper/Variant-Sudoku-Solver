pub mod status;
use status::Status;
use regex::Regex;
use ansi_term::Colour;

pub struct Game {
  pub board: [[char; 9]; 9],
}

impl Game {
  pub const DIGITS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
  pub const EMPTY_DIGIT: char = ' ';

  pub fn format_game_terminal(&self, in_progress: bool, highlight_errors: bool) -> String {
    if highlight_errors {
      println!("highlighting errors");
    }
    let status: Status = self.get_status(in_progress);

    let problematic_cells = self.get_problematic_cells(&status);

    let mut s: String = String::new();
    //top edge of board
    s.push(' ');
    for j in 0..9 {
      if j == 3 || j == 6 {s.push(' ')}
      s.push('-');
      if j != 8 {s.push(' ')}
    };
    s.push('\n');
    //main board
    for i in 0..9 {
      if i == 3 || i == 6 {
        s.push(' ');
        for j in 0..9 {
          if j == 3 || j == 6 {s.push(' ')}
          s.push('-');
          if j != 8 {s.push(' ')}
        };
        s.push('\n')
      }
      s.push('|');
      for j in 0..9 {
        if j == 3 || j == 6 {s.push('|')}
        let is_problematic = problematic_cells.contains(&(i as usize, j as usize));
        s.push_str(&(self.format_cell(self.board[i][j], is_problematic)));
        if j != 8 {s.push(' ')}
      };
      s.push('|');
      if i != 8 {s.push('\n')}
    };
    s.push('\n');
    s.push(' ');
    //bottom edge of board
    for j in 0..9 {
      if j == 3 || j == 6 {s.push(' ')}
      s.push('-');
      if j != 8 {s.push(' ')}
    };
    s.push('\n');
    s.push('\n');
    
    s.push_str(&status.format(true));
    if !in_progress && status.clone().0 {
      s.push_str("\n");
      s.push_str("Puzzle Complete!");
    }
    return s
  }

  fn get_problematic_cells(&self, status: &Status) -> Vec<(usize, usize)> {
    let mut problematic_cells: Vec<(usize, usize)> = Vec::new();
    let re = Regex::new(r"\((\d+),\s*(\d+)\)").unwrap();
    for msg in &status.1 {
      for cap in re.captures_iter(msg) {
        if let (Some(i_str), Some(j_str)) = (cap.get(1), cap.get(2)) {
          if let (Ok(i), Ok(j)) = (i_str.as_str().parse::<usize>(), j_str.as_str().parse::<usize>()) {
            problematic_cells.push((i, j));
          }
        }
      }
    }
    problematic_cells
  }

  fn format_cell(&self, val: char, is_problematic: bool) -> String {
    if !is_problematic {
      return val.to_string();
    } else {
      return Colour::Red.paint(val.to_string()).to_string();
    }
  }

  pub fn get_status(&self, in_progress: bool) -> Status {
    let mut error_statuses: Vec<Status> = Vec::new();
    //check that each individual cell is valid and that all the rows are valid
    for i in 0..9 {
      for j in 0..9 {
        let mut digit_is_valid = false;
        for k in 0..9 {
          if self.board[i][j] == Game::DIGITS[k] {
            digit_is_valid = true;
            break;
          }
          if in_progress && self.board[i][j] == Game::EMPTY_DIGIT {
            digit_is_valid = true;
          }
        }
        if digit_is_valid == false {
          let mut vec: Vec<String> = Vec::new();
          vec.push(format!("invalid digit: {} at ({}, {})", self.board[i][j], i, j));
          error_statuses.push(Status(false, vec));
        }
      }
      let row_status = self.check_row(i);
      if row_status.0 == false {
        error_statuses.push(row_status);
      }
    }
    
    //check that all the cols are valid
    for j in 0..9 {
      let col_status = self.check_col(j);
      if col_status.0 == false {
        error_statuses.push(col_status);
      }
    }
    
    //check that all the boxes are valid
    for box_i in 0..3 {
      for box_j in 0..3 {
        let box_status = self.check_box(box_i, box_j);
        if box_status.0 == false {
          error_statuses.push(box_status);
        }
      }
    }
    return self.combine_statuses(error_statuses);
  }

  fn combine_statuses(&self, error_statuses: Vec<Status>) -> Status {
    if error_statuses.len() != 0 {
      let mut overall_error_status_messages: Vec<String> = Vec::new();
      for es in error_statuses {
        overall_error_status_messages.push(es.format(false));
      }
      return Status(false, overall_error_status_messages);
    };
    return Status(true, Vec::new());
  }

  fn check_row(&self, i: usize) -> Status {
    let mut error_statuses: Vec<Status> = Vec::new();
    for j1 in 0..9 {
      for j2 in 0..9 {
        if self.board[i][j1] != Game::EMPTY_DIGIT && j1 < j2 && self.board[i][j1] == self.board[i][j2] {
          let mut v: Vec<String> = Vec::new();
          v.push(format!("duplicate {}s in a row at ({}, {}), ({}, {})", self.board[i][j1], i, j1, i, j2).to_string());
          let s = Status(false, v);
          error_statuses.push(s);
        }
      }
    }
    return self.combine_statuses(error_statuses);
  }

  fn check_col(&self, j: usize) -> Status {
    let mut error_statuses: Vec<Status> = Vec::new();
    for i1 in 0..9 {
      for i2 in 0..9 {
        if self.board[i1][j] != Game::EMPTY_DIGIT && i1 < i2 && self.board[i1][j] == self.board[i2][j] {
          let mut v: Vec<String> = Vec::new();
          v.push(format!("duplicate {}s in a col at ({}, {}), ({}, {})", self.board[i1][j], i1, j, i2, j).to_string());
          let s = Status(false, v);
          error_statuses.push(s);
        }
      }
    }
    return self.combine_statuses(error_statuses);
  }

  fn check_box(&self, box_i: usize, box_j: usize) -> Status {
    let mut error_statuses: Vec<Status> = Vec::new();
    for i1 in (3 * box_i)..(3 * box_i + 3) {
      for j1 in (3 * box_j)..(3 * box_j + 3) {
        for i2 in (3 * box_i)..(3 * box_i + 3) {
          for j2 in (3 * box_j)..(3 * box_j + 3) {
            if self.board[i1][j1] != Game::EMPTY_DIGIT && (i1 + j1) < (i2 + j2) && self.board[i1][j1] == self.board[i2][j2] {
              let mut v: Vec<String> = Vec::new();
              v.push(format!("duplicate {}s in a box at ({}, {}), ({}, {})", self.board[i1][j1], i1, j1, i2, j2).to_string());
              let s = Status(false, v);
              error_statuses.push(s);
            }
          }
        }
      }
    }
    return self.combine_statuses(error_statuses);
  }
}