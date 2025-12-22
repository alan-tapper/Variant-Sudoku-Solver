use crate::game::Game;
use std::error::Error;
use std::iter::Enumerate;
use std::fs::File;
use csv::StringRecordsIter;

pub fn parse(_records: &mut Enumerate<StringRecordsIter<'_, File>>, _game: &Game) -> Result<(), Box<dyn Error + 'static>> {
  // No parsing needed for antiking
  Ok(())
}