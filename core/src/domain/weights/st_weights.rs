use crate::domain::value_objects::position_weights::PositionWeights;

pub static ST_WEIGHTS: PositionWeights = PositionWeights {
  diving: 1,
  handling: 1,
  reflexes: 1,
  distribution: 1,

  decision: 5,
  leadership: 2,
  determination: 3,

  pace: 4,
  stamina: 3,
  jumping: 4,
  strength: 4,
  acceleration: 5,

  vision: 3,
  passing: 3,
  heading: 4,
  crossing: 1,
  tackling: 1,
  dribbling: 3,
  finishing: 5,
};