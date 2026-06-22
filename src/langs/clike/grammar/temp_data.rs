
use std::{collections::{HashMap, HashSet}, fmt::Debug, ops::Range};

use crate::clike::tokenizer::ValueContainer;

// use crate::build::Loc;
use super::super::tokenizer::TokenIterContainer;

use super::node::*;


#[derive(Clone, Debug)]
pub struct TempTakeableStart2<'t,'g> {
    pub grammar:GrammarNode<'g>,
    pub tokens_start:TokenIterContainer<'t>,
    pub group_ind:usize,
}

#[derive(Clone, Debug)]
pub struct WorkTakeable2<'t> {
    pub tokens:TokenIterContainer<'t>,
    pub tokens_start:TokenIterContainer<'t>,
    pub group_ind:usize,
    pub inner_groups:Range<usize>, //groups inside the takeable?
}


// #[derive(Clone, Debug)]
// pub struct TempTakeableStart<'t,'g> {
//     pub grammar:GrammarNode<'g>,
//     pub tokens_start:TokenIterContainer<'t>,
//     pub group_ind:usize,
// }

// #[derive(Clone, Debug)]
// pub struct WorkTakeable<'t> {
//     pub tokens:TokenIterContainer<'t>,
//     pub tokens_start:TokenIterContainer<'t>,
//     pub group_ind:usize,
//     pub inner_groups:Range<usize>, //groups inside the takeable?
// }

#[derive(Clone, Default, Debug)]
pub struct TempGroupsElement<'t,'g> {
    pub groups:Vec<TempGroupInfo<'t,'g>>,
    // pub tokens_start:usize, //not used?
}

#[derive(Clone)]
pub struct TempOrElement<'t,'g> {
    pub groups:Vec<TempGroupInfo<'t,'g>>,
    pub tokens_after:TokenIterContainer<'t>,
}
#[derive(Clone)]
pub struct TempGroupInfo<'t,'g> {
    pub name:&'g str,
    pub parent:usize, //group
    pub tokens:TokenIterContainer<'t>,
}

impl<'t,'g> Debug for  TempGroupInfo<'t,'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TempGroupInfo")
        .field("name", &self.name)
        .field("parent", &self.parent)
        // .field("primitives", &self.primitives)
        .field("primitive_ind_start", &self.tokens.inds().start)
        .finish()
    }
}

#[derive(Default,Copy,Clone)]
pub struct WorkExpected<'g> {
    pub id:u64,
    pub priority:u32,
    pub name:&'g str,
}

#[derive(Clone)]
pub struct Work<'t,'g> {
    pub grammar:GrammarNode<'g>,
    pub success_len:usize,
    pub fail_len:usize,
    pub tokens:TokenIterContainer<'t>,
    pub group_ind:usize,
    pub group_len:usize, //only used for removing unused groups ... but even then it is not required, mainly used for debugging
    pub visiteds:HashSet<(&'g str,usize)>, //used for checking recursive nonterms
    pub grammar_debug_len:usize,
    pub expected:WorkExpected<'g>, //(u64,u32,&'g str), //id,priority,expected
    pub and_id:usize, //for take, to know when continuing on an And, or leaving

    // pub groups_stk_len:usize, //used for take

    // pub takeable_starts_len:usize,
    // pub takeables:HashMap<GrammarNode<'g>,WorkTakeable<'t>>,

    // pub discard:bool,
    // pub opt:bool,


    pub from_user:bool, //gramamr added by input grammar, not walker
    pub takeable_starts_ind2:usize,
    pub takeable_starts_len2:usize,
    pub takeables2:HashMap<GrammarNode<'g>,WorkTakeable2<'t>>,
    // pub ends_in : Option<GrammarNode<'g>>,
    pub or_stk_len:usize,
    pub is_first:bool,
}


