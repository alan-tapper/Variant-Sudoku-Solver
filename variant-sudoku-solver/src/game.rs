pub mod status;
use crate::variants::{self, Variant};
use status::{Status, combine_statuses};
use regex::Regex;
use ansi_term::Colour;

pub struct Game {
  pub board: [[char; 9]; 9],
  pub variants_used: Vec<Variant>,
}

impl Game {
  pub const DIGITS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
  pub const EMPTY_DIGIT: char = ' ';

  pub fn render_game_to_terminal(&self, status: Status, in_progress: bool, highlight_errors: bool) -> String {
    if highlight_errors {
      println!("highlighting errors");
    }

    let problematic_cells = self.get_problematic_cells_from_status(&status);

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

  pub fn validate(&self, in_progress: bool) -> Status {
    let mut statuses = Vec::new();
    for v  in self.variants_used.clone() {
      statuses.push(variants::apply_additional_validation(&v, self, in_progress));
    }
    return combine_statuses(statuses);
  }

  fn get_problematic_cells_from_status(&self, status: &Status) -> Vec<(usize, usize)> {
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
    return problematic_cells;
  }

  fn format_cell(&self, val: char, is_problematic: bool) -> String {
    if !is_problematic {
      return val.to_string();
    } else {
      return Colour::Red.paint(val.to_string()).to_string();
    }
  }

  
}