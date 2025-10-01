use serde::{Deserialize, Serialize};

use crate::{domain::enums::position::Position, errors::{check_range, CoreError}};

const MIN_ATTRIBUTE: u8 = 1;
const MAX_ATTRIBUTE: u8 = 20;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mental {
  pub decision: u32,
  pub leadership: u32,
  pub determination: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Physical {
  pub pace: u32,
  pub stamina: u32,
  pub jumping: u32,
  pub strength: u32,
  pub acceleration: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Technical {
  pub vision: u32,
  pub passing: u32,
  pub heading: u32,
  pub crossing: u32,
  pub tackling: u32,
  pub dribbling: u32,
  pub finishing: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Goalkeeping {
  pub diving: u32,
  pub handling: u32,
  pub reflexes: u32,
  pub distribution: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerAttributes {
  pub id: i32,
  pub mental: Mental,
  pub physical: Physical,
  pub technical: Technical
}

impl Mental {
  pub fn new(decision: u32, leadership: u32, determination: u32) -> Result<Self, CoreError> {
    let entity_name = "Mental";
    
    check_range(decision.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(leadership.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(determination.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;

    Ok( Mental { decision, leadership, determination } )
  }

  pub fn calc_weighted_score(&self, position: &Position) -> Result<u32, CoreError> {
    let weight = Position::get_weights(position);

    let total_mental: u32 = ((self.decision * weight.decision) 
      + (self.determination * weight.determination) 
      + (self.leadership * weight.leadership)).into();

    Ok(total_mental.into())
  }
}

impl Physical {
  pub fn new(pace: u32, stamina: u32, jumping: u32, strength: u32, acceleration: u32) -> Result<Self, CoreError> {
    let entity_name = "Physical";
    
    check_range(pace.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(stamina.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(jumping.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(strength.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(acceleration.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;

    Ok( Physical { pace, stamina, jumping, strength, acceleration } )
  }

  pub fn calc_weighted_score(&self, position: &Position) -> Result<u32, CoreError> {
    let weight = Position::get_weights(position);

    let total_physical: u32 = ((self.acceleration * weight.acceleration) 
      + (self.jumping * weight.jumping) 
      + (self.pace * weight.pace) 
      + (self.stamina * weight.stamina) 
      + (self.strength * weight.strength)).into();

    Ok(total_physical.into())
  }
}

impl Technical {
  pub fn new(vision: u32, passing: u32, heading: u32, crossing: u32, tackling: u32, dribbling: u32, finishing: u32) -> Result<Self, CoreError> {
    let entity_name = "Technical";
    
    check_range(vision.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(passing.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(heading.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(crossing.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(tackling.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(dribbling.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(finishing.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;

    Ok( Technical { vision, passing, heading, crossing, tackling, dribbling, finishing } )
  }

  pub fn calc_weighted_score(&self, position: &Position) -> Result<u32, CoreError> {
    let weight = Position::get_weights(position);

    let total_technical: u32 = ((self.crossing * weight.crossing) 
      + (self.dribbling * weight.dribbling) 
      + (self.finishing * weight.finishing) 
      + (self.heading * weight.heading) 
      + (self.passing * weight.passing) 
      + (self.tackling * weight.passing) 
      + (self.vision * weight.vision)).into();

    Ok(total_technical.into())
  }
}

impl Goalkeeping {
  pub fn new(diving: u32, handling: u32, reflexes: u32, distribution: u32  ) -> Result<Self, CoreError> {
    let entity_name = "Goalkeeping";

    check_range(diving.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(handling.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(reflexes.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(distribution.try_into().unwrap(), MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;

    Ok( Goalkeeping { diving, handling, reflexes, distribution } )
  }

  pub fn calc_weighted_score(&self, position: &Position) -> Result<u32, CoreError> {
    let weight = Position::get_weights(position);

    let total_goalkeeping: u32 = ((self.distribution * weight.distribution) 
      + (self.diving * weight.diving) 
      + (self.handling * weight.handling) 
      + (self.reflexes * weight.reflexes)).into();

    Ok(total_goalkeeping.into())
  }
}

impl PlayerAttributes {
  pub fn new(id: i32, mental: Mental, physical: Physical, technical: Technical) -> Result<Self, CoreError> {
    Ok( PlayerAttributes { id, mental, physical, technical })
  }
}