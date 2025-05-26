use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiceError {
    #[error("Invalid dice string")]
    Unparseable,
    #[error("Error parsing dice count")]
    ParseCount,
    #[error("Error missing dice sides")]
    MissingSides,
    #[error("Error parsing dice sides")]
    ParseSides,
    #[error("Error parsing dice modifier")]
    ParseModifier,
}
