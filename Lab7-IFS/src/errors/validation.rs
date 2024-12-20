use crate::ui::windows::message::MessageWindow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Probability have to be in range 0..=1")]
    BadProbability(String),

    #[error("Probability sum have to be lower than 1")]
    BadProbabilitySum(String),
}

impl ValidationError {
    pub fn additional_info(&self) -> Option<String> {
        match self {
            Self::BadProbability(value) | Self::BadProbabilitySum(value) => Some(value.clone()),
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
