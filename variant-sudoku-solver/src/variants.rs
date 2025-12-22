pub mod standard;
pub mod antidiagonal;
pub mod antiking;
pub mod antiknight;

use strum::{EnumCount};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, EnumCountMacro, EnumIter)]
pub enum VariantIdentifier {
  STANDARD,
  ANTIKNIGHT,
  ANTIKING,
  ANTIDIAGONAL
}

#[derive(Debug)]
pub struct Variant {
  pub identifier: VariantIdentifier,
  pub alias: &'static str,
  pub path_to_rules: &'static str,
}

impl Variant {
  pub const ALLOWED_VARIANTS: [Variant; VariantIdentifier::COUNT] = [
    Variant {
      identifier: VariantIdentifier::STANDARD,
      alias: "Standard",
      path_to_rules: "standard"
    },
    Variant {
      identifier: VariantIdentifier::ANTIKNIGHT,
      alias: "Anti-Knight",
      path_to_rules: "antiknight"
    },
    Variant {
      identifier: VariantIdentifier::ANTIKING,
      alias: "Anti-King",
      path_to_rules: "antiking"
    },
    Variant {
      identifier: VariantIdentifier::ANTIDIAGONAL,
      alias: "Diagonal",
      path_to_rules: "antidiagonal"
    }
  ];
  pub const BASE_PATH_STRING: &str = "variants";
}