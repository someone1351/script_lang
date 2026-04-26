


use crate::{build::Loc, clike::tokenizer::TokenizerErrorType};

#[derive(Debug,Clone)]
pub enum  ParserErrorType {
    Tokenizer(TokenizerErrorType),
    Expected(String),
//     Unexpected,
//     ClosingQuoteExpected(&'static str),
//     // UnexpectedChar,
//     // ClosingBracketExpected(&'static str),
//     // NoMatchingOpeningBracket(&'static str),
//     ClosingCommentExpected,

//     // ExpectingClosingCurlyBracket,
//     // ExpectingClosingSquareBracket,
//     // ExpectingClosingParentheses,
}


#[derive(Debug,Clone)]
pub struct ParserError {
    pub loc : Loc,
    pub error_type : ParserErrorType,
    // pub msg:String,
}
