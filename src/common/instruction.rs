
/*
can't allow params to be set,
    if not all params provided, then there is no where to set
    and if allowed the params provided to be set, then can't
      rely on being able to set the param
      unless provided space on heap instead?
    should make it an error in the compiler
    and not have setstack instructions allow params
    what about captured params? no
      stackmakeref instr should do nothing if param missing
      stackgetref instr on missing param should just return nil

on params pop will need to only push thosed pushed, ... currently not exactly that?

on get stack offset in a function, check if stack ind < func stack start, and return nil
    if param offset refers to vararg, return vararg
on set param or make ref, add local of same name (without a copy)
    then MakeStackVarRef,GetStackVarDeref,SetStackVarDeref,CallStackVarDeref,SetStackVar will only be used on locals and not params
        only GetStackVar will be used for params, but could split it into GetStackVar(offset) and GetStackParam(param_ind)
    but in compiler can't just add local param, as can't easily remove them if wanting to recompile (not that you would)
        should instead add the local params at the instr output, func start,
        during calc vars, can calc how many locals needed for params (ones that are captured, and/or set)
            slightly wasteful on the stack for ones that are captured, since will need a new stack push for those, leaving the old val alone
    what about varag,
        can't really set a vararg?
        only set its varag[ind], which uses set_field(vararg,ind,val), but can't really detect that

        btw anything captured needs to be copied, so when vararg is captured, is converted into an array
            which might be confusing, since will be vararg if not captured, and array if captured
            currently vararg is just a local var, if set will just not be a vararg anymore

need to add copies in instructions for params, set var, get var,
  not return var since will be copied when stored, except for machine run's return
*/
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
    SetStackVarDeref(usize,bool,bool), //stack_offset, init,allow_void
    GetStackVarDeref(usize), //stack_offset
    CallStackVarDeref(usize,usize), //stack_offset, params_num

    SetStackVar(usize,bool), //stack_offset,allow_void
    GetStackVar(usize), //stack_offset,
    // GetStackParam(usize), //param_ind,


    StackPush,
    StackLocals(usize), //amount
    StackPop(usize), //amount
    StackSwap,
    StackRotRight, //currently rot right, but rot left would be more useful, only need single rot for set_field instead of 2
    StackRotLeft,
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

    GetGlobalOrConstOrCallMethod(usize,bool), //symbol_ind, get_global
    // GetFields(usize), //params_num
    // SetFields(usize), //params_num

    GetField{is_field_symbol:bool}, //is_field_static
    SetField{is_field_symbol:bool,is_last:bool,}, //is_field_static

}