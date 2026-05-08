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
                    // writeln!(f,"a{indent}{}:",cur.name());
                    // writeln!(f,"{indent}group{}: {:?} {:?} {:?}",cur.group_ind,cur.name(),cur.tokens().inds(), cur.tokens());
                    writeln!(f,"{indent}group: {:?}",cur.name(),)?;

                    // writeln!(f,"group {}",cur.group_ind);

                    // let mut todos=Vec::new();
                    let mut cur_tokens = cur.tokens();
                    // writeln!(f,"\tcur_tokens={cur_tokens:?}");

                    // let mut last_tokens_end= cur_tokens.inds().end;
                    // let mut c=cur.children().len();
                    for child_group in cur.children().rev() {
                        // writeln!(f,"{indent}cg{} {:?} {}..{}, {}..{}",
                        //     child_group.group_ind,child_group.name(),
                        //     child_group.tokens().inds().start,child_group.tokens().inds().end,
                        //     cur_tokens.inds().start,cur_tokens.inds().end,
                        // );

                        // writeln!(f,"cg {}",child_group.group_ind);
                    //     // writeln!(f,"hmmm  ",);

                        let child_tokens=child_group.tokens();
                        // writeln!(f,"\n\t\t0 cg{} {} {:?}",child_group.group_ind,child_group.name(),child_group.tokens());
                        // writeln!(f,"\t\t1 child_tokens={child_tokens:?}");
                        // writeln!(f,"\t\t1 cur_tokens={cur_tokens:?}");
                        // // let ps_start=child_tokens.inds().end;
                        // // let ps_end=cur_tokens.inds().end;
                        // // writeln!(f,"{ps_start} {ps_end}");
                        // // let ps_len=ps_end-ps_start;
                        // writeln!(f,"\t\t  cur_tokens.end={} child_tokens.end={}",cur_tokens.end,child_tokens.end);

                        let ps_len=cur_tokens.end-child_tokens.end;
                        // // writeln!(f,"ps_len={ps_len}");

                        let ps=cur_tokens.pop_back_amount(ps_len).unwrap();

                        // writeln!(f,"\t\t2 cur_tokens={cur_tokens:?}");
                        // // writeln!(f,"c{} {ps_start} .. {ps_end}, {}, {}..{}",c-1,cur_tokens.inds().start,child_tokens.inds().start,child_tokens.inds().end);

                        // // let ps=cur_tokens.get_range(ps_start .. ps_end).unwrap();
                        stk.extend(ps.map(|t|(Thing::Token(t),depth+1)).rev());
                        stk.push((Thing::Group(child_group),depth+1));

                        cur_tokens.pop_back_amount(child_tokens.len()).unwrap();

                        // writeln!(f,"\t\t3 cur_tokens={cur_tokens:?}");
                        // // last_tokens_end=child_tokens.start;
                        // // c-=1;
                    }

                    //
                    // // let ps_start= cur_tokens.start;
                    // // let ps_end=ps_start+last_tokens_end-cur_tokens.start;
                    // // writeln!(f,"{ps_start} .. {ps_end}, {}..{}",cur_tokens.inds().start,cur_tokens.inds().end);
                    // // let ps=cur_tokens.get_range(ps_start .. ps_end).unwrap();
                    stk.extend(cur_tokens.map(|t|(Thing::Token(t),depth+1)).rev());
                }
                Thing::Token(cur) => {
                    // writeln!(f,"{indent}{:?}",cur.ind());
                    writeln!(f,"{indent}{cur:?}")?;
                }
            }
        }

        Ok(())
    }
}