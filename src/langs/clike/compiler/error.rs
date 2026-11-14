use std::{fmt::Debug, path::Path};

use crate::{ast, clike::compiler::tokeniser2::TokenizerErrorType2, error_msg, Loc };
use super::super::tokenizer::TokenizerErrorType;


use crate::{compiler::builder::BuilderError, };


#[derive(Debug,Clone,Copy)]
pub enum BuilderErrorType {

    ContinueNotInLoop,
    BreakNotInLoop,
    ReturnNotInFunc,

}

impl BuilderErrorType {
    pub fn loc_err<T>(&self,loc:Loc) -> Result<T,BuilderError<BuilderErrorType>> {
        Err(BuilderError{ loc, error_type:*self })
    }
}



#[derive(Debug,Clone)]
pub enum  TokeniserErrorType {
    Unexpected,
    ClosingQuote,
    ClosingComment,
    LoneCarriageReturn,
}



#[derive(Debug,Clone)]
pub enum CompileErrorType<'t> {
    Tokenizer(TokenizerErrorType),
    Tokeniser(&'t str),
    Tokeniser2(TokenizerErrorType2),
    ParserExpected(String),
    ParserUnexpected,
    Builder(BuilderErrorType),
    AstVar(ast::error::AstVarErrorType),
}



#[derive(Clone)]
pub struct CompileError<'p,'t,> {
    pub src : Option<&'t str>,
    pub path : Option<&'p Path>,
    pub error_type : CompileErrorType<'t>,
    pub loc : Loc,
}

impl<'p,'t,> CompileError<'p,'t,> {
    pub fn msg(&self) -> String {
        error_msg(&self.error_type, self.loc, self.src, self.path)
    }
}

impl<'p,'t,> std::fmt::Display for CompileError<'p,'t,> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",error_msg(&self.error_type,self.loc,self.src,self.path,),)
    }
}

impl<'p,'t,> std::error::Error for CompileError<'p,'t,> {
    fn description(&self) -> &str {
        "scriptlang compile error"
    }
}

impl<'p,'t,> Debug for CompileError<'p,'t,> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompileError")
            // .field("src", &self.src)
            .field("path", &self.path)
            .field("error_type", &self.error_type)
            .field("loc", &self.loc)
            .finish()
    }
}