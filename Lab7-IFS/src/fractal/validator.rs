use crate::errors::validation::ValidationError;

pub fn probability_range(systems: &Vec<[f32; 7]>) -> Result<(), ValidationError> {
    for row in systems {
        let probability = &row[6];

        if !(0.0..=1.0).contains(probability) {
            return Err(ValidationError::BadProbability(format!(
                "Value: {}",
                probability
            )));
        }
    }

    Ok(())
}

pub fn probability_sum(systems: &Vec<[f32; 7]>) -> Result<(), ValidationError> {
    let mut sum = 0.0;
    for row in systems {
        sum += row[6];
    }

    if sum > 1.0 {
        return Err(ValidationError::BadProbability(format!(
            "Sum is {:.2}",
            sum
        )));
    }

    Ok(())
}
