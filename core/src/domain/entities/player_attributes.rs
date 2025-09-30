use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mental {
  pub decision: u8,
  pub leadership: u8,
  pub determination: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Physical {
  pub decision: u8,
  pub leadership: u8,
  pub determination: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Technical {
  pub vision: u8,
  pub passing: u8,
  pub heading: u8,
  pub crossing: u8,
  pub tackling: u8,
  pub dribbling: u8,
  pub finishing: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerAttributes {
  pub id: i32,
  pub mental: Mental,
  pub physical: Physical,
  pub technical: Technical
}