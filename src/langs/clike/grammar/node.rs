use std::{fmt::Debug, rc::Rc};

// use super::super::grammar::error::GrammarWalkError;



#[derive(Clone,Hash,PartialEq,Eq)]
pub enum GrammarNode<'g> {
    Many(Rc<GrammarNode<'g>>),
    And(Box<[Rc<GrammarNode<'g>>]>,usize), //stow_first, error_ind
    Or( Box<[Rc<GrammarNode<'g>>]>,), //should store reversed?
    NonTerm(&'g str),

    Group(Rc<GrammarNode<'g>>,&'g str,),
    Expect(Rc<GrammarNode<'g>>, &'g str,),
    // NoExpect(Rc<GrammarNode<'g>>, ),
    // Stow(Rc<GrammarNode<'g>>),

    Was(Rc<GrammarNode<'g>>, &'g str),
    // Had(Rc<GrammarNode<'g>>, &'g str),
    Had(&'g str),

    // Prev(Rc<GrammarNode<'g>>),

    String,
    Identifier,
    Int,
    Float,
    Symbol(&'g str),
    Keyword(&'g str),
    Eol,

    Always, //always succeeds
    // Error(GrammarWalkError<'g>),
    Error,


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
    pub fn expect(self,name: &'g str,) -> GrammarNode<'g> {
        Self::Expect(self.into(),name)
    }
    pub fn no_expect(self,) -> GrammarNode<'g> {
        // Self::NoExpect(self.into(),)
        self.expect("")
    }
    // pub fn stow(self) -> GrammarNode<'g> {
    //     Self::Stow(self.into())
    // }
    // pub fn prev(self) -> GrammarNode<'g> {
    //     Self::Prev(self.into())
    // }
    pub fn was(self,name: &'g str,) -> GrammarNode<'g> {
        Self::Was(self.into(),name)
    }
    pub fn had(self,name: &'g str,) -> GrammarNode<'g> {
        // Self::Had(self.into(),name)
        [self.into(),Self::Had(name)].and()
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
    // pub fn is_stow(&self) -> bool {
    //     if let Self::Stow(..)=self {
    //         true
    //     } else {
    //         false
    //     }
    // }
    pub fn is_expect(&self) -> bool {
        if let GrammarNode::Expect(..)=self {
            true
        }else{
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
    // pub fn is_prev(&self) -> bool {
    //     if let GrammarNode::Prev(_)=self {
    //         true
    //     }else{
    //         false
    //     }
    // }

    pub fn is_was(&self) -> bool {
        if let GrammarNode::Was(..)=self {
            true
        }else{
            false
        }
    }
    pub fn is_had(&self) -> bool {
        if let GrammarNode::Had(..)=self {
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
    fn and1(self) -> GrammarNode<'g>;
    fn or(self) -> GrammarNode<'g>;
}

impl<'g,const N: usize> GrammarArrayTrait <'g> for [GrammarNode<'g>; N] {
    fn and(self) -> GrammarNode<'g> {
        // GrammarNode::And(self.into())
        GrammarNode::And(self.into_iter().map(|x|x.into()).collect(),0)
    }
    fn and1(self) -> GrammarNode<'g> {
        // GrammarNode::And(self.into())
        GrammarNode::And(self.into_iter().map(|x|x.into()).collect(),1)
    }
    fn or(self) -> GrammarNode<'g> {
        // GrammarNode::Or(self.into())
        GrammarNode::Or(self.into_iter().map(|x|x.into()).collect(),)
    }
}

pub trait GrammarStrTrait<'g> {
    fn non_term(self) -> GrammarNode<'g>;
    fn symbol(self) -> GrammarNode<'g>;
    fn keyword(self) -> GrammarNode<'g>;
    // fn had(self) -> GrammarNode<'g>;
}

impl<'g> GrammarStrTrait <'g> for &'g str {
    fn non_term(self) -> GrammarNode<'g> {
        GrammarNode::NonTerm(self)
    }
    fn symbol(self) -> GrammarNode<'g> {
        GrammarNode::Symbol(self)
    }
    fn keyword(self) -> GrammarNode<'g> {
        GrammarNode::Keyword(self)
    }
    // fn had(self) -> GrammarNode<'g> {
    //     GrammarNode::Had(self)
    // }
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
            // Self::And(arg0, arg1, arg2) => {
            //     let x=&arg0[*arg1..];
            //     f.debug_tuple("And").field(&x).field(arg1).field(arg2).finish()
            // },
            // Self::Or(arg0, arg1) => {
            //     let x=&arg0[*arg1..];
            //     f.debug_tuple("Or").field(&x).finish()
            // },
            Self::And(arg0, arg1,  ) => {

                f.debug_tuple("And").field(arg0).field(arg1).finish()
            },
            Self::Or(arg0, ) => {
                f.debug_tuple("Or").field(arg0).finish()
            },
            Self::NonTerm(arg0) => f.debug_tuple("NonTerm").field(arg0).finish(),
            Self::Group(arg0, arg1) => f.debug_tuple("Group").field(arg0).field(arg1).finish(),
            Self::Expect(arg0, arg1) => f.debug_tuple("Expect").field(arg0).field(arg1).finish(),
            // Self::NoExpect(arg0, ) => f.debug_tuple("NoExpect").field(arg0).finish(),
            // Self::Stow(arg0) => f.debug_tuple("Stow").field(arg0).finish(),

            // Self::Prev(arg0) => f.debug_tuple("Prev").field(arg0).finish(),
            Self::Was(arg0, arg1) => f.debug_tuple("Was").field(arg0).field(arg1).finish(),
            // Self::Had(arg0, arg1) => f.debug_tuple("Had").field(arg0).field(arg1).finish(),
            Self::Had(arg0) => f.debug_tuple("Had").field(arg0).finish(),


            Self::String => write!(f, "String"),
            Self::Identifier => write!(f, "Identifier"),
            Self::Int => write!(f, "Int"),
            Self::Float => write!(f, "Float"),
            Self::Symbol(arg0) => f.debug_tuple("Symbol").field(arg0).finish(),
            Self::Keyword(arg0) => f.debug_tuple("Keyword").field(arg0).finish(),
            Self::Eol => write!(f, "Eol"),
            Self::Always => write!(f, "Always"),
            // Self::Error(arg0) => f.debug_tuple("Error").field(arg0).finish(),
            Self::Error => f.debug_tuple("Error").finish(),
        }
    }
}