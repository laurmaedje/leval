/// Representation of any Error that can occur while parsing
/// Can be returned by tokenize()

#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    UnknownSymbol,
    NumberExpected,
}
