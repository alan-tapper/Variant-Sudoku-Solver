use crate::Game;
use crate::game::status::{self, Status};
pub fn validate(_game: &Game, _in_progress: bool) -> status::Status {
  return Status(true, vec!["No errors found".to_string()]);
}