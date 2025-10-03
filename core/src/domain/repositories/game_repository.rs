use uuid::Uuid;
use std::{io::Error, path::PathBuf};

use crate::domain::{entities::game::Game, value_objects::save_metadata::SaveMetadata};

pub trait GameRepository {
  fn load(&self, id: Uuid) -> Result<Game, Error>;
  fn save(&self, game: &Game) -> Result<PathBuf, Error>;
  fn creat(&self, metadata: SaveMetadata) -> Result<Game, Error>;
}