use error::*;
use token::MathToken;
use token::MathToken::*;

/// Calculate a tokenized RPN-notated Expression
pub fn calc(toks: Vec<MathToken>) -> Result<f64, ParseError> {
    let mut stack: Vec<f64> = Vec::new();

    for tok in toks {
        match tok {
            // Number
            Num(num) => stack.push(num),

            // Binary operation (like '+', '-', '*', '/')
            BinOp(op) => {
                if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                    stack.push(match op {
                        '+' => b + a,
                        '-' => b - a,
                        '*' => b * a,
                        '/' => b / a,
                        '^' => b.powf(a),
                        _ => return Err(ParseError::UnknownSymbol),
                    });
                } else {
                    return Err(ParseError::FactorExpected);
                }
            }

            Func(name) => {
                if let Some(v) = stack.pop() {
                    stack.push(match name {
                        's' => v.sin(),
                        'c' => v.cos(),
                        't' => v.tan(),
                        'r' => v.sqrt(),
                        'l' => v.ln(),
                        _ => return Err(ParseError::UnknownFunction),
                    });
                } else {
                    return Err(ParseError::FactorExpected);
                }
            }

            // Unary operation (like '~')
            UnOp(op) => {
                if let Some(v) = stack.pop() {
                    if op == '~' {
                        stack.push(-v);
                    }
                } else {
                    return Err(ParseError::FactorExpected);
                }
            }
        }
    }

    let len = stack.len();
    if len == 0 {
        Err(ParseError::FactorExpected)
    } else if len == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err(ParseError::OperatorExpected)
    }
}
