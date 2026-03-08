


use super::super::super::build::Loc;

#[derive(Debug,Clone)]
pub enum  ParserErrorType {
    Unknown,
    ClosingQuoteExpected(&'static str),
    UnexpectedChar,
    ClosingBlockExpected,
    UnmatchedClosingBlock,
    ExpectedField,
    ExpectingClosingCurlyBracket,
    ExpectingClosingSquareBracket,
    ExpectingClosingParentheses,
}


#[derive(Debug,Clone)]
pub struct ParseError {
    pub loc : Loc,
    pub error_type : ParserErrorType,
}
