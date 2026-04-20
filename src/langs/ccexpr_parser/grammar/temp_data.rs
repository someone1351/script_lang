
use std::collections::{HashMap, HashSet};

// use crate::build::Loc;
use crate::ccexpr_parser::tokenizer::PrimitiveIterContainer;

use super::node::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct PrimitiveInfo {
    // name:&'a str,
    // depth:usize,
    pub group:usize,
    pub discard:bool,
}

#[derive(Debug)]
pub struct GroupInfo<'a> {
    pub name:&'a str,
    pub parent:usize, //group
    pub primitive_ind_start:usize,
}

pub struct Work<'a> {
    pub grammar:GrammarNode<'a>,
    pub success_len:usize,
    pub fail_len:usize,
    pub primitives:PrimitiveIterContainer<'a>,
    pub group_ind:usize,

    pub group_len:usize, //only used for removing unused groups ... but even then it is not required, mainly used for debugging
    pub output_len:usize,

    pub discard:bool,

    // takeable_starts:HashSet<(GrammarItem<'a>,usize)>, //[(g,output_ind_start)]
    pub takeable_starts_len:usize,
    pub opt:bool,

    pub visiteds:HashSet<(&'a str,usize)>, //used for checking recursive nonterms

    pub takeables:HashMap<GrammarNode<'a>,PrimitiveIterContainer<'a>>, //[non_term]
}

