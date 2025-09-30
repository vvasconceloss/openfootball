use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
  #[error("An error occurred while validating the entity {entity}: {reason}")]
  ValidationError {
    entity: &'static str,
    reason: String
  },
  #[error("An unknown internal error has occurred")]
  Unknown
}