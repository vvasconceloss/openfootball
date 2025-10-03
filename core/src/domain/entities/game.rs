use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::{
  entities::{
    club::Club, 
    fixture::Fixture, 
    player::Player, 
    season::Season
  }, 
  value_objects::save_metadata::SaveMetadata
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
  pub metadata: SaveMetadata,

  pub season: Season,
  pub current_date: NaiveDate,

  pub clubs: Vec<Club>,
  pub players: Vec<Player>,
  pub fixtures: Vec<Fixture>
}