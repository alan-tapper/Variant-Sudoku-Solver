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
  pub alias: &'static str
}

impl Variant {
  pub const ALLOWED_VARIANTS: [Variant; VariantIdentifier::COUNT] = [
    Variant {
      identifier: VariantIdentifier::STANDARD,
      alias: "Standard"
    },
    Variant {
      identifier: VariantIdentifier::ANTIKNIGHT,
      alias: "Anti-Knight"
    },
    Variant {
      identifier: VariantIdentifier::ANTIKING,
      alias: "Anti-King"
    },
    Variant {
      identifier: VariantIdentifier::ANTIDIAGONAL,
      alias: "Diagonal"
    }
  ];
}