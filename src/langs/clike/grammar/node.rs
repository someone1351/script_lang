use std::{fmt::Debug, rc::Rc};

// use super::super::grammar::error::GrammarWalkError;

/*
TODO
* could add Trim node, that sets Work::trim to true, then on primitive, calls tokens.trim() before trying to get primitive
** would use eg Symbol("+").trim()
*** have it trim on group and primitive
** or have it immediately trim tokens instead of setting flag
*** could call it before a group so that eol is trimmed out of the group
**** eg Symbol("+").group("plus").trim()
*/


// #[derive(PartialEq,Eq,)]
pub enum GrammarNode<'g,NT,P>
where
    P:Clone, //+core::hash::Hash+PartialEq+Eq,
{
    Many(Rc<GrammarNode<'g,NT,P>>),
    And(Box<[Rc<GrammarNode<'g,NT,P>>]>,usize), //stow_first, error_ind
    Or( Box<[Rc<GrammarNode<'g,NT,P>>]>,), //should store reversed?
    NonTerm(NT),

    Group(Rc<GrammarNode<'g,NT,P>>,&'g str,),
    Expect(Rc<GrammarNode<'g,NT,P>>, &'g str,),
    // NoExpect(Rc<GrammarNode<'g,NT,P>>, ),
    // Stow(Rc<GrammarNode<'g,NT,P>>),

    Was(Rc<GrammarNode<'g,NT,P>>, &'g str),
    // Had(Rc<GrammarNode<'g,NT,P>>, &'g str),
    Had(&'g str),

    // Prev(Rc<GrammarNode<'g,NT,P>>),

    Primitive(P),
    // String,
    // Identifier,
    // Int,
    // Float,
    // Symbol(&'g str),
    // Keyword(&'g str),
    // Eol,

    Always, //always succeeds
    // Error(GrammarWalkError<'g>),
    Error,


    // Stow(Rc<GrammarNode<'g,NT,P>>),

    // Mark(Rc<GrammarNode<'g,NT,P>>),
}

impl<'g,NT,P> Clone for GrammarNode<'g,NT,P>
where
    NT:Clone,
    P:Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Many(arg0) => Self::Many(arg0.clone()),
            Self::And(arg0, arg1) => Self::And(arg0.clone(), arg1.clone()),
            Self::Or(arg0) => Self::Or(arg0.clone()),
            Self::NonTerm(arg0) => Self::NonTerm(arg0.clone()),
            Self::Group(arg0, arg1) => Self::Group(arg0.clone(), arg1.clone()),
            Self::Expect(arg0, arg1) => Self::Expect(arg0.clone(), arg1.clone()),
            Self::Was(arg0, arg1) => Self::Was(arg0.clone(), arg1.clone()),
            Self::Had(arg0) => Self::Had(arg0.clone()),
            Self::Primitive(arg0) => Self::Primitive(arg0.clone()),
            Self::Always => Self::Always,
            Self::Error => Self::Error,
        }
    }
}

impl<'g,NT,P> GrammarNode<'g,NT,P>
where
    NT:Clone,
    P:Clone, //+core::hash::Hash+PartialEq+Eq,
{
    pub fn many0(self) -> GrammarNode<'g,NT,P> {
        Self::Many(self.into())
    }
    pub fn many1(self) -> GrammarNode<'g,NT,P> {
        [self.clone(),self.many0(),].and()
    }
    pub fn opt(self) -> GrammarNode<'g,NT,P> {
        [self.into(),Self::Always].or()
    }
    pub fn group(self,name: &'g str) -> GrammarNode<'g,NT,P> {
        Self::Group(self.into(),name)
    }
    pub fn expect(self,name: &'g str,) -> GrammarNode<'g,NT,P> {
        Self::Expect(self.into(),name)
    }
    pub fn no_expect(self,) -> GrammarNode<'g,NT,P> {
        // Self::NoExpect(self.into(),)
        self.expect("")
    }
    // pub fn expected(self,) -> GrammarNode<'g,NT,P> {
    //     let name = match &self {
    //         GrammarNode::NonTerm(s) => s,
    //         GrammarNode::Had(s) => s,

    //         GrammarNode::Group(_, s) => s,
    //         GrammarNode::Was(_, s) => s,

    //         GrammarNode::String => "string",
    //         GrammarNode::Identifier => "identifier",
    //         GrammarNode::Int => "int",
    //         GrammarNode::Float => "float",
    //         GrammarNode::Symbol(s) => s,
    //         GrammarNode::Keyword(s) => s,
    //         GrammarNode::Eol => "eol",

    //         _ => "",
    //     };

    //     if name.is_empty() {
    //         self
    //     } else {
    //         Self::Expect(self.into(),name)
    //     }
    // }
    // // pub fn stow(self) -> GrammarNode<'g,NT,P> {
    // //     Self::Stow(self.into())
    // // }
    // // pub fn prev(self) -> GrammarNode<'g,NT,P> {
    // //     Self::Prev(self.into())
    // // }
    pub fn was(self,name: &'g str,) -> GrammarNode<'g,NT,P> {
        Self::Was(self.into(),name)
    }
    pub fn had(self,name: &'g str,) -> GrammarNode<'g,NT,P> {
        // Self::Had(self.into(),name)
        [self.into(),Self::Had(name)].and()
    }
    // pub fn stow(self) -> GrammarNode<'g,NT,P> {
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
    // pub fn is_eol(&self) -> bool {
    //     if let GrammarNode::Eol=self {true} else {false}
    // }
    // pub fn get_non_term_name(&self) -> Option<&'g str> {
    //     if let Self::NonTerm(n)=self {
    //         Some(n)
    //     } else {
    //         None
    //     }
    // }
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
        if let Self::Primitive(..)=self {true}else{false}

        // match self {
        //     // GrammarNode::Many(grammar_node) => todo!(),
        //     // GrammarNode::And(grammar_nodes) => todo!(),
        //     // GrammarNode::Or(grammar_nodes) => todo!(),
        //     // GrammarNode::NonTerm(_) => todo!(),
        //     // GrammarNode::Group(grammar_node, _) => todo!(),
        //     // GrammarNode::Expected(grammar_node, _) => todo!(),
        //     // GrammarNode::Prev(grammar_node) => todo!(),
        //     GrammarNode::String => true,
        //     GrammarNode::Identifier => true,
        //     GrammarNode::Int => true,
        //     GrammarNode::Float => true,
        //     GrammarNode::Symbol(_) => true,
        //     GrammarNode::Keyword(_) => true,
        //     GrammarNode::Eol => true,
        //     // GrammarNode::Always => todo!(),
        //     // GrammarNode::Error(grammar_walk_error) => todo!(),
        //     _ => false,
        // }
    }
}

//todo have array stored in rev for or/and
pub trait GrammarArrayTrait<'g,NT,P>

where
    P:Clone, //+core::hash::Hash+PartialEq+Eq,
{
    fn and(self) -> GrammarNode<'g,NT,P>;
    fn and1(self) -> GrammarNode<'g,NT,P>;
    fn or(self) -> GrammarNode<'g,NT,P>;
}

impl<'g,NT,P,const N: usize> GrammarArrayTrait <'g,NT,P> for [GrammarNode<'g,NT,P>; N]
where
    P:Clone, //+core::hash::Hash+PartialEq+Eq,
{
    fn and(self) -> GrammarNode<'g,NT,P> {
        // GrammarNode::And(self.into())
        GrammarNode::And(self.into_iter().map(|x|x.into()).collect(),0)
    }
    fn and1(self) -> GrammarNode<'g,NT,P> {
        // GrammarNode::And(self.into())
        GrammarNode::And(self.into_iter().map(|x|x.into()).collect(),1)
    }
    fn or(self) -> GrammarNode<'g,NT,P> {
        // GrammarNode::Or(self.into())
        GrammarNode::Or(self.into_iter().map(|x|x.into()).collect(),)
    }
}

