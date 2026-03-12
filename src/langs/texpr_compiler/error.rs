
use std::path::PathBuf;
use crate::error_msg;
use crate::Loc;
use crate::StringVal;

use super::texpr_parser;


use super::ast;




#[derive(Debug,Clone)]
pub enum BuilderErrorType {
    ExpectSymbol(u32),
    NoSymbolPrefixAllowed,
    // ExpectList,
    ExpectString,
    IncorrectParamsNum,
    NoParamsAllowed,
    InvalidParam,
    // ExpectExpr,
    // DeclFuncNotRoot,
    // ExpectValue,
    // ExpectParamName,
    VariadicMustBeAtEnd,
    // EmptySExpr, //
    // // BuilderAst(BuilderAstError),


    ContinueNotInLoop,
    BreakNotInLoop,
    ReturnNotInMethodOrLambda,

    ExpectBlock,
    NoSemiColonsAllowed, //only used within func param decl
    NoBlocksAllowed,
    NoFieldsAllowed,
    InvalidStringSymbol,
    InvalidSymbol,
    // NoCmdFound,
    // NoArgsAllowed,
    CannotCallGetVar,
}





#[derive(Debug,Clone)]
pub enum TexprCompileErrorType {
    TexprBuilder(BuilderErrorType),
    TexprParser(texpr_parser::ParserErrorType),
    AstVar(ast::error::AstVarErrorType),
}

#[derive(Debug,Clone)]
pub struct CompileError {
    pub src : StringVal,
    pub path : Option<PathBuf>,
    pub error_type : TexprCompileErrorType,
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


