use crate::game::Game;
use crate::variants;
use crate::variants::Variant;
use std::error::Error;
use csv::ReaderBuilder;
use std::path::PathBuf;

fn read_game_from_csv(board_type: &str, board_num: &str) -> Result<Game, Box<dyn Error>> {
  let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  path.push("src");
  path.push("sample_boards");
  path.push(board_type);
  path.push(format!("{}.csv", board_num));

  let mut rdr = ReaderBuilder::new()
    .comment(Some(b'#'))
    .has_headers(false)
    .flexible(true)
    .from_path(&path)?;

  let mut records = rdr.records().enumerate();

  //parse and render title
  let title = records.next().unwrap().1?;
  println!("{}", title.as_slice());

  //parse and render author
  let author = records.next().unwrap().1?;
  println!("{}", author.as_slice());

  //parse and render difficulty
  let difficulty = records.next().unwrap().1?;
  println!("{}", difficulty.as_slice());

  //parse and render variants/board type
  let variants_record = records.next().unwrap().1?;
  let variants_strings: Vec<&str> = variants_record.iter().collect();
  let mut variants_used: Vec<Variant> = Vec::new();
  for vstr in variants_strings {
    let mut found = false;
    for av in Variant::ALLOWED_VARIANTS {
      if vstr.trim() == av.alias {
        variants_used.push(av);
        found = true;
      }
    }
    if found == false {
      panic!("invalid variant: {}", vstr);
    }
  }
  let variants_used_string = variants_used.iter()
    .map(|vu| vu.alias)
    .collect::<Vec<_>>()
    .join(", ");
   println!("{}", variants_used_string);
   
   let mut board: [[char; 9]; 9] = [[' '; 9]; 9];
   let mut game = Game { board };
   let mut row_count: usize = 0;
 
  //parse and render board given the variant(s)
  variants::standard::parsing::apply(&mut records, &mut game)?;

  // run variant-specific post-processing (if any)
  for v in &variants_used {
    match v.identifier {
      crate::variants::VariantIdentifier::STANDARD => { /* nothing to do */ }
      crate::variants::VariantIdentifier::ANTIKNIGHT => {
        variants::antiknight::parsing::apply(&mut records, &mut game)?;
      }
      crate::variants::VariantIdentifier::ANTIKING => {
        variants::antiking::parsing::apply(&mut records, &mut game)?;
      }
      crate::variants::VariantIdentifier::ANTIDIAGONAL => {
        variants::antidiagonal::parsing::apply(&mut records, &mut game)?;
      }
    }
  }
  Ok(game)
}

pub fn game_from_sample_board(board_type: &str, board_num : &str) -> Game {
  let game = match read_game_from_csv(board_type, board_num) {
    Ok(b) => b,
    Err(e) => {
      eprintln!("Failed to read board {} {}: {}", board_type, board_num, e);
      std::process::exit(1)
    }
  };

  return game;
}