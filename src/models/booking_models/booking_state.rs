use std::collections::HashMap;
use uuid::Uuid;

use super::super::event_store_models::aggregate::AggregateState;
use super::booking_data::BookingData;
use super::blocking_data::BlockData;

pub struct BookingState {
  pub entity_id: String,
  pub booking_list: HashMap<String, BookingData>, // Key could be staff_id ??
  pub block_list: HashMap<String, BlockData>,
  pub generation: u64
}

impl AggregateState for BookingState {
  fn generation(&self) -> u64 {
    self.generation
  }
}

impl Clone for BookingState {
  fn clone(&self) -> BookingState {
    BookingState {
      entity_id: self.entity_id.clone(),
      booking_list: self.booking_list.clone(),
      block_list: self.block_list.clone(),
      generation: self.generation
    }
  }
}