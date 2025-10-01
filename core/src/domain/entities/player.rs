use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{domain::{enums::position::Position, value_objects::player_attributes::{Goalkeeping, PlayerAttributes}}, errors::CoreError};

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
  pub goalkeeping: Option<Goalkeeping>,
  
  pub primary_position: Position,
  pub secondary_positions: Vec<Position>,
}

impl Player {
  pub fn calc_overall(position: &Position, attributes: &PlayerAttributes, goalkeeping: &Option<Goalkeeping>) -> Result<f32, CoreError> {
    let weighted_avg: f32;
    let weights_sum: u32 = Position::calc_weights_sum(position)?;

    match position {
      Position::Goalkeeper => {
        let gk_data = goalkeeping.as_ref().unwrap(); 
        let score_goalkeeping = gk_data.calc_weighted_score(position)?;

        weighted_avg = (score_goalkeeping / weights_sum) as f32;
      }
      _ => {
        let score_mental = attributes.mental.calc_weighted_score(position)?;
        let score_physical = attributes.physical.calc_weighted_score(position)?;
        let score_technical = attributes.technical.calc_weighted_score(position)?;

        let total_weighting: u32 = score_mental + score_physical + score_technical;
    
        weighted_avg = (total_weighting / weights_sum) as f32;
      }
    }

    Ok(weighted_avg.ceil())
  }

  pub fn new(
    id: i32, 
    potential: u8, 
    nation_id: i32, 
    last_name: String, 
    first_name: String,
    birth_date: NaiveDate,
    attributes: PlayerAttributes,
    primary_position: Position,
    secondary_positions: Vec<Position>,
    diving: Option<u32>,
    handling: Option<u32>,
    reflexes: Option<u32>,
    distribution: Option<u32>
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

    let is_goalkeeper: Option<Goalkeeping>;

    match primary_position {
      Position::Goalkeeper => {
        if diving.is_none() || handling.is_none() || reflexes.is_none() || distribution.is_none() {
          return Err(CoreError::ValidationError { 
            entity: "Player", 
            reason: "Goalkeeper attributes are required for GK position.".to_string()
          })
        }

        let goalkeeper = Goalkeeping::new(
          diving.unwrap(), 
          handling.unwrap(), 
          reflexes.unwrap(), 
          distribution.unwrap()
        )?;

        is_goalkeeper = Some(goalkeeper);
      },
      _ => {
        is_goalkeeper = None;
      }
    }

    let overall = Player::calc_overall(&primary_position, &attributes, &is_goalkeeper)? as u8;

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
      secondary_positions,
      goalkeeping: is_goalkeeper
    } )
  }
}