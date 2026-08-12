
use std::{collections::{HashMap, HashSet}, fmt::Debug, ops::Range};
use std::rc::Rc;

// use crate::clike::tokenizer::ValueContainer;

// use crate::build::Loc;
use super::super::tokenizer::TokenIterContainer;

use super::node::*;





#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TempExpectType<'g> {
    Expected(&'g str),

    Int,
    Float,
    String,
    Identifier,
    Symbol(&'g str),
    Keyword(&'g str),

    Eol,
    // Prev, //remove?
}

#[derive(Clone, Debug, )]
pub struct TempExpect<'t,'g> {
    pub expected_type:TempExpectType<'g>,
    pub parent:Option<usize>,
    pub tokens_start:TokenIterContainer<'t>,
    // pub last:bool,
}

#[derive(Clone, Debug)]
pub struct TempStowNew<'t,'g> {
    pub grammar:Rc<GrammarNode<'g>>,
    pub tokens_start:TokenIterContainer<'t>,
    pub group_len:usize,
    pub stow_len:usize,
}

#[derive(Clone, Debug)]
pub struct TempWas<'g> {
    pub name:&'g str,
}

#[derive(Clone,Debug,)]
pub enum TempStowWas<'g> {
    Was(TempWas<'g>),
    Primitive,
    None,
}


#[derive(Clone,Debug,)]
pub struct TempStowSuccess<'t,'g> {
    pub grammar: Rc<GrammarNode<'g>>,
    pub tokens_after:TokenIterContainer<'t>,
    pub stow_groups_end:usize,
    pub was:TempStowWas<'g>,
}
#[derive(Clone,Debug,)]
pub struct TempStowFail<'g> {
    pub grammar:Rc<GrammarNode<'g>>,
}

// #[derive(Clone,Debug,)]
// pub enum TempStowVal<'t,'g> {
//     Success {
//         grammar: Rc<GrammarNode<'g>>,
//         tokens_after:TokenIterContainer<'t>,
//         stow_groups_end:usize,
//         was:TempStowWas<'g>,
//     },
//     Fail {
//         grammar:Rc<GrammarNode<'g>>,
//     },
//     None,
// }


#[derive(Clone,Debug,)]
pub struct TempStow<'t,'g> {
    pub stow_groups_start:usize,
    // pub val : TempStowVal<'t,'g>,
    pub tokens_start_ind:usize,

    pub success : Option<TempStowSuccess<'t,'g>>,
    pub fail : Option<TempStowFail<'g>>,
}

#[derive(Clone)]
pub struct TempGroup<'t,'g> {
    pub name:&'g str,
    pub parent:usize, //group
    pub tokens:TokenIterContainer<'t>,
}

impl<'t,'g> Debug for  TempGroup<'t,'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TempGroupInfo")
        .field("name", &self.name)
        .field("parent", &self.parent)
        // .field("primitives", &self.primitives)
        .field("primitive_ind_start", &self.tokens.inds().start)
        .finish()
    }
}

#[derive(Clone)]
pub struct Work<'t,'g> {
    pub grammar:Rc<GrammarNode<'g>>,
    pub tokens:TokenIterContainer<'t>,

    pub grammar_ind:usize,
    pub user:bool, //gramamr added by input grammar, not walker //used to know whether to push hist_begins stk or not //used with and/or/many
    pub first:bool, //used to know whether to store a HistStow

    pub work_success_len:usize,
    pub work_fail_len:usize,

    pub group_ind:usize,
    pub group_len:usize, //only used for removing unused groups ... but even then it is not required, mainly used for debugging

    pub was_new_len:usize,
    pub was_ind:usize, //to have more than one WAS at a time (ie nested ones), need a was_len

    pub stow_new_len:usize,
    pub stow_len:usize,

    pub expect_ind:Option<usize>,
    pub expect_len:usize,
}

