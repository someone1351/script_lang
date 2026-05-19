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
    pub fn tokens(&self) -> TokenIterContainer<'t> {
        self.group().tokens
    }
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
        enum Thing<'t,'g> {
            Token(TokenContainer<'t>),
            Group(WalkGroupContainer<'t,'g>),
        }

        let mut stk = vec![(Thing::Group(*self),0)];

        while let Some((cur,depth))=stk.pop() {
            let indent="    ".repeat(depth);

            match cur {
                Thing::Group(cur) => {
                    writeln!(f,"{indent}group: {:?}",cur.name(),)?;
                    let mut cur_tokens = cur.tokens();

                    for child_group in cur.children().rev() {
                        let child_tokens=child_group.tokens();
                        let ps_len=cur_tokens.end-child_tokens.end;
                        let ps=cur_tokens.pop_back_amount(ps_len).unwrap();

                        stk.extend(ps.map(|t|(Thing::Token(t),depth+1)).rev());
                        stk.push((Thing::Group(child_group),depth+1));
                        cur_tokens.pop_back_amount(child_tokens.len()).unwrap();
                    }

                    //
                    stk.extend(cur_tokens.map(|t|(Thing::Token(t),depth+1)).rev());
                }
                Thing::Token(cur) => {
                    writeln!(f,"{indent}{cur:?}")?;
                }
            }
        }

        Ok(())
    }
}