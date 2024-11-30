use crate::ui::windows::message::MessageWindow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FractalValidationError {
    #[error("The angle value is either greater than 360 degrees or less than 0 degrees.")]
    WrongAngleValue,

    #[error("The 'iterations' value is lower than 1.")]
    WrongIterationsValue,

    #[error("Wrong syntax. Symbols ' -> ' are not found in the rule.")]
    WrongRuleSyntax(String),
}

impl FractalValidationError {
    pub fn additional_info(&self) -> Option<String> {
        match self {
            Self::WrongRuleSyntax(value) => Some(value.clone()),
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
