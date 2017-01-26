use std::collections::LinkedList;

use error::*;

use self::MathToken::*;

/// A MathToken is a Token in a RPN-notated Expression
#[derive(Debug, PartialEq, Clone)]
enum MathToken {
    Num(f64),
    BinOp(char),
    UnOp(char),
    Func(String),
}

/// Evaluate an expression 
/// # Examples
/// ```
/// # use eval::*;
/// assert_eq!(evaluate("12"), Ok(12.0));
/// assert_eq!(evaluate("2+5*3"), Ok(17.0));
/// assert_eq!(evaluate("2^(3+5)"), Ok(256.0));
/// assert_eq!(evaluate("10/5"), Ok(2.0));
/// assert_eq!(evaluate("4^2 * 1.8"), Ok(28.8)); 
/// ```
pub fn evaluate<S>(expression: S) -> Result<f64, ParseError> where S: Into<String> {
    // PARSING
    let mut chars: LinkedList<char> = expression.into().chars().collect::<LinkedList<char>>();
    let mut stack: Vec<char> = Vec::new();
    let mut toks: Vec<MathToken> = Vec::with_capacity(chars.len());

    // This is for recognizing unary operators
    let mut last = ' ';

    // Iterate through all chars in the list
    while let Some(chr) = chars.pop_front() {
        match chr {
            // Ignore whitespace and don't change <last> then
            ' ' => continue,

            // Numbers
            digit @ '0'...'9' => {
                let mut buf = digit.to_string();
                let mut point = false;
                while let Some(&next) = chars.front() {
                    match next {
                        '0'...'9' => buf.push(chars.pop_front().unwrap()),
                        // There already was a point in this number            
                        '.' if point => break,
                        // First point
                        '.' => { 
                            buf.push(chars.pop_front().unwrap());
                            point = true;
                        },
                        _ => break,
                    }
                }
                toks.push(Num(buf.parse::<f64>().unwrap()));
            },

            // Function  
            character @ 'a' ... 'z' => {
                let mut buf = character.to_string();
                while let Some(&next) = chars.front() {
                    match next {
                        'a' ... 'z' => buf.push(chars.pop_front().unwrap()),
                        '(' => break,
                        _ => return Err(ParseError::UnbalancedParens),
                    }
                }    
                toks.push(Func(buf))
            }

            // Operator
            o1 if is_op(o1) => {
                // o1 is an unary operator
                if is_op(last) || last == '(' || last == '[' || last == ' ' {
                    match o1 {
                        '+' => {},
                        '-' => stack.push('~'),
                        _ => return Err(ParseError::UnknownSymbol), 
                    }
                } else {
                    while let Some(&o2) = stack.last() {                        
                        if !is_op(o2) { break; }

                        // Pop operators from the stack as long as there precedency is higher (or equal)
                        if (!right_assoc(o1) && precedency(o1) <= precedency(o2)) || (right_assoc(o1) && precedency(o1) < precedency(o2)) {                                
                            toks.push(if is_unop(o2) { UnOp(o2) } else { BinOp(o2) });
                            stack.pop();
                        } else {
                            break;
                        }
                    }
                    stack.push(o1); 
                }
            },

            // Opening Parenthesis
            paren @ '(' | paren @ '[' => stack.push(paren),

            // Closing Parenthesis
            paren @ ')' | paren @ ']' => {
                loop {
                    match (paren, stack.pop()) {
                        // Parens are ok
                        (')', Some('(')) | (']', Some('[')) => break,                          
                        (_, Some(op)) => toks.push(if is_unop(op) { UnOp(op) } else { BinOp(op) }),
                        // Parens are unbalanced (or the wrong closing paren occured)
                        _ => return Err(ParseError::UnbalancedParens),
                    }
                }
            },

            // Invalid Symbol
            _ => return Err(ParseError::UnknownSymbol),
        }
        last = chr;
    }

    // Push the remaining Operators onto the stack
    stack.reverse();
    for item in stack {
        match item {
            op if is_op(op) => toks.push(if is_unop(op) { UnOp(op) } else { BinOp(op) }),
            _ => return Err(ParseError::UnbalancedParens),
        }
    }

    // EVALUATION
    let mut stack: Vec<f64> = Vec::new();

    // Go through all tokens
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
            },

            Func(name) => {
                if let Some(v) = stack.pop() {
                    stack.push(match name.as_ref() {
                        "sqrt" => v.sqrt(),
                        _ => return Err(ParseError::UnknownFunction),
                    });
                } else {
                    return Err(ParseError::FactorExpected);
                }
            },

            // Unary operation (like '~')
            UnOp(op) => {
                if let Some(v) = stack.pop() {
                    if op == '~' {
                        stack.push(-v);
                    }
                } else {
                    return Err(ParseError::FactorExpected);
                }
            },
        }
    }

    // Everything went well
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
        // Stack should be empty, but is not
    } else {
        Err(ParseError::FactorExpected)
    }
}

// ----------------------------------------------------------------------------------
// Helper functions

/// Get level of precedency of an operator
fn precedency(op: char) -> u8 {
    match op {
        '+' | '-' => 2,
        '~' => 3,
        '/' | '*' => 4,
        '^' => 5,
        _ => panic!("not an operator: {}", op),
    }
}

/// Check whether an operator is left or right associative
fn right_assoc(op: char) -> bool {
    match op {
        '+' | '-' | '*' | '/' | '~' => true,
        '^' => false,
        _ => panic!("not an operator: {}", op),
    }
}

/// Check whether a char is an operator
fn is_op(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '~'
}

/// Check whether a char is an unary operator
fn is_unop(c: char) -> bool {
    c == '~'
}


// ----------------------------------------------------------------------------------
// Unit Tests
#[cfg(test)]
mod tests {
    use eval::*;
    use error::*;

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

        // assert_eq!(evaluate("sqrt(9)"), Ok(3.0));

        assert_eq!(evaluate("5+%+4"), Err(ParseError::UnknownSymbol));        
        assert_eq!(evaluate("5+3*"), Err(ParseError::FactorExpected));  
        assert_eq!(evaluate("[2]*(5))"), Err(ParseError::UnbalancedParens));        
        assert_eq!(evaluate("5+3*"), Err(ParseError::FactorExpected));                   
    }
}