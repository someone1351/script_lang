use std::ops::Range;

use super::super::{grammar::container::WalkGroupContainer, tokenizer::TokenIterContainer};


pub struct WalkGroup<'t,'g> {
    pub name:&'g str,
    pub children:Range<usize>,
    pub tokens:TokenIterContainer<'t>,
    // pub tokens : Range<usize>,
}
pub struct Walk<'t,'g> {
    pub groups : Vec<WalkGroup<'t,'g>>,
    // pub tokenizer:&'a Tokenizer
}

impl<'t,'g> Walk<'t,'g> {
    pub fn root(&'g self) -> WalkGroupContainer<'t,'g> {
        WalkGroupContainer { walk: self, group_ind: 0 }
    }
}