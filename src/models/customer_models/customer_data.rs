use serde::Serialize;

#[derive(Debug, Serialize, Hash, Eq, PartialEq)]
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