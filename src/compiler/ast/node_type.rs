

// use super::super::super::common::{*,instruction::JmpCond};
use super::super::super::common::*;
use super::misc::*;

#[derive(Debug,Clone,Copy)]
pub enum AstNodeType<'a> {
    Root,
    Block {label:Option<&'a str>},
    
    Function{
        func_ind:usize,
        // params_num:usize, //replace with params ? no because params would include the variadic name, which isn't counted in params_num
        // captures_num:usize, //todo remove, can just be calculated from captured_params ? no because it also includes captured locals?
        // params:Vec<&'a str>,
        // captured_params:BTreeSet<usize>, //param_inds
        // variadic:bool,
    }, 

    // FunctionInstance{func_node_ind:usize,captures_num:usize}, //lambda_ast_node_ind,captures_num

    ToBlockStart{
        // cond:Option<bool>,
        cond:JmpCond,
        block_node_ind:usize,
    },
    ToBlockEnd{
        // cond:Option<bool>,
        cond:JmpCond,
        block_node_ind:usize,
    },

    ResultNil,
    ResultVoid,
    ResultFloat(FloatT),
    ResultInt(IntT),
    ResultBool(bool),
    ResultString(&'a str),
    ResultVararg,


    //
    StackSwap,
    StackRot,
    StackPush,
    StackDup,
    StackPop(usize),
    
    // GetStackVar{stack_var:AstStackVar, deref_on_ref:bool, name:Option<&'a str>}, //var,deref_on_ref

    //
    // GetGlobalVarRef{name:&'a str},

    //
    CallMethod{name:&'a str,params_num:usize},
    TryCallMethod{name:&'a str,params_num:usize},
    // HasMethod{name:&'a str,},
    CallResult{params_num:usize}, 

    //
    Include(&'a str),


    // DeclGlobalVar{name:&'a str},
    // DeclLocalVar{name:&'a str, local_ind:usize}, //, captured:bool
    // DeclLocalVararg{name:&'a str, local_ind:usize}, //, captured:bool

    // GetStackVar{stack_var:AstStackVar, deref_on_ref:bool, name:&'a str}, //var,deref_on_ref
    // GetGlobalVar(&'a str),

    // SetStackVar{stack_var:AstStackVar, name:&'a str},
    // SetGlobalVar(&'a str),

    // CallStackVar{stack_var:AstStackVar,params_num:usize, name:&'a str}, 
    // CallGlobal{name:&'a str,params_num:usize},
    
    // DeclGlobalVar{name:&'a str,},
    // DeclParamVar{name:&'a str,func_ind:usize,param_ind:usize,},
    // Local{local_ind:usize,},

    DeclVarStart{name:&'a str,decl:AstDeclVar,anon_id:Option<usize>}, //the anon_id is actually an id for each "cmd" used, 
    DeclVarEnd{name:&'a str,decl:AstDeclVar,anon_id:Option<usize>},
    GetVar{name:&'a str,var:AstAccessVar,anon_id:Option<usize>}, //,no_deref:bool
    SetVar{name:&'a str,var:AstAccessVar,anon_id:Option<usize>},
    CallVarOrMethod{name:&'a str,params_num:usize,var:AstAccessVar,anon_id:Option<usize>},

    GetVarOrCallMethod{name:&'a str,var:AstAccessVar},
    //
    //for dynamic scoping ...
    // ScopePush,
    // ScopePop,
    //ScopeVar(&'a str), //name
}

