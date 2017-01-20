#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod parse_error;
mod term;

pub use parse_error::ParseError;
pub use term::Term;

pub fn evaluate<S>(expression: S) -> Result<f64, ParseError> where S: Into<String> {
    match Term::new(expression) {
        Ok(term) => term.eval(),
        Err(err) => Err(err),
    }
}


// ----------------------------------------------------------------------------------
// Unit Tests
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn eval() {
        assert_eq!(evaluate("5+5"), Ok(10f64));
    }
}