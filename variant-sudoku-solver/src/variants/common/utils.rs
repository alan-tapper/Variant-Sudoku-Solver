use crate::game::{Game, status};

pub fn check_for_duplicates(game: &Game, vec: &Vec<(usize, usize)>, set_name: &str) -> status::Status {
  let mut statuses: Vec<status::Status> = Vec::new();
  for i in 0..vec.len() {
    for j in 0..vec.len(){
      if game.board[vec[i].0][vec[i].1] == Game::EMPTY_DIGIT || game.board[vec[j].0][vec[j].1] == Game::EMPTY_DIGIT {
        continue;
      }
      if i < j && game.board[vec[i].0][vec[i].1] == game.board[vec[j].0][vec[j].1] {
        let mut v: Vec<String> = Vec::new();
        v.push(format!("duplicate {}s in {} at ({}, {}), ({}, {})", game.board[vec[i].0][vec[i].1], set_name, vec[i].0, vec[i].1, vec[j].0, vec[j].1).to_string());
        let s = status::Status(false, v);
        statuses.push(s);
      }
    }
  }
  return status::combine_statuses(statuses);
}