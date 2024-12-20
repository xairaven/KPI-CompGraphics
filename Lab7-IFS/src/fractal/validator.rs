use crate::errors::validation::ValidationError;

pub fn probability(p: f32) -> Result<(), ValidationError> {
    if !(0.0..=1.0).contains(&p) {
        return Err(ValidationError::BadProbability(format!("Value: {}", p)));
    }

    Ok(())
}
