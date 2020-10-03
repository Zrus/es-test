use chrono::prelude::*;
use uuid::Uuid;

use super::super::event_store_models::aggregate::Aggregate;
use super::super::event_store_models::cloud_event::Result;
use super::booking_commands::BookingCommands;
use super::blocking_data::BlockData;
use super::booking_data::BookingData;
use super::booking_events::BookingEvents;
use super::booking_state::BookingState;

use super::super::customer_models::customer_data::Customer;
use super::super::service_models::service_data::Service;
use super::super::staff_models::staff_data::Staff;

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
          true => {
            // add logic here
            let events_loaded = BookingAggregate::load_events();
            // logic ended

            BookingEvents::BookingAdded(
              id.to_string(),
              BookingData {
                id: id.to_string(),
                customer: customer.clone(),
                staff: staff.clone(),
                service: service.clone(),
                created_date: created_date.clone(),
                booking_date: booking_date.clone(),
              },
            )
          }
          false => BookingEvents::BookingAddFailed(
            id.to_string(),
            BookingData {
              id: id.to_string(),
              customer: customer.clone(),
              staff: staff.clone(),
              service: service.clone(),
              created_date: created_date.clone(),
              booking_date: booking_date.clone(),
            },
          ),
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

impl BookingAggregate {
  pub fn load_events() -> Vec<BookingEvents> {
    vec![
      BookingEvents::BookingAdded(
        "c9cf925d-7a08-4758-ad9e-94e6a676aed7".to_owned(),
        BookingData {
          id: "5ec38249-84e4-4a04-847b-d4f4d8f8d577".to_owned(),
          customer: Customer {
            id: "customer123".to_owned(),
            name: "Tuong".to_owned(),
            phone_number: "0909909009".to_owned(),
            address: "HoChiMinh".to_owned(),
            email: "zentech.demo@gmail.com".to_owned(),
          },
          staff: Staff {
            id: "staff123".to_owned(),
            name: "Tri".to_owned(),
            services: vec![Service {
              id: "service123".to_owned(),
              name: "HairCut".to_owned(),
              duration: 15,
            }],
          },
          service: Service {
            id: "service123".to_owned(),
            name: "HairCut".to_owned(),
            duration: 15,
          },
          created_date: Utc.ymd(2020, 12, 31).and_hms(13, 15, 00),
          booking_date: Utc.ymd(2021, 1, 1).and_hms(18, 00, 00),
        },
      ),
      BookingEvents::BookingAdded(
        "e01a8441-0f33-438b-8a1f-6d0b140f7ee2".to_owned(),
        BookingData {
          id: "097cf1a7-4eaa-4347-9015-7cb919b3f1d6".to_owned(),
          customer: Customer {
            id: "customer123".to_owned(),
            name: "Tuong".to_owned(),
            phone_number: "0909909009".to_owned(),
            address: "HoChiMinh".to_owned(),
            email: "zentech.demo@gmail.com".to_owned(),
          },
          staff: Staff {
            id: "staff123".to_owned(),
            name: "Tri".to_owned(),
            services: vec![Service {
              id: "service123".to_owned(),
              name: "HairCut".to_owned(),
              duration: 15,
            }],
          },
          service: Service {
            id: "service123".to_owned(),
            name: "HairCut".to_owned(),
            duration: 15,
          },
          created_date: Utc.ymd(2020, 12, 30).and_hms(13, 15, 00),
          booking_date: Utc.ymd(2021, 1, 2).and_hms(18, 00, 00),
        },
      ),
    ]
  }
  pub fn load_blocks() -> Vec<BlockData> {
    vec![BlockData {
      staff: Staff {
        id: "staff123".to_owned(),
        name: "Tri".to_owned(),
        services: vec![Service {
          id: "service123".to_owned(),
          name: "HairCut".to_owned(),
          duration: 15,
        }],
      },
      start_time: Utc.ymd(2020, 12, 31).and_hms(13, 15, 00),
      end_time: Utc.ymd(2020, 12, 31).and_hms(18, 00, 00),
    }]
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_handle_commands() {
    println!("Oke dumamay");
  }
}
