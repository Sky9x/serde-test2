use serde::{de, ser};
use std::fmt::{self, Display, Formatter};

/// A de/serialization error.
#[derive(Clone, Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: impl Display) -> Self {
        Error {
            msg: msg.to_string(),
        }
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
