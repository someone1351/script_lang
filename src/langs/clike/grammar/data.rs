use std::ops::Range;

use super::super::{grammar::container::WalkGroupContainer, tokenizer::TokenIterContainer};


pub struct WalkGroup<'g> {
    pub name:&'g str,
    pub children:Range<usize>,
    // pub tokens:TokenIterContainer<'a>,
    pub tokens : Range<usize>,
}
pub struct Walk<'g> {
    pub groups : Vec<WalkGroup<'g>>,
    // pub tokenizer:&'a Tokenizer
}

impl<'g> Walk<'g> {
    pub fn root(&'g self) -> WalkGroupContainer<'g> {
        WalkGroupContainer { walk: self, group_ind: 0 }
    }
}