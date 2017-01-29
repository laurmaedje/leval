use std::fmt;

/// Representation of any Error that can occur while parsing.
/// # Examples
/// ```
/// # use leval::*;
/// assert_eq!(evaluate("5+%+4"), Err(ParseError::UnknownSymbol));
/// assert_eq!(evaluate("5+3*"), Err(ParseError::FactorExpected));
/// assert_eq!(evaluate("[2]*(5))"), Err(ParseError::UnbalancedParens));
/// assert_eq!(evaluate("5+3*"), Err(ParseError::FactorExpected));
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseError {
    /// An unknown symbol appeared
    UnknownSymbol,
    /// This function is not existant or not supported
    UnknownFunction,
    /// This constant is not existant or not supported
    UnknownConstant,
    /// There are too many closing or opening parens, or a function was used without parens.
    UnbalancedParens,
    /// A factor (like a number or a parenthezised expression) is missing
    FactorExpected,
    /// There are two factors but no operator connecting them
    OperatorExpected,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}
