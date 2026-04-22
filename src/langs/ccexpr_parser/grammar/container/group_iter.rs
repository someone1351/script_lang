use std::ops::{Bound, RangeBounds};

use crate::ccexpr_parser::grammar::{container::WalkGroupContainer, data::Walk};

#[derive(Clone, Copy)]
pub struct WalkGroupIterContainer<'a> {
    pub walk:&'a Walk<'a>,
    pub start:usize,
    pub end:usize,
}

impl<'a> WalkGroupIterContainer<'a> {
    pub fn pop_front(&mut self) -> Option<WalkGroupContainer<'a>,> {
        if self.start < self.end {
            let group_ind=self.start;
            self.start+=1;
            Some(WalkGroupContainer { walk: self.walk, group_ind }) //last_loc:self.last_loc,
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.end-self.start
    }

    pub fn get(&self, ind:usize) -> Option<WalkGroupContainer<'a>,> {
        let group_ind= self.start+ind;

        if group_ind < self.end {
            Some(WalkGroupContainer { walk: self.walk, group_ind,})
        } else {
            None
        }
    }

    pub fn get_range<R:RangeBounds<usize>>(&self,r:R) -> WalkGroupIterContainer<'a> {

        let range_start=match r.start_bound() {
            Bound::Included(x)=>*x,
            Bound::Excluded(_)=>panic!(""),
            Bound:: Unbounded=>0,
        };

        let range_end=match r.start_bound() {
            Bound::Included(x)=>*x+1,
            Bound::Excluded(x)=>*x,
            Bound::Unbounded=>self.len(),
        };

        if range_start>range_end { //if range start==end is same as empty iter
            return WalkGroupIterContainer {start: 0, end: 0, walk: self.walk};
        }

        let x_len=range_end-range_start;

        if x_len>self.len() {
            return WalkGroupIterContainer {start: 0, end: 0, walk: self.walk};
        }

        let x_start=self.start+range_start;
        let x_end = x_start+x_len;

        WalkGroupIterContainer {start: x_start, end: x_end, walk: self.walk}
    }

    pub fn is_empty(&self) -> bool {
        self.start==self.end
    }

    pub fn first(&self) -> Option<WalkGroupContainer<'a>,> {
        self.get(0)
    }
}


impl<'a> Iterator for WalkGroupIterContainer<'a> {
    type Item = WalkGroupContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {

            let x=WalkGroupContainer {group_ind: self.start,walk: self.walk,};
            self.start+=1;

            Some(x)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for WalkGroupIterContainer<'a> {
    fn next_back(&mut self) -> Option<WalkGroupContainer<'a>> {
        if self.end > self.start {
            self.end-=1;
            let group_ind=self.end;

            Some(WalkGroupContainer {group_ind,walk: self.walk,}) //last_loc
        } else {
            None
        }
    }
}

impl<'a> std::fmt::Debug for WalkGroupIterContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        f.write_fmt(format_args!("[{}]", self.clone().map(|p|format!("{p:?}")).collect::<Vec<String>>().join(", ")))

    }
}