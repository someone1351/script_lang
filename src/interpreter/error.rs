use std::num::TryFromIntError;
use std::path::PathBuf;


// use crate::interpreter::custom::CustomError;
use crate::custom_data::*;

use super::super::common::*;
use super::machine::*;


#[derive(Debug,Eq,PartialEq)]
pub enum MachineErrorType{
    JmpErr(usize), //instr_pos,
    JmpUpErr(usize), //instr_offset_ind,
    JmpDownErr(usize), //instr_offset_ind,
    InvalidStackAccess(usize), //stack_offset
    // CannotSetStackParam(usize), //param_ind
    // InvalidStackPop(usize), //
    // StackPop,
    MissingSymbol(usize), //symbol_ind
    GlobalOrConstNotFound(String),
    // GlobalNotAFunc(String),
    // LocalNotAFunc,
    ValueNotAFunc(String),
    GlobalFuncOrMethodNotFound(String,Vec<String>),
    // BoundFuncNotFound(String),
    MethodNotFound(String,Vec<String>),
    FieldNotFound(String,Vec<String>),
    MethodOrGlobalVarNotFound(String),
    MethodRunError(String), //msg //Option<String>, Option<Vec<String>>,
    // InvalidFunctionParamsNum(usize), //req params num
    IncludeResolveError(PathBuf),
    // CustomDataBorrowError,
    CustomDataBorrowMutError,
    // CustomInstanceEmpty,
    // CustomIdEmpty,
    // CustomOwnerIdEmpty,
    CustomDataDead,
    CustomDataEmpty,
    CustomDataNotMut,
    CustomDataNotNonMut,
    CustomDataInvalidCast{given_type:String,expecting_type:String,},


    Custom(CustomError),

    FuncBorrowMutError,
    GetUndefinedVar,
    SetUndefinedVar,

    VoidNotExpr, //necessary?
    StackLimitReached(usize),

    IntInto,
    FloatInto,



}

impl From<CustomError> for MachineErrorType {
    fn from(value: CustomError) -> Self {
        MachineErrorType::Custom(value)
    }
}

impl From<CustomError> for MachineError {
    fn from(value: CustomError) -> Self {
        MachineError::new(MachineErrorType::Custom(value))
    }
}

impl From<TryFromIntError> for MachineError {
    fn from(_value: TryFromIntError) -> Self {
        MachineError::new(MachineErrorType::IntInto)
    }
}
impl From<IntValErr> for MachineError {
    fn from(_value: IntValErr) -> Self {
        MachineError::new(MachineErrorType::IntInto)
    }
}

impl From<FloatValErr> for MachineError {
    fn from(_value: FloatValErr) -> Self {
        MachineError::new(MachineErrorType::FloatInto)
    }
}

#[derive(Debug)]
pub struct MachineError{
    // pub path : Option<PathBuf>,
    pub build : Option<BuildT>,
    pub loc : Option<Loc>,
    pub error_type : MachineErrorType,
}

impl MachineError{
    pub fn from_machine<X>(machine:&Machine<X>, error_type : MachineErrorType) -> Self {
        Self{
            build : machine.cur_build(),
            loc : machine.cur_build().and_then(|cur_build|cur_build.instr_locs.get(&machine.instr_pos()).cloned()),
            error_type,
        }
    }

    // pub fn param_new<S: Into<String>>(machine:&Machine, param_ind:usize,msg : S) -> Self {
    //     let msg=msg.into();
    //     let error_type=MachineErrorType::MethodRunError(msg);

    //     Self{
    //         build : machine.cur_build(),
    //         loc : machine.cur_build().and_then(|cur_build|cur_build.instr_locs.get(&machine.instr_pos()).cloned()),
    //         error_type,
    //     }
    // }

    pub fn method<S: Into<String>>(msg : S) -> Self {
        Self { build : None, loc : None, error_type:MachineErrorType::MethodRunError(msg.into()), }
    }

    pub fn new(error_type : MachineErrorType) -> Self {
        // if error_type==MachineErrorType::CustomDataInvalidCast {
        //     // panic!("");

        // }
        Self{
            // path : machine.cur_build().path.clone(),
            build : None,
            loc : None,
            error_type,
        }
    }

    // pub fn bound_func_new<S: Into<String>>(msg:S) -> Self {
    //     // panic!("");
    //     Self::new(MachineErrorType::BoundFuncError(msg.into()))

    // }

    pub fn eprint(&self, msg : Option<String>) {
        // let path = self.build.as_ref().and_then(|b|b.path.clone());
        // let path = path.as_ref().map(|p|p.as_path());

        // let src=self.build.clone().and_then(|b|b.src.clone());
        // let src=src.as_ref().map(|s|s.as_str());

        // eprint_error(&self.error_type,path,self.loc,src,msg);
        // let src=self.build.as_ref().and_then(|b|b.src.as_ref()).map(|s|s.as_str());



        eprintln!("{}{}",
            msg.map(|x|format!("{x}: ")).unwrap_or_default(),
            error_msg(
                &self.error_type,
                self.loc.unwrap_or(Loc::zero()),
                self.build.as_ref().and_then(|b|b.src.as_ref()).map(|s|s.as_str()),
                self.build.as_ref().and_then(|b|b.path()),
            ),
        );

    }
}

impl std::fmt::Display for MachineError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f,"At {} : {:?}",self.loc, self.error_type)
        // write!(f,"At {:?}",self.error_type)
        write!(f,"{}",error_msg(&self.error_type,self.loc.unwrap_or(Loc::zero()),None,self.build.as_ref().and_then(|b|b.path())))
    }
}

impl std::error::Error for MachineError{
    fn description(&self) -> &str {
        "Machine Error"
    }
}
