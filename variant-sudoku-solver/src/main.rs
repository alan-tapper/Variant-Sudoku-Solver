struct Game {
  board: [[char; 9]; 9],
  digits: [char; 9]
}

impl Game {
  fn format_game(&self) -> String {
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

  fn is_valid(&self) -> bool {
    for i in 0..9 {
      for j in 0..9 {
        let mut v = false;
        for k in 0..9 {
          if self.board[i][j] == self.digits[k] {
            v = true;
          }
        }
        if v == false {
          return false;
        }
      }
      if self.check_row(i) == false {
        return false;
      }
    }
    for j in 0..9 {
      if self.check_col(j) == false {
        return false;
      }
    }
    return true;
  }

  fn check_row(&self, i: usize) -> bool {
    for j1 in 0..9 {
      for j2 in 0..9 {
        if j1 != j2 && self.board[i][j1] == self.board[i][j2] {
          return false;
        }
      }
    }
    return true;
  }

  fn check_col(&self, j: usize) -> bool {
    for i1 in 0..9 {
      for i2 in 0..9 {
        if i1 != i2 && self.board[1][j] == self.board[i2][j] {
          return false;
        }
      }
    }
    return true;
  }

}

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
    println!("{}", game.is_valid());
}
