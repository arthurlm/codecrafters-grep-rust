use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum GrepError {
    #[error("invalid pattern")]
    InvalidPattern,
}
