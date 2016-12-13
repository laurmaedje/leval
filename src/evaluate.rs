#![allow(unused_mut)]

use tokenizer::*;
use tokenizer::Token::*;
use parse_error::ParseError;




/// Evaluate the value of a token stream
/// For example:
/// vec![Num(16), Op('+'), Num(2), Op('*'), Num(3)] is equal to 22i32
pub fn evaluate(tokens: Vec<Token>) -> Result<i32, ParseError> {
    Ok(0)
}

fn expr(mut toks: &mut Vec<Token>) -> Result<i32, ParseError> {
    Ok(0)
}

fn factor(mut toks: &mut Vec<Token>) -> Result<i32, ParseError> {
    Ok(0)
}


// -----------------------------------------------------------------------------------------------------------
// Unit Test
#[cfg(test)]
mod tests {
    // use tokenizer::*;
    use tokenizer::Token::*;
    use evaluate::*;

    #[test]
    fn evaluater_working() {
        assert_eq!(evaluate(vec![Num(16), Op('+'), Num(2), Op('*'), Num(3)]).unwrap(), 22);
    }
}
