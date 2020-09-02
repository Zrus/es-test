use crate::models::service_models::service_data::Service;

use std::fmt::{Debug};

#[derive(Debug)]
pub struct Staff {
  pub id: String,
  pub name: String,
  pub services: Vec<Service>
}