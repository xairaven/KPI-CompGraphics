use crate::errors::validation::FractalValidationError;
use crate::model::view::FractalViewModel;

pub struct FractalValidator {
    reserved_terminals: Vec<char>,
    rule_delimiter: String,
}

impl Default for FractalValidator {
    fn default() -> Self {
        Self {
            reserved_terminals: vec!['F', '+', '-'],
            rule_delimiter: String::from(" -> "),
        }
    }
}

impl FractalValidator {
    pub fn check(&self, view_model: &FractalViewModel) -> Result<(), FractalValidationError> {
        self.axiom_is_not_empty(&view_model.axiom)?;
        self.angle(view_model.angle)?;
        self.iterations(view_model.iterations)?;
        self.rules(&view_model.rules)?;

        Ok(())
    }

    fn axiom_is_not_empty(&self, axiom: &str) -> Result<(), FractalValidationError> {
        if axiom.is_empty() {
            return Err(FractalValidationError::AxiomIsEmpty);
        }

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
        let mut alphabet: Vec<char> = Vec::new();
        let mut conditions: Vec<String> = Vec::new();

        for (index, line) in rules.iter().enumerate() {
            self.right_rule_syntax(line, index)?;

            let (letter, condition) = line.split_once(self.rule_delimiter.as_str()).ok_or(
                FractalValidationError::WrongRuleSyntax(format!("Rule: {}", index + 1)),
            )?;

            self.rule_constant_single_symbol(letter, index)?;
            let letter = self.is_valid_char(letter, index)?;
            alphabet.push(letter);

            self.rule_condition_not_empty(condition, index)?;
            conditions.push(condition.to_string());
        }

        self.all_symbols_in_alphabet(&alphabet, &conditions)?;

        Ok(())
    }

    fn right_rule_syntax(&self, rule: &str, index: usize) -> Result<(), FractalValidationError> {
        if rule.len() < 5 || !rule[1..=4].eq(self.rule_delimiter.as_str()) {
            return Err(FractalValidationError::WrongRuleSyntax(format!(
                "Rule: {}",
                index + 1
            )));
        }

        Ok(())
    }

    fn rule_constant_single_symbol(
        &self, letter: &str, index: usize,
    ) -> Result<(), FractalValidationError> {
        if letter.len() > 1 {
            return Err(
                FractalValidationError::RuleConstantConsistsOfMultipleSymbols(format!(
                    "Rule: {}",
                    index + 1
                )),
            );
        }

        if letter.is_empty() {
            return Err(FractalValidationError::RuleConstantIsEmpty(format!(
                "Rule: {}",
                index + 1
            )));
        }

        Ok(())
    }

    fn is_valid_char(&self, letter: &str, index: usize) -> Result<char, FractalValidationError> {
        letter
            .chars()
            .nth(0)
            .ok_or(FractalValidationError::RuleConstantIsNotValidChar(format!(
                "Rule: {}",
                index + 1
            )))
    }

    fn rule_condition_not_empty(
        &self, condition: &str, index: usize,
    ) -> Result<(), FractalValidationError> {
        if condition.is_empty() {
            return Err(FractalValidationError::RuleConditionIsEmpty(format!(
                "Rule: {}",
                index + 1
            )));
        }

        Ok(())
    }

    fn all_symbols_in_alphabet(
        &self, alphabet: &[char], conditions: &[String],
    ) -> Result<(), FractalValidationError> {
        for (index, condition) in conditions.iter().enumerate() {
            for symbol in condition.chars() {
                if !alphabet.contains(&symbol) && !self.reserved_terminals.contains(&symbol) {
                    return Err(FractalValidationError::SymbolNotFromAlphabet(format!(
                        "Rule: {}\nSymbol: {}",
                        index + 1,
                        symbol
                    )));
                }
            }
        }

        Ok(())
    }
}
