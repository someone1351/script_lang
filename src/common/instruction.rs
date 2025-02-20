
use super::types::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum JmpCond {
    None,
    True,False,
    Undefined,NotUndefined,
}

impl JmpCond {
    pub fn not(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Undefined => Self::NotUndefined,
            Self::NotUndefined => Self::Undefined,
        }
    }
}
#[derive(Debug,Clone)]
pub enum Instruction {
    Include(usize), //path_ind

    Jmp{
        // cond:Option<bool>,
        cond:JmpCond,
        instr_pos:usize,
        debug:(usize,i64),
    },
    
    ResultBool(bool),
    ResultInt(IntT),
    ResultFloat(FloatT),
    ResultSymbol(usize),//symbol_ind

    ResultNil,
    ResultVoid,

    ResultVararg,

    // ResultVararg,
    // ResultCopy,
    // StackMakeRef,
    // Deref,
    // SetStackDeref,

    ResultFunc(usize,usize),//lambda_ind,captures_num

    MakeStackVarRef(usize), //stack_offset,
    SetStackVarDeref(usize,bool), //stack_offset, init
    GetStackVarDeref(usize), //stack_offset,
    CallStackVarDeref(usize,usize), //stack_offset, params_num

    SetStackVar(usize), //stack_offset,
    GetStackVar(usize), //stack_offset,

    StackPush,
    StackLocals(usize), //amount
    StackPop(usize), //amount
    StackSwap,
    StackRot,
    // StackDup(usize), //stack_offset,
    StackDup,
    
    GetGlobalVarRef(usize), //symbol_ind, read,write,call //readonly
    GetGlobalAccessRef(usize), //symbol_ind

    DeclGlobalVar(usize), //symbol_ind ////, store
    SetGlobalVar(usize), //symbol_ind

    GetGlobalVarOrConst(usize,bool), //symbol_ind, get_global
    
    CallGlobalOrMethod(usize,usize), //symbol_ind, params_num

    CallMethod(usize,usize), //symbol_ind, params_num
    TryCallMethod(usize,usize), //symbol_ind, params_num
    // HasMethod(usize), //symbol_ind,
    CallResult(usize), //params_num

    GetGlobalOrConstOrCallMethod(usize,bool) //symbol_ind, get_global
    // GetFields(usize), //params_num
    // SetFields(usize), //params_num

}