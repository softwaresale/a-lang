use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::frontend::location::{HasLocation, SourceRange};

#[derive(Debug, Clone)]
pub struct SourceError {
    /// the actual error message to display to the user
    msg: String,
    /// The location where the error occurs
    err_loc: SourceRange,
    /// optional source range to show around the error for context
    context_loc: Option<SourceRange>,
}

impl SourceError {
    pub fn new<StrT: Into<String>, RangeT: HasLocation>(msg: StrT, loc: RangeT) -> Self {
        Self {
            msg: msg.into(),
            err_loc: loc.source_range(),
            context_loc: None,
        }
    }

    pub fn with_context_location(mut self, loc: SourceRange) -> Self {
        self.context_loc = Some(loc);
        self
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
    pub fn err_loc(&self) -> SourceRange {
        self.err_loc
    }
    pub fn context_loc(&self) -> Option<SourceRange> {
        self.context_loc
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
