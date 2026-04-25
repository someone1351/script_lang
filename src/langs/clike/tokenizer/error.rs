


use crate::build::Loc;

#[derive(Debug,Clone)]
pub enum  TokenizerErrorType {
    Unexpected,
    ClosingQuoteExpected(&'static str),
    // UnexpectedChar,
    // ClosingBracketExpected(&'static str),
    // NoMatchingOpeningBracket(&'static str),
    ClosingCommentExpected,

    // ExpectingClosingCurlyBracket,
    // ExpectingClosingSquareBracket,
    // ExpectingClosingParentheses,
}


#[derive(Debug,Clone)]
pub struct TokenizerError {
    pub loc : Loc,
    pub error_type : TokenizerErrorType,
}
