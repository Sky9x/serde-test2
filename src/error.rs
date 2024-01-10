use serde::{de, ser};
use std::fmt::{self, Display, Formatter};

/// A de/serialization error.
#[derive(Clone, Debug)]
pub struct Error {
    msg: String,
    kind: ErrorKind,
}

impl Error {
    pub fn new(msg: impl Display) -> Self {
        Error {
            msg: msg.to_string(),
            kind: ErrorKind::Custom,
        }
    }

    pub(crate) fn assert_failed(msg: impl Display) -> Self {
        Error {
            msg: msg.to_string(),
            kind: ErrorKind::AssertFailed,
        }
    }
}

#[derive(Clone, Debug)]
enum ErrorKind {
    Custom,
    /// An assertion failed
    ///
    /// Matched on in the assert_tokens to panic on assertion failure.
    /// We shouldn't panic in de/serializer impls because of track_caller hell
    AssertFailed,
}

pub type TestResult<T = ()> = Result<T, Error>;

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::new(msg)
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::new(msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(self.msg())
    }
}

impl std::error::Error for Error {}

impl PartialEq<str> for Error {
    fn eq(&self, other: &str) -> bool {
        self.msg() == other
    }
}

impl PartialEq<&str> for Error {
    fn eq(&self, other: &&str) -> bool {
        self.msg() == *other
    }
}
