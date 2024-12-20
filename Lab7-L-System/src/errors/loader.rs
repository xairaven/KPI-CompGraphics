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

    #[error("Initial angle not found.")]
    InitialAngleNotFound,

    #[error("Iterations not found.")]
    IterationsNotFound,

    #[error("Failed to parse angle.")]
    FailedToParseAngle(String),

    #[error("Failed to parse initial angle.")]
    FailedToParseInitialAngle(String),

    #[error("Failed to parse iterations.")]
    FailedToParseIterations(String),
}

impl FractalLoaderError {
    pub fn additional_info(&self) -> Option<String> {
        match self {
            Self::FailedToOpenFile(value)
            | Self::FailedToParseLine(value)
            | Self::FailedToParseAngle(value)
            | Self::FailedToParseInitialAngle(value)
            | Self::FailedToParseIterations(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn window(&self) -> MessageWindow {
        let mut message = format!("Loading error: {}", self);
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
