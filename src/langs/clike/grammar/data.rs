use std::ops::Range;

use crate::clike::grammar::TokenIterTrait;

use super::super::{grammar::container::WalkGroupContainer, tokenizer::TokenIterContainer};


pub struct WalkGroup<GR,TS>
where
    TS:Clone,
{
    pub name:Option<GR>,
    pub children:Range<usize>,
    pub tokens:TS,
    // pub tokens : Range<usize>,
}
pub struct Walk<GR,TS>
where
    TS:Clone,
{
    pub groups : Vec<WalkGroup<GR,TS>>,
    // pub tokenizer:&'a Tokenizer
}

impl<'g,GR,TS> Walk<GR,TS>
where
    TS:Clone+TokenIterTrait,
{
    pub fn root(&'g self) -> WalkGroupContainer<'g,GR,TS> {
        WalkGroupContainer { walk: self, group_ind: 0 }
    }
}