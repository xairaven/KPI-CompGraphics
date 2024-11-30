use crate::errors::validation::FractalValidationError;
use crate::model::view::FractalViewModel;

#[derive(Default)]
pub struct FractalValidator {}

impl FractalValidator {
    pub fn check(&self, view_model: &FractalViewModel) -> Result<(), FractalValidationError> {
        self.angle(view_model.angle)?;
        self.iterations(view_model.iterations)?;
        self.rules(&view_model.rules)?;

        Ok(())
    }

    fn angle(&self, angle: f32) -> Result<(), FractalValidationError> {
        if !(0.0..=360.0).contains(&angle) {
            return Err(FractalValidationError::WrongAngleValue);
        }

        Ok(())
    }

    fn iterations(&self, iterations: usize) -> Result<(), FractalValidationError> {
        if iterations < 1 {
            return Err(FractalValidationError::WrongIterationsValue);
        }

        Ok(())
    }

    fn rules(&self, rules: &[String]) -> Result<(), FractalValidationError> {
        for (index, line) in rules.iter().enumerate() {
            self.rule(line, index)?
        }

        Ok(())
    }

    fn rule(&self, rule: &str, index: usize) -> Result<(), FractalValidationError> {
        if rule.len() < 5 || !rule[1..=4].eq(" -> ") {
            return Err(FractalValidationError::WrongRuleSyntax(format!(
                "Rule: {}",
                index + 1
            )));
        }

        Ok(())
    }
}
