use chrono::prelude::*;
use uuid::Uuid;
use serde::Serialize;

use crate::models::customer_models::customer_data::Customer;
use crate::models::service_models::service_data::Service;
use crate::models::staff_models::staff_data::Staff;

#[derive(Debug, Serialize, Hash, Eq, PartialEq)]
pub struct BookingData {
  pub id: String,
  pub customer: Customer,
  pub staff: Vec<Staff>,
  pub service: Vec<Service>,
  pub created_date: DateTime<Utc>,
  pub booking_date: DateTime<Utc>
}

impl BookingData {
  pub fn new(
    customer: Customer,
    staff: Vec<Staff>,
    service: Vec<Service>,
    created_date: DateTime<Utc>,
    booking_date: DateTime<Utc>,
  ) -> BookingData {
    BookingData {
      id: Uuid::new_v4().to_string(),
      customer,
      staff,
      service,
      created_date,
      booking_date,
    }
  }
}

impl Clone for BookingData {
  fn clone(&self) -> BookingData {
    BookingData {
    id: self.id.clone(),
    customer: self.customer.clone(),
    staff: self.staff.clone(),
    service: self.service.clone(),
    created_date: self.created_date,
    booking_date: self.booking_date
    }
  }
}

