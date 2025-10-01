use crate::domain::value_objects::position_weights::PositionWeights;

pub static FB_WEIGHTS: PositionWeights = PositionWeights {
  diving: 1,
  handling: 1,
  reflexes: 1,
  distribution: 1,

  decision: 4,
  leadership: 2,
  determination: 3,

  pace: 5,
  stamina: 5,
  jumping: 2,
  strength: 3,
  acceleration: 4,

  vision: 3,
  passing: 3,
  heading: 2,
  crossing: 4,
  tackling: 5,
  dribbling: 3,
  finishing: 1,
};