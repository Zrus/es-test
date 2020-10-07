use serde::Serialize;

#[derive(Debug, Serialize, Hash, Eq, PartialEq)]
pub struct Service {
  pub id: String,
  pub name: String,
  pub duration: i32
}

impl Clone for Service {
  fn clone(&self) -> Service {
    Service {
      id: self.id.clone(),
      name: self.name.clone(),
      duration: self.duration
    }
  }
}