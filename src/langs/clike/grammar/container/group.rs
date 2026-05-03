use std::fmt::Display;

use super::super::super::{grammar::{container::WalkGroupIterContainer, data::{Walk, WalkGroup}}, tokenizer::{TokenContainer, TokenIterContainer}};

#[derive(Clone, Copy)]
pub struct WalkGroupContainer<'t,'g> {
    pub walk:&'g Walk<'t,'g>,
    pub group_ind:usize,
}

impl<'t,'g> WalkGroupContainer<'t,'g> {
    fn group(&self) -> &WalkGroup<'t,'g> {
        &self.walk.groups[self.group_ind]
    }
    pub fn name(&self) -> &'g str {
        self.group().name
    }
    pub fn children(&self) -> WalkGroupIterContainer<'t,'g> {
        let group=self.group();
        WalkGroupIterContainer{ walk: self.walk, start: group.children.start, end: group.children.end }

    }
    // pub fn tokens(&self) -> TokenIterContainer<'a> {
    //     self.group().tokens
    // }

}

impl<'t,'g> std::fmt::Debug for WalkGroupContainer<'t,'g> {
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

impl<'t,'g> Display for WalkGroupContainer<'t,'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        // enum Thing<'a> {
        //     Token(TokenContainer<'a>),
        //     Group(WalkGroupContainer<'a>),
        // }

        // let mut stk = vec![(Thing::Group(*self),0)];

        // while let Some((cur,depth))=stk.pop() {
        //     let indent="    ".repeat(depth);


        //     match cur {
        //         Thing::Group(cur) => {
        //             // println!("a{indent}{}:",cur.name());
        //             println!("a");

        //             let mut todos=Vec::new();
        //             let mut cur_tokens = cur.tokens();

        //             // if let Some(first_group)=cur.children().first() {
        //             //     while cur_tokens.inds().start != first_group.tokens().inds().start {
        //             //         todos.push(Thing::Token(cur_tokens.pop_front().unwrap()));
        //             //     }
        //             // }

        //             for child_group in cur.children() {
        //                 // println!("hmmm  ",);
        //                 let ps_amount=child_group.tokens().inds().start-cur_tokens.inds().start;
        //                 let ps=cur_tokens.pop_front_amount(ps_amount).unwrap();
        //                 todos.extend(ps.map(|p|Thing::Token(p)));

        //                 // while cur_tokens.inds().start < child_group.tokens().inds().start {
        //                 //     todos.push(Thing::Token(cur_tokens.pop_front().unwrap()));
        //                 // }
        //                 todos.push(Thing::Group(child_group));
        //             }

        //             todos.extend(cur_tokens.map(|p|Thing::Token(p)));

        //             stk.extend(todos.into_iter().rev().map(|x|(x,depth+1)));
        //         }
        //         Thing::Token(cur) => {
        //             // println!("b{indent}{cur:?}");
        //             println!("b");
        //         }
        //     }
        // }

        Ok(())
    }
}