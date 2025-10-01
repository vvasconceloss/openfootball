use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MatchEventType {
  Goal { assitent_id: i32 },
  RedCard,
  HalfTime,
  PeriodEnd,
  YellowCard,
  PeriodStart,
}