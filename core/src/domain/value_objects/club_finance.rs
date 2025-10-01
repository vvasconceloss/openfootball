use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubFinance {
  pub balance: f32,
  pub salary_budget: f32,
  pub transfer_budget: f32,
}