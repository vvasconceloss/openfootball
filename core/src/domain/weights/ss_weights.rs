use crate::domain::value_objects::position_weights::PositionWeights;

pub static SS_WEIGHTS: PositionWeights = PositionWeights {
  diving: 1,
  handling: 1,
  reflexes: 1,
  distribution: 1,

  decision: 5,
  leadership: 2,
  determination: 3,

  pace: 4,
  stamina: 3,
  jumping: 2,
  strength: 3,
  acceleration: 5,

  vision: 4,
  passing: 4,
  heading: 1,
  crossing: 1,
  tackling: 1,
  dribbling: 4,
  finishing: 5,
};