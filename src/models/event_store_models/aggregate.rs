use super::cloud_event::*;

pub trait Aggregate {
  type Event: Event;
  type Command;
  type State: AggregateState + Clone;

  fn apply_event(state: &Self::State, evt: &Self::Event) -> Result<Self::State>;
  fn handle_command(state: &Self::State, cmd: &Self::Command) -> Result<Vec<Self::Event>>;
  fn aggregate_id(&self) -> String;

  fn apply_all(state: &Self::State, evts: &[Self::Event]) -> Result<Self::State> {
    Ok(evts.iter().fold(state.clone(), |acc_state, event| {
      Self::apply_event(&acc_state, event).unwrap()
    }))
  }
}

pub trait AggregateState {
  fn generation(&self) -> u64;
}
