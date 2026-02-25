

use super::super::super::common::*;
use super::node_type::*;
use super::misc::*;


#[derive(Debug,Clone)]
pub struct AstNode<'a> {
    pub node_type : AstNodeType<'a>,

    // pub last_stack_size : usize,
    // pub stack_size : usize,
    // pub relative_stack_var_size : usize,
    // pub relative_stack_push_size : usize,

    // pub local_node_inds : Vec<usize>, //[node_ind]
    // pub local_decl_inds : HashMap<usize,usize>, //[node_ind]=body_local_ind
    pub local_decls : Vec<AstLocalDecl<'a>>,
    pub stack_pushed_num : usize,
    pub last_stack_size : usize, //size before locals etc declared?
    pub stack_size : usize, //size after locals etc declared

    pub children : Vec<usize>,

    pub parent : Option<usize>,
    pub child_ind : usize,
    // pub prev_sibling : Option<usize>,
    // pub next_sibling : Option<usize>,


    pub depth : usize,
    pub loc : Option<Loc>,
}

impl<'a> AstNode<'a> {
    pub fn new(node_type:AstNodeType<'a>, parent:Option<usize>, child_ind:usize, loc : Option<Loc>, depth:usize) -> Self {
        Self {
            node_type,
            parent,
            child_ind,
            // next_sibling : None,
            depth,
            loc,

            children : Vec::new(),

            // local_node_inds : Vec::new(),
            // local_decl_inds : HashMap::new(),
            local_decls : Vec::new(),
            stack_pushed_num: 0,
            stack_size:0,
            last_stack_size:0,

            // last_stack_size : 0,
            // stack_size : 0,
            // relative_stack_var_size : 0,
            // relative_stack_push_size : 0,
        }
    }
}