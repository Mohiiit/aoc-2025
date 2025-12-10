use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputPathError {
    #[error("Missing Input Path")]
    Missing,
    #[error("Unable to read input from: {0}")]
    UnableToReadFrom(String),
    #[error("Parse Error: {0}")]
    ParseError(InstructionParseError),
}

#[derive(Debug, Error)]
pub enum InstructionParseError {
    #[error("Wrong Direction given: {0}")]
    WrongDirection(char),
    #[error("No Direction given")]
    NoDirection,
    #[error("Steps parsing failed")]
    StepsParsingFailed,
}
