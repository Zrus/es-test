use std::fmt::{Debug};

#[derive(Debug)]
pub struct Customer {
  pub id: String,
  pub name: String,
  pub phone_number: String,
  pub address: String,
  pub email: String
}