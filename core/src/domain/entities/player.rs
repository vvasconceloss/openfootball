use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::entities::player_attributes::PlayerAttributes;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
  pub id: i32,
  pub overall: u8,
  pub potential: u8,
  pub nation_id: i32,
  pub last_name: String,
  pub first_name: String,
  pub birth_date: NaiveDate,
  pub attributes: PlayerAttributes
}

impl Player {
  pub fn new(
    id: i32, 
    overall: u8, 
    potential: u8, 
    nation_id: i32, 
    last_name: impl Into<String>, 
    first_name: impl Into<String>,
    birth_date: NaiveDate,
    attributes: PlayerAttributes
  ) -> Self {
    Self { 
      id, 
      overall, 
      potential, 
      nation_id, 
      last_name: last_name.into(), 
      first_name: first_name.into(),
      birth_date,
      attributes
    }
  }
}