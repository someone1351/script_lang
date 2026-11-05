// use std::{fmt::Display, marker::PhantomData};

// use crate::{clike::grammar::TokensTrait, Loc};

use crate::clike::grammar::TokensTrait;

use super::super::super::{grammar::{container::WalkGroupIterContainer, data::{Walk, WalkGroup}}, tokenizer::{TokenContainer, TokenIterContainer}};

#[derive(Clone, Copy)]

// pub enum WalkGroupContainer<'g,GR,TS>
// where
//     TS:Clone,
// {
//     Group{  walk:&'g Walk<GR,TS>,  group_ind:usize,},
//     Between {  walk:&'g Walk<GR,TS>,  parent_ind:usize, between_ind:usize,},
// }
pub struct WalkGroupContainer<'g,GR,TS>
where
    TS:Clone,
{
    pub walk:&'g Walk<GR,TS>,
    pub group_ind:usize,
    pub between_ind:Option<usize>,
}

impl<'g,GR,TS> WalkGroupContainer<'g,GR,TS>
where
    GR: Clone,
    TS:Clone+TokensTrait, //+Iterator,
{
    fn group(&self) -> &WalkGroup<GR,TS> {
        // let Self::Group { walk, group_ind } = self.clone() else {panic!("");};
        &self.walk.groups[self.group_ind]
    }

    fn between_tokens(&self,between_ind:usize,) -> Option<TS> {

        //between_ind is ind between  child groups
        //  eg
        //    0 = before child_group_ind0,
        //    1 = between child_group_ind0 and child_group_ind1
        //    n = after child_group_ind_last
        // let Self::Group { walk, group_ind } = self.clone() else {panic!("");};

        // let mut cur_tokens=self.group().tokens.clone();

        // let x = self.tokens().inds2().start;
        let cur_tokens=self.group().tokens.clone();
        let x=cur_tokens.inds2().start;

        let group=self.group();
        let child_num=group.children.len();

        if //self.child_num()==0 || between_ind > self.child_num()
            child_num==0 || between_ind>child_num
        { //out of bounds
            // println!("0 between_ind={between_ind}, child_num={child_num}, {:?}",group.children);

            None
        } else if between_ind==0 { //before start
            // println!("1 between_ind={between_ind}, {:?}",group.children);

            // let n=self.first().unwrap().tokens().inds2().start - self.tokens().inds2().start;
            let a= self.inner_child(0).unwrap().tokens().inds2().start;
            let ts=cur_tokens.clone().take2(a-x).unwrap();

            // (!ts.is_empty2()).then_some(ts)
            Some(ts)

        } else if between_ind == child_num { //afters end
            // println!("2 between_ind={between_ind}, {:?}",group.children);

            // let n=self.tokens().inds2().end - self.last().unwrap().tokens().inds2().end;

            let a=self.inner_child(child_num-1).unwrap().tokens().inds2().end;

            let mut ts = cur_tokens.clone();
            ts.take2(a-x).unwrap();


            // (!ts.is_empty2()).then_some(ts)
            Some(ts)
        } else { //in between
            // println!("3 between_ind={between_ind}, {:?}",group.children);
            let a=self.inner_child(between_ind-1).unwrap().tokens().inds2().end;
            let b=self.inner_child(between_ind).unwrap().tokens().inds2().start;

            let mut ts = cur_tokens.clone();
            ts.take2(a-x).unwrap();
            let ts=ts.take2(b-a).unwrap();

            // (!ts.is_empty2()).then_some(ts)
            Some(ts)
        }

    }
    // pub fn name(&self) -> GR {
    //     self.group().name.unwrap_or("")
    // }
    pub fn name2(&self) -> Option<GR> {
        if self.between_ind.is_some() {
            return None;
        }
        // match self.clone() {
        //     WalkGroupContainer::Group { .. } => {
                self.group().name.clone()
        //     }
        //     WalkGroupContainer::Between { .. } => {
        //         None
        //     }
        // }
    }
    pub fn children(&self) -> WalkGroupIterContainer<'g,GR,TS> {
        self.children_ext(false)
        // if self.between_ind.is_some() {
        //     return WalkGroupIterContainer{betweens:false,parent:0,walk:self.walk, start: 0, end: 0, }
        // }

        // // match self.clone() {
        //     // WalkGroupContainer::Group { walk, group_ind } => {
        //         let group=self.group();
        //         WalkGroupIterContainer{
        //             betweens:false,
        //             parent:self.group_ind,
        //             walk:self.walk, start: group.children.start, end: group.children.end,
        //             // betweens:false,parent:group_ind,
        //         }
        //     // }
        // //     WalkGroupContainer::Between { walk,.. } => {
        // //         WalkGroupIterContainer{ walk, start: 0, end: 0,betweens:false,parent:0, }
        // //     }
        // // }
    }

    pub fn children_ext(&self, betweens:bool,) -> WalkGroupIterContainer<'g,GR,TS> {
        if self.between_ind.is_some() {
            return WalkGroupIterContainer{betweens:false,parent_ind:0,walk:self.walk, start: 0, end: 0, }
        }

        // match self.clone() {
            // WalkGroupContainer::Group { walk, group_ind } => {
                let group=self.group();

                // let start=if betweens {group.children.start*2} else {group.children.start};
                // let end=if betweens {group.children.end*2+1} else {group.children.end};

                let start=0;
                let end=if betweens {group.children.len()*2+1} else {group.children.len()};

                WalkGroupIterContainer{
                    betweens,
                    parent_ind:self.group_ind,
                    walk:self.walk,
                    start, end,
                    // betweens:false,parent:group_ind,
                }
            // }
        //     WalkGroupContainer::Between { walk,.. } => {
        //         WalkGroupIterContainer{ walk, start: 0, end: 0,betweens:false,parent:0, }
        //     }
        // }
    }
    // pub fn children_ext(&self,betweens:bool,) -> WalkGroupIterContainer<'g,GR,TS> {
    //     match self.clone() {
    //         WalkGroupContainer::Group { walk, group_ind } => {
    //             let group=self.group();
    //             let (start,end) = if betweens {
    //                 (group.children.start*2+1, group.children.end*2+1)

    //             } else {
    //                 (group.children.start, group.children.end)
    //             };

    //             WalkGroupIterContainer{ walk, start, end, betweens, parent:group_ind, }
    //         }
    //         WalkGroupContainer::Between { walk,.. } => {
    //             WalkGroupIterContainer{ walk, start: 0, end: 0,betweens:false,parent:0, }
    //         }
    //     }
    // }

    fn inner_child(&self,ind:usize) -> Option<WalkGroupContainer<'g,GR,TS>> {

        let group=self.group();

        if ind < group.children.len() {
            Some(WalkGroupContainer{ between_ind:None, walk:self.walk, group_ind: group.children.start+ind,  })
        } else {
            None
        }
    }
    pub fn child(&self,ind:usize) -> Option<WalkGroupContainer<'g,GR,TS>> {
        if self.between_ind.is_some() {
            return None;
        }

        // match self.clone() {
        //     WalkGroupContainer::Group { walk, .. } => {
                let group=self.group();

                if ind < group.children.len() {
                    Some(WalkGroupContainer{ between_ind:None, walk:self.walk, group_ind: group.children.start+ind,  })
                } else {
                    None
                }
            // }
        //     WalkGroupContainer::Between { .. } => {
        //        None
        //     }
        // }
    }
    pub fn child_num(&self) -> usize {
        if self.between_ind.is_some() {
            return 0;
        }

        // match self.clone() {
        //     WalkGroupContainer::Group {  .. } => {
                let group=self.group();
                group.children.len()
        //     }
        //     WalkGroupContainer::Between { .. } => {
        //         0
        //     }
        // }
    }
    // pub fn first(&self) -> Option<WalkGroupContainer<'g,GR,TS>> {
    //     self.child(0)
    // }
    // pub fn last(&self) -> Option<WalkGroupContainer<'g,GR,TS>> {
    //     if self.children().is_empty() {
    //         None
    //     } else {
    //         self.child(self.children().len()-1)
    //     }
    // }

    pub fn tokens(&self) -> TS {
        if let Some(between_ind)=self.between_ind {
            // println!("=== between_ind={between_ind}, cnum={}",self.group().children.len());
            return self.between_tokens(between_ind).unwrap(); //.unwrap_or_else(||self.walk.groups[0].tokens.clone().take2(0).unwrap());
            // return self.group().tokens.clone();
        }


        // match self.clone() {
        //     WalkGroupContainer::Group { .. } => {
                self.group().tokens.clone()
            // }
            // WalkGroupContainer::Between { walk, parent_ind, between_ind } => {
            //     let parent_group1=&walk.groups[parent_ind];
            //     let parent_group2=WalkGroupContainer::Group { walk, group_ind: parent_ind, };
            //     let all_ind=between_ind*2;

            //     let sibling_all_range=(parent_group1.children.start*2+1) .. (parent_group1.children.end*2+1);

            //     let mut tokens = parent_group2.tokens();

            //     if all_ind + 1 == sibling_all_range.start { //first between
            //         let sibling0=parent_group2.first().unwrap();
            //         let n=sibling0.tokens().inds2().start-parent_group2.tokens().inds2().start;
            //         let x=tokens.clone().take2(n);

            //     } else if all_ind == sibling_all_range.end + 1 { //last between

            //     } else if sibling_all_range.contains(&all_ind) { //in between

            //     } else {
            //         panic!("{all_ind}, {sibling_all_range:?}");
            //     }

            //     // 0 1 2 => 0 [1] 2 [3] 4
            //     // 0 1   => 0 [1] 2

            //     tokens
            // }
        // }
    }
    // pub fn start_loc(&self) -> Loc {
    //     let mut t=self.group().tokens.clone();

    //     // self.group().tokens.trimmed().start_loc()
    // }
    // pub fn between_tokens(&self) -> Vec<TS> {
    //     let mut v=Vec::new();

    //     // writeln!(f,"{indent}group: {:?}",cur.name(),)?;
    //     let mut cur_tokens = self.tokens();

    //     for child_group in self.children() {
    //         let child_tokens=child_group.tokens();
    //         let between_tokens_len=child_tokens.inds2().start-cur_tokens.inds2().start;

    //         // if between_tokens_len!=0 {
    //             let mut between_tokens=cur_tokens.clone();
    //             between_tokens.truncate2(between_tokens_len);
    //             v.push(between_tokens);
    //         // }

    //         cur_tokens.eat2(between_tokens_len+child_tokens.len2());
    //     }

    //     // if cur_tokens.len2()!=0 && v.len() {
    //         v.push(cur_tokens);
    //     // }

    //     v
    // }
}

