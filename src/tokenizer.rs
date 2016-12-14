use parse_error::ParseError;

/// A Token is one unit in an expression: a number, an operation, or a paren
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(i32),
    AddOp(char),
    MulOp(char),
    LParen(char),
    RParen(char),
}

/// Takes a string slice and returns a vector of Tokens (wrapped in a Result) or a ParseError
/// Example: `tokenize("16 + 2*3")` is equal to `Ok(vec![Num(16), Op('+'), Num(2), Op('*'), Num(3)])`
/// Another: `tokenize("(5+2)*3")` is equal to 
///     `Ok(vec![LParen('('), Num(5), AddOp('+'), Num(2), RParen(')'), MulOp('*'), Num(3)])`
pub fn tokenize(expression: &str) -> Result<Vec<Token>, ParseError> {
    // Parse the passed expression into a vector of chars
    let mut chars = expression.chars().collect::<Vec<char>>();

    // Buffer for the calculated output tokens
    let mut toks = Vec::<Token>::new();

    // Iterate through the whole string
    while !chars.is_empty() {
        match chars.remove(0) {
            // Remove empty space
            ' ' => (),

            // Paren
            p @ '(' | p @ '[' => toks.push(Token::LParen(p)),
            p @ ')' | p @ ']' => toks.push(Token::RParen(p)),

            // Add operation token to toks list
            p @ '+' | p @ '-' => toks.push(Token::AddOp(p)),
            p @ '*' | p @ '/' => toks.push(Token::MulOp(p)),

            // Add number to toks list
            dig if dig.is_digit(10) => {
                // Go ahead while there is a digit and buffer into buf variable
                let mut buf = dig.to_string();
                while let Some(&chr) = chars.first() {
                    if !chr.is_digit(10) {
                        break;
                    }
                    buf.push(chars.remove(0));
                }

                toks.push(Token::Num(buf.parse().unwrap()))
            }

            // The current head symbol is unknown -> error
            _ => return Err(ParseError::UnknownSymbol),
        }
    }

    // Everything went well -> return the token list
    Ok(toks)
}


// -----------------------------------------------------------------------------------------------------------
// Unit Test
#[cfg(test)]
mod tests {
    use tokenizer::*;
    use tokenizer::Token::*;

    #[test]
    fn tokenizer_working() {
        assert_eq!(
            tokenize("55+7-3*7"),
            Ok(vec![Num(55), AddOp('+'), Num(7), AddOp('-'), Num(3), MulOp('*'), Num(7)]));

        assert_eq!(
            tokenize("7+5"), 
            Ok(vec![Num(7), AddOp('+'), Num(5)]));

        assert_eq!(
            tokenize("5+5"), 
            tokenize("5 + 5"));
        
        assert_eq!(
            tokenize("(4+3)*4"), 
            Ok(vec![LParen('('), Num(4), AddOp('+'), Num(3), RParen(')'), MulOp('*'), Num(4)]));
    }
}
