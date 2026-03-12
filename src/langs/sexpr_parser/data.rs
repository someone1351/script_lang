use std::path::{PathBuf, Path};

use super::container::SExprContainerIter;

use super::super::super::build::Loc;


#[derive(Debug,Clone)]
pub struct SExprTree {
    pub sexprs : Vec<SExpr>,
    pub src : Option<String>,
    pub path : Option<PathBuf>,
}

impl SExprTree {
    pub fn new(sexprs : Vec<SExpr>, src : Option<&str>,path : Option<&Path>) -> Self {
        Self {
            sexprs,
            src : src.and_then(|x|Some(x.to_string())),
            path : path.and_then(|x|Some(x.to_path_buf())),
        }

    }
    pub fn src(&self) -> Option<&str> {
        self.src.as_ref().and_then(|x|Some(x.as_str()))
    }
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref().and_then(|x|Some(x.as_path()))
    }
    // // pub fn root(&self) -> SExprContainer {
    // //     SExprContainer {
    // //         sexpr_index:0,
    // //         sexpr_tree:self,
    // //     }
    // // }

    pub fn sexprs(&self) -> SExprContainerIter<'_> {
        SExprContainerIter::new(self, 0)
    }
    // // pub fn root(&self) -> SExprIter {

    // // }
}

#[derive(Debug,Clone)]
pub enum SExprVal {
    List(Vec<usize>),
    Symbol(String),
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
}

#[derive(Debug,Clone)]
pub struct SExpr {
    pub val : SExprVal,
    pub start_loc : Loc,
    pub end_loc : Loc,
    pub depth : usize,
    pub child_ind : usize,
}
