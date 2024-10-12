


use super::super::super::common::Loc;

#[derive(Debug,Clone)]
pub enum  ParserErrorType {
    Unknown,
    ClosingQuoteExpected(&'static str),
    UnexpectedChar,
    ClosingBlockExpected,
    UnmatchedClosingBlock,
    ExpectedField,
}


#[derive(Debug,Clone)]
pub struct ParseError { //<'a>
    // pub src:Option<&'a str>,
    // pub src:&'a str,
    // pub path:Option<&'a Path>,
    // pub path:Option<PathBuf>,
    pub loc : Loc,
    pub error_type : ParserErrorType,
}
