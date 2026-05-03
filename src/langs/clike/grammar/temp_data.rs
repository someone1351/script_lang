
use std::{collections::{HashMap, HashSet}, fmt::Debug};

use crate::clike::tokenizer::{TokenContainer, ValueContainer};

// use crate::build::Loc;
use super::super::tokenizer::TokenIterContainer;

use super::node::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct TempPrimitiveInfo {
    // name:&'a str,
    // depth:usize,
    pub group:usize,
    pub discard:bool,
}

#[derive(Clone)]
pub struct TempGroupInfo<'a,'f> {
    pub name:&'f str,
    pub parent:usize, //group
    // pub primitive_ind_start:usize,
    pub primitives:TokenIterContainer<'a>,
}

impl<'a,'f> Debug for  TempGroupInfo<'a,'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TempGroupInfo")
        .field("name", &self.name)
        .field("parent", &self.parent)
        // .field("primitives", &self.primitives)
        .field("primitive_ind_start", &self.primitives.inds().start)
        .finish()
    }
}

pub struct Work<'a,'f> {
    pub grammar:GrammarNode<'f>,
    pub success_len:usize,
    pub fail_len:usize,
    pub primitives:TokenIterContainer<'a>,
    pub group_ind:usize,

    pub group_len:usize, //only used for removing unused groups ... but even then it is not required, mainly used for debugging
    pub output_len:usize,

    pub discard:bool,

    // takeable_starts:HashSet<(GrammarItem<'a>,usize)>, //[(g,output_ind_start)]
    pub takeable_starts_len:usize,
    pub opt:bool,

    pub visiteds:HashSet<(&'f str,usize)>, //used for checking recursive nonterms

    pub takeables:HashMap<GrammarNode<'f>,TokenIterContainer<'a>>, //[non_term]
    pub grammar_debug_len:usize,
    // pub grammar_debug_no_add:bool,
}



#[derive(Clone,Debug,)]
pub enum TempGrammarNodeDebug<'t,'g> {
    Many(
        // Option<Box<Self>>
        Vec<Self>
    ),
    And(Vec<Self>),
    Or(Vec<Self>),
    Opt(Option<Box<Self>>),
    Cede(Option<Box<Self>>),
    Take(Option<Box<Self>>),
    Group(&'g str,Option<Box<Self>>),

    String(Option<ValueContainer<'t,&'t str>>),
    Identifier(Option<ValueContainer<'t,&'t str>>),
    Int(Option<ValueContainer<'t,i64>>),
    Float(Option<ValueContainer<'t,f64>>),
    Symbol(Option<ValueContainer<'t,&'t str>>),
    Keyword(Option<ValueContainer<'t,&'t str>>),
    Eol(Option<ValueContainer<'t,()>>),

    NonTerm(&'g str,Option<Box<Self>>),
    Always,
    Error,
    Discard(Option<Box<Self>>),
}