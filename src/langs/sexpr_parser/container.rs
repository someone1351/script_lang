use std::path::{PathBuf, Path};

// use super::loc::*;
// use super::Loc;
use super::super::super::common::Loc;


#[derive(Debug,Clone)]
pub struct SExprTree {
    sexprs : Vec<SExpr>,
    src : Option<String>,
    path : Option<PathBuf>,
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
    // pub fn root(&self) -> SExprContainer {
    //     SExprContainer {
    //         sexpr_index:0,
    //         sexpr_tree:self,
    //     }
    // }

    pub fn sexprs(&self) -> SExprContainerIter<'_> {
        SExprContainerIter::new(self, 0)
    }
    // pub fn root(&self) -> SExprIter {

    // }
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


#[derive(Debug,Clone,Copy)]
pub enum SExprValContainer<'a> {
    List(SExprContainerIter<'a>),
    Symbol(&'a str),
    String(&'a str),
    Float(f64),
    Int(i64),
    Bool(bool),
}


#[derive(Debug,Clone,Copy)]
pub struct SExprContainer<'a> {
    sexpr_index : usize,
    sexpr_tree : &'a SExprTree,
}



impl<'a> SExprContainer<'a> {
    // fn sexpr(&self)->&SExpr {
    //     &self.sexpr_tree.sexprs[self.sexpr_index]
    // }

    fn get_sexpr(&self, sexpr_index : usize)->&'a SExpr {
        &self.sexpr_tree.sexprs[sexpr_index]
    }

    pub fn new(sexpr_tree : &'a SExprTree, sexpr_index : usize, ) -> Self{
        Self { sexpr_tree:sexpr_tree, sexpr_index, } //records, path
    }

    pub fn sexpr_index(&self) -> usize {
        self.sexpr_index
    }

    pub fn child_ind(&self) -> usize {
        self.get_sexpr(self.sexpr_index).child_ind
    }

    pub fn val(&self) -> SExprValContainer<'a> {
        let sexpr = &self.sexpr_tree.sexprs[self.sexpr_index];

        match &sexpr.val {
            SExprVal::List(_) => SExprValContainer::List(SExprContainerIter::new(self.sexpr_tree, self.sexpr_index)),
            SExprVal::Symbol(v) => SExprValContainer::Symbol(v.as_str()),
            SExprVal::String(v) => SExprValContainer::String(v.as_str()),
            SExprVal::Float(v) => SExprValContainer::Float(*v),
            SExprVal::Int(v) => SExprValContainer::Int(*v),
            SExprVal::Bool(v) => SExprValContainer::Bool(*v),
        }
    }

    pub fn list_iter(&self) -> SExprContainerIter<'a> {
        self.list_iter_from(0)
    }

    pub fn list_iter_from(&self, child_index:usize) -> SExprContainerIter<'a> {
        //if not a list, the iter will return an empty one?
        // let sexpr = &self.sexpr_tree.sexprs[self.sexpr_index];
        SExprContainerIter::new_from(self.sexpr_tree,self.sexpr_index,child_index)
    }

    pub fn is_list(&self) -> bool {
        let sexpr = &self.sexpr_tree.sexprs[self.sexpr_index];

        if let SExprVal::List(_)=sexpr.val {
            true
        } else {
            false
        }
    }

    pub fn is_symbol(&self) -> bool {
        let sexpr = &self.sexpr_tree.sexprs[self.sexpr_index];

        if let SExprVal::Symbol(_)=sexpr.val {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        let sexpr = &self.sexpr_tree.sexprs[self.sexpr_index];

        if let SExprVal::String(_)=sexpr.val {
            true
        } else {
            false
        }
    }

    pub fn get(&self, child_ind : usize) -> Option<SExprContainer<'a>> {
        if let SExprVal::List(child_sexpr_inds)=&self.get_sexpr(self.sexpr_index).val {
            if child_ind < child_sexpr_inds.len() {
                Some(Self::new(self.sexpr_tree, child_sexpr_inds[child_ind]))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn first(&self) -> Option<SExprContainer<'a>> {
        self.get(0)
    }

    pub fn second(&self) -> Option<SExprContainer<'a>> {
        self.get(1)
    }

    pub fn last(&self) -> Option<SExprContainer<'a>> {
        self.get(self.len()-1)
    }

    pub fn len(&self) -> usize {
        if let SExprVal::List(child_sexpr_inds)=&self.get_sexpr(self.sexpr_index).val {
            child_sexpr_inds.len()
        } else {
            0
        }
    }

    pub fn symbol(&self) -> Option<&'a str> {
        if let SExprVal::Symbol(symbol)=&self.get_sexpr(self.sexpr_index).val {
            Some(symbol.as_str())
        } else {
            None
        }
    }
    pub fn string(&self) -> Option<&'a str> {
        if let SExprVal::String(string)=&self.get_sexpr(self.sexpr_index).val {
            Some(string.as_str())
        } else {
            None
        }
    }
    pub fn path(&self) -> Option<&'a Path> {
        self.sexpr_tree.path.as_ref().and_then(|x|Some(x.as_path()))
    }

    pub fn src(&self) -> Option<&'a str> {
        self.sexpr_tree.src.as_ref().and_then(|x|Some(x.as_str()))
    }

    pub fn depth(&self) -> usize {
        let sexpr = &self.sexpr_tree.sexprs[self.sexpr_index];
        sexpr.depth-1 //depth 1 is actual root, but depth 0 is like the container, so when getting depth sub 1, as the container of roots isn't exposed
    }
    pub fn start_loc(&self) -> Loc {
        let sexpr = &self.sexpr_tree.sexprs[self.sexpr_index];
        sexpr.start_loc
    }
    pub fn end_loc(&self) -> Loc {
        let sexpr = &self.sexpr_tree.sexprs[self.sexpr_index];
        sexpr.end_loc
    }
}

