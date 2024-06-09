use thiserror::Error;

#[derive(Debug, Error)]
pub enum CEsmError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Glob pattern error: {0}")]
    PatternError(#[from] glob::PatternError),

    #[error("Glob error: {0}")]
    GlobError(#[from] glob::GlobError),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
}
