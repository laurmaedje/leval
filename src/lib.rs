#![allow(unused_variables)]
#![allow(dead_code)]

/// A Token is one unit in an expression, e.g. a number or an operation
#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u32),
    Op(char),
}

/// Representation of any Error that can occur while parsing
/// Can be returned by tokenize()
#[derive(Debug)]
pub enum ParseError {
    UnknownSymbol,
}

/// Takes a string slice and returns a vector of Tokens (wrapped in a Result) or a ParseError
/// For example:
/// tokenize("55+7-3*7").unwrap() is equal to vec![Num(55), Op('+'), Num(7), Op('-'), Num(3), Op('*'), Num(7)]
pub fn tokenize(expression: &str) -> Result<Vec<Token>, ParseError> {
    // Iterator of the passed expression
    let mut s = expression.to_string();

    // Buffer for calculated tokens
    let mut toks = Vec::<Token>::new();

    // Iterate through the whole string
    while !s.is_empty() {
        match s.remove(0) {
            // Add operation token to toks list
            op @ '+' | op @ '-' | op @ '*' | op @ '/' => toks.push(Token::Op(op)),

            // Add number to toks list
            dig if dig.is_digit(10) => { 
                // Go ahead while there is a digit and buffer into buf variable                
                let mut buf = dig.to_string();
                while let Some(chr) = s.chars().nth(0) {
                    if chr.is_digit(10) {
                        buf.push(s.remove(0));
                    } else {
                        break;
                    }
                }
                toks.push(Token::Num(buf.parse().unwrap()))
            },

            // The current head symbol is unknown -> error
            _ => return Err(ParseError::UnknownSymbol),
        }
    }

    // Everything went well -> return the token list
    Ok(toks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_working() {
        use Token::*;

        assert_eq!(tokenize("55+7-3*7").unwrap(), 
            vec![Num(55), Op('+'), Num(7), Op('-'), Num(3), Op('*'), Num(7)]);
    }
}