#[derive(Clone,Debug)]
pub enum TempGrammarNodeDebug<'t,'g> {
    Many(Vec<Self>),
    And(Vec<Self>),
    Or(Vec<Self>),
    Opt(Option<Box<Self>>),
    Group(&'g str,Option<Box<Self>>),
    Expected(u32,&'g str,Option<Box<Self>>),
    NonTerm(&'g str,Option<Box<Self>>),

    // EndsIn(Option<Box<Self>>, ), //Box<GrammarNode<'g>>
    Prev(Option<Box<Self>>),

    // Cede(Option<Box<Self>>),
    // Take(Option<Box<Self>>),

    // Discard(Option<Box<Self>>),

    String(Option<ValueContainer<'t,&'t str>>),
    Identifier(Option<ValueContainer<'t,&'t str>>),
    Int(Option<ValueContainer<'t,i64>>),
    Float(Option<ValueContainer<'t,f64>>),
    Symbol(Option<ValueContainer<'t,&'t str>>),
    Keyword(Option<ValueContainer<'t,&'t str>>),
    Eol(Option<ValueContainer<'t,()>>),

    Always,
    Error,
}


impl<'t,'g> std::fmt::Display for TempGrammarNodeDebug<'t,'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        enum Work<'t, 'g, 'a> {
            Node(&'a TempGrammarNodeDebug<'t, 'g>),
            Write(&'a str),
        }
        let mut stk: Vec<Work<'t, 'g,'_>>=vec![Work::Node(self)];

        while let Some(cur)=stk.pop() {
            match cur {
                Work::Node(n) => {
                    match n {
                        Self::Many(arg0) => {
                            stk.push(Work::Write(")"));
                            for (i,x) in arg0.into_iter().enumerate().rev() {
                                stk.push(Work::Node(x));
                                if i!=0 { stk.push(Work::Write(",")); }
                            }
                            write!(f,"Many(")?;
                            if arg0.is_empty() {write!(f,"_")?;}
                        }
                        Self::And(arg0) => {
                            stk.push(Work::Write(")"));
                            for (i,x) in arg0.into_iter().enumerate().rev() {

                                stk.push(Work::Node(x));
                                if i!=0{ stk.push(Work::Write(",")); }

                            }
                            write!(f,"And(")?;
                            if arg0.is_empty() {write!(f,"_")?;}
                        }
                        Self::Or(arg0) => {
                            stk.push(Work::Write(")"));
                            for (i,x) in arg0.into_iter().enumerate().rev() {
                                stk.push(Work::Node(x));
                                if i!=0 { stk.push(Work::Write(",")); }
                            }
                            write!(f,"Or(")?;
                            if arg0.is_empty() {write!(f,"_")?;}
                        }
                        Self::Opt(arg0) => {
                            stk.push(Work::Write(")"));
                            stk.push(if let Some(x)=arg0 {Work::Node(x)} else {Work::Write("_")});
                            write!(f,"Opt(")?;
                        }
                        // Self::Cede(arg0) => {
                        //     stk.push(Work::Write(")"));
                        //     stk.push(if let Some(x)=arg0 {Work::Node(x)} else {Work::Write("_")});
                        //     write!(f,"Cede(")?;
                        // }
                        // Self::Take(arg0) => {
                        //     stk.push(Work::Write(")"));
                        //     stk.push(if let Some(x)=arg0 {Work::Node(x)} else {Work::Write("_")});
                        //     write!(f,"Take(")?;
                        // }
                        // Self::EndsIn(arg0, ) => {
                        //     stk.push(Work::Write(")"));
                        //     stk.push(if let Some(x)=arg0 {Work::Node(x)} else {Work::Write("_")});
                        //     write!(f,"EndsIn(")?;
                        // }
                        Self::Prev(arg0, ) => {
                            stk.push(Work::Write(")"));
                            stk.push(if let Some(x)=arg0 {Work::Node(x)} else {Work::Write("_")});
                            write!(f,"Prev(")?;
                        }
                        Self::Group(arg0, arg1) => {
                            stk.push(Work::Write(")"));
                            stk.push(if let Some(x)=arg1 {Work::Node(x)} else {Work::Write("_")});
                            write!(f,"Group({arg0:?},")?;
                        }
                        Self::Expected(arg0, arg1, arg2) => {
                            stk.push(Work::Write(")"));
                            stk.push(if let Some(x)=arg2 {Work::Node(x)} else {Work::Write("_")});
                            write!(f,"Expect({arg0}:{arg1:?},")?;
                        }
                        // Self::Discard(arg0) => {
                        //     stk.push(Work::Write(")"));
                        //     stk.push(if let Some(x)=arg0 {Work::Node(x)} else {Work::Write("_")});
                        //     write!(f,"Discard(")?;
                        // }
                        Self::NonTerm(arg0, arg1) => {
                            stk.push(Work::Write(")"));
                            stk.push(if let Some(x)=arg1 {Work::Node(x)} else {Work::Write("_")});
                            write!(f,"NonTerm({arg0:?},")?;
                        }
                        Self::String(arg0) => {
                            write!(f,"String({})",if let Some(x)=arg0{format!("{}:{:?}",x.token.ind(),x.value)}else{"_".into()})?;

                        }
                        Self::Identifier(arg0) => {
                            write!(f,"Identifier({})",if let Some(x)=arg0{format!("{}:{:?}",x.token.ind(),x.value)}else{"_".into()})?;
                        }
                        Self::Int(arg0) => {
                            write!(f,"Int({})",if let Some(x)=arg0{format!("{}:{}",x.token.ind(),x.value)}else{"_".into()})?;
                        }
                        Self::Float(arg0) => {
                            write!(f,"Float({})",if let Some(x)=arg0{format!("{}:{}",x.token.ind(),x.value)}else{"_".into()})?;

                        }
                        Self::Symbol(arg0) => {
                            write!(f,"Symbol({})",if let Some(x)=arg0{format!("{}:{}",x.token.ind(),x.value)}else{"_".into()})?;
                        }
                        Self::Keyword(arg0) => {
                            write!(f,"Keyword({})",if let Some(x)=arg0{format!("{}:{}",x.token.ind(),x.value)}else{"_".into()})?;
                        }
                        Self::Eol(arg0) => {
                            write!(f,"Eol({})",if let Some(x)=arg0{format!("{}:",x.token.ind(),)}else{"_".into()})?;
                        }
                        Self::Always => {
                            write!(f, "Always")?;
                        }
                        Self::Error => {
                            write!(f, "Error")?;
                        }
                    }
                }
                Work::Write(s) => {
                    write!(f,"{s}")?;
                }
            }
        }

        Ok(())
    }
}