use std::{fmt::Debug, path::PathBuf};

use crate::{ast, builder::BuilderError, ccexpr_parser::tokenizer::ParserErrorType, error_msg, Loc, StringVal};



#[derive(Debug,Clone,Copy)]
pub enum BuilderErrorType {
    ExpectedParenthesis,
    ExpectedCurlyBraces,
    ExpectedString,
    ExpectedEnd,
    ExpectedIdentifier,

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

impl BuilderErrorType {
    pub fn loc_err<T>(&self,loc:Loc) -> Result<T,BuilderError<BuilderErrorType>> {
        Err(BuilderError{ loc, error_type:*self })
    }
}



// // impl FnOnce(Loc) -> Result<ValueContainer<'a, PrimitiveIterContainer<'a>>, BuilderError<BuilderErrorType>>
// impl<T> Into<Box<dyn FnOnce(Loc)->Result<T,BuilderError<BuilderErrorType>>>> for BuilderErrorType {
//     fn into(self) -> Box<dyn FnOnce(Loc)->Result<T,BuilderError<BuilderErrorType>>> {
//         let error_type=self;
//         Box::new(move|loc|Err(BuilderError{ loc, error_type }))
//     }
// }

// //ValueContainer<'a, PrimitiveIterContainer<'a>>
// // impl               FnOnce(Loc)->Result<T, BuilderError<BuilderErrorType>>

// //Box< FnOnce(Loc)->Result<T,BuilderError<BuilderErrorType>>>
// impl<T,R:FnOnce(Loc)->Result<T,BuilderError<BuilderErrorType>>> Into<R> for BuilderErrorType


// {
//     fn into(self) -> R {
//         let error_type=self;
//         // Box::new()
//         move|loc|Err(BuilderError{ loc, error_type })
//     }
// }

// |loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedParenthesis })

#[derive(Debug,Clone)]
pub enum CompileErrorType {
    Builder(BuilderErrorType),
    Parser(ParserErrorType),
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