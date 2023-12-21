use std::error::Error;
use std::fmt::{Display, Formatter};

/// Internal error - used for general errors, not source specific
#[derive(Debug)]
pub struct InternalError {
    /// Some message we want to share
    msg: String,
    /// An optional error
    cause: Option<Box<dyn Error>>
}

impl InternalError {
    pub fn new<StrT: Into<String>>(msg: StrT) -> Self {
        Self {
            msg: msg.into(),
            cause: None
        }
    }

    pub fn with_cause<ErrT: Into<Box<dyn Error>>>(mut self, cause: ErrT) -> Self {
        self.cause = Some(cause.into());
        self
    }

    pub fn new_with_cause<StrT: Into<String>, ErrT: Into<Box<dyn Error>>>(msg: StrT, cause: ErrT) -> Self {
        Self {
            msg: msg.into(),
            cause: Some(cause.into())
        }
    }
}

impl Display for InternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self.msg)?;
        Ok(())
    }
}

impl Error for InternalError {
    fn description(&self) -> &str {
        &self.msg
    }
}

