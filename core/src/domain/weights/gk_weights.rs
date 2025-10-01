use crate::domain::value_objects::position_weights::PositionWeights;

pub static GK_WEIGHTS: PositionWeights = PositionWeights {
  diving: 5,
  handling: 5,
  reflexes: 5,
  distribution: 4,

  decision: 5,
  leadership: 3,
  determination: 3,

  pace: 0,
  stamina: 0,
  jumping: 0,
  strength: 0,
  acceleration: 0,

  vision: 0,
  passing: 0,
  heading: 0,
  crossing: 0,
  tackling: 0,
  dribbling: 0,
  finishing: 0,
};