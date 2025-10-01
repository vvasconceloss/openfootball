use crate::domain::value_objects::position_weights::PositionWeights;

pub static CM_WEIGHTS: PositionWeights = PositionWeights {
  diving: 1,
  handling: 1,
  reflexes: 1,
  distribution: 1,

  decision: 5,
  leadership: 3,
  determination: 3,

  pace: 4,
  stamina: 5,
  jumping: 3,
  strength: 3,
  acceleration: 4,

  vision: 5,
  passing: 5,
  heading: 3,
  crossing: 1,
  tackling: 4,
  dribbling: 4,
  finishing: 3,
};