use serde::{Deserialize, Serialize};

use crate::domain::value_objects::match_event::MatchEvent;

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchStats {
  pub id: i32,
  pub home_goals: u8,
  pub away_goals: u8,
  pub total_shots_home: u8,
  pub total_shots_away: u8,
  pub home_possession: f32,
  pub away_possession: f32,
  pub match_events: Vec<MatchEvent>
}