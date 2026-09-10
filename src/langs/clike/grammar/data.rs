use std::ops::Range;

use super::super::{grammar::container::WalkGroupContainer, tokenizer::TokenIterContainer};


pub struct WalkGroup<'g,TS>
where
    TS:Clone,
{
    pub name:&'g str,
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
    TS:Clone,
{
    pub fn root(&'g self) -> WalkGroupContainer<'g,TS> {
        WalkGroupContainer { walk: self, group_ind: 0 }
    }
}