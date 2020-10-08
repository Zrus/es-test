use serde::Serialize;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Eq)]
pub struct Customer {
  pub id: String,
  pub name: String,
  pub phone_number: String,
  pub address: String,
  pub email: String
}

impl Clone for Customer {
  fn clone(&self) -> Customer {
    Customer {
      id: self.id.clone(),
      name: self.name.clone(),
      phone_number: self.phone_number.clone(),
      address: self.address.clone(),
      email: self.email.clone()
    }
  }
}

impl PartialEq for Customer {
  fn eq(&self, customer: &Customer) -> bool {
    self.id == customer.id && self.name == customer.name && self.phone_number == customer.phone_number && self.address == customer.address && self.email == customer.email
  }
}

impl Hash for Customer {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
    self.name.hash(state);
    self.phone_number.hash(state);
    self.address.hash(state);
    self.email.hash(state);
  }
}