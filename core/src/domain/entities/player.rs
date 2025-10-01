use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{domain::{enums::position::Position, value_objects::player_attributes::PlayerAttributes}, errors::CoreError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
  pub id: i32,
  pub overall: u8,
  pub potential: u8,
  pub nation_id: i32,
  pub last_name: String,
  pub first_name: String,
  pub birth_date: NaiveDate,
  pub attributes: PlayerAttributes,
  
  pub primary_position: Position,
  pub secondary_positions: Vec<Position>,
}

impl Player {
  pub fn new(
    id: i32, 
    overall: u8, 
    potential: u8, 
    nation_id: i32, 
    last_name: String, 
    first_name: String,
    birth_date: NaiveDate,
    attributes: PlayerAttributes,
    primary_position: Position,
    secondary_positions: Vec<Position>
  ) -> Result<Self, CoreError> {
    if nation_id < 1 {
      return Err(CoreError::ValidationError { 
        entity: "Player", 
        reason: "The nation ID cannot be negative" .to_string()
      })
    }

    if last_name.trim().is_empty() {
      return Err(CoreError::ValidationError { 
        entity: "Player", 
        reason: "The last name of the player cannot be empty".to_string() 
      });
    }

    if first_name.trim().is_empty() {
      return Err(CoreError::ValidationError { 
        entity: "Player", 
        reason: "The first name of the player cannot be empty".to_string() 
      });
    }

    Ok( Player { 
      id, 
      overall, 
      potential, 
      nation_id, 
      last_name,
      first_name,
      birth_date, 
      attributes,
      primary_position,
      secondary_positions
    } )
  }
}