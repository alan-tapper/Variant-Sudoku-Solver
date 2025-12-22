use crate::game::Game;
use std::error::Error;
use std::iter::Enumerate;
use std::fs::File;
use csv::StringRecordsIter;

pub fn apply(records: &mut Enumerate<StringRecordsIter<'_, File>>, game: &mut Game) -> Result<(), Box<dyn Error + 'static>> {
    let mut row_count: usize = 0;
    for i in 0..9 {
        let record = records.next().unwrap().1?;

        if record.len() != 9 {
            return Err(format!("expected 9 columns in row {}, got {}", i + 1, record.len()).into());
        }

        for j in 0..9 {
            let cell = record.get(j).unwrap().trim();
            game.board[i][j] = match cell {
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
    Ok(())
}