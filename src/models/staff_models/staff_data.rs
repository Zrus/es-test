use crate::models::service_models::service_data::Service;

use serde::Serialize;

#[derive(Debug, Serialize)]
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