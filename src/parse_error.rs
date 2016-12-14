use std::fmt;

/// Representation of any Error that can occur while parsing.
/// # Examples
/// ```
/// use eval::{evaluate, ParseError};
///
/// let err1 = evaluate("4+?");
/// assert_eq!(err1, Err(ParseError::UnknownSymbol));
///
/// let err2 = evaluate("4+6+");
/// assert_eq!(err2, Err(ParseError::NumberExpected));
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    UnknownSymbol,
    NumberExpected,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}
