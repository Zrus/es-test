use uuid::Uuid;

use super::super::event_store_models::aggregate::Aggregate;
use super::super::event_store_models::cloud_event::Result;
use super::booking_commands::BookingCommands;
use super::booking_data::BookingData;
use super::booking_events::BookingEvents;
use super::booking_state::BookingState;

pub struct BookingAggregate {}
impl Aggregate for BookingAggregate {
  type Event = BookingEvents;
  type Command = BookingCommands;
  type State = BookingState;

  fn apply_event(state: &Self::State, evt: &Self::Event) -> Result<Self::State> {
    let event = match evt {
      BookingEvents::BookingAdded(id, booking) => BookingState {
        entity_id: state.entity_id.clone(),
        booking_list: {
          let mut new_booking_list = state.booking_list.clone();
          new_booking_list.insert(id.clone(), booking.clone());
          new_booking_list
        },
        block_list: state.block_list.clone(),
        generation: state.generation + 1,
      },
      BookingEvents::BookingAddFailed(_id, _booking) => state.clone(),
    };
    Ok(event)
  }

  fn handle_command(_state: &Self::State, cmd: &Self::Command) -> Result<Vec<Self::Event>> {
    let events = match cmd {
      BookingCommands::AddBooking {
        customer,
        staff,
        service,
        created_date,
        booking_date,
      } => {
        let id = Uuid::new_v4();
        match validate_booking() {
          true => BookingEvents::BookingAdded(id, BookingData {
            id,
            customer: customer.clone(),
            staff: staff.clone(),
            service: service.clone(),
            created_date: created_date.clone(),
            booking_date: booking_date.clone(),
          }),
          false => BookingEvents::BookingAddFailed(id, BookingData {
            id,
            customer: customer.clone(),
            staff: staff.clone(),
            service: service.clone(),
            created_date: created_date.clone(),
            booking_date: booking_date.clone(),
          }),
        }
      }
    };
    Ok(vec![events])
  }

  fn aggregate_id(&self) -> String {
    "booking_aggregate_01".to_owned()
  }
}

fn validate_booking() -> bool {
  true
}
