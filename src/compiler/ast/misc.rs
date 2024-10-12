
#[derive(Debug,Clone)]
pub struct AstCapture<'a> {
    pub name : &'a str,
    pub var : AstAccessVar, //<'a>,
    pub stack_ind : usize,
}

#[derive(Debug,Clone)]
pub struct AstFuncParam<'a> {
    pub name : &'a str,
    pub captured : bool,
    pub stack_ind : usize,
}

#[derive(Debug,Clone)]
pub struct AstFunc<'a> {
    pub params:Vec<AstFuncParam<'a>>,
    pub variadic:bool,
    pub captures : Vec<AstCapture<'a>>,
    pub node_ind:usize,
}


#[derive(Debug,Clone,Copy)]
pub enum AstDeclVar {
    // Uninit,

    //only used for global decls ie ones predefined outside cur script aren't going to be recognised at compile time
    Global,

    //only used for (var somedecl _) where somedecl matches cur func param name in root of the func, ie not in a block etc,
    Param{func_ind:usize,param_ind:usize,}, 

    Local{local_ind:usize,},
    // Vararg{local_ind:usize,},
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum AstAccessVar { //<'a>
    None,
    // OuterGlobal,
    Global, //{name:&'a str}, //in env
    // GlobalDecl,
    // Global(&'a str),
    Param{
        // stack_param_ind:usize, //todo remove, just calc from param_ind
        func_ind:usize,
        param_ind:usize,
    },
    // Vararg {
    //     local_stack_ind:usize,
    //     node_ind:usize,
    //     // func_node_index:usize,
    // },
    Local{
        // local_stack_ind:usize,
        node_ind:usize,
        local_ind : usize,
        // anon : bool,
    },
    Capture{ //aka capture param, an additional param representing captured variables (from outside func scope) to funcs
        func_ind:usize,
        capture_ind:usize,
    }, //refers to capture "param" (not a param or a local var in the current scope that has been captured) that is passed to a func
}


#[derive(Copy,Debug,Clone,PartialEq,Eq,Hash)]
pub struct VarName<'a> {
    pub name : &'a str,
    pub anon_id : Option<usize>,
}

#[derive(Debug,Clone)]
pub struct AstLocalDecl<'a> {
    // pub name : &'a str,
    // pub anon : bool,
    pub name : VarName<'a>,
    pub captured : bool,
    pub stack_ind : usize,
}
