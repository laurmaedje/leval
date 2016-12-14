#![allow(unused_mut)]

use tokenizer::*;
use tokenizer::Token::*;
use parse_error::ParseError;

/// Return the first number in current token stream if there is one
/// Otherwise return a ParseError::NumberExpected
macro_rules! parse {
    (num => $vec:ident) => (
        match $vec.get(0) {
            Some(&Num(n)) => { $vec.remove(0); n },
            _ => return Err(ParseError::NumberExpected),
        }
    );
    (fact => $vec:ident) => (
        match factor($vec) {
            Ok(n) => n,
            _ => return Err(ParseError::FactorExpected),
        }
    );
}

/// Evaluate the value of a token stream
/// For example:
/// vec![Num(16), Op('+'), Num(2), Op('*'), Num(3)] is equal to 22i32


pub fn expr(toks: &mut Vec<Token>) -> Result<i32, ParseError> {
    println!("{:?}", toks);
    // Parse the first factor
    let mut sum = parse!(fact => toks);

    // As long as the look-ahead Token is an AddOp, parse it and take a new factor
    while let Some(&AddOp(op)) = toks.get(0) {
        toks.remove(0);
        let v = parse!(fact => toks);
        match op {
            '+' => sum += v,
            '-' => sum -= v,
            _ => unreachable!()
        }
    }
    Ok(sum) 
}

pub fn factor(mut toks: &mut Vec<Token>) -> Result<i32, ParseError> {
    // Parse a first number
    let mut product = parse!(num => toks);
    
    // As long as the look-ahead Token is a MulOp, parse it and take a new number
    while let Some(&MulOp(op)) = toks.get(0) {
        toks.remove(0);
        let v = parse!(num => toks);
        match op {
            '*' => product *= v,
            '/' => product /= v,
            _ => unreachable!()
        }
    }
    Ok(product)
}


// -----------------------------------------------------------------------------------------------------------
// Unit Test
#[cfg(test)]
mod tests {
    // use tokenizer::*;
    use tokenizer::Token::*;
    use evaluator::*;

    #[test]
    fn evaluater_working() {
        //assert_eq!(evaluate(vec![Num(16), MulOp('*'), Num(2), AddOp('+'), Num(3)]).unwrap(), 0);

        let mut v = vec![Num(16), MulOp('*'), Num(2), AddOp('+'), Num(3)];
        assert_eq!(expr(&mut v).unwrap(), 35);
    }
}
