use crate::game::Game;
use std::error::Error;
use std::iter::Enumerate;
use std::fs::File;
use csv::StringRecordsIter;

pub fn apply(records: &mut Enumerate<StringRecordsIter<'_, File>>, game: &mut Game) -> Result<(), Box<dyn Error + 'static>> {
  // TODO: implement variant-specific transformations
  Ok(())
}