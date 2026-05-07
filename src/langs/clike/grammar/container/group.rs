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
                    // println!("a{indent}{}:",cur.name());
                    // println!("{indent}group{}: {:?} {:?} {:?}",cur.group_ind,cur.name(),cur.tokens().inds(), cur.tokens());
                    println!("{indent}group: {:?}",cur.name(),);

                    // println!("group {}",cur.group_ind);

                    // let mut todos=Vec::new();
                    let mut cur_tokens = cur.tokens();
                    // println!("\tcur_tokens={cur_tokens:?}");

                    // let mut last_tokens_end= cur_tokens.inds().end;
                    // let mut c=cur.children().len();
                    for child_group in cur.children().rev() {
                        // println!("{indent}cg{} {:?} {}..{}, {}..{}",
                        //     child_group.group_ind,child_group.name(),
                        //     child_group.tokens().inds().start,child_group.tokens().inds().end,
                        //     cur_tokens.inds().start,cur_tokens.inds().end,
                        // );

                        // println!("cg {}",child_group.group_ind);
                    //     // println!("hmmm  ",);

                        let child_tokens=child_group.tokens();
                        // println!("\n\t\t0 cg{} {} {:?}",child_group.group_ind,child_group.name(),child_group.tokens());
                        // println!("\t\t1 child_tokens={child_tokens:?}");
                        // println!("\t\t1 cur_tokens={cur_tokens:?}");
                        // // let ps_start=child_tokens.inds().end;
                        // // let ps_end=cur_tokens.inds().end;
                        // // println!("{ps_start} {ps_end}");
                        // // let ps_len=ps_end-ps_start;
                        // println!("\t\t  cur_tokens.end={} child_tokens.end={}",cur_tokens.end,child_tokens.end);

                        let ps_len=cur_tokens.end-child_tokens.end;
                        // // println!("ps_len={ps_len}");

                        let ps=cur_tokens.pop_back_amount(ps_len).unwrap();

                        // println!("\t\t2 cur_tokens={cur_tokens:?}");
                        // // println!("c{} {ps_start} .. {ps_end}, {}, {}..{}",c-1,cur_tokens.inds().start,child_tokens.inds().start,child_tokens.inds().end);

                        // // let ps=cur_tokens.get_range(ps_start .. ps_end).unwrap();
                        stk.extend(ps.map(|t|(Thing::Token(t),depth+1)).rev());
                        stk.push((Thing::Group(child_group),depth+1));

                        cur_tokens.pop_back_amount(child_tokens.len()).unwrap();

                        // println!("\t\t3 cur_tokens={cur_tokens:?}");
                        // // last_tokens_end=child_tokens.start;
                        // // c-=1;
                    }

                    //
                    // // let ps_start= cur_tokens.start;
                    // // let ps_end=ps_start+last_tokens_end-cur_tokens.start;
                    // // println!("{ps_start} .. {ps_end}, {}..{}",cur_tokens.inds().start,cur_tokens.inds().end);
                    // // let ps=cur_tokens.get_range(ps_start .. ps_end).unwrap();
                    stk.extend(cur_tokens.map(|t|(Thing::Token(t),depth+1)).rev());
                }
                Thing::Token(cur) => {
                    // println!("{indent}{:?}",cur.ind());
                    println!("{indent}{cur:?}");
                }
            }
        }

        Ok(())
    }
}