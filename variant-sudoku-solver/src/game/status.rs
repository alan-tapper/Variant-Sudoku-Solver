pub struct Status (pub bool, pub Vec<String>);

impl Status {
  pub fn format(&self, include_bool: bool) -> String {
    let mut s: String = "".to_string();
    if include_bool {
      s = format!("{}valid:{}", s, self.0);
    }
    if !self.0 {
      for m in self.1.clone() {
        if s.len() == 0 {
          s = m
        }
        else {
          s = format!("{}\n{}", s, m)
        }
      }
    }
    return s;
  }

  pub fn clone(&self) -> Status {
    return Status(self.0, self.1.clone())
  }
}