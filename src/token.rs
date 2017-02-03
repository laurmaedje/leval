/// A MathToken is a Token in an Expression, like a Number or Operator
#[derive(Debug, PartialEq, Clone)]
pub enum MathToken {
    Num(f64),
    BinOp(char),
    UnOp(char),
    Func(char),
}