#[derive(Debug,Copy,Clone)]
pub struct SExprContainerIter<'a> {
    sexpr_index : usize,
    child_index : usize, //if 0, then 0 hasnt been traversed yet
    child_back_index : usize, //if last_ind then last_ind has been traversed
    sexpr_tree :&'a SExprTree,
}

impl<'a> SExprContainerIter<'a> {
    pub fn new(sexpr_tree :&'a SExprTree, sexpr_index : usize) -> Self {
        Self::new_from(sexpr_tree, sexpr_index, 0)
    }

    pub fn new_from(sexpr_tree :&'a SExprTree, sexpr_index : usize, from_ind:usize) -> Self{
        let len=if let SExprVal::List(children)=&sexpr_tree.sexprs[sexpr_index].val {
            children.len()
        } else {0};

        Self::new_from_to(sexpr_tree, sexpr_index, from_ind, len)
    }

    pub fn new_from_to(sexpr_tree :&'a SExprTree, sexpr_index : usize, from_ind:usize,to_ind:usize) -> Self {
        Self {
            sexpr_index,
            child_index:from_ind,
            child_back_index:to_ind,
            sexpr_tree,
        }
    }
}
impl<'a> Iterator for SExprContainerIter<'a> {
    type Item = SExprContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let sexpr = &self.sexpr_tree.sexprs[self.sexpr_index];

        if let SExprVal::List(children)=&sexpr.val {
            if self.child_index < self.child_back_index {
                let child_sexpr_index = children[self.child_index];
                self.child_index+=1;
                Some(SExprContainer::new(self.sexpr_tree,child_sexpr_index))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for SExprContainerIter<'a> {
    fn next_back(&mut self) -> Option<SExprContainer<'a>> {
        let sexpr = &self.sexpr_tree.sexprs[self.sexpr_index];

        if let SExprVal::List(children)=&sexpr.val {
            if self.child_back_index > self.child_index {
                self.child_back_index-=1;
                let child_sexpr_index = children[self.child_back_index];
                Some(SExprContainer::new(self.sexpr_tree,child_sexpr_index))
            } else {
                None
            }
        } else {
            None
        }
    }
}