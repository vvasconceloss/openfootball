use crate::domain::value_objects::position_weights::PositionWeights;

pub static SW_WEIGHTS: PositionWeights = PositionWeights {
  diving: 1,
  handling: 1,
  reflexes: 1,
  distribution: 1,

  decision: 4,
  leadership: 2,
  determination: 3,

  pace: 5,
  stamina: 4,
  jumping: 1,
  strength: 2,
  acceleration: 5,

  vision: 3,
  passing: 3,
  heading: 1,
  crossing: 4,
  tackling: 1,
  dribbling: 5,
  finishing: 4,
};