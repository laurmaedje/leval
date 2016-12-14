use tokenizer::*;
use tokenizer::Token::*;
use parse_error::ParseError;

/// Takes a mutable token stream and eats it. Returns either the result as i32 or a ParseError
/// Example: expr(&mut vec![Num(16), Op('+'), Num(2), Op('*'), Num(3)]) is equal to Ok(22)
pub fn expr(mut toks: &mut Vec<Token>) -> Result<i32, ParseError> {
    // Parse the first factor
    let mut sum = factor(&mut toks)?;

    // As long as the look-ahead Token is an AddOp, parse it and take a new factor
    while let Some(&AddOp(op)) = toks.get(0) {
        toks.remove(0);
        let v = factor(&mut toks)?;
        match op {
            '+' => sum += v,
            '-' => sum -= v,
            _ => unreachable!(),
        }
    }
    Ok(sum)
}

/// Parse a factor: a number or a product, return the result as i32 or a ParseError
fn factor(mut toks: &mut Vec<Token>) -> Result<i32, ParseError> {
    // Parse a first number
    let mut product = num(&mut toks)?;
    // As long as the look-ahead Token is a MulOp, parse it and take a new number
    while let Some(&MulOp(op)) = toks.get(0) {
        toks.remove(0);
        let v = num(&mut toks)?;
        match op {
            '*' => product *= v,
            '/' => product /= v,
            _ => unreachable!(),
        }
    }
    Ok(product)
}

/// Return the first number of token stream as i32, if there is none return a ParseError::NumberExpected
fn num(mut toks: &mut Vec<Token>) -> Result<i32, ParseError> {
    match toks.get(0) {
        Some(&Num(n)) => {
            toks.remove(0);
            Ok(n)
        }
        _ => return Err(ParseError::NumberExpected),
    }
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
        assert_eq!(expr(&mut vec![Num(16), MulOp('*'), Num(2), AddOp('+'), Num(3)]).unwrap(), 35);
    }
}
