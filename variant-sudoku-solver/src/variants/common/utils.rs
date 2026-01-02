use crate::game::{Game, status};

pub fn check_for_duplicates_in_set(game: &Game, vec: &Vec<(usize, usize)>, set_name: &str) -> status::Status {
  let mut statuses: Vec<status::Status> = Vec::new();
  for i in 0..vec.len() {
    for j in 0..vec.len(){
      let pos_i = vec[i];
      let pos_j = vec[j];
      if game.board[pos_i.0][pos_i.1] == Game::EMPTY_DIGIT || game.board[pos_j.0][pos_j.1] == Game::EMPTY_DIGIT {
        continue;
      }
      if i < j && game.board[pos_i.0][pos_i.1] == game.board[pos_j.0][pos_j.1] {
        let mut v: Vec<String> = Vec::new();
        v.push(format!("duplicate {}s in {} at ({}, {}), ({}, {})", game.board[pos_i.0][pos_i.1], set_name, pos_i.0, pos_i.1, pos_j.0, pos_j.1).to_string());
        let s = status::Status(false, v);
        statuses.push(s);
      }
    }
  }
  return status::combine_statuses(statuses);
}