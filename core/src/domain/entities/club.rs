use serde::{Deserialize, Serialize};

use crate::domain::{entities::stadium::Stadium, value_objects::club_finance::ClubFinance};

#[derive(Debug, Serialize, Deserialize)]
pub struct Club {
  pub id: i32,
  pub name: String,
  pub nation_id: i32,
  pub reputation: i32,
  pub stadium: Stadium,
  pub finance: ClubFinance,
  pub logo_path: Option<String>
}