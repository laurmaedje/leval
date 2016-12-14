use std::collections::HashMap;

use tokenizer::*;
use tokenizer::Token::*;
use parse_error::ParseError;

/// Saves a dictionary of variables as `(name, value)`-pairs.
pub struct Vars {
    vars: HashMap<String, i32>,
}

impl Vars {
    /// Create a new empty Vars object. 
    /// Should be created mutable, to make adding variables possible.
    /// # Examples
    /// ```
    /// # use eval::Vars;
    /// let mut vars = Vars::new();
    /// ```
    pub fn new() -> Vars {
        Vars { vars: HashMap::new() }
    }

    /// Set a variables value. If the variable exists, the value is updated, if not it's created.
    /// # Examples
    /// ```
    /// # use eval::Vars;
    /// let mut vars = Vars::new();
    /// vars.set("x", 5);
    /// ```
    pub fn set(&mut self, name: &str, value: i32) {
        &self.vars.insert(name.to_string(), value);
    }
}

/// Takes a mutable token stream and eats it. Returns either the result as i32 or a ParseError
/// Example: expr(&mut vec![Num(16), Op('+'), Num(2), Op('*'), Num(3)]) is equal to Ok(22)
pub fn expr(mut toks: &mut Vec<Token>, vars: Option<&Vars>) -> Result<i32, ParseError> {
    // Parse the first factor
    let mut sum = factor(&mut toks, vars)?;

    // As long as the look-ahead Token is an AddOp, parse it and take a new factor
    while let Some(&AddOp(op)) = toks.first() {
        toks.remove(0);
        let v = factor(&mut toks, vars)?;
        match op {
            '+' => sum += v,
            '-' => sum -= v,
            _ => unreachable!(),
        }
    }
    Ok(sum)
}

/// Parse a factor: a number or a product, return the result as i32 or a ParseError
fn factor(mut toks: &mut Vec<Token>, vars: Option<&Vars>) -> Result<i32, ParseError> {
    // Parse a first number
    let mut product = atomic(&mut toks, vars)?;
    // As long as the look-ahead Token is a MulOp, parse it and take a new number
    while let Some(&MulOp(op)) = toks.first() {
        toks.remove(0);
        let v = atomic(&mut toks, vars)?;
        match op {
            '*' => product *= v,
            '/' => product /= v,
            _ => unreachable!(),
        }
    }
    Ok(product)
}

/// Return the first number of token stream or an expression wrapped in parenthesis as i32.
/// If there is none return a ParseError::NumberExpected
/// If parens are wrong, returns ParseError::WrongClosingParen or ParseError::MissingClosingParen
fn atomic(mut toks: &mut Vec<Token>, vars: Option<&Vars>) -> Result<i32, ParseError> {
    if toks.is_empty() {
        return Err(ParseError::NumberExpected);
    }
    match toks.remove(0) {
        // Found a number
        Num(n) => Ok(n),
        // Found a Paren, tell if it is the correct one, and if not return
        // ParseError::WrongClosingParen or ParseError::MissingClosingParen
        LParen(lp) => {
            let value = expr(&mut toks, vars)?;
            match toks.first() { 
                Some(&RParen(rp)) => {
                    match (lp, rp) {
                        // Parens are correct
                        ('(', ')') | ('[', ']') => (),
                        // The wrong closing paren appeared
                        ('(', ']') | ('[', ')') => return Err(ParseError::WrongClosingParen),
                        _ => unreachable!(),
                    }
                }
                _ => return Err(ParseError::MissingClosingParen),
            }
            toks.remove(0);
            Ok(value)
        },
        // Found a Variable
        Var(name) => {
            match vars {
                // refactor soon
                Some(v) => match v.vars.get(&name) {
                    Some(&value) => Ok(value),
                    None => return Err(ParseError::UnknownVariable),
                },
                None => return Err(ParseError::UnknownVariable),
            }   
        },
        // There should be a number
        _ => return Err(ParseError::NumberExpected),
    }
}


// -----------------------------------------------------------------------------------------------------------
// Unit Test
#[cfg(test)]
mod tests {
    use tokenizer::Token::*;
    use evaluator::*;

    #[test]
    fn evaluater_working() {
        assert_eq!(
            expr(&mut vec![Num(16), MulOp('*'), Num(2), AddOp('+'), Num(3)], None),
            Ok(35));

        assert_eq!(
            expr(&mut vec![LParen('('), Num(5), AddOp('+'), Num(5), RParen(')'), MulOp('*'), Num(3)], None), 
            Ok(30));
    }
}
