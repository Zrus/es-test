use booking_data::*;

pub enum BookingEvents {
  BookingAdded(BookingData),
  BookingAddFailed(String),
  BookingUpdated(BookingData),
  BookingUpdateFailed(String),
  BookingVerified(BookingData),
  BookingVerifyFailed(String),
  BookingCanceled(BookingData),
  BookingCancelFailed(String),
  BookingBlocked(BlockData),
  BookingBlockFailed(String)
}

impl Event for BookingEvent {
  fn event_type_version(&self) -> &str {
    "0.1"
  }
  fn event_type(&self) -> &str {
    "BOOKING_EVENT"
  }
  fn event_stream_id(&self) -> i64 {
    1
  }
  fn event_stream_sequence(&self) -> &str {
    "BOOKING_01"
  }
}
