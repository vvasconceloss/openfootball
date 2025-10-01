use serde::{Deserialize, Serialize};

use crate::domain::{value_objects::position_weights::PositionWeights, weights::{am_weights::AM_WEIGHTS, cb_weights::CB_WEIGHTS, cm_weights::CM_WEIGHTS, dm_weights::DM_WEIGHTS, fb_weights::FB_WEIGHTS, gk_weights::GK_WEIGHTS, sm_weights::SM_WEIGHTS, ss_weights::SS_WEIGHTS, st_weights::ST_WEIGHTS, sw_weights::SW_WEIGHTS}};

#[derive(Debug, Serialize, Deserialize)]
pub enum Position {
  Goalkeeper,
  CenterBack,
  LeftBack,
  RightBack,

  DefensiveMidfielder,
  CentralMidfielder,
  AttackingMidfielder,
  LeftMidfielder,
  RightMidfielder,

  LeftWinger,
  RightWinger,
  Striker,
  SecondStriker
}

impl Position {
  pub fn get_weights(&self) -> &PositionWeights {
    match self {
      Position::Goalkeeper => &GK_WEIGHTS,
      Position::CenterBack => &CB_WEIGHTS,
      Position::LeftBack => &FB_WEIGHTS,
      Position::RightBack => &FB_WEIGHTS,
      Position::DefensiveMidfielder => &DM_WEIGHTS,
      Position::CentralMidfielder => &CM_WEIGHTS,
      Position::AttackingMidfielder => &AM_WEIGHTS,
      Position::LeftMidfielder => &SM_WEIGHTS,
      Position::RightMidfielder => &SM_WEIGHTS,
      Position::LeftWinger => &SW_WEIGHTS,
      Position::RightWinger => &SW_WEIGHTS,
      Position::SecondStriker => &SS_WEIGHTS,
      Position::Striker => &ST_WEIGHTS
    }
  }
}