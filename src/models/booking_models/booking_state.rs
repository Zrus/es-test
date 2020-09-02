pub struct BookingState {
  entity_id: String,
  BookingList: HashMap<String, BookingData>, // Key could be staff_id ??
  BlockList: HashMap<String, BlockData>,
  generation: u64
}

impl AggregateState for BookingState {
  fn generation(&self) -> u64 {
    self.generation
  }
}