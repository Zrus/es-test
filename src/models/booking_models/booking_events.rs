use serde::Serialize;
use uuid::Uuid;

use super::super::event_store_models::cloud_event::Event;
use super::booking_data::*;

#[derive(Serialize)]
pub enum BookingEvents {
  BookingAdded(Uuid, BookingData),
  BookingAddFailed(Uuid, BookingData),
}

impl Event for BookingEvents {
  fn event_type_version(&self) -> &str {
    "0.1"
  }
  fn event_type(&self) -> &str {
    "BOOKING_EVENT"
  }
  fn event_stream_sequence(&self) -> i64 {
    1
  }
  fn event_stream_id(&self) -> &str {
    "BOOKING_01"
  }
}
