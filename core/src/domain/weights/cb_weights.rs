use crate::domain::value_objects::position_weights::PositionWeights;

pub static CB_WEIGHTS: PositionWeights = PositionWeights {
  diving: 1,
  handling: 1,
  reflexes: 1,
  distribution: 1,

  decision: 5,
  leadership: 3,
  determination: 3,

  pace: 4,
  stamina: 3,
  jumping: 5,
  strength: 5,
  acceleration: 4,

  vision: 3,
  passing: 2,
  heading: 4,
  crossing: 1,
  tackling: 5,
  dribbling: 1,
  finishing: 1,
};