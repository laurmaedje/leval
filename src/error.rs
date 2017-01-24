use std::fmt;

/// Representation of any Error that can occur while parsing.
/// # Examples
/// ```
/// # use eval::{Term, MathError};
/// let err1 = Term::new("4+?");
/// assert_eq!(err1, Err(MathError::UnknownSymbol));
/// ```
/// ```
/// # use eval::{Term, MathError};
/// let err3 = Term::new("(4+2]*3");
/// assert_eq!(err3, Err(MathError::UnbalancedParens));
///
/// let err4 = Term::new("3*(5+2");
/// assert_eq!(err4, Err(MathError::UnbalancedParens));
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum MathError {
    /// An unknown symbol appeared
    UnknownSymbol,
    /// A number is missing
    FactorExpected,
    /// After a dollar sign no variable appeared
    VariableExpected,
    /// A parenthesized expression is not closed by a closing paren or there are two much opening parens
    UnbalancedParens,
    /// A used variable was not defined
    UnknownVariable,
    /// This operation is not defined
    UnknownOperation,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}
