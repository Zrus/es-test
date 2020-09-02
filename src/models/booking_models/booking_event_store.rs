use booking.data::BookingData;
use booking.events::BookingEvents;

pub struct BookingEventStore {
  data: HashMap<String, BookingData>
}
impl BookingEventStore {
  fn get_events_with_data_field(&self, event_type: &str, data_field: &str, value: &str)
  -> Result<Vec<BookingEvents>> {
    let events = data.iter().filter(|(_, v)| v.data_field.eq_ignore_ascii_case(value)).collect();
    Ok(events)
  }
  fn append(&self, event: impl BookingEvents) {
    self.data.push(event).clone();
  }
}