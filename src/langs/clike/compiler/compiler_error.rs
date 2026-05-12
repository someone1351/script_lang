use std::{fmt::Debug, path::PathBuf};

use crate::{ast, clike::compiler::builder_error::BuilderErrorType, compiler::builder::BuilderError, error_msg, Loc, StringVal};
use super::super::tokenizer::TokenizerErrorType;




#[derive(Debug,Clone)]
pub enum CompileErrorType {
    Tokenizer(TokenizerErrorType),
    ParserExpected(String),
    Builder(BuilderErrorType),
    AstVar(ast::error::AstVarErrorType),
}



#[derive(Clone)]
pub struct CompileError {
    pub src : StringVal,
    pub path : Option<PathBuf>,
    pub error_type : CompileErrorType,
    pub loc : Loc,
}

impl CompileError {
    pub fn msg(&self) -> String {
        error_msg(&self.error_type, self.loc, Some(self.src.as_str()), self.path.as_ref().map(|p|p.as_path()))
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",error_msg(&self.error_type,self.loc,Some(self.src.as_str()),self.path.as_ref().map(|p|p.as_path())))
    }
}

impl std::error::Error for CompileError {
    fn description(&self) -> &str {
        "scriptlang compile error"
    }
}

impl Debug for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompileError")
            // .field("src", &self.src)
            .field("path", &self.path)
            .field("error_type", &self.error_type)
            .field("loc", &self.loc)
            .finish()
    }
}