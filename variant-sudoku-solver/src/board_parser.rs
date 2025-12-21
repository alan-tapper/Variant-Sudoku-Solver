use crate::game::Game;
use std::error::Error;
use csv::ReaderBuilder;
use std::path::PathBuf;

fn read_game(board_type: &str, board_num: &str) -> Result<[[char;9];9], Box<dyn Error>> {
  let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  path.push("src");
  path.push("sample_boards");
  path.push(board_type);
  path.push(format!("{}.csv", board_num));

  let mut rdr = ReaderBuilder::new()
    .has_headers(false)
    .flexible(true)
    .from_path(&path)?;

  let mut board: [[char; 9]; 9] = [[' '; 9]; 9];
  let mut row_count: usize = 0;

  for (i, result) in rdr.records().enumerate() {
      // if i >= 9 { break; }
      let record = result?;
      // eprintln!("record #{}: {:?}", i + 1, record);

      if record.len() != 9 {
          return Err(format!("expected 9 columns in row {}, got {}", i + 1, record.len()).into());
      }

      for j in 0..9 {
          let cell = record.get(j).unwrap().trim();
          board[i][j] = match cell {
              "_" | "" => Game::EMPTY_DIGIT,
              d if d.len() == 1 => {
                  let ch = d.chars().next().unwrap();
                  if Game::DIGITS.contains(&ch) {
                      ch
                  } else {
                      return Err(format!("invalid digit '{}' at row {}, col {}", ch, i + 1, j + 1).into());
                  }
              }
              other => return Err(format!("invalid cell '{}' at row {}, col {}", other, i + 1, j + 1).into()),
          };
      }
      row_count += 1;
  }

  if row_count != 9 {
      return Err(format!("expected 9 rows, got {}", row_count).into());
  }

  Ok(board)
}

pub fn game_from_sample_board(board_type: &str, board_num : &str) -> Game {
  let board = match read_game(board_type, board_num) {
    Ok(b) => b,
    Err(e) => {
      eprintln!("Failed to read board {} {}: {}", board_type, board_num, e);
      std::process::exit(1)
    }
  };

  return Game {
    board
  }
}