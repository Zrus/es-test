mod models;

use models::booking_models::booking_commands::BookingCommands;
use models::booking_models::booking_data::{BlockData, BookingData};
use models::customer_models::customer_data::Customer;
use models::service_models::service_data::Service;
use models::staff_models::staff_data::Staff;

use chrono::prelude::*;

fn main() {
  let booking = BookingData::new(
    Customer {
      id: "customer123".to_owned(),
      name: "Tuong".to_owned(),
      phone_number: "0909909009".to_owned(),
      address: "HoChiMinh".to_owned(),
      email: "zentech.demo@gmail.com".to_owned(),
    },
    Staff {
      id: "staff123".to_owned(),
      name: "Tri".to_owned(),
      services: vec![Service {
        id: "service123".to_owned(),
        name: "HairCut".to_owned(),
        duration: 15,
      }],
    },
    Service {
      id: "service123".to_owned(),
      name: "HairCut".to_owned(),
      duration: 15,
    },
    Utc::now().round_subsecs(0),
    Utc.ymd(2020, 12, 31).and_hms(9, 15, 00),
  );
  println!("{:#?}", booking);
  let block = BlockData::new(
    Staff {
      id: "staff123".to_owned(),
      name: "Tri".to_owned(),
      services: vec![Service {
        id: "service123".to_owned(),
        name: "HairCut".to_owned(),
        duration: 15,
      }],
    },
    Utc.ymd(2020, 12, 31).and_hms(13, 15, 00),
    Utc.ymd(2020, 12, 31).and_hms(18, 00, 00),
  );
  println!("{:#?}", block);
  let command = BookingCommands::add_booking(
    Customer {
      id: "customer123".to_owned(),
      name: "Tuong".to_owned(),
      phone_number: "0909909009".to_owned(),
      address: "HoChiMinh".to_owned(),
      email: "zentech.demo@gmail.com".to_owned(),
    },
    Staff {
      id: "staff123".to_owned(),
      name: "Tri".to_owned(),
      services: vec![Service {
        id: "service123".to_owned(),
        name: "HairCut".to_owned(),
        duration: 15,
      }],
    },
    Service {
      id: "service123".to_owned(),
      name: "HairCut".to_owned(),
      duration: 15,
    },
    Utc::now().round_subsecs(0),
    Utc.ymd(2020, 12, 31).and_hms(9, 15, 00),
  );
  println!("{:#?}", command);
}
