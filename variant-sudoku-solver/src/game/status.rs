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

pub fn combine_statuses(statuses: Vec<Status>) -> Status {
    let mut overall_status_messages: Vec<String> = Vec::new();
    let mut error_found = false;
    for s in statuses {
      overall_status_messages.push(s.format(false));
      if !s.0 {
        error_found = true;
      }
    }
    return Status(!error_found, overall_status_messages);
  }