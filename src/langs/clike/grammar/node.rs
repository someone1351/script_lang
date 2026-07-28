use std::{fmt::Debug, rc::Rc};

use super::super::grammar::error::GrammarWalkError;



#[derive(Clone,Hash,PartialEq,Eq)]
pub enum GrammarNode<'g> {
    Many(Rc<GrammarNode<'g>>),
    And(Rc<[Rc<GrammarNode<'g>>]>,usize), //should store reversed?
    Or( Rc<[Rc<GrammarNode<'g>>]>,usize), //should store reversed?
    NonTerm(&'g str),

    Group(Rc<GrammarNode<'g>>,&'g str,),
    Expected(Rc<GrammarNode<'g>>, &'g str,),

    Prev(Rc<GrammarNode<'g>>),

    String,
    Identifier,
    Int,
    Float,
    Symbol(&'g str),
    Keyword(&'g str),
    Eol,

    Always, //always succeeds
    Error(GrammarWalkError<'g>),


    // Stow(Rc<GrammarNode<'g>>),

    // Mark(Rc<GrammarNode<'g>>),
}

impl<'g> GrammarNode<'g> {
    pub fn many0(self) -> GrammarNode<'g> {
        Self::Many(self.into())
    }
    pub fn many1(self) -> GrammarNode<'g> {
        [self.clone(),self.many0(),].and()
    }
    pub fn opt(self) -> GrammarNode<'g> {
        [self.into(),Self::Always].or()
    }
    pub fn group(self,name: &'g str) -> GrammarNode<'g> {
        Self::Group(self.into(),name)
    }
    pub fn expected(self,name: &'g str,) -> GrammarNode<'g> {
        Self::Expected(self.into(),name)
    }
    pub fn prev(self) -> GrammarNode<'g> {
        Self::Prev(self.into())
    }
    // pub fn stow(self) -> GrammarNode<'g> {
    //     Self::Stow(self.into())
    // }
    pub fn is_many(&self) -> bool {
        if let GrammarNode::Many(..)=self {true} else {false}
    }
    // pub fn is_nonterm(&self) -> bool {
    //     if let GrammarNode::NonTerm(..)=self {true} else {false}
    // }
    pub fn is_or(&self) -> bool {
        if let GrammarNode::Or(..)=self {true} else {false}
    }
    pub fn is_and(&self) -> bool {
        if let GrammarNode::And(..)=self {true} else {false}
    }
    pub fn get_non_term_name(&self) -> Option<&'g str> {
        if let Self::NonTerm(n)=self {
            Some(n)
        } else {
            None
        }
    }
    pub fn is_non_term(&self) -> bool {
        if let Self::NonTerm(..)=self {
            true
        } else {
            false
        }
    }
    pub fn is_always(&self) -> bool {
        if let GrammarNode::Always=self {
            true
        }else{
            false
        }
    }
    pub fn is_prev(&self) -> bool {
        if let GrammarNode::Prev(_)=self {
            true
        }else{
            false
        }
    }
    pub fn is_primtive(&self) -> bool {
        match self {
            // GrammarNode::Many(grammar_node) => todo!(),
            // GrammarNode::And(grammar_nodes) => todo!(),
            // GrammarNode::Or(grammar_nodes) => todo!(),
            // GrammarNode::NonTerm(_) => todo!(),
            // GrammarNode::Group(grammar_node, _) => todo!(),
            // GrammarNode::Expected(grammar_node, _) => todo!(),
            // GrammarNode::Prev(grammar_node) => todo!(),
            GrammarNode::String => true,
            GrammarNode::Identifier => true,
            GrammarNode::Int => true,
            GrammarNode::Float => true,
            GrammarNode::Symbol(_) => true,
            GrammarNode::Keyword(_) => true,
            GrammarNode::Eol => true,
            // GrammarNode::Always => todo!(),
            // GrammarNode::Error(grammar_walk_error) => todo!(),
            _ => false,
        }
    }
}

//todo have array stored in rev for or/and
pub trait GrammarArrayTrait<'g> {
    fn and(self) -> GrammarNode<'g>;
    fn or(self) -> GrammarNode<'g>;
}

impl<'a,const N: usize> GrammarArrayTrait <'a> for [GrammarNode<'a>; N] {
    fn and(self) -> GrammarNode<'a> {
        // GrammarNode::And(self.into())
        GrammarNode::And(self.into_iter().map(|x|x.into()).collect(),0)
    }
    fn or(self) -> GrammarNode<'a> {
        // GrammarNode::Or(self.into())
        GrammarNode::Or(self.into_iter().map(|x|x.into()).collect(),0)
    }
}

// impl<'a, const N: usize> From<[GrammarItem<'a>; N]> for  GrammarItem<'a> {
//     fn from(value: [GrammarItem<'a>; N]) -> Self {
//         Self::And(value.into())
//     }
// }

// #[macro_export]
// macro_rules! and {
//     ( $( $x:expr ),* $(,)? ) => {{
//         let mut v = Vec::new();
//         $( v.push($x); )*
//         GrammarItem::And(v.into())
//     }};
// }

// #[macro_export]
// macro_rules! or {
//     ( $( $x:expr ),* $(,)? ) => {{
//         let mut v = Vec::new();
//         $( v.push($x); )*
//         GrammarItem::And(v.into())
//     }};
// }


impl<'g> Debug for GrammarNode<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Many(arg0) => f.debug_tuple("Many").field(arg0).finish(),
            Self::And(arg0, arg1) => {
                let x=&arg0[*arg1..];
                f.debug_tuple("And").field(&x).finish()
            },
            Self::Or(arg0, arg1) => {
                let x=&arg0[*arg1..];
                f.debug_tuple("Or").field(&x).finish()
            },
            Self::NonTerm(arg0) => f.debug_tuple("NonTerm").field(arg0).finish(),
            Self::Group(arg0, arg1) => f.debug_tuple("Group").field(arg0).field(arg1).finish(),
            Self::Expected(arg0, arg1) => f.debug_tuple("Expected").field(arg0).field(arg1).finish(),
            Self::Prev(arg0) => f.debug_tuple("Prev").field(arg0).finish(),
            Self::String => write!(f, "String"),
            Self::Identifier => write!(f, "Identifier"),
            Self::Int => write!(f, "Int"),
            Self::Float => write!(f, "Float"),
            Self::Symbol(arg0) => f.debug_tuple("Symbol").field(arg0).finish(),
            Self::Keyword(arg0) => f.debug_tuple("Keyword").field(arg0).finish(),
            Self::Eol => write!(f, "Eol"),
            Self::Always => write!(f, "Always"),
            Self::Error(arg0) => f.debug_tuple("Error").field(arg0).finish(),
        }
    }
}