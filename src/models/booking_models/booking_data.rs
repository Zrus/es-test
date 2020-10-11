use chrono::prelude::*;
use serde::Serialize;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

use crate::models::customer_models::customer_data::Customer;
use crate::models::service_models::service_data::Service;
use crate::models::staff_models::staff_data::Staff;

#[derive(Debug, Serialize, Clone, Eq, PartialEq, Hash)]
pub struct ServiceInfo {
  pub staff: Staff,
  pub service: Service,
  pub time_start: DateTime<Utc>,
  pub time_end: DateTime<Utc>,
}

#[derive(Debug, Serialize, Eq)]
pub struct BookingData {
  pub id: String,
  pub customer: Customer,
  pub service: Vec<ServiceInfo>,
  pub created_date: DateTime<Utc>,
  pub booking_date: DateTime<Utc>,
}

impl BookingData {
  pub fn new(
    customer: Customer,
    service: Vec<ServiceInfo>,
    created_date: DateTime<Utc>,
    booking_date: DateTime<Utc>,
  ) -> BookingData {
    BookingData {
      id: Uuid::new_v4().to_string(),
      customer,
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
      service: self.service.clone(),
      created_date: self.created_date,
      booking_date: self.booking_date,
    }
  }
}

impl PartialEq for BookingData {
  fn eq(&self, booking: &BookingData) -> bool {
    self.id == booking.id
      && self.customer == booking.customer
      && self.service == booking.service
      && self.created_date == booking.created_date
      && self.booking_date == booking.booking_date
  }
}

impl Hash for BookingData {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
    self.customer.hash(state);
    self.service.hash(state);
    self.created_date.hash(state);
    self.booking_date.hash(state);
  }
}
