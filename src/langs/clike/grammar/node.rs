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
// where
//     P:Clone, //+core::hash::Hash+PartialEq+Eq,
{
    Many(&'g GrammarNode<'g,NT,P>),
    And(&'g [&'g GrammarNode<'g,NT,P>],), //stow_first, error_ind
    Or( &'g [&'g GrammarNode<'g,NT,P>],), //should store reversed?
    NonTerm(NT),

    Group(&'g GrammarNode<'g,NT,P>,&'g str,),
    Expect(&'g GrammarNode<'g,NT,P>, &'g str,),

    Was(&'g GrammarNode<'g,NT,P>, &'g str),

    Had(&'g str), // Had(&'g GrammarNode<'g,NT,P>, &'g str),
    Primitive(P),

    Always, //always succeeds
    Error,
    // Prev(&'g GrammarNode<'g,NT,P>),

    // NoExpect(&'g GrammarNode<'g,NT,P>, ),
    // Stow(&'g GrammarNode<'g,NT,P>),
    // String,
    // Identifier,
    // Int,
    // Float,
    // Symbol(&'g str),
    // Keyword(&'g str),
    // Eol,

    // Error(GrammarWalkError<'g>),


    // Stow(&'g GrammarNode<'g,NT,P>),

    // Mark(&'g GrammarNode<'g,NT,P>),
}


// impl<'g,NT,P> GrammarNode<'g,NT,P>
// where
//     NT:Clone,
//     P:Clone, //+core::hash::Hash+PartialEq+Eq,
// {
//     pub fn many0(&'g self) -> GrammarNode<'g,NT,P> {
//         Self::Many(self)
//     }
//     pub fn many1(  self,g:&'g GrammarNode<'g,NT,P>) -> GrammarNode<'g,NT,P> {
//         Self::Expect(g, "")
//         // Self::And(
//         //     &[
//         //         // &Self::Always,
//         //         g,
//         //     ], 0)
//         // // [self,& Self::Many(self),].and()
//     }
//     pub fn opt(&'g self) -> GrammarNode<'g,NT,P> {
//         [self,Self::Always].or()
//     }
//     pub fn group(self,name: &'g str) -> GrammarNode<'g,NT,P> {
//         Self::Group(self,name)
//     }
//     pub fn expect(self,name: &'g str,) -> GrammarNode<'g,NT,P> {
//         Self::Expect(self,name)
//     }

//     pub fn was(self,name: &'g str,) -> GrammarNode<'g,NT,P> {
//         Self::Was(self,name)
//     }
//     pub fn had(self,name: &'g str,) -> GrammarNode<'g,NT,P> {
//         // Self::Had(self,name)
//         [self,Self::Had(name)].and()
//     }

// }


// #[macro_export]
// macro_rules! opt {
//     ($v:expr) => {
//         f($v, 3)
//         GrammarNode::Or(&[&$v,GrammarNode::Always])
//     };
// }

// #[macro_export]
// macro_rules! and {
//     ($($x:expr),* $(,)?) => {
//         {
//             GrammarNode::And(&[ $( &$x ),* ],0)
//         }
//     };
// }


// #[macro_export]
// macro_rules! or {
//     ($($x:expr),* $(,)?) => {
//         {
//             GrammarNode::Or(&[ $( &$x ),* ])
//         }
//     };
// }

// // #[macro_export]
// // macro_rules! and {
// //     ( $( $x:expr ),* $(,)? ) => {{
// //         let mut v = Vec::new();
// //         $( v.push($x); )*
// //         GrammarNode::And(v.into())
// //     }};
// // }

// // #[macro_export]
// // macro_rules! or {
// //     ( $( $x:expr ),* $(,)? ) => {{
// //         let mut v = Vec::new();
// //         $( v.push($x); )*
// //         GrammarNode::And(v.into())
// //     }};
// // }


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
            Self::And(arg0,  ) => {

                f.debug_tuple("And").field(arg0).finish()
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