use crate::models::service_models::service_data::Service;

use serde::Serialize;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Eq)]
pub struct Staff {
  pub id: String,
  pub name: String,
  pub services: Vec<Service>
}

impl Clone for Staff {
  fn clone(&self) -> Staff {
    Staff {
    id: self.id.clone(),
    name: self.name.clone(),
    services: self.services.clone()
    }
  }
}

impl PartialEq for Staff {
  fn eq(&self, staff: &Staff) -> bool {
    self.id == staff.id && self.name == staff.name && self.services == staff.services
  }
}

impl Hash for Staff {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
    self.name.hash(state);
    self.services.hash(state);
  }
}