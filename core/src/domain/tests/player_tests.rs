#[cfg(test)]
mod tests {
  use std::error::Error;
  use chrono::NaiveDate;

  use crate::domain::{
    entities::player::Player, 
    enums::position::Position, 
    value_objects::player_attributes::{
      Mental, 
      Physical, 
      PlayerAttributes, 
      Technical
    }
  };

  #[test]
  fn test_calc_overall() -> Result<(), Box<dyn Error>> {
    let mental = Mental::new(15, 15, 15)?;
    let physical = Physical::new(15, 15, 15, 15, 15)?;
    let technical = Technical::new(15, 15, 15, 15, 15, 15, 15)?;

    let attributes = PlayerAttributes::new(1, mental, physical, technical)?;

    let birth_date = NaiveDate::parse_from_str("08-07-2006", "%d-%m-%Y")?;

    let primary_position = Position::Striker;
    let mut secondary_positions = Vec::new();

    secondary_positions.push(Position::LeftWinger);
    secondary_positions.push(Position::RightWinger);

    let player = Player::new(
      1, 
      20, 
      1, 
      "Yamal".into(), 
      "Lamine".into(), 
      birth_date,
      attributes,
      primary_position,
      secondary_positions, 
      None, 
      None, 
      None, 
      None
    )?;

    let expected_overall: u8 = 15;

    assert_eq!(
      player.overall,
      expected_overall,
      "The overall does not correspond to the expected value."
    );

    Ok(())
  }
}