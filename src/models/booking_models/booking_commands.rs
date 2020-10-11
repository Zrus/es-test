use chrono::prelude::*;

use super::super::booking_models::booking_data::ServiceInfo;
use super::super::customer_models::customer_data::Customer;
use super::super::service_models::service_data::Service;
use super::super::staff_models::staff_data::Staff;

#[derive(Debug)]
pub enum BookingCommands {
  AddBooking {
    customer: Customer,
    service: Vec<ServiceInfo>,
    created_date: DateTime<Utc>,
    booking_date: DateTime<Utc>,
  },
}

impl BookingCommands {
  pub fn add_booking(
    customer: Customer,
    service: Vec<ServiceInfo>,
    created_date: DateTime<Utc>,
    booking_date: DateTime<Utc>,
  ) -> BookingCommands {
    BookingCommands::AddBooking {
      customer,
      service,
      created_date,
      booking_date,
    }
  }
}