// pub trait GrammarStrTrait<'g,P>

// where
//     P:Clone+core::hash::Hash+PartialEq+Eq,
// {
//     fn non_term(self) -> GrammarNode<'g,NT,P>;
//     // fn symbol(self) -> GrammarNode<'g,NT,P>;
//     // fn keyword(self) -> GrammarNode<'g,NT,P>;
//     // // fn had(self) -> GrammarNode<'g,NT,P>;
// }

// impl<'g,P> GrammarStrTrait <'g,P> for &'g str

// where
//     P:Clone+core::hash::Hash+PartialEq+Eq,
// {
//     fn non_term(self) -> GrammarNode<'g,NT,P> {
//         GrammarNode::NonTerm(self)
//     }
//     // fn symbol(self) -> GrammarNode<'g,NT,P> {
//     //     GrammarNode::Symbol(self)
//     // }
//     // fn keyword(self) -> GrammarNode<'g,NT,P> {
//     //     GrammarNode::Keyword(self)
//     // }
//     // // fn had(self) -> GrammarNode<'g,NT,P> {
//     // //     GrammarNode::Had(self)
//     // // }
// }

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


impl<'g,NT,P> Debug for GrammarNode<'g,NT,P>

where
    NT:Debug,
    P:Clone+Debug,
{
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

            Self::Primitive(arg0) => f.debug_tuple("Primitive").field(arg0).finish(),
            // Self::String => write!(f, "String"),
            // Self::Identifier => write!(f, "Identifier"),
            // Self::Int => write!(f, "Int"),
            // Self::Float => write!(f, "Float"),
            // Self::Symbol(arg0) => f.debug_tuple("Symbol").field(arg0).finish(),
            // Self::Keyword(arg0) => f.debug_tuple("Keyword").field(arg0).finish(),
            // Self::Eol => write!(f, "Eol"),
            Self::Always => write!(f, "Always"),
            // Self::Error(arg0) => f.debug_tuple("Error").field(arg0).finish(),
            Self::Error => f.debug_tuple("Error").finish(),
        }
    }
}