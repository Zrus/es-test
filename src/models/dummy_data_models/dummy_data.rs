use chrono::prelude::*;

use super::super::booking_models::blocking_data::BlockData;
use super::super::booking_models::booking_data::BookingData;
use super::super::booking_models::booking_data::ServiceInfo;
use super::super::booking_models::booking_events::BookingEvents;
use super::super::customer_models::customer_data::Customer;
use super::super::service_models::service_data::Service;
use super::super::staff_models::staff_data::Staff;

pub struct DummyData {}
impl DummyData {
  pub fn load_bookings() -> Vec<BookingData> {
    vec![
      BookingData {
        id: "booking-01".to_owned(),
        customer: Customer {
          id: "customer-01".to_owned(),
          name: "Vinh".to_owned(),
          phone_number: "0123456789".to_owned(),
          address: "HoChiMinh".to_owned(),
          email: "vinh@gmail.com".to_owned(),
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
              name: "HairCut".to_owned(),
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
              id: "service-01".to_owned(),
              name: "HairCut".to_owned(),
              duration: 20,
            },
            time_start: Utc.ymd(2020, 10, 5).and_hms(8, 20, 0),
            time_end: Utc.ymd(2020, 10, 5).and_hms(8, 50, 0),
          },
        ],
        created_date: Utc.ymd(2020, 10, 1).and_hms(0, 0, 0),
        booking_date: Utc.ymd(2020, 10, 5).and_hms(8, 0, 0),
      },
      BookingData {
        id: "booking-02".to_owned(),
        customer: Customer {
          id: "customer-02".to_owned(),
          name: "Thong".to_owned(),
          phone_number: "0987654321".to_owned(),
          address: "HoChiMinh".to_owned(),
          email: "thong@gmail.com".to_owned(),
        },
        service: vec![ServiceInfo {
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
          time_start: Utc.ymd(2020, 10, 5).and_hms(10, 0, 0),
          time_end: Utc.ymd(2020, 10, 5).and_hms(10, 30, 0),
        }],
        created_date: Utc.ymd(2020, 10, 1).and_hms(0, 0, 0),
        booking_date: Utc.ymd(2020, 10, 5).and_hms(10, 0, 0),
      },
      BookingData {
        id: "booking-03".to_owned(),
        customer: Customer {
          id: "customer-03".to_owned(),
          name: "Dao Cuong".to_owned(),
          phone_number: "0909909009".to_owned(),
          address: "Chicago".to_owned(),
          email: "daocuong@gmail.com".to_owned(),
        },
        service: vec![ServiceInfo {
          staff: Staff {
            id: "staff-03".to_owned(),
            name: "Tuanbeo".to_owned(),
            services: vec![
              Service {
                id: "service-01".to_owned(),
                name: "Haircut".to_owned(),
                duration: 20,
              },
              Service {
                id: "service-02".to_owned(),
                name: "Manicure".to_owned(),
                duration: 30,
              },
            ],
          },
          service: Service {
            id: "service-02".to_owned(),
            name: "Manicure".to_owned(),
            duration: 30,
          },
          time_start: Utc.ymd(2020, 10, 5).and_hms(11, 0, 0),
          time_end: Utc.ymd(2020, 10, 5).and_hms(11, 30, 0),
        }],
        created_date: Utc.ymd(2020, 10, 1).and_hms(0, 0, 0),
        booking_date: Utc.ymd(2020, 10, 5).and_hms(11, 0, 0),
      },
    ]
  }

  pub fn load_events() -> Vec<BookingEvents> {
    vec![
      BookingEvents::BookingAdded(
        "booking-event-c9cf925d-7a08-4758-ad9e-94e6a676aed7".to_owned(),
        BookingData {
          id: "booking-data-5ec38249-84e4-4a04-847b-d4f4d8f8d577".to_owned(),
          customer: Customer {
            id: "customer123".to_owned(),
            name: "Tuong".to_owned(),
            phone_number: "0909909009".to_owned(),
            address: "HoChiMinh".to_owned(),
            email: "zentech.demo@gmail.com".to_owned(),
          },
          service: vec![ServiceInfo {
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
            time_start: Utc.ymd(2021, 1, 1).and_hms(18, 00, 00),
            time_end: Utc.ymd(2021, 1, 1).and_hms(18, 15, 00),
          }],
          created_date: Utc.ymd(2020, 12, 31).and_hms(13, 15, 00),
          booking_date: Utc.ymd(2021, 1, 1).and_hms(18, 00, 00),
        },
      ),
      BookingEvents::BookingAdded(
        "booking-event-e01a8441-0f33-438b-8a1f-6d0b140f7ee2".to_owned(),
        BookingData {
          id: "booking-data-097cf1a7-4eaa-4347-9015-7cb919b3f1d6".to_owned(),
          customer: Customer {
            id: "customer123".to_owned(),
            name: "Tuong".to_owned(),
            phone_number: "0909909009".to_owned(),
            address: "HoChiMinh".to_owned(),
            email: "zentech.demo@gmail.com".to_owned(),
          },
          service: vec![ServiceInfo {
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
            time_start: Utc.ymd(2021, 1, 2).and_hms(18, 00, 00),
            time_end: Utc.ymd(2021, 1, 2).and_hms(18, 15, 00),
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
          duration: 15,
        }],
      },
      start_time: Utc.ymd(2020, 12, 31).and_hms(13, 15, 00),
      end_time: Utc.ymd(2020, 12, 31).and_hms(18, 00, 00),
    }]
  }
}
