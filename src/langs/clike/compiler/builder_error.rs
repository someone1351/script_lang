use std::fmt::Debug;

use crate::{compiler::builder::BuilderError, Loc};


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


