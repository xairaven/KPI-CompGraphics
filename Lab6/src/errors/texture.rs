use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("Cannot open file using specified path.")]
    CantOpenFile(String),

    #[error("File does not contain pattern.")]
    FileNotContainsPattern,

    #[error("Invalid regex pattern used.")]
    InvalidRegexPattern(String),

    #[error("There's no coordinates in some line.")]
    LineDoesNotContainsPattern(String),

    #[error("Failed to parse number.")]
    ParsingToFloatFailed(String),

    #[error("Undefined String in the file.")]
    UndefinedLineString(String),
}

impl TextureError {
    pub fn get_additional_info(&self) -> Option<&String> {
        // Match only variants with String fields dynamically
        match self {
            Self::CantOpenFile(s)
            | Self::InvalidRegexPattern(s)
            | Self::LineDoesNotContainsPattern(s)
            | Self::ParsingToFloatFailed(s)
            | Self::UndefinedLineString(s) => Some(s),
            _ => None,
        }
    }
}
