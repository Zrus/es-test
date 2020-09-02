use chrono::prelude::*;
use uuid::Uuid;
use serde::Serialize;

use crate::models::customer_models::customer_data::Customer;
use crate::models::service_models::service_data::Service;
use crate::models::staff_models::staff_data::Staff;


#[derive(Debug, Serialize)]
pub struct BookingData {
  pub id: Uuid,
  pub customer: Customer,
  pub staff: Staff,
  pub service: Service,
  pub created_date: DateTime<Utc>,
  pub booking_date: DateTime<Utc>,
}

impl BookingData {
  pub fn new(
    customer: Customer,
    staff: Staff,
    service: Service,
    created_date: DateTime<Utc>,
    booking_date: DateTime<Utc>,
  ) -> BookingData {
    BookingData {
      id: Uuid::new_v4(),
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
    id: self.id,
    customer: self.customer.clone(),
    staff: self.staff.clone(),
    service: self.service.clone(),
    created_date: self.created_date,
    booking_date: self.booking_date
    }
  }
}

#[derive(Debug, Serialize)]
pub struct BlockData {
  pub staff: Staff,
  pub start_time: DateTime<Utc>,
  pub end_time: DateTime<Utc>,
}

impl BlockData {
  pub fn new(
    staff: Staff, 
    start_time: DateTime<Utc>, 
    end_time: DateTime<Utc>
  ) -> BlockData {
    BlockData {
      staff,
      start_time,
      end_time,
    }
  }
}

impl Clone for BlockData {
  fn clone(&self) -> BlockData {
    BlockData {
      staff: self.staff.clone(),
      start_time: self.start_time,
      end_time: self.end_time
    }
  }
}
