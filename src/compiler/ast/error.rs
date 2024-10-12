use super::super::super::common::Loc;


#[derive(Debug,Clone)]
pub enum AstError {
    // LocalVarDeclPushSizeNotZero,
    CallNotEnoughParamsPushedOnStack,
    StackSizeAlreadyZero,
    // LocalVarDeclNotInBlockOrRoot,
    ClosingFunctionNotMatching,
    ClosingBlockNotMatching,
    BlockOffsetNotFound(usize),
    LocalPushValuesNotZero(usize),
    // ToBlockNotFound,
    FuncParamsZeroAndVariadic,
    DeclStartNotMatching,
    // VarNotDecl,
    // AnonDeclAtRoot,
}

impl std::fmt::Display for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl std::error::Error for AstError {
    fn description(&self) -> &str {
        "scriptlang ast error"
    }
}


#[derive(Debug,Clone)]
pub enum AstVarErrorType {
    GlobalNotDecl(String),
}


#[derive(Debug,Clone)]
pub struct AstVarError {
    pub loc:Loc,
    pub error_type:AstVarErrorType,
    
}



impl std::fmt::Display for AstVarError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl std::error::Error for AstVarError {
    fn description(&self) -> &str {
        "scriptlang ast var error"
    }
}
