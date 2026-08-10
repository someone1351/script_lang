
use std::{collections::{HashMap, HashSet}, fmt::Debug, ops::Range};
use std::rc::Rc;

// use crate::clike::tokenizer::ValueContainer;

// use crate::build::Loc;
use super::super::tokenizer::TokenIterContainer;

use super::node::*;





#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TempExpectedType<'g> {
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
pub struct TempExpected<'t,'g> {
    pub expected_type:TempExpectedType<'g>,
    pub parent:Option<usize>,
    pub tokens_start:TokenIterContainer<'t>,
}

#[derive(Clone, Debug)]
pub struct TempHistNew<'t,'g> {
    pub grammar:Rc<GrammarNode<'g>>,
    pub tokens_start:TokenIterContainer<'t>,
    // pub is_first:bool,
    // pub group_ind:usize,
    pub group_len:usize,
    pub hist_stow_len:usize,

    // pub hist_fails_len:usize,
}

#[derive(Clone, Debug)]
pub struct TempWas<'g> {
    pub name:&'g str,
}


// #[derive(Clone, Debug,Default)] //
// pub struct TempHistFail2<'g> { //TempHistPrev
//     pub grammers: HashSet<Rc<GrammarNode<'g>>>,
//     // // pub tokens_start_ind:usize,

// }

// #[derive(Clone, Debug,)] //Default
// pub struct TempHistFail<'g> { //TempHistPrev
//     // pub grammers: HashSet<Rc<GrammarNode<'g>>>,
//     // // pub tokens_start_ind:usize,

//     pub grammar:Rc<GrammarNode<'g>>,
// }

#[derive(Clone,Debug,)]
pub enum TempHistStowWas<'g> {
    Was(TempWas<'g>),
    Primitive,
    None,
}


#[derive(Clone,Debug,)]
pub enum TempHistStowVal2<'t,'g> {
    Success {
        grammar: Rc<GrammarNode<'g>>,
        tokens_after:TokenIterContainer<'t>,
        stow_groups_end:usize,
        was:TempHistStowWas<'g>,
    },
    Fail {
        grammar:Rc<GrammarNode<'g>>,
    },
    None,
}

// #[derive(Clone,Debug,)]
// pub struct TempHistStowVal<'t,'g> { //TempHistStow
//     pub grammar: Rc<GrammarNode<'g>>,
//     pub tokens_after:TokenIterContainer<'t>,
//     pub stow_groups_end:usize,
//     // pub stow_prevs_end:usize,
//     pub was:TempHistStowWas<'g>,
// }

#[derive(Clone,Debug,)]
pub struct TempHistStow<'t,'g> { //TempHistStow
    // pub success_val : Option<TempHistStowVal<'t,'g>>,
    pub stow_groups_start:usize,
    // // pub stow_prevs_start:usize,
    // pub fail_val:Option<TempHistFail<'g>>,

    // pub fail_vals:TempHistFail2<'g>,
    pub val : TempHistStowVal2<'t,'g>,
    pub tokens_start_ind:usize,
}

// #[derive(Clone, Debug)]
// pub struct TempHistPrev<'g> {
//     pub grammar: Rc<GrammarNode<'g>>,
//     pub tokens_start_ind:usize,
// }


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
    pub work_success_len:usize,
    pub work_fail_len:usize,
    pub tokens:TokenIterContainer<'t>,
    pub group_ind:usize,
    pub group_len:usize, //only used for removing unused groups ... but even then it is not required, mainly used for debugging

    pub from_user:bool, //gramamr added by input grammar, not walker //used to know whether to push hist_begins stk or not //used with and/or/many
    pub first:bool, //used to know whether to store a HistStow
    pub or_id:usize,

    pub hist_new_len:usize,
    pub hist_stow_len:usize,

    pub expected_ind:Option<usize>,
    pub expected_len:usize,

    pub was_new_len:usize,
    pub was_ind:usize, //to have more than one WAS at a time (ie nested ones), need a was_len

}

