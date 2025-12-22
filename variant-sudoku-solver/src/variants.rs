pub mod standard;
pub mod antidiagonal;
pub mod antiking;
pub mod antiknight;

use strum::{EnumCount};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
use std::error::Error;
use std::iter::Enumerate;
use std::fs::File;
use csv::StringRecordsIter;
use crate::game::Game;
use crate::game::status::Status;

#[derive(Debug, EnumCountMacro, EnumIter, Clone, PartialEq)]
pub enum VariantIdentifier {
  STANDARD,
  ANTIKNIGHT,
  ANTIKING,
  ANTIDIAGONAL
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
  pub identifier: VariantIdentifier,
  pub alias: &'static str,
}

impl Variant {
  pub const ALLOWED_VARIANTS: [Variant; VariantIdentifier::COUNT] = [
    Variant {
      identifier: VariantIdentifier::STANDARD,
      alias: "Standard",
    },
    Variant {
      identifier: VariantIdentifier::ANTIKNIGHT,
      alias: "Anti-Knight",
    },
    Variant {
      identifier: VariantIdentifier::ANTIKING,
      alias: "Anti-King",
    },
    Variant {
      identifier: VariantIdentifier::ANTIDIAGONAL,
      alias: "Anti-Diagonal",
    }
  ];

  pub fn from_alias(alias: &str) -> Result<&'static Variant, &'static str> {
    for variant in Variant::ALLOWED_VARIANTS.iter() {
      if variant.alias.eq_ignore_ascii_case(alias) {
        return Ok(variant);
      }
    }
    Err("Invalid variant alias")
  }
}

pub fn apply_additional_parsing(
    variant: &Variant,
    records: &mut Enumerate<StringRecordsIter<'_, File>>,
    game: &mut Game,
) -> Result<(), Box<dyn Error>> {
    match variant.identifier {
        VariantIdentifier::STANDARD => standard::parsing::parse(records, game),
        VariantIdentifier::ANTIKNIGHT => antiknight::parsing::parse(records, game),
        VariantIdentifier::ANTIKING => antiking::parsing::parse(records, game),
        VariantIdentifier::ANTIDIAGONAL => antidiagonal::parsing::parse(records, game),
    }
}

pub fn apply_additional_validation(
    variant: &Variant,
    game: &Game,
    in_progress: bool,
) -> Status {
    match variant.identifier {
        VariantIdentifier::STANDARD => standard::validation::validate(game, in_progress),
        VariantIdentifier::ANTIKNIGHT => antiknight::validation::validate(game, in_progress),
        VariantIdentifier::ANTIKING => antiking::validation::validate(game, in_progress),
        VariantIdentifier::ANTIDIAGONAL => antidiagonal::validation::validate(game, in_progress),
    }
}