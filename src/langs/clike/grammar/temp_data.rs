
use std::{collections::{HashMap, HashSet}, fmt::Debug, ops::Range};
use std::rc::Rc;

// use crate::clike::tokenizer::ValueContainer;

// use crate::build::Loc;
use super::super::tokenizer::TokenIterContainer;

use super::node::*;





// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
// pub enum TempExpectType<'g> {
//     // NoExpect,
//     Expect(&'g str),
//     Int,
//     Float,
//     String,
//     Identifier,
//     Symbol(&'g str),
//     Keyword(&'g str),
//     Eol,
// }

// impl<'g> TempExpectType<'g> {
//     pub fn is_expect(&self) -> bool {
//         if let Self::Expect(..)=self {
//             true
//         } else {
//             false
//         }
//     }
// }

#[derive(Clone, )]
pub struct TempExpectNew2<'g, TS>
where
    TS:Clone,
{
    pub expect_type:&'g str,
    pub tokens_start:TS,
    pub expect_len:usize,
}

#[derive(Clone,  )]
pub struct TempExpect2<'g,TS>
where
    TS:Clone,
{
    pub expect_type:&'g str,
    pub tokens_start:TS,
}

#[derive(Clone,  )]
pub struct TempExpect1<'g, TS> {
    pub expect_type:&'g str,
    pub parent:Option<usize>,
    pub tokens_start:TS,
    // pub last:bool,
}

#[derive(Clone, )]
pub struct TempStowNew<NT,TS>
where
    // P:Clone+core::hash::Hash+PartialEq+Eq,
    TS:Clone,
{
    // pub grammar:Rc<GrammarNode<'g,P>>,
    pub non_term:NT,
    pub tokens_start:TS,
    pub group_len:usize,
    pub stow_len:usize,
    pub trim:bool,
}

#[derive(Clone, )]
pub struct TempWas<'g> {
    pub name:&'g str,
}

#[derive(Clone,)]
pub enum TempStowWas<'g> {
    Was(TempWas<'g>),
    Primitive,
    None,
}


// #[derive(Clone,)]
pub struct TempStowSuccess<'g,NT,TS>
where
    // P:Clone+core::hash::Hash+PartialEq+Eq,
    TS: Clone,
{
    // pub grammar: Rc<GrammarNode<'g,P>>,
    pub non_term : NT,
    pub tokens_after:TS,
    pub stow_groups_end:usize,
    pub was:TempStowWas<'g>,
    pub trim:bool,
}
// #[derive(Clone,)]
pub struct TempStowFail<NT>
// where
//     P:Clone+core::hash::Hash+PartialEq+Eq,
{
    // pub grammar:Rc<GrammarNode<'g,P>>,
    pub non_term : NT,
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


// #[derive(Clone,)]
pub struct TempStow<'g,NT,TS>
where

    // P:Clone+core::hash::Hash+PartialEq+Eq,
    TS:Clone,
{
    pub stow_groups_start:usize,
    // pub val : TempStowVal<'t,'g>,
    pub tokens_start_ind:usize,

    pub success : Option<TempStowSuccess<'g,NT,TS>>,
    pub fail : Option<TempStowFail<NT>>,
}

#[derive(Clone)]
pub struct TempGroup<'g,TS>
where
    TS:Clone,
{
    pub name:&'g str,
    pub parent:usize, //group
    pub tokens:TS,
    pub trim:bool, //trim on close, except root (0)
}

// impl<'t,'g> Debug for  TempGroup<'t,'g> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("TempGroupInfo")
//         .field("name", &self.name)
//         .field("parent", &self.parent)
//         // .field("primitives", &self.primitives)
//         .field("primitive_ind_start", &self.tokens.inds().start)
//         .finish()
//     }
// }

#[derive(Clone)]
pub struct Work<'g,NT,P,TS>
where

    P:Clone, //+core::hash::Hash+PartialEq+Eq,
    // T:Clone,
    // I: Iterator<Item=T>+Clone,
    TS:Clone,
{
    pub grammar:&'g GrammarNode<'g,NT,P>,
    // pub tokens:TokenIterContainer<'t>,
    pub tokens:TS,

    pub grammar_ind:usize,
    pub user:bool, //gramamr added by input grammar, not walker //used to know whether to push hist_begins stk or not //used with and/or/many
    pub first:bool, //used to know whether to store a HistStow
    pub stow:bool,

    pub work_success_len:usize,
    pub work_fail_len:usize,

    pub group_ind:usize,
    pub group_len:usize, //only used for removing unused groups ... but even then it is not required, mainly used for debugging

    pub was_new_len:usize,
    pub was_ind:usize, //to have more than one WAS at a time (ie nested ones), need a was_len

    pub stow_new_len:usize,
    pub stow_len:usize,

    pub in_expect:bool,
    // pub no_expect:bool,


    pub expect_new_len2:usize,
    pub expect_len2:usize,

    // pub trim:bool,
}

