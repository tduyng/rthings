use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexError {
    #[error("Unexpected character: {0} at position {1}")]
    UnexpectedChar(char, usize),

    #[error("Invalid number: {0} at position {1}")]
    InvalidNumber(String, usize),

    #[error("Unterminated string starting at position {0}")]
    UnterminatedString(usize),
}
