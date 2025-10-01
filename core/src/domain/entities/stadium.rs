use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stadium {
  pub id: i32,
  pub name: String,
  pub capacity: i32,
  pub nation_id: i32
}