// impl<'g,TS> std::fmt::Debug for WalkGroupContainer<'g,P,TS>
// where
//     TS:Clone,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}::{:?}", &self.group_ind,&self.name()))
//     }
// }

// impl<'g,TS> Display for WalkGroupContainer<'g,P,TS>
// where
//     TS:Clone,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         enum Thing<'g,TS> {
//             Token(TokenContainer<'t>),
//             Group(WalkGroupContainer<'g,P,TS>),
//         }

//         let mut stk = vec![(Thing::Group(*self),0)];

//         while let Some((cur,depth))=stk.pop() {
//             let indent="    ".repeat(depth);

//             match cur {
//                 Thing::Group(cur) => {
//                     writeln!(f,"{indent}group: {:?}",cur.name(),)?;
//                     let mut cur_tokens = cur.tokens();

//                     for child_group in cur.children().rev() {
//                         let child_tokens=child_group.tokens();
//                         let ps_len=cur_tokens.end-child_tokens.end;
//                         let ps=cur_tokens.pop_back_amount(ps_len).unwrap();

//                         stk.extend(ps.map(|t|(Thing::Token(t),depth+1)).rev());
//                         stk.push((Thing::Group(child_group),depth+1));
//                         cur_tokens.pop_back_amount(child_tokens.len()).unwrap();
//                     }

//                     //
//                     stk.extend(cur_tokens.map(|t|(Thing::Token(t),depth+1)).rev());
//                 }
//                 Thing::Token(cur) => {
//                     writeln!(f,"{indent}{cur:?}")?;
//                 }
//             }
//         }

//         Ok(())
//     }
// }