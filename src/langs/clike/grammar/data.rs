use std::ops::Range;

use crate::clike::grammar::TokenIterTrait;

use super::super::{grammar::container::WalkGroupContainer, tokenizer::TokenIterContainer};


pub struct WalkGroup<'g,TS>
where
    TS:Clone,
{
    pub name:Option<&'g str>,
    pub children:Range<usize>,
    pub tokens:TS,
    // pub tokens : Range<usize>,
}
pub struct Walk<'g,TS>
where
    TS:Clone,
{
    pub groups : Vec<WalkGroup<'g,TS>>,
    // pub tokenizer:&'a Tokenizer
}

impl<'g,TS> Walk<'g,TS>
where
    TS:Clone+TokenIterTrait,
{
    pub fn root(&'g self) -> WalkGroupContainer<'g,TS> {
        WalkGroupContainer { walk: self, group_ind: 0 }
    }
}