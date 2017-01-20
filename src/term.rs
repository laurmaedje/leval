use std::collections::LinkedList;
use std::collections::HashMap;

use parse_error::*;

use self::MathToken::*;

/// A MathToken is a Token in a RPN-notated Term
#[derive(Debug, PartialEq, Clone)]
pub enum MathToken {
    Num(f64),
    BinOp(char),
    UnOp(char),
    Var(char),
}

/// A mathematical Term
#[derive(Debug, PartialEq, Clone)]
pub struct Term {
   /// An expression in postfix Form
    pub toks: Vec<MathToken>,
    has_var: bool,
}

impl Term {
    /// Create a new `Term` from a string using Shunting-Yard algorithm.
    /// # Example
    /// ```
    /// # use eval::*; 
    /// # #[allow(unused_variables)]
    /// let term = Term::new("2*x + 4^2").unwrap();
    /// ```
    pub fn new<S>(expression: S) -> Result<Term, ParseError> where S: Into<String> {            
        let mut chars = expression.into().chars().collect::<LinkedList<char>>();
        let mut stack = Vec::new();
        let mut toks = Vec::with_capacity(chars.len());

        // This is for recognizing unary operators
        let mut last = ' ';
        let mut has_var = false;

        // Iterate through all chars in the list
        while let Some(chr) = chars.pop_front() {
            match chr {
                // Ignore whitespace
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

                // Variable
                character @ 'a'...'z' => {
                     toks.push(Var(character));
                     has_var = true;
                },

                // Operation
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
                            if !right_assoc(o1) && precedency(o1) <= precedency(o2) ||
                            right_assoc(o1) && precedency(o1) < precedency(o2) {                                
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

        Ok(Term { toks: toks, has_var: has_var })
    }

    /// Evaluate the `Term` assuming there are no variables
    /// # Example
    /// ```
    /// # use eval::*;
    /// let term = Term::new("2+5*3").unwrap();
    ///
    /// assert_eq!(term.eval(), Ok(17_f64));
    /// ```
    pub fn eval(&self) -> Result<f64, ParseError> {
        self.private_eval(None)
    }

    /// Evaluate the `Term` assuming there is one variable
    /// # Example
    /// ```
    /// # use eval::*;
    /// let term = Term::new("2 + x*3").unwrap();
    ///
    /// assert_eq!(term.eval_var('x', 7_f64), Ok(23_f64));
    /// ```
    pub fn eval_var(&self, name: char, value: f64) -> Result<f64, ParseError> {
        if self.has_var {
            let mut map = HashMap::new();
            map.insert(name, value);
            self.private_eval(Some(map))
        } else {
            Err(ParseError::UnknownVariable)
        }
    }

    /// Evaluate the `Term` and pass multiple variables inside a `HashMap<char, Number>`
    /// # Example
    /// ```
    /// # use eval::*; use std::collections::HashMap;
    /// let term = Term::new("3*(2+y) + x^2").unwrap();
    ///
    /// let mut map = HashMap::new();
    /// map.insert('x', 4_f64);
    /// map.insert('y', 3_f64);
    ///
    /// assert_eq!(term.eval_vars(map), Ok(15_f64 + 16_f64));
    /// ```
    pub fn eval_vars(&self, vars: HashMap<char, f64>) -> Result<f64, ParseError> {
        if self.has_var {
            self.private_eval(Some(vars))
        } else {
            Err(ParseError::UnknownVariable)
        }   
    }

    fn private_eval(&self, op_vars: Option<HashMap<char, f64>>) -> Result<f64, ParseError> {
        let mut stack = Vec::new();

        // Go through all tokens
        for tok in &self.toks {
            match tok {
                // Number
                &Num(num) => stack.push(num),

                // Variable (like 'x', 'y')
                &Var(name) => {
                    match op_vars {
                        Some(ref vars) => stack.push(match vars.get(&name) {
                            Some(&num) => num,
                            None => return Err(ParseError::UnknownVariable)
                        }),
                        None => return Err(ParseError::UnknownVariable)
                    }
                },

                // Binary operation (like '+', '-', '*', '/')
                &BinOp(op) => {
                    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                        stack.push(match op {
                            '+' => b + a,
                            '-' => b - a,
                            '*' => b * a,
                            '/' => b / a,
                            '^' => b.powf(a),
                            _ => return Err(ParseError::UnknownOperation),
                        });
                    } else {
                        return Err(ParseError::FactorExpected);
                    }
                },

                // Unary operation (like '~')
                &UnOp(op) => {
                    if let Some(v) = stack.pop() {
                        stack.push(match op {
                            '~' => -v,
                            _ => return Err(ParseError::UnknownOperation),                            
                        });
                    } else { 
                        return Err(ParseError::FactorExpected);                        
                    }
                }
            }
        }

        // Everything went wel
        if stack.len() == 1 {
            Ok(stack.pop().unwrap())
        // Stack should be empty, but is not
        } else {
            Err(ParseError::FactorExpected)
        }
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
    use std::collections::HashMap;
    use term::*;
    use parse_error::*;

    #[test]
    fn term() {
        {
            let term = Term::new("3*(2+x)+x^2").unwrap();
            assert_eq!(term.eval_var('x', 4f64), Ok(18f64+16f64));
        }
        {
            let term = Term::new("2+5*3").unwrap();
            assert_eq!(term.eval(), Ok(17f64));
        }
        {
            let term = Term::new("5 + x*3").unwrap();
            assert_eq!(term.eval_var('x', 4f64), Ok(17f64));
        }
        {
            let term = Term::new("10/5").unwrap();
            assert_eq!(term.eval(), Ok(2f64));
        }
        {
            let term = Term::new("2-5").unwrap();
            assert_eq!(term.eval(), Ok(-3f64));
        }
        {
            let term = Term::new("4^2 * 1.5").unwrap();
            assert_eq!(term.eval(), Ok(24f64));
        }
        {
            let term = Term::new("5*y + x*3").unwrap();
            let mut map = HashMap::new();
            map.insert('x', 4f64);
            map.insert('y', 8f64);
            assert_eq!(term.eval_vars(map), Ok(52f64));
        }
        {
            let term = Term::new("-3").unwrap();
            assert_eq!(term.eval(), Ok(-3f64));
        }
        {
            let term = Term::new("2*[5-3]").unwrap();
            assert_eq!(term.eval(), Ok(4f64));
        }
        {
            let term = Term::new("5+(-3)").unwrap();
            assert_eq!(term.eval(), Ok(2f64));
        }
        {
            let term = Term::new("+15+3").unwrap();
            assert_eq!(term.eval(), Ok(18f64));
        }
        {
            let term = Term::new("3*-5").unwrap();
            assert_eq!(term.eval(), Ok(-15f64));
        }
        {
            let term = Term::new("5+3*").unwrap();
            assert_eq!(term.eval(), Err(ParseError::FactorExpected));
        }
        {
            let term = Term::new("-5^2").unwrap();
            assert_eq!(term.eval(), Ok(-25f64));
        }
    }
}
