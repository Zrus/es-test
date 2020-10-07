use chrono::prelude::*;

use super::super::staff_models::staff_data::Staff;
use super::super::customer_models::customer_data::Customer;
use super::super::service_models::service_data::Service;

#[derive(Debug)]
pub enum BookingCommands {
  AddBooking {
    customer: Customer,
    staff: Vec<Staff>,
    service: Vec<Service>,
    created_date: DateTime<Utc>,
    booking_date: DateTime<Utc>
  },
}

impl BookingCommands {
  pub fn add_booking(customer: Customer, staff: Vec<Staff>, service: Vec<Service>, created_date: DateTime<Utc>, booking_date: DateTime<Utc>) -> BookingCommands {
    BookingCommands::AddBooking {
      customer,
      staff,
      service,
      created_date,
      booking_date
    }
  }
}