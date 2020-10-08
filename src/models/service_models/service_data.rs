use serde::Serialize;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Eq)]
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

impl PartialEq for Service {
  fn eq(&self, service: &Service) -> bool {
    self.id == service.id && self.name == service.name && self.duration == service.duration
  }
}

impl Hash for Service {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
    self.name.hash(state);
    self.duration.hash(state);
  }
}