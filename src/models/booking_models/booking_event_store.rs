use super::super::event_store_models::cloud_event::Result;
use super::booking_data::BookingData;
use super::booking_events::BookingEvents;
use std::collections::HashMap;

pub struct BookingEventStore {
  data: HashMap<String, BookingEvents>,
}
impl BookingEventStore {
  fn get_events_with_data_field(
    &self,
    event_type: &str,
    data_field: &str,
    value: &str,
  ) -> Result<Vec<BookingEvents>> {
    unimplemented!()
  }
  fn append(&self, event: BookingEvents) {
    unimplemented!()
  }
}
