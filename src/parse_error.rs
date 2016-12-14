use std::fmt;

/// Representation of any Error that can occur while parsing.
/// # Examples
/// ```
/// # use eval::{evaluate, ParseError};
/// let err1 = evaluate("4+?");
/// assert_eq!(err1, Err(ParseError::UnknownSymbol));
///
/// let err2 = evaluate("4+6+");
/// assert_eq!(err2, Err(ParseError::NumberExpected));
/// ```
/// ```
/// # use eval::{evaluate, ParseError};
/// let err3 = evaluate("(4+2]*3");
/// assert_eq!(err3, Err(ParseError::WrongClosingParen));
///
/// let err4 = evaluate("3*(5+2");
/// assert_eq!(err4, Err(ParseError::MissingClosingParen));
/// ```
/// ```
/// # use eval::{evaluate, ParseError};
/// let err5 = evaluate("y+3");
/// assert_eq!(err5, Err(ParseError::UnknownVariable));
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// An unknown symbol appeared
    UnknownSymbol,
    /// A number is missing
    NumberExpected,
    /// A parenthesized expression is not closed by a closing paren
    MissingClosingParen,
    /// A parenthesized expression started with one paren and is closed by a different one
    WrongClosingParen,
    /// A used variable was not defined
    UnknownVariable,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}
