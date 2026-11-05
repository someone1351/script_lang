use std::ops::{Bound, RangeBounds};

use crate::clike::grammar::TokensTrait;

use super::super::super::grammar::{container::WalkGroupContainer, data::Walk};

#[derive(Clone, )]
pub struct WalkGroupIterContainer<'g,GR,TS>
where
    TS:Clone,
{
    pub walk:&'g Walk<GR,TS>,

    //if between mul 2 and add 1
    pub start:usize,
    pub end:usize,

    // pub betweens:bool,
    // pub parent : usize, //no iterator for root ever given, so no need for Option<>
}

impl<'g,GR,TS> WalkGroupIterContainer<'g,GR,TS>
where
    TS:Clone,
{
    // pub fn pop_front(&mut self) -> Option<WalkGroupContainer<'g,GR,TS>,> {
    //     if self.start < self.end {
    //         let group_ind=self.start;
    //         self.start+=1;
    //         Some(WalkGroupContainer { walk: self.walk, group_ind }) //last_loc:self.last_loc,
    //     } else {
    //         None
    //     }
    // }

    pub fn len(&self) -> usize {
        self.end-self.start
    }

    // pub fn get(&self, ind:usize) -> Option<WalkGroupContainer<'g,GR,TS>,> {
    //     let group_ind= self.start+ind;

    //     if group_ind < self.end {
    //         Some(WalkGroupContainer { walk: self.walk, group_ind,})
    //     } else {
    //         None
    //     }
    // }

    // pub fn get_range<R:RangeBounds<usize>>(&self,r:R) -> WalkGroupIterContainer<'g,GR,TS> {

    //     let range_start=match r.start_bound().cloned() {
    //         Bound::Included(x)=>x,
    //         Bound::Excluded(_)=>panic!(""),
    //         Bound::Unbounded=>0,
    //     };

    //     let range_end=match r.end_bound().cloned() {
    //         Bound::Included(x)=>x+1,
    //         Bound::Excluded(x)=>x,
    //         Bound::Unbounded=>self.len(),
    //     };

    //     if range_start>range_end { //if range start==end is same as empty iter
    //         return WalkGroupIterContainer {start: 0, end: 0, walk: self.walk, between:None,};
    //     }

    //     let x_len=range_end-range_start;

    //     if x_len>self.len() {
    //         return WalkGroupIterContainer {start: 0, end: 0, walk: self.walk, between:None};
    //     }

    //     let x_start=self.start+range_start;
    //     let x_end = x_start+x_len;

    //     WalkGroupIterContainer {start: x_start, end: x_end, walk: self.walk}
    // }

    pub fn is_empty(&self) -> bool {
        self.start==self.end
    }

    // pub fn first(&self) -> Option<WalkGroupContainer<'g,GR,TS>,> {
    //     self.get(0)
    // }
    // pub fn split<F>(&self,f:F) -> Vec<WalkGroupIterContainer<'g,GR,TS>>
    // where
    //     F:Fn(WalkGroupContainer<'g,GR,TS>) -> bool,
    // {
    //     if self.len()==0 {
    //         return Vec::new();
    //     }

    //     //
    //     let mut v=Vec::new();
    //     let mut x=self.start;

    //     // print!("!!!!");
    //     // print!("gs is {self:?}");
    //     for i in self.start ..self.end {
    //         let g=self.get(i-self.start).unwrap();

    //         if f(g) {
    //             if x!=i {
    //                 v.push(WalkGroupIterContainer{
    //                     walk: self.walk, start: x, end: i
    //                 });
    //             }

    //             x=i+1;
    //         }
    //     }

    //     // if x==self.start {
    //     //     return vec![self.clone()];
    //     // }

    //     if x!=self.end {
    //         v.push(WalkGroupIterContainer{ walk: self.walk, start: x, end: self.end });
    //     }

    //     v
    // }



    // pub fn split_between<F>(&self,f:F) -> Vec<WalkGroupIterContainer<'g,GR,TS>>
    // where
    //     F:Fn(WalkGroupContainer<'g,GR,TS>) -> bool,
    // {
    //     if self.len()==0 {
    //         return Vec::new();
    //     }

    //     //
    //     let mut v=Vec::new();
    //     let mut x=self.start;

    //     for i in self.start ..self.end {
    //         let g=self.get(i-self.start).unwrap();

    //         let fg=f(g);
    //         // println!("f(g)={fg}",);
    //         if fg {
    //             if x!=i {
    //                 v.push(WalkGroupIterContainer{ walk: self.walk, start: x, end: i });
    //             }

    //             v.push(WalkGroupIterContainer{ walk: self.walk, start: i, end: i+1 });

    //             x=i+1;
    //         }
    //     }

    //     // if x==self.start {
    //     //     return vec![self.clone()];
    //     // }

    //     if x!=self.end {
    //         v.push(WalkGroupIterContainer{ walk: self.walk, start: x, end: self.end });
    //     }

    //     v
    // }
}


impl<'g,GR,TS> Iterator for WalkGroupIterContainer<'g,GR,TS>
where
    TS:Clone,
{
    type Item = WalkGroupContainer<'g,GR,TS>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {


            let x=WalkGroupContainer {group_ind: self.start,walk: self.walk,};

            // 9 = 0 [1] 2 [3] 4 [5] 6 [7] 8
            // 5 = 0 2 4 6 8

            // 7 = 0 [1] 2 [3] 4 [5] 6
            // 4 = 0 2 4 6

            // 5 = 0 [1] 2 [3] 4
            // 3 = 0 2 4

            // 3 = 0 [1] 2
            // 2 = 0 2

            // let x = if self.betweens {
            //     if self.start%2 == 0 {
            //         WalkGroupContainer::Between {between_ind: self.start/2+1,walk: self.walk,parent_ind:self.parent,}
            //     } else {
            //         WalkGroupContainer::Group {group_ind: (self.start-1)/2,walk: self.walk,}
            //     }
            // } else {
            //     WalkGroupContainer::Group {group_ind: self.start,walk: self.walk,}
            // };

            self.start+=1;

            Some(x)
        } else {
            None
        }
    }
}

impl<'g,GR,TS> DoubleEndedIterator for WalkGroupIterContainer<'g,GR,TS>
where
    TS:Clone,
{
    fn next_back(&mut self) -> Option<WalkGroupContainer<'g,GR,TS>> {
        if self.end > self.start {
            self.end-=1;

            let group_ind=self.end;
            Some(WalkGroupContainer {group_ind,walk: self.walk,}) //last_loc

            // let x = if self.betweens {
            //     if self.end%2 == 0 {
            //         WalkGroupContainer::Between {between_ind: self.end/2+1,walk: self.walk,parent_ind:self.parent,}
            //     } else {
            //         WalkGroupContainer::Group {group_ind: (self.end-1)/2,walk: self.walk,}
            //     }
            // } else {
            //     WalkGroupContainer::Group {group_ind: self.end,walk: self.walk,}
            // };

            // Some(x)
        } else {
            None
        }
    }
}

// impl<'g,TS> std::fmt::Debug for WalkGroupIterContainer<'g,GR,TS> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//         f.write_fmt(format_args!("[{}]", self.clone().map(|p|format!("{p:?}")).collect::<Vec<String>>().join(", ")))

//     }
// }