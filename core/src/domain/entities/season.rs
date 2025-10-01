use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::enums::season_status::SeasonStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
  pub id: i32,
  pub end_year: i32,
  pub start_year: i32,
  pub end_date: NaiveDate,
  pub status: SeasonStatus,
  pub start_date: NaiveDate,
  pub competitions: Vec<i32>
}