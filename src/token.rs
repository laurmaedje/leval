/// A MathToken is a Token in a RPN-notated Expression
#[derive(Debug, PartialEq, Clone)]
pub enum MathToken {
    Num(f64),
    BinOp(char),
    UnOp(char),
    Func(char),
}
