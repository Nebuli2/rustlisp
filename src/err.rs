#[derive(Debug)]
pub struct RLError {
    pub description: String,
}

impl AsRef<str> for RLError {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.description
    }
}

impl Into<String> for RLError {
    #[inline]
    fn into(self) -> String {
        self.description
    }
}

impl From<String> for RLError {
    /// Produces an `Error` with a description equal to the specified string.
    #[inline]
    fn from(s: String) -> RLError {
        RLError { description: s }
    }
}

impl<'a> From<&'a str> for RLError {
    /// Produces an `Error` with a description equal to the specified string
    /// slice.
    fn from(s: &str) -> RLError {
        RLError {
            description: s.to_string(),
        }
    }
}

use std::error::Error;
use std::io;

impl From<io::Error> for RLError {
    fn from(err: io::Error) -> RLError {
        RLError::from(err.description())
    }
}

use std::fmt;
impl fmt::Display for RLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for RLError {
    fn description(&self) -> &str {
        &self.description
    }
}
