use status::Status;

pub mod status;

pub struct Game {
  pub board: [[char; 9]; 9],
}

impl Game {
  pub const DIGITS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
  pub const EMPTY_DIGIT: char = ' ';

  pub fn format_game(&self) -> String {
    let mut s: String = String::new();
    s.push(' ');
    for j in 0..9 {
      if j == 3 || j == 6 {s.push(' ')}
      s.push('-');
      if j != 8 {s.push(' ')}
    };
    s.push('\n');
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
        s.push(self.board[i][j]);
        if j != 8 {s.push(' ')}
      };
      s.push('|');
      if i != 8 {s.push('\n')}
    };
    s.push('\n');
    s.push(' ');
    for j in 0..9 {
      if j == 3 || j == 6 {s.push(' ')}
      s.push('-');
      if j != 8 {s.push(' ')}
    };
    s
  }

  pub fn is_valid(&self, in_progress: bool) -> Status {
    let mut error_statuses: Vec<Status> = Vec::new();
    //check that each individual cell is valid and that all the rows are valid
    for i in 0..9 {
      for j in 0..9 {
        let mut v = false;
        for k in 0..9 {
          if self.board[i][j] == Game::DIGITS[k] {
            v = true;
            break;
          }
          if in_progress && self.board[i][j] == Game::EMPTY_DIGIT {
            v = true;
          }
        }
        if v == false {
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
        overall_error_status_messages.push(es.1.concat());
        overall_error_status_messages.push("\n".to_string());
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
          v.push(format!("duplicate {}s in a row at ({}, {}), ({}, {})", self.board[i1][j], i1, j, i2, j).to_string());
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