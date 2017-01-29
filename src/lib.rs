mod shunt;
mod calc;
mod token;
mod error;

use std::collections::LinkedList;
use calc::calc;
use shunt::shunt;

pub use error::ParseError;

/// Evaluate an infix notated expression. 
/// The function uses the Shunting-Yard-Algorithm to convert the expression into RPN-Form. Then it is calculated.
/// Integers and floating point numbers are supported, the available operators are `+, -, *, /, ^`. 
/// Besides the functions `sqrt(x), ln(x), sin(x), cos(x), tan(x)`, aswell as the constants `pi, e` can be used.
/// 
/// # Examples
/// ```
/// # use leval::*;
/// assert_eq!(evaluate("12"), Ok(12.0));
/// assert_eq!(evaluate("2+5*3"), Ok(17.0));
/// assert_eq!(evaluate("2^(3+5)"), Ok(256.0));
/// assert_eq!(evaluate("10/5"), Ok(2.0));
/// assert_eq!(evaluate("4^2 * 1.8"), Ok(28.8)); 
/// ```
/// ```
/// # use leval::*;
/// assert_eq!(evaluate("ln(1)"), Ok(0.0));
/// assert_eq!(evaluate("sqrt(4*10-4)"), Ok(6.0));    
/// assert_eq!(evaluate("(cos(ln(1)+sqrt(1) - 1))*2"), Ok(2.0));
/// ```
pub fn evaluate<S>(expression: S) -> Result<f64, ParseError> where S: Into<String> {
    let chars = expression.into().chars().collect::<LinkedList<char>>();
    let toks = shunt(chars)?;
    let result = calc(toks)?;
    Ok(result)
}


// ----------------------------------------------------------------------------------
// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_test() {
        assert_eq!(evaluate("12"), Ok(12.0));
        assert_eq!(evaluate("2+5*3"), Ok(17.0));
        assert_eq!(evaluate("2^(3+5)"), Ok(256.0));
        assert_eq!(evaluate("10/5"), Ok(2.0));
        assert_eq!(evaluate("4^2 * 1.8"), Ok(28.8));
        assert_eq!(evaluate("2-5"), Ok(-3.0));
        assert_eq!(evaluate("-3"), Ok(-3.0));
        assert_eq!(evaluate("2*(3-5) +  1"), Ok(-3.0));
        assert_eq!(evaluate("5+(-3)"), Ok(2.0));
        assert_eq!(evaluate("-5^2"), Ok(-25.0));
        assert_eq!(evaluate("(-5)^2"), Ok(25.0));
        assert_eq!(evaluate("4^0.5"), Ok(2.0));

        assert_eq!(evaluate("5+%+4"), Err(ParseError::UnknownSymbol));        
        assert_eq!(evaluate("5+3*"), Err(ParseError::FactorExpected));  
        assert_eq!(evaluate("[2]*(5))"), Err(ParseError::UnbalancedParens));        
        assert_eq!(evaluate("5+3*"), Err(ParseError::FactorExpected)); 
        assert_eq!(evaluate("ff"), Err(ParseError::UnknownConstant)); 

        assert_eq!(evaluate("sqrt((9))"), Ok(3.0));
        assert_eq!(evaluate("ln(1)"), Ok(0.0));
        assert_eq!(evaluate("sqrt(4*10-4)"), Ok(6.0));    
        assert_eq!(evaluate("(cos(ln(1)+sqrt(1) - 1))*2"), Ok(2.0));
        assert_eq!(evaluate("-sqrt(9)*2"), Ok(-6.0)); 
        
        assert_eq!(evaluate("e^(ln(2.7))"), Ok(2.7));
        assert!(evaluate("pi^2").unwrap() < 10.0); 
        assert_eq!(evaluate("pi"), evaluate("PI"));                                
    }
}