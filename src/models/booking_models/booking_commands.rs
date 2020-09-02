pub enum BookingCommands {
  AddBooking {
    customer_id: String,
    staff_ids: Vec<String>,
    service_ids: Vec<String>,
    date: DateTime<Utc>
  },
  UpdateBooking {
    id: String,
    staff_ids: Vec<String>,
    service_ids: Vec<String>,
    date: DateTime<Utc>
  },
  VerifyBooking {
    id: String,
    customer_id: String
  },
  CancelBooking {
    id: String
  },
  BlockBooking {
    staff: Staff,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>
  }
}