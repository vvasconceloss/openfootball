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

pub fn check_range(value: u8, min: u8, max: u8, entity_name: &'static String) -> Result<(), CoreError> {
  let reason_string = format!("Must be in range {min} to {max}");

  if value < min || value > max {
    return Err(CoreError::ValidationError { 
      entity: entity_name, 
      reason: reason_string.to_string()
    });
  }

  Ok(())
}