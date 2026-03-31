
// use crate::{cexpr_parser::data::{Parsed, Primitive, PrimitiveType}, Loc};

use super::*;

#[derive(Clone,Copy)]
pub enum PrimitiveTypeContainer<'a> {
    // Root(BlockContainer<'a>),
    CurlyBlock(PrimitiveIterContainer<'a>),
    SquareBlock(PrimitiveIterContainer<'a>),
    ParenthesesBlock(PrimitiveIterContainer<'a>),
    Float(f64),
    Int(i64),
    String(&'a str),
    Symbol(&'a str),
    Identifier(&'a str),
    End,
}

impl<'a> std::fmt::Debug for PrimitiveTypeContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurlyBlock(_) => f.debug_tuple("CurlyBlock").finish(),
            Self::SquareBlock(_) => f.debug_tuple("SquareBlock").finish(),
            Self::ParenthesesBlock(_) => f.debug_tuple("ParenthesesBlock").finish(),
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
            Self::Int(arg0) => f.debug_tuple("Int").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Symbol(arg0) => f.debug_tuple("Symbol").field(arg0).finish(),
            Self::Identifier(arg0) => f.debug_tuple("Identifier").field(arg0).finish(),
            Self::End => write!(f, "End"),
        }
    }
}