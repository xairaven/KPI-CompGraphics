use crate::errors::validation::ValidationError;
use crate::ui::windows::message::MessageWindow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FractalLoaderError {
    #[error("Failed to open file.")]
    FailedToOpenFile(String),

    #[error("Failed to parse the line.")]
    FailedToParseRecord(String),

    #[error("Failed to parse string element to float.")]
    FailedToParseFloat(String),

    #[error("Invalid record length. Record must have 7 elements.")]
    InvalidRecordLength(String),

    #[error("Invalid record format.")]
    ValidationError(ValidationError),
}

impl FractalLoaderError {
    pub fn additional_info(&self) -> Option<String> {
        match self {
            Self::FailedToOpenFile(value)
            | Self::FailedToParseRecord(value)
            | Self::FailedToParseFloat(value)
            | Self::InvalidRecordLength(value) => Some(value.clone()),
            Self::ValidationError(error) => {
                let mut message = error.to_string();
                if let Some(additional_info) = error.additional_info() {
                    message = format!("{}. {}", message, additional_info);
                }
                Some(message)
            },
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
