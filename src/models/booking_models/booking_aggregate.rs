use chrono::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use uuid::Uuid;

use super::super::event_store_models::aggregate::Aggregate;
use super::super::event_store_models::cloud_event::Result;
use super::blocking_data::BlockData;
use super::booking_commands::BookingCommands;
use super::booking_data::BookingData;
use super::booking_data::ServiceInfo;
use super::booking_events::BookingEvents;
use super::booking_state::BookingState;

use super::super::customer_models::customer_data::Customer;
use super::super::dummy_data_models::dummy_data::DummyData;
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
        service,
        created_date,
        booking_date,
      } => {
        let id = Uuid::new_v4();
        let list_of_conflict_services = service
          .iter()
          .filter(|service| {
            let current_staff = &service.staff;
            let current_staff_list_bookings = BookingAggregate::load_bookings(&current_staff);
            let current_staff_list_services_from_list_bookings = current_staff_list_bookings
              .iter()
              .map(|booking| {
                booking
                  .service
                  .iter()
                  .map(|service| service.clone())
                  .collect::<Vec<ServiceInfo>>()
              })
              .collect::<Vec<Vec<ServiceInfo>>>();
            let current_staff_service_time = (service.time_start, service.time_end);

            current_staff_list_services_from_list_bookings
              .iter()
              .any(|booking| {
                booking.iter().any(|service| {
                  let service_time = (service.time_start, service.time_end);
                  !(current_staff_service_time.0 > service_time.1
                    || current_staff_service_time.1 < service_time.0)
                })
              })
          })
          .collect::<Vec<&ServiceInfo>>();
        match (
          BookingAggregate::validate_booking(),
          list_of_conflict_services.is_empty(),
        ) {
          (true, true) => BookingEvents::BookingAdded(
            id.to_string(),
            BookingData {
              id: id.to_string(),
              customer: customer.clone(),
              service: service.clone(),
              created_date: created_date.clone(),
              booking_date: booking_date.clone(),
            },
          ),
          (_, _) => BookingEvents::BookingAddFailed(
            id.to_string(),
            BookingData {
              id: id.to_string(),
              customer: customer.clone(),
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

impl BookingAggregate {
  pub fn validate_booking() -> bool {
    true
  }

  pub fn load_bookings(new_staff: &Staff) -> Vec<BookingData> {
    let dummy = DummyData::load_bookings();

    let mut booking_hs = HashSet::new();
    for booking in &dummy {
      let staff: Vec<Staff> = booking
        .service
        .iter()
        .map(|s| -> Staff { s.staff.clone() })
        .collect();
      if staff.iter().any(|s| new_staff == s) {
        booking_hs.insert(booking.clone());
      }
    }
    Vec::from_iter(booking_hs.into_iter())
  }
}

#[cfg(test)]
mod tests {
  use crate::models::booking_models::booking_aggregate::BookingAggregate;
  use crate::models::booking_models::booking_data::BookingData;
  use crate::models::booking_models::booking_data::ServiceInfo;
  use crate::models::customer_models::customer_data::Customer;
  use crate::models::service_models::service_data::Service;
  use crate::models::staff_models::staff_data::Staff;
  use chrono::prelude::*;

  #[test]
  fn test_handle_commands() {
    let booking = BookingData {
      id: "non-id".to_owned(),
      customer: Customer {
        id: "customer-04".to_owned(),
        name: "Thai Ngo".to_owned(),
        phone_number: "0123456780".to_owned(),
        address: "HoChiMinh".to_owned(),
        email: "thaingo@gmail.com".to_owned(),
      },
      service: vec![
        ServiceInfo {
          staff: Staff {
            id: "staff-01".to_owned(),
            name: "Rin".to_owned(),
            services: vec![Service {
              id: "service-01".to_owned(),
              name: "HairCut".to_owned(),
              duration: 20,
            }],
          },
          service: Service {
            id: "service-01".to_owned(),
            name: "Haircut".to_owned(),
            duration: 20,
          },
          time_start: Utc.ymd(2020, 10, 5).and_hms(8, 0, 0),
          time_end: Utc.ymd(2020, 10, 5).and_hms(8, 20, 0),
        },
        ServiceInfo {
          staff: Staff {
            id: "staff-02".to_owned(),
            name: "Halu".to_owned(),
            services: vec![Service {
              id: "service-02".to_owned(),
              name: "Manicure".to_owned(),
              duration: 30,
            }],
          },
          service: Service {
            id: "service-02".to_owned(),
            name: "Manicure".to_owned(),
            duration: 30,
          },
          time_start: Utc.ymd(2020, 10, 5).and_hms(8, 20, 0),
          time_end: Utc.ymd(2020, 10, 5).and_hms(8, 50, 0),
        },
      ],
      created_date: Utc.ymd(2020, 10, 1).and_hms(0, 0, 0),
      booking_date: Utc.ymd(2020, 10, 5).and_hms(8, 0, 0),
    };
  }
}
