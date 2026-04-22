use std::ops::Range;

use crate::ccexpr_parser::{grammar::container::WalkGroupContainer, tokenizer::TokenIterContainer};


pub struct WalkGroup<'a> {
    pub name:&'a str,
    pub children:Range<usize>,
    pub tokens:TokenIterContainer<'a>,
}
pub struct Walk<'a> {
    pub groups : Vec<WalkGroup<'a>>,
}

impl<'a> Walk<'a> {
    pub fn root(&'a self) -> WalkGroupContainer<'a> {
        WalkGroupContainer { walk: self, group_ind: 0 }
    }
}