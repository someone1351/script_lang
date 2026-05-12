use std::{fmt::Debug, path::PathBuf};

use crate::{ast, compiler::builder::BuilderError, error_msg, Loc, StringVal};
use super::super::tokenizer::TokenizerErrorType;


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


