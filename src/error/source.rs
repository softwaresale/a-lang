use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::location::{SourceRange};

#[derive(Debug)]
pub struct SourceError {
    /// the actual error message to display to the user
    msg: String,
    /// The location where the error occurs
    err_loc: SourceRange,
    /// optional source range to show around the error for context
    context_loc: Option<SourceRange>,
}

impl SourceError {
    pub fn new<StrT: Into<String>>(msg: StrT, loc: SourceRange) -> Self {
        Self {
            msg: msg.into(),
            err_loc: loc,
            context_loc: None,
        }
    }

    pub fn with_context_location(mut self, loc: SourceRange) -> Self {
        self.context_loc = Some(loc);
        self
    }
}

impl Display for SourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at {}: ", self.err_loc)?;
        write!(f, "{}", self.msg)
    }
}

impl Error for SourceError {}

impl Into<Vec<SourceError>> for SourceError {
    fn into(self) -> Vec<SourceError> {
        vec![self]
    }
}
