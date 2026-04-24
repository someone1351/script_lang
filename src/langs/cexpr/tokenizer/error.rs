


use crate::build::Loc;

#[derive(Debug,Clone)]
pub enum  TokenizeErrorType {
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
pub struct TokenizeError {
    pub loc : Loc,
    pub error_type : TokenizeErrorType,
}
