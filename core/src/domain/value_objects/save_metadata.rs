use uuid::Uuid;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveMetadata {
  pub id: Uuid,
  pub name: String,
  pub version: u32,
  pub created_at: NaiveDate,
  pub engine_version: String
}