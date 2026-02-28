// use super::loc::*;
// use super::Loc;

use super::super::super::build::Loc;

#[derive(Debug,Clone)]
pub enum ParserErrorType {
    MismatchedBraceTypes,
    NoMatchingOpenForClosingBrace,
    ExpectedClosingBrace,
    ExpectedEscapedChar,
    ExpectedClosingDoubleQuote,
    FailedToLexToken,
}
#[derive(Debug,Clone)]
pub struct ParserError {
    pub loc : Loc,
    pub error_type : ParserErrorType,
}
