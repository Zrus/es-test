use chrono::prelude::*;

use booking_events::BookingEvents;
use booking_commands::BookingCommands;
use booking_state::BookingState;

pub struct BookingAggregate {
  // properties
};
impl BookingAggregate {
  load(date_time: DateTime<Utc>) {
    
  }
}
impl Aggregate for BookingAggregate {
  type Event = BookingEvents;
  type Command = BookingCommands;
  type State = BookingState;

  fn apply_event(state: &Self::State, evt: &Self::Event) -> Result<Self::State> {
    let event = match evt {
      BookingEvents::BookingAdded(booking) => {
        BookingState {
          entity_id: state.entity_id,
          events: state.insert(booking.id, booking).clone(),
          generation: state.generation + 1
        }
      },
      BookingEvents::BookingAddFailed(id) => state.clone(),
      BookingEvents::BookingUpdated(booking) => {
        BookingState {
          entity_id: state.entity_id,
          events: state.insert(booking.id, booking).clone(),
          generation: state.generation + 1
        }
      }
      BookingEvents::BookingUpdateFailed(id) => {
        BookingState {
          entity_id: state.entity_id,
          events: state.clone(),
          generation: state.generation + 1
        }
      }
      BookingEvents::BookingCanceled(booking) => {
        BookingState {
          entity_id: state.entity_id,
          events: state.remove(booking.id).clone(),
          generation: state.generation + 1
        }
      },
      BookingEvents::BookingCancelFailed(id) => state.clone()
    };
    Ok(event)
  }

  fn handle_command(state: &Self::State, cmd: &Self::Command) -> Result<Vec<Self::Event>> {
    let events = match *cmd {
      BookingCommands::AddBooking{customer_id, staff_ids, service_ids, date} => {
        let id = Uuid::new_v4();
        match validate_booking() {
          true => BookingEvents::BookingAdded(BookingData {
            id,
            customer,
            staff,
            service,
            created_date,
            booking_date
          }),
          false => BookingEvents::BookingAddFailed(id)
        }
      },
      // BookingCommands::UpdateBooking{id, staff_ids, service_ids, date} => {
      //   match state.events.get(id) {
      //     Some(booking_data) => {
      //       match validate_booking() {
      //         true => BookingEvents::BookingUpdated(BookingData {
      //           id,
      //           customer_id: booking_data.customer_id,
      //           staff_ids, 
      //           service_ids,
      //           date
      //         }),
      //         false => BookingEvents::BookingUpdateFailed(id)
      //       }
      //     }
      //     None => BookingEvents::BookingUpdateFailed(id)
      //   }
      // },
      BookingCommands::VerifyBooking{id, customer_id} => {
        match state.events.get(id) {
          Some(booking_data) => BookingEvents::BookingVerified(booking_data),
          None => BookingEvents::BookingVerifyFailed(id)
        }
      }
      BookingCommands::CancelBooking{id} => {
        match state.events.get(id) {
          Some(booking_data) => BookingEvents::BookingCanceled(booking_data),
          None => BookingEvents::BookingCancelFailed(id)
        }
      },
      BookingCommands::BlockBooking{staff, start_time, end_time} => {
        
      }
    };
    Ok(vec![events])
  }
}

fn validate_booking() {
  true
}