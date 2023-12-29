use std::error::Error;
use crate::error::source::SourceError;

/// determine what kind of parser error we encounter
#[derive(Clone)]
pub enum ParseErr {
    /// this error was fatal. We should stop parsing all together
    Fatal(SourceError),
    /// we tried this route and it just didn't work
    NonFatal(SourceError)
}

impl ParseErr {
    pub fn into_fatal(self) -> Self {
        match self {
            ParseErr::NonFatal(non_fatal) => Self::Fatal(non_fatal),
            other => other,
        }
    }

    pub fn into_non_fatal(self) -> Self {
        match self {
            ParseErr::Fatal(fatal) => Self::NonFatal(fatal),
            other => other
        }
    }
}

impl Into<SourceError> for ParseErr {
    fn into(self) -> SourceError {
        match self {
            ParseErr::Fatal(err) => err,
            ParseErr::NonFatal(err) => err,
        }
    }
}
