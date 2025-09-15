use thiserror::Error;

/// Domain errors
#[derive(Debug, Error)]
pub enum DomainError {
    /// Generic message error
    #[error("{0}")]
    Message(String),
}

impl DomainError {
    /// Create a generic error from string
    pub fn message(msg: &str) -> Self {
        Self::Message(msg.to_owned())
    }
}
