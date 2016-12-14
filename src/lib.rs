#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod parse_error;
mod tokenizer;
mod evaluator;

pub use parse_error::ParseError;

/// Takes a string slice and returns its computed value as i32 or a ParseError
/// As Parentheses either `(...)` or `[...]` can be used:
/// # Examples
/// ```
/// use eval::{evaluate, ParseError};
///
/// let v1 = evaluate("5*5+2-4");
/// assert_eq!(v1, Ok(23));
///
/// let v2 = evaluate("(2+5) * 3");
/// assert_eq!(v2, Ok(21));
/// 
/// let v3 = evaluate("(3+2] * 2");
/// assert_eq!(v3, Err(ParseError::WrongClosingParen));
///
/// let v4 = evaluate("3+5$2");
/// assert_eq!(v4, Err(ParseError::UnknownSymbol));
/// ```
pub fn evaluate(s: &str) -> Result<i32, parse_error::ParseError> {
    let mut toks = tokenizer::tokenize(s)?;
    evaluator::expr(&mut toks)
}


// -----------------------------------------------------------------------------------------------------------
// Unit Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluator_working() {
        assert_eq!(evaluate("55+7-3*7"), Ok(41));
        assert_eq!(evaluate("5+4$3"), Err(ParseError::UnknownSymbol));
        assert_eq!(evaluate("5*5+2-4"), Ok(23));
    }
}
