use std::ops::{Bound, RangeBounds};

use super::super::super::grammar::{container::WalkGroupContainer, data::Walk};

#[derive(Clone, Copy)]
pub struct WalkGroupIterContainer<'t,'g> {
    pub walk:&'g Walk<'t,'g>,
    pub start:usize,
    pub end:usize,
}

impl<'t,'g> WalkGroupIterContainer<'t,'g> {
    pub fn pop_front(&mut self) -> Option<WalkGroupContainer<'t,'g>,> {
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

    pub fn get(&self, ind:usize) -> Option<WalkGroupContainer<'t,'g>,> {
        let group_ind= self.start+ind;

        if group_ind < self.end {
            Some(WalkGroupContainer { walk: self.walk, group_ind,})
        } else {
            None
        }
    }

    pub fn get_range<R:RangeBounds<usize>>(&self,r:R) -> WalkGroupIterContainer<'t,'g> {

        let range_start=match r.start_bound().cloned() {
            Bound::Included(x)=>x,
            Bound::Excluded(_)=>panic!(""),
            Bound::Unbounded=>0,
        };

        let range_end=match r.end_bound().cloned() {
            Bound::Included(x)=>x+1,
            Bound::Excluded(x)=>x,
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

    pub fn first(&self) -> Option<WalkGroupContainer<'t,'g>,> {
        self.get(0)
    }
}


impl<'t,'g> Iterator for WalkGroupIterContainer<'t,'g> {
    type Item = WalkGroupContainer<'t,'g>;

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

impl<'t,'g> DoubleEndedIterator for WalkGroupIterContainer<'t,'g> {
    fn next_back(&mut self) -> Option<WalkGroupContainer<'t,'g>> {
        if self.end > self.start {
            self.end-=1;
            let group_ind=self.end;

            Some(WalkGroupContainer {group_ind,walk: self.walk,}) //last_loc
        } else {
            None
        }
    }
}

impl<'t,'g> std::fmt::Debug for WalkGroupIterContainer<'t,'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        f.write_fmt(format_args!("[{}]", self.clone().map(|p|format!("{p:?}")).collect::<Vec<String>>().join(", ")))

    }
}