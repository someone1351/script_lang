
use std::{collections::{HashMap, HashSet}, fmt::Debug};

// use crate::clike::tokenizer::ValueContainer;

// use crate::build::Loc;
use super::super::tokenizer::TokenIterContainer;

use super::node::*;


#[derive(Clone, Debug)]
pub struct TempExpectedNew<'g> {
    // pub name:&'g str,
    // pub grammar:GrammarNode<'g>,
    pub expected_type:TempExpectedType<'g>,
}

// #[derive(Clone, Debug)]
// pub struct TempExpectNews<'g> {

// }



#[derive(Clone, Debug)]
pub enum TempExpectedType<'g> {
    Expected(&'g str),

    Int,
    Float,
    String,
    Identifier,
    Symbol(&'g str),
    Keyword(&'g str),

    Eol,
    Prev,
}

#[derive(Clone, Debug)]
pub struct TempExpected<'g> {
    pub expect_type:TempExpectedType<'g>,
}

#[derive(Clone, Debug)]
pub struct TempHistNew<'t,'g> {
    pub grammar:GrammarNode<'g>,
    pub tokens_start:TokenIterContainer<'t>,
    pub is_first:bool,
    pub group_ind:usize,
}


#[derive(Clone,Debug)]
pub struct TempHistBegin<'t,'g> {
    pub grammar: GrammarNode<'g>,
    pub groups:Vec<TempGroupInfo<'t,'g>>, //inside the grammar this represents
    pub hist_ends:Vec<TempHistEnd<'g>>, //inside the grammar this represents //todo
    pub tokens_after:TokenIterContainer<'t>,

    pub tokens_start_ind:usize,
    // pub group_ind:usize,
    // pub inner_group_ind:usize,
    // pub inner_hist_ends_ind:usize,
}

#[derive(Clone, Debug)]
pub struct TempHistEnd<'g> {
    pub grammar: GrammarNode<'g>,
    pub tokens_start_ind:usize,
    // pub tokens:TokenIterContainer<'t>,
    // pub tokens_start:TokenIterContainer<'t>,
    // pub group_ind:usize,
    // pub inner_groups:Range<usize>, //groups inside the takeable?
}

#[derive(Clone,Default,Debug)]
pub struct TempHistBeginsInfo {
    pub ind_start:usize,
    pub ind_end:usize,
}
#[derive(Clone,Default,Debug)]
pub struct TempHistBegins<'t,'g> {
    // pub elements:HashMap<GrammarNode<'g>,TempHistBegin<'t,'g>>,
    pub hist_begins:Vec<TempHistBegin<'t,'g>>,
    pub groups:Vec<TempGroupInfo<'t,'g>>, //inside the grammar this represents
    pub hist_ends:Vec<TempHistEnd<'g>>, //inside the grammar this represents //todo

    // pub last_or_stk_len:usize, //used for
    //todo: store groups, hist_ends here, and in TempHistBegin store inds into them
}

// #[derive(Clone,Default,Debug)]
// pub struct TempHistEnds<'g> {
//     // pub elements:HashMap<GrammarNode<'g>,TempHistEnd>,
//     pub elements:Vec<TempHistEnd<'g>>,
// }

#[derive(Clone)]
pub struct TempGroupInfo<'t,'g> {
    pub name:&'g str,
    pub parent:usize, //group
    pub tokens:TokenIterContainer<'t>,
}

impl<'t,'g> Debug for  TempGroupInfo<'t,'g> {
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
    pub grammar:GrammarNode<'g>,
    pub success_len:usize,
    pub fail_len:usize,
    pub tokens:TokenIterContainer<'t>,
    pub group_ind:usize,
    pub group_len:usize, //only used for removing unused groups ... but even then it is not required, mainly used for debugging
    pub visiteds:HashSet<(&'g str,usize)>, //used for checking recursive nonterms
    pub grammar_debug_len:usize,
    pub and_id:usize, //for take, to know when continuing on an And, or leaving

    pub from_user:bool, //gramamr added by input grammar, not walker
    pub is_first:bool,

    pub hist_news_len:usize,

    // pub hist_begins_ind:usize,
    // pub hist_begins_len:usize,
    pub hist_begins_stk_len:usize,

    pub hist_ends_ind:usize,
    pub hist_ends_len:usize,

    // pub hist_begins_stk_len:usize,
    // pub hist_ends_stk_len:usize,

    pub expected_news_len:usize,
}

