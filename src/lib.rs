#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod parse_error;
mod tokenizer;
mod evaluator;

pub fn evaluate(s: &str) -> Result<i32, parse_error::ParseError> {
    let mut toks = try!(tokenizer::tokenize(s));
    evaluator::expr(&mut toks)
}

// -----------------------------------------------------------------------------------------------------------
// Unit Test
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn evaluator_working() {
        assert_eq!(evaluate("55+7-3*7").unwrap(), 41);        
    }
}


