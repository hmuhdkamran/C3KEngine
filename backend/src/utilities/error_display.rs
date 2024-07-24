use std::{error::Error as StdError, fmt::Display, fmt::Formatter, fmt::Result};

#[derive(Debug)]
pub struct ParseError(String);

impl StdError for ParseError {}

impl Display for ParseError {
    /// Formats the `ParseError` for display.
    ///
    /// # Parameters
    /// - `f`: A mutable reference to a `Formatter` used to format the output.
    ///
    /// # Returns
    /// A `Result` which is `Ok` if the formatting was successful, or an `Err` if it failed.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

/// Converts a string slice into a `ParseError`.
///
/// # Parameters
/// - `msg`: A string slice that holds the error message.
///
/// # Returns
/// A `ParseError` instance containing the provided error message.
impl From<&str> for ParseError {
    fn from(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

/// Converts a `String` into a `ParseError`.
///
/// # Parameters
/// - `msg`: A `String` that holds the error message.
///
/// # Returns
/// A `ParseError` instance containing the provided error message.
impl From<String> for ParseError {
    fn from(msg: String) -> Self {
        Self(msg)
    }
}
