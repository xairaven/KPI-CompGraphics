use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Probability have to be in range 0..=1")]
    BadProbability(String),

    #[error("Probability sum have to be lower than 1")]
    BadProbabilitySum(String),
}
