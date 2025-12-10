use thiserror::Error;

pub enum Part {
    First,
    Second,
}
#[derive(Debug, Error)]
pub enum PartError {
    #[error("Invalid Part: {0}")]
    Invalid(String),
    #[error("Missing Part")]
    Missing,
}

pub fn parse_part(arg: Option<String>) -> Result<Part, PartError> {
    match arg.as_deref() {
        Some("1") => Ok(Part::First),
        Some("2") => Ok(Part::Second),
        Some(random) => Err(PartError::Invalid(random.to_string())),
        None => Err(PartError::Missing),
    }
}
