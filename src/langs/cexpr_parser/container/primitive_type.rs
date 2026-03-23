
use crate::{cexpr_parser::data::{Parsed, Primitive, PrimitiveType}, Loc};

use super::*;

#[derive(Clone,Copy)]
pub enum PrimitiveTypeContainer<'a> {
    // Root(BlockContainer<'a>),
    CurlyBlock(BlockContainer<'a>),
    SquareBlock(BlockContainer<'a>),
    ParenthesesBlock(BlockContainer<'a>),
    Float(f64),
    Int(i64),
    String(&'a str),
    Symbol(&'a str),
    Identifier(&'a str),
    // End,
    Eol,Eob,
}
