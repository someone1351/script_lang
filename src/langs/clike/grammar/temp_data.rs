
use std::{collections::{HashMap, HashSet}, fmt::Debug};

use crate::clike::tokenizer::ValueContainer;

// use crate::build::Loc;
use super::super::tokenizer::TokenIterContainer;

use super::node::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct TempPrimitiveInfo {
    // name:&'a str,
    // depth:usize,
    pub group:usize,
    pub discard:bool,
}

#[derive(Clone)]
pub struct TempGroupInfo<'a,'f> {
    pub name:&'f str,
    pub parent:usize, //group
    // pub primitive_ind_start:usize,
    pub primitives:TokenIterContainer<'a>,
}

impl<'a,'f> Debug for  TempGroupInfo<'a,'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TempGroupInfo")
        .field("name", &self.name)
        .field("parent", &self.parent)
        // .field("primitives", &self.primitives)
        .field("primitive_ind_start", &self.primitives.inds().start)
        .finish()
    }
}

#[derive(Default,Copy,Clone)]
pub struct WorkExpected<'g> {
    pub id:u64,
    pub priority:u32,
    pub name:&'g str,
}
pub struct Work<'t,'g> {
    pub grammar:GrammarNode<'g>,
    pub success_len:usize,
    pub fail_len:usize,
    pub tokens:TokenIterContainer<'t>,
    pub group_ind:usize,

    pub group_len:usize, //only used for removing unused groups ... but even then it is not required, mainly used for debugging
    pub output_len:usize,

    pub discard:bool,

    // takeable_starts:HashSet<(GrammarItem<'a>,usize)>, //[(g,output_ind_start)]
    pub takeable_starts_len:usize,
    pub opt:bool,

    pub visiteds:HashSet<(&'g str,usize)>, //used for checking recursive nonterms

    pub takeables:HashMap<GrammarNode<'g>,TokenIterContainer<'t>>, //[non_term]
    pub grammar_debug_len:usize,
    // pub grammar_debug_no_add:bool,
    // pub expected:Option<&'g str>,
    pub expected:WorkExpected<'g>, //(u64,u32,&'g str), //id,priority,expected
}



#[derive(Clone,Debug)]
pub enum TempGrammarNodeDebug<'t,'g> {
    Many(Vec<Self>),
    And(Vec<Self>),
    Or(Vec<Self>),

    Opt(Option<Box<Self>>),
    Cede(Option<Box<Self>>),
    Take(Option<Box<Self>>),
    Group(&'g str,Option<Box<Self>>),
    Expected(u32,&'g str,Option<Box<Self>>),
    // Discard(Option<Box<Self>>),
    NonTerm(&'g str,Option<Box<Self>>),

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

// impl<'t,'g> TempGrammarNodeDebug<'t,'g> {
//     pub fn token_val_mut(&mut self)
// }


// impl<'t,'g> Into for Option<TempGrammarNodeDebug<'t,'g>> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Some(x) => write!(f,"{x}"),
//             None => write!(f,"_"),
//         }
//     }
// }

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
                        Self::Cede(arg0) => {
                            stk.push(Work::Write(")"));
                            stk.push(if let Some(x)=arg0 {Work::Node(x)} else {Work::Write("_")});
                            write!(f,"Cede(")?;
                        }
                        Self::Take(arg0) => {
                            stk.push(Work::Write(")"));
                            stk.push(if let Some(x)=arg0 {Work::Node(x)} else {Work::Write("_")});
                            write!(f,"Take(")?;
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
                            write!(f,"String({})",if let Some(x)=arg0{format!("{}:{:?}",x.primitive.ind(),x.value)}else{"_".into()})?;

                        }
                        Self::Identifier(arg0) => {
                            write!(f,"Identifier({})",if let Some(x)=arg0{format!("{}:{:?}",x.primitive.ind(),x.value)}else{"_".into()})?;
                        }
                        Self::Int(arg0) => {
                            write!(f,"Int({})",if let Some(x)=arg0{format!("{}:{}",x.primitive.ind(),x.value)}else{"_".into()})?;
                        }
                        Self::Float(arg0) => {
                            write!(f,"Float({})",if let Some(x)=arg0{format!("{}:{}",x.primitive.ind(),x.value)}else{"_".into()})?;

                        }
                        Self::Symbol(arg0) => {
                            write!(f,"Symbol({})",if let Some(x)=arg0{format!("{}:{}",x.primitive.ind(),x.value)}else{"_".into()})?;
                        }
                        Self::Keyword(arg0) => {
                            write!(f,"Keyword({})",if let Some(x)=arg0{format!("{}:{}",x.primitive.ind(),x.value)}else{"_".into()})?;
                        }
                        Self::Eol(arg0) => {
                            write!(f,"Eol({})",if let Some(x)=arg0{format!("{}:",x.primitive.ind(),)}else{"_".into()})?;
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