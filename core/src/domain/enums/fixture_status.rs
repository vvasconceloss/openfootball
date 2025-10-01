use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FixtureStatus {
  Played,
  Scheduled,
  Postponed,
}