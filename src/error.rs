use std::fmt;

/// Representation of any Error that can occur while parsing.
/// # Examples
/// ```
/// # use eval::*;
/// assert_eq!(evaluate("5+%+4"), Err(ParseError::UnknownSymbol));        
/// assert_eq!(evaluate("5+3*"), Err(ParseError::FactorExpected));  
/// assert_eq!(evaluate("[2]*(5))"), Err(ParseError::UnbalancedParens));        
/// assert_eq!(evaluate("5+3*"), Err(ParseError::FactorExpected)); 
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// An unknown symbol appeared
    UnknownSymbol,
    /// A factor is missing
    FactorExpected,
    /// A parenthesized expression is not closed by a closing paren or there are two much opening parens
    UnbalancedParens,
    /// This function is wrong or not supported
    UnknownFunction,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}
