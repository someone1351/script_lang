use std::fmt::Display;

use crate::ccexpr_parser::{grammar::{container::WalkGroupIterContainer, data::{Walk, WalkGroup}}, tokenizer::TokenIterContainer};

#[derive(Clone, Copy)]
pub struct WalkGroupContainer<'a> {
    pub walk:&'a Walk<'a>,
    pub group_ind:usize,
}

impl<'a> WalkGroupContainer<'a> {
    fn group(&self) -> &WalkGroup<'a> {
        &self.walk.groups[self.group_ind]
    }
    pub fn name(&self) -> &'a str {
        self.group().name
    }
    pub fn children(&self) -> WalkGroupIterContainer<'a> {
        let group=self.group();
        WalkGroupIterContainer{ walk: self.walk, start: group.children.start, end: group.children.end }

    }
    pub fn tokens(&self) -> TokenIterContainer<'a> {
        self.group().tokens
    }

}

impl<'a> std::fmt::Debug for WalkGroupContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::{:?}", &self.group_ind,&self.name()))
        // f.wr
        // f.debug_struct("Primitive")
        // // .field("parsed", &self.parsed)
        // .field("primitive_ind", &self.primitive_ind)
        // .field("loc", &self.start_loc())
        // .field("primitive_type", &format!("{:?}",self.primitive_type()))
        // .finish()
    }
}

impl<'a> Display for WalkGroupContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        Ok(())
    }
}