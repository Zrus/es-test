use chrono::prelude::*;
use serde::*;
use std::fmt;
use uuid::Uuid;
/// An event sourcing error
#[derive(Debug)]
pub struct Error {
  pub kind: Kind,
}

pub trait EventStore {
  fn append(&self, evt: impl Event) -> Result<CloudEvent>;
  fn get_events_with_stream(&self, stream_id: &str) -> Result<Vec<CloudEvent>>;
  fn get_range_events_with_stream(
    &self,
    stream_id: &str,
    start: i64,
    end: i64,
  ) -> Result<Vec<CloudEvent>>;
  fn events_stream_in_day(&self, stream_id: &str, date: DateTime<Local>)
    -> Result<Vec<CloudEvent>>;
  fn get_events_from_index(&self, stream_id: &str, from_index: i64) -> Result<Vec<CloudEvent>>;
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    "An eventsourcing error ocurred"
  }

  fn cause(&self) -> Option<&dyn std::error::Error> {
    None
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.kind {
      Kind::ApplicationFailure(ref s) => fmt::Display::fmt(s, f),
      Kind::CommandFailure(ref s) => fmt::Display::fmt(s, f),
      Kind::StoreFailure(ref s) => fmt::Display::fmt(s, f),
    }
  }
}

/// Indicates the kind of event sourcing error that occurred.
#[derive(Debug)]
pub enum Kind {
  ApplicationFailure(String),
  CommandFailure(String),
  StoreFailure(String),
}

/// A Result where failure is an event sourcing error
pub type Result<T> = std::result::Result<T, Error>;

pub trait Event: Serialize {
  fn event_type_version(&self) -> &str;
  fn event_type(&self) -> &str;
  fn event_stream_sequence(&self) -> i64;
  fn event_stream_id(&self) -> &str;
}
/// CloudEvent provides a data structure that is JSON-compliant with v1.0 of the CloudEvents
/// specification. This means that any system with which you want to communicate that is
/// also CloudEvents-aware can accept the serialized version of this data structure.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloudEvent {
  #[serde(rename = "specversion")]
  pub cloud_events_version: String,
  #[serde(rename = "eventtype")]
  pub event_type: String,
  #[serde(rename = "typeversion")]
  pub event_type_version: String,
  #[serde(rename = "id")]
  pub event_id: String,
  #[serde(rename = "time")]
  pub event_time: i64,
  #[serde(rename = "datacontenttype")]
  pub content_type: String,
  #[serde(rename = "streamid")]
  pub event_stream_id: String, //Aggregate ID
  #[serde(rename = "streamsequence")]
  pub stream_sequence: i64,
  pub data: serde_json::Value,
}

impl<E> From<E> for CloudEvent
where
  E: Event,
{
  fn from(source: E) -> Self {
    let raw_data = serde_json::to_string(&source).unwrap();

    CloudEvent {
      cloud_events_version: "1.0".to_owned(),
      event_type: source.event_type().to_owned(),
      event_type_version: source.event_type_version().to_owned(),
      event_id: Uuid::new_v4().to_hyphenated().to_string(),
      event_time: Utc::now().timestamp_millis(),
      content_type: "application/json".to_owned(),
      event_stream_id: source.event_stream_id().to_owned(),
      stream_sequence: source.event_stream_sequence(),
      data: serde_json::from_str(&raw_data).unwrap(),
    }
  }
}
