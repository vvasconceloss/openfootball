use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Position {
  Goalkeeper,
  CenterBack,
  LeftBack,
  RightBack,

  DefensiveMidfielder,
  CentralMidfielder,
  AttackingMidfielder,
  LeftMidfielder,
  RightMidfielder,

  LeftWinger,
  RightWinger,
  Striker,
  SecondStriker
}