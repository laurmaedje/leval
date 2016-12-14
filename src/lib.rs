#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod parse_error;
mod tokenizer;
mod evaluator;

pub use parse_error::ParseError;
pub use evaluator::Vars;

/// Takes a string slice and returns its computed value as `i32` or a `ParseError`.
/// As Parentheses either `(...)` or `[...]` can be used
/// # Examples
/// ```
/// # use eval::{evaluate, ParseError};
/// let v1 = evaluate("5*5+2-4");
/// assert_eq!(v1, Ok(23));
///
/// let v2 = evaluate("(2+5) * 3");
/// assert_eq!(v2, Ok(21));
/// ```
/// If and error occures, it gets returned
/// 
/// ```
/// # use eval::{evaluate, ParseError};
/// let v3 = evaluate("(3+2] * 2");
/// assert_eq!(v3, Err(ParseError::WrongClosingParen));
///
/// let v4 = evaluate("3+5$2");
/// assert_eq!(v4, Err(ParseError::UnknownSymbol));
/// ```
pub fn evaluate(s: &str) -> Result<i32, parse_error::ParseError> {
    let mut toks = tokenizer::tokenize(s)?;
    evaluator::expr(&mut toks, None)
}

/// Same as `evaluate`, except that next to string a `&vars` paramater needs to be passed,
/// which contains information about used variables.
/// 
/// Note that the `vars` parameter must be passed by reference.
/// # Examples
/// 
/// 
/// ```
/// # use eval::{eval_with_vars, ParseError, Vars};
/// let mut v = Vars::new();
/// v.set("x", 7);
/// assert_eq!(eval_with_vars("x+5", &v), Ok(12)); 
/// ``` 
///
/// If a variable is not set, `ParseError::UnknownVariable` is returned
/// 
/// ```
/// # use eval::{eval_with_vars, ParseError, Vars};
/// let mut v2 = Vars::new();
/// assert_eq!(eval_with_vars("y+3", &v2), Err(ParseError::UnknownVariable));
/// ```
pub fn eval_with_vars(s: &str, vars: &Vars) -> Result<i32, parse_error::ParseError> {
    let mut toks = tokenizer::tokenize(s)?;
    evaluator::expr(&mut toks, Some(vars))
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

    #[test]
    fn eval_with_vars_working() {
        let mut v = Vars::new();
        v.set("x", 7);
        assert_eq!(eval_with_vars("x+5", &v), Ok(12));
    }
}
