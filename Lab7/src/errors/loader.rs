use crate::ui::windows::message::MessageWindow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FractalLoaderError {
    #[error("Failed to open file.")]
    FailedToOpenFile(String),

    #[error("Failed to parse line.")]
    FailedToParseLine(String),

    #[error("Not enough data in a file. There must be at least four lines, an axiom, an angle, iterations, and a rule.")]
    NotEnoughData,

    #[error("Axiom not found.")]
    AxiomNotFound,

    #[error("Angle not found.")]
    AngleNotFound,

    #[error("Iterations not found.")]
    IterationsNotFound,

    #[error("Failed to parse angle.")]
    FailedToParseAngle(String),

    #[error("The angle value is either greater than 360 degrees or less than 0 degrees.")]
    WrongAngleValue,

    #[error("Failed to parse iterations.")]
    FailedToParseIterations(String),

    #[error("The 'iterations' value is lower than 1.")]
    WrongIterationsValue,

    #[error("Symbols ' -> ' are not found in the rule.")]
    WrongRuleSyntax(String),
}

impl FractalLoaderError {
    pub fn additional_info(&self) -> Option<String> {
        match self {
            FractalLoaderError::FailedToOpenFile(value)
            | FractalLoaderError::FailedToParseLine(value)
            | FractalLoaderError::FailedToParseAngle(value)
            | FractalLoaderError::FailedToParseIterations(value)
            | FractalLoaderError::WrongRuleSyntax(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn window(&self) -> MessageWindow {
        let mut message = format!("Error: {}", self);
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
