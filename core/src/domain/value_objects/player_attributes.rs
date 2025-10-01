use serde::{Deserialize, Serialize};

use crate::{domain::enums::position::Position, errors::{check_range, CoreError}};

const MIN_ATTRIBUTE: u8 = 1;
const MAX_ATTRIBUTE: u8 = 20;

#[derive(Debug, Serialize, Deserialize)]
pub struct Mental {
  pub decision: u8,
  pub leadership: u8,
  pub determination: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Physical {
  pub pace: u8,
  pub stamina: u8,
  pub jumping: u8,
  pub strength: u8,
  pub acceleration: u8,
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
pub struct Goalkeeping {
  pub diving: u8,
  pub handling: u8,
  pub reflexes: u8,
  pub distribution: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerAttributes {
  pub id: i32,
  pub mental: Mental,
  pub physical: Physical,
  pub technical: Technical
}

impl Mental {
  pub fn new(decision: u8, leadership: u8, determination: u8) -> Result<Self, CoreError> {
    let entity_name = "Mental";
    
    check_range(decision, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(leadership, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(determination, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;

    Ok( Mental { decision, leadership, determination } )
  }

  pub fn calc_weighted_score(&self, position: &Position) -> Result<u32, CoreError> {
    let weight = Position::get_weights(position);

    let total_mental = (self.decision * weight.decision) 
      + (self.determination * weight.determination) 
      + (self.leadership * weight.leadership);

    Ok(total_mental.into())
  }
}

impl Physical {
  pub fn new(pace: u8, stamina: u8, jumping: u8, strength: u8, acceleration: u8) -> Result<Self, CoreError> {
    let entity_name = "Physical";
    
    check_range(pace, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(stamina, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(jumping, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(strength, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(acceleration, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;

    Ok( Physical { pace, stamina, jumping, strength, acceleration } )
  }

  pub fn calc_weighted_score(&self, position: &Position) -> Result<u32, CoreError> {
    let weight = Position::get_weights(position);

    let total_physical = (self.acceleration * weight.acceleration) 
      + (self.jumping * weight.jumping) 
      + (self.pace * weight.pace) 
      + (self.stamina * weight.stamina) 
      + (self.strength * weight.strength);

    Ok(total_physical.into())
  }
}

impl Technical {
  pub fn new(vision: u8, passing: u8, heading: u8, crossing: u8, tackling: u8, dribbling: u8, finishing: u8) -> Result<Self, CoreError> {
    let entity_name = "Technical";
    
    check_range(vision, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(passing, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(heading, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(crossing, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(tackling, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(dribbling, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(finishing, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;

    Ok( Technical { vision, passing, heading, crossing, tackling, dribbling, finishing } )
  }

  pub fn calc_weighted_score(&self, position: &Position) -> Result<u32, CoreError> {
    let weight = Position::get_weights(position);

    let total_technical = (self.crossing * weight.crossing) 
      + (self.dribbling * weight.dribbling) 
      + (self.finishing * weight.finishing) 
      + (self.heading * weight.heading) 
      + (self.passing * weight.passing) 
      + (self.tackling * weight.passing) 
      + (self.vision * weight.vision);

    Ok(total_technical.into())
  }
}

impl Goalkeeping {
  pub fn new(diving: u8, handling: u8, reflexes: u8, distribution: u8  ) -> Result<Self, CoreError> {
    let entity_name = "Goalkeeping";

    check_range(diving, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(handling, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(reflexes, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;
    check_range(distribution, MIN_ATTRIBUTE, MAX_ATTRIBUTE, entity_name)?;

    Ok( Goalkeeping { diving, handling, reflexes, distribution } )
  }

  pub fn calc_weighted_score(&self, position: &Position) -> Result<u32, CoreError> {
    let weight = Position::get_weights(position);

    let total_goalkeeping = (self.distribution * weight.distribution) 
      + (self.diving * weight.diving) 
      + (self.handling * weight.handling) 
      + (self.reflexes * weight.reflexes);

    Ok(total_goalkeeping.into())
  }
}

impl PlayerAttributes {
  pub fn new(id: i32, mental: Mental, physical: Physical, technical: Technical) -> Result<Self, CoreError> {
    Ok( PlayerAttributes { id, mental, physical, technical })
  }
}