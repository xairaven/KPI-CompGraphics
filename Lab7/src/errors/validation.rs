use crate::ui::windows::message::MessageWindow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FractalValidationError {
    #[error("Axiom value is empty.")]
    AxiomIsEmpty,

    #[error("The angle value is either greater than 360 degrees or less than 0 degrees.")]
    WrongAngleValue,

    #[error("The 'iterations' value is lower than 1.")]
    WrongIterationsValue,

    #[error("Wrong syntax. Symbols ' -> ' are not found in the rule.")]
    WrongRuleSyntax(String),

    #[error("Rule constant consists of multiple symbols.")]
    RuleConstantConsistsOfMultipleSymbols(String),
    #[error("Rule constant consists of less than 1 symbol.")]
    RuleConstantIsEmpty(String),
    #[error("Rule constant is not a valid UTF-8 symbol.")]
    RuleConstantIsNotValidChar(String),

    #[error("Rule condition consists of less than 1 symbol.")]
    RuleConditionIsEmpty(String),

    #[error("There's symbol in a rule that is not from an alphabet.")]
    SymbolNotFromAlphabet(String),
}

impl FractalValidationError {
    pub fn additional_info(&self) -> Option<String> {
        match self {
            Self::WrongRuleSyntax(value)
            | Self::RuleConstantConsistsOfMultipleSymbols(value)
            | Self::RuleConstantIsEmpty(value)
            | Self::RuleConstantIsNotValidChar(value)
            | Self::RuleConditionIsEmpty(value)
            | Self::SymbolNotFromAlphabet(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn window(&self) -> MessageWindow {
        let mut message = format!("Validation error: {}", self);
        if let Some(additional_info) = self.additional_info() {
            message += &format!("\n\nAdditional Info:\n{}", additional_info);
        }

        MessageWindow::default()
            .with_message(message)
            .with_name("Error ❎")
            .with_height(500.0)
            .with_width(300.0)
            .with_collapsible(false)
    }
}
