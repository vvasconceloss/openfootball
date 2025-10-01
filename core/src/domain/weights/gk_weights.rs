use crate::domain::value_objects::position_weights::PositionWeights;

pub static GK_WEIGHTS: PositionWeights = PositionWeights {
  diving: 5,
  handling: 5,
  reflexes: 5,
  distribution: 4,

  decision: 5,
  leadership: 3,
  determination: 3,

  pace: 1,
  stamina: 3,
  jumping: 5,
  strength: 3,
  acceleration: 1,

  vision: 2,
  passing: 2,
  heading: 1,
  crossing: 1,
  tackling: 1,
  dribbling: 1,
  finishing: 1,
};