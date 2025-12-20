pub struct Status (pub bool, pub Vec<String>);

impl Status {
  pub fn format(&self) -> String {
    let mut s = format!("valid:{}", self.0);
    for m in self.1.clone() {
      s = format!("{}\n{}", m, s)
    }
    return s;
  }
}