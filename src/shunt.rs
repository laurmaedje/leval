use std::collections::LinkedList;
use error::ParseError;
use token::MathToken;
use token::MathToken::*;

pub fn shunt(mut chars: LinkedList<char>) -> Result<Vec<MathToken>, ParseError> {
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
                // Holds the numer as a String
                let mut buf = digit.to_string();
                let mut point = false;

                while let Some(&next) = chars.front() {
                    match next {
                        // Just another digit, so push it to the buffer
                        '0'...'9' => buf.push(chars.pop_front().unwrap()),   

                        // There already was a point in this number
                        '.' if point => break,

                        // First point
                        '.' => {
                            buf.push(chars.pop_front().unwrap());
                            point = true;
                        }

                        // End of number
                        _ => break,
                    }
                }
                toks.push(Num(buf.parse::<f64>().unwrap()));
            }

            // Function or Constant
            character @ 'a'...'z' |
            character @ 'A'...'Z' => {
                use std::f64::consts;

                // Holds the function name as string
                let mut buf = character.to_string();

                // The read buffer is a function or a constant, typus holds what is is
                let mut typus = 'c';

                while let Some(&next) = chars.front() {
                    match next {
                        // Just another character of the function
                        'a'...'z' | 'A'...'Z' => buf.push(chars.pop_front().unwrap()),

                        _ => {
                            typus = if next == '(' { 'f' } else { 'c' };
                            break;
                        }
                    }
                }

                if typus == 'f' {
                    stack.push(match buf.as_ref() {
                        "sin" => 's',
                        "cos" => 'c',
                        "tan" => 't',
                        "sqrt" => 'r',
                        "ln" => 'l',
                        _ => return Err(ParseError::UnknownFunction),
                    });
                } else if typus == 'c' {
                    toks.push(Num(match buf.as_ref() {
                        "pi" | "PI" => consts::PI,
                        "e" => consts::E,
                        _ => return Err(ParseError::UnknownConstant),
                    }));
                }
            }

            // Operator
            o1 if is_op(o1) => {
                // o1 is an unary operator
                if is_op(last) || last == '(' || last == '[' || last == ' ' {
                    match o1 {
                        '+' => {}
                        '-' => stack.push('~'),
                        _ => return Err(ParseError::UnknownSymbol), 
                    }
                    // o1 is a binary operator or a function
                } else {
                    while let Some(&o2) = stack.last() {
                        if !is_op(o2) {
                            break;
                        }

                        // Pop operators from the stack as long as there precedency is higher (or equal)
                        if (!right_assoc(o1) && precedency(o1) <= precedency(o2)) ||
                           (right_assoc(o1) && precedency(o1) < precedency(o2)) {
                            toks.push(op_to_tok(o2));
                            stack.pop();
                        } else {
                            break;
                        }
                    }
                    stack.push(o1);
                }
            }

            // Opening Parenthesis
            paren @ '(' | paren @ '[' => stack.push(paren),

            // Closing Parenthesis
            paren @ ')' | paren @ ']' => {
                loop {
                    match (paren, stack.pop()) {
                        // Parens are ok
                        (')', Some('(')) | (']', Some('[')) => break,                          
                        (_, Some(op)) => toks.push(op_to_tok(op)),
                        // Parens are unbalanced (or the wrong closing paren occured)
                        _ => return Err(ParseError::UnbalancedParens),
                    }
                }
                if let Some(&op) = stack.last() {
                    if is_func(op) {
                        toks.push(op_to_tok(stack.pop().unwrap()));
                    }
                }
            }

            // Invalid Symbol
            _ => return Err(ParseError::UnknownSymbol),
        }
        last = chr;
    }

    // Push the remaining Operators onto the stack
    stack.reverse();
    for item in stack {
        match item {
            op if is_op(op) => toks.push(op_to_tok(op)),
            _ => return Err(ParseError::UnbalancedParens),
        }
    }

    Ok(toks)
}


// ----------------------------------------------------------------------------------
// Helper functions

/// Get level of precedency of an operator
#[inline]
fn precedency(op: char) -> u8 {
    match op {
        's' | 'c' | 't' | 'l' | 'r' => 1,
        '+' | '-' => 2,
        '~' => 3,
        '/' | '*' => 4,
        '^' => 5,
        _ => panic!("not an operator: {}", op),
    }
}

/// Check whether an operator is left or right associative
#[inline]
fn right_assoc(op: char) -> bool {
    match op {
        '+' | '-' | '*' | '/' | '~' | 's' | 'c' | 't' | 'l' | 'r' => true,
        '^' => false,
        _ => panic!("not an operator: {}", op),
    }
}

/// Check whether a char is an operator
#[inline]
fn is_op(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '~' || is_func(c)
}

/// Check wheter a char is the short form of a function
#[inline]
fn is_func(c: char) -> bool {
    c == 's' || c == 'c' || c == 't' || c == 'l' || c == 'r'
}

/// Convert an char operator into a MathToken Operation/ Function
#[inline]
fn op_to_tok(op: char) -> MathToken {
    match op {
        '+' | '-' | '*' | '/' | '^' => BinOp(op),
        '~' => UnOp(op),
        's' | 'c' | 't' | 'l' | 'r' => Func(op),
        _ => panic!("not an operator: {}", op),
    }
}
