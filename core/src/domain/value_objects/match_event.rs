use serde::{Deserialize, Serialize};

use crate::domain::enums::match_event_type::MatchEventType;

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchEvent {
  pub minute: u8,
  pub player_id: Option<i32>,
  pub event_type: MatchEventType
}