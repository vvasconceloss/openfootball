use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::enums::fixture_status::FixtureStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct Fixture {
  pub id: i32,
  pub date: NaiveDate,
  pub home_club_id: i32,
  pub away_club_id: i32,
  pub status: FixtureStatus,
  pub match_stats_id: Option<i32>,
}