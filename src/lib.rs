mod error;
mod term;

pub use error::MathError;
pub use term::Term;

pub fn evaluate<S>(expression: S) -> Result<f64, MathError> where S: Into<String> {
    match Term::new(expression) {
        Ok(term) => term.eval(),
        Err(err) => Err(err),
    }
}

// ----------------------------------------------------------------------------------
// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval() {
        assert_eq!(evaluate("5+5"), Ok(10.0));
        assert_eq!(evaluate("10*(3.5+4)-3"), Ok(72.0));
    }
}