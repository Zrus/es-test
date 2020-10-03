use chrono::prelude::*;
use serde::Serialize;

use crate::models::staff_models::staff_data::Staff;

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