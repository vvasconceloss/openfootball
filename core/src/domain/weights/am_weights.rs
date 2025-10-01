use crate::domain::value_objects::position_weights::PositionWeights;

pub static AM_WEIGHTS: PositionWeights = PositionWeights {
  diving: 1,
  handling: 1,
  reflexes: 1,
  distribution: 1,

  decision: 5,
  leadership: 3,
  determination: 3,

  pace: 3,
  stamina: 3,
  jumping: 1,
  strength: 2,
  acceleration: 4,

  vision: 5,
  passing: 5,
  heading: 1,
  crossing: 2,
  tackling: 2,
  dribbling: 5,
  finishing: 4,
};