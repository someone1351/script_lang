use std::{fmt::Display, marker::PhantomData};

use crate::{clike::grammar::TokenIterTrait, Loc};

use super::super::super::{grammar::{container::WalkGroupIterContainer, data::{Walk, WalkGroup}}, tokenizer::{TokenContainer, TokenIterContainer}};

#[derive(Clone, Copy)]
pub struct WalkGroupContainer<'g,TS>
where
    TS:Clone+TokenIterTrait,
{
    pub walk:&'g Walk<'g,TS>,
    pub group_ind:usize,
}

impl<'g,TS> WalkGroupContainer<'g,TS>
where
    TS:Clone+TokenIterTrait,
{
    fn group(&self) -> &WalkGroup<'g,TS> {
        &self.walk.groups[self.group_ind]
    }
    pub fn name(&self) -> &'g str {
        self.group().name.unwrap_or("")
    }
    pub fn children(&self) -> WalkGroupIterContainer<'g,TS> {
        let group=self.group();
        WalkGroupIterContainer{ walk: self.walk, start: group.children.start, end: group.children.end }
    }
    pub fn child(&self,ind:usize) -> Option<WalkGroupContainer<'g,TS>> {
        let group=self.group();

        if ind < group.children.len() {
            Some(WalkGroupContainer{ walk: self.walk, group_ind: group.children.start+ind })
        } else {
            None
        }
    }
    pub fn first(&self) -> Option<WalkGroupContainer<'g,TS>> {
        self.child(0)
    }
    pub fn last(&self) -> Option<WalkGroupContainer<'g,TS>> {
        if self.children().is_empty() {
            None
        } else {
            self.child(self.children().len()-1)
        }
    }

    pub fn tokens(&self) -> TS {
        self.group().tokens.clone()
    }
    // pub fn start_loc(&self) -> Loc {
    //     let mut t=self.group().tokens.clone();

    //     // self.group().tokens.trimmed().start_loc()
    // }
    pub fn between_tokens(&self) -> Vec<TS> {
        let mut v=Vec::new();

        // writeln!(f,"{indent}group: {:?}",cur.name(),)?;
        let mut cur_tokens = self.tokens();

        for child_group in self.children() {
            let child_tokens=child_group.tokens();
            let between_tokens_len=child_tokens.inds2().start-cur_tokens.inds2().start;

            // if between_tokens_len!=0 {
                let mut between_tokens=cur_tokens.clone();
                between_tokens.truncate(between_tokens_len);
                v.push(between_tokens);
            // }

            cur_tokens.take2(between_tokens_len+child_tokens.len2());
        }

        // if cur_tokens.len2()!=0 && v.len() {
            v.push(cur_tokens);
        // }

        v
    }
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