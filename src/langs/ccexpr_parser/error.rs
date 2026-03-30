


use super::super::super::build::Loc;

#[derive(Debug,Clone)]
pub enum  ParserErrorType {
    Unexpected,
    ClosingQuoteExpected(&'static str),
    UnexpectedChar,
    ClosingBracketExpected(&'static str),
    NoMatchingOpeningBracket(&'static str),
    ClosingCommentExpected,

    // ExpectingClosingCurlyBracket,
    // ExpectingClosingSquareBracket,
    // ExpectingClosingParentheses,
}


#[derive(Debug,Clone)]
pub struct ParseError {
    pub loc : Loc,
    pub error_type : ParserErrorType,
}
