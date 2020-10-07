use chrono::prelude::*;
use uuid::Uuid;
use std::collections::HashSet;


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
        booking_date
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
                booking_date: booking_date.clone()
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
              booking_date: booking_date.clone()
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
          staff: vec![Staff {
            id: "staff01".to_owned(),
            name: "Tri".to_owned(),
            services: vec![Service {
              id: "service01".to_owned(),
              name: "HairCut".to_owned(),
              duration: 20
            }]
          }],
          service: vec![Service {
            id: "service02".to_owned(),
            name: "HairCut".to_owned(),
            duration: 15
          }],
          created_date: Utc.ymd(2020, 12, 31).and_hms(13, 15, 00),
          booking_date: Utc.ymd(2021, 1, 1).and_hms(18, 00, 00),
        }
      ),
      BookingEvents::BookingAdded(
        "e01a8441-0f33-438b-8a1f-6d0b140f7ee2".to_owned(),
        BookingData {
          id: "097cf1a7-4eaa-4347-9015-7cb919b3f1d6".to_owned(),
          customer: Customer {
            id: "customer123".to_owned(),
            name: "Vi".to_owned(),
            phone_number: "0909909009".to_owned(),
            address: "HoChiMinh".to_owned(),
            email: "zentech.demo@gmail.com".to_owned()
          },
          staff: vec![Staff {
            id: "staff01".to_owned(),
            name: "Tri".to_owned(),
            services: vec![Service {
              id: "service01".to_owned(),
              name: "HairCut".to_owned(),
              duration: 20
            }]
          }],
          service: vec![Service {
            id: "service01".to_owned(),
            name: "HairCut".to_owned(),
            duration: 30
          }],
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
          duration: 15
        }]
      },
      start_time: Utc.ymd(2020, 12, 31).and_hms(13, 15, 00),
      end_time: Utc.ymd(2020, 12, 31).and_hms(18, 00, 00)
    }]
  }

  pub fn load_bookings() -> Vec<BookingData> {
    vec![BookingData {
      id: "ebc9f2cd-af10-42f4-bfcd-9c12be2941fc".to_owned(),
      customer: Customer {
        id: "customer01".to_owned(),
        name: "Vinh".to_owned(),
        phone_number: "123456789".to_owned(),
        address: "HoChiMinh".to_owned(),
        email: "vinh@gmail.com".to_owned(),
      },
      staff: vec![Staff {
        id: "staff01".to_owned(),
        name: "Tri".to_owned(),
        services: vec![Service {
          id: "service01".to_owned(),
          name: "HairCut".to_owned(),
          duration: 20,
        }],
      },
      Staff {
        id: "staff02".to_owned(),
        name: "Halu".to_owned(),
        services: vec![Service {
          id: "service02".to_owned(),
          name: "Manicure".to_owned(),
          duration: 30,
        }],
      }],
      service: vec![Service {
        id: "service01".to_owned(),
        name: "HairCut".to_owned(),
        duration: 20,
      },
      Service {
        id: "service02".to_owned(),
        name: "Manicure".to_owned(),
        duration: 30,
      }],
      created_date: Utc.ymd(2020, 10, 1).and_hms(9, 00, 00),
      booking_date: Utc.ymd(2020, 10, 5).and_hms(9, 00, 00),
    }, 
    
    BookingData {
      id: "d2054836-8e5e-44ff-a9fa-35c64cd6cf2e".to_owned(),
      customer: Customer {
        id: "customer02".to_owned(),
        name: "Thong".to_owned(),
        phone_number: "987654321".to_owned(),
        address: "HoChiMinh".to_owned(),
        email: "thong@gmail.com".to_owned(),
      },
      staff: vec![Staff {
        id: "staff02".to_owned(),
        name: "Halu".to_owned(),
        services: vec![Service {
          id: "service02".to_owned(),
          name: "Manicure".to_owned(),
          duration: 30,
        }],
      }],
      service: vec![Service {
        id: "service02".to_owned(),
        name: "Manicure".to_owned(),
        duration: 30,
      }],
      created_date: Utc.ymd(2020, 10, 1).and_hms(9, 00, 00),
      booking_date: Utc.ymd(2020, 10, 5).and_hms(10, 00, 00),
    },

    BookingData {
      id: "d2054836-8e5e-44ff-a9fa-35c64cd6cf2e".to_owned(),
      customer: Customer {
        id: "customer02".to_owned(),
        name: "Daocuong".to_owned(),
        phone_number: "909909009".to_owned(),
        address: "Chicago".to_owned(),
        email: "daocuong@gmail.com".to_owned(),
      },
      staff: vec![Staff {
        id: "staff03".to_owned(),
        name: "Tuanbeo".to_owned(),
        services: vec![Service {
          id: "service02".to_owned(),
          name: "Manicure".to_owned(),
          duration: 30,
        }],
      }],
      service: vec![Service {
        id: "service02".to_owned(),
        name: "Manicure".to_owned(),
        duration: 30,
      }],
      created_date: Utc.ymd(2020, 10, 1).and_hms(9, 00, 00),
      booking_date: Utc.ymd(2020, 10, 5).and_hms(11, 00, 00),
    }]
  }

  pub fn validate_booking_date(new_booking: &BookingData) {
    let bookings = BookingAggregate::load_bookings();
    let mut valid_bookings = HashSet::new();
    for booking in &bookings {
      if booking.staff.iter().any(|staff| new_booking.staff.contains(&staff)) {
          println!("{:?}", booking);
          valid_bookings.insert(booking);
      }
  }
  }
}

#[cfg(test)]
mod tests {
  use crate::booking_aggregate::BookingAggregate;
  use crate::models::customer_models::customer_data::Customer;
  use crate::models::service_models::service_data::Service;
  use crate::models::staff_models::staff_data::Staff;
  use crate::models::booking_models::booking_data::BookingData;
  use chrono::prelude::*;
  #[test]
  fn test_handle_commands() {
    // println!("{:?}", BookingAggregate::load_bookings());
    let new_booking = BookingData {
      id: "097cf1a7-4eaa-4347-9015-7cb919b3f1d6".to_owned(),
          customer: Customer {
            id: "customer4".to_owned(),
            name: "Vi".to_owned(),
            phone_number: "0909909009".to_owned(),
            address: "HoChiMinh".to_owned(),
            email: "vi@gmail.com".to_owned()
          },
          staff: vec![Staff {
            id: "staff02".to_owned(),
            name: "Halu".to_owned(),
            services: vec![Service {
              id: "service02".to_owned(),
              name: "Manicure".to_owned(),
              duration: 30
            }]
          }],
          service: vec![Service {
            id: "service02".to_owned(),
            name: "Manicure".to_owned(),
            duration: 30
          }],
          created_date: Utc.ymd(2020, 12, 30).and_hms(13, 15, 00),
          booking_date: Utc.ymd(2021, 1, 2).and_hms(18, 00, 00),
    };
    BookingAggregate::validate_booking_date(&new_booking);

  }
}
