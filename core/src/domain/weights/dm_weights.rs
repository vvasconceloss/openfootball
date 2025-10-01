use crate::domain::value_objects::position_weights::PositionWeights;

pub static DM_WEIGHTS: PositionWeights = PositionWeights {
  diving: 1,
  handling: 1,
  reflexes: 1,
  distribution: 1,

  decision: 5,
  leadership: 3,
  determination: 3,

  pace: 3,
  stamina: 5,
  jumping: 3,
  strength: 4,
  acceleration: 3,

  vision: 5,
  passing: 4,
  heading: 3,
  crossing: 1,
  tackling: 5,
  dribbling: 3,
  finishing: 2,
};