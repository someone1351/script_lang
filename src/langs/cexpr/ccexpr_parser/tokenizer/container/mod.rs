
mod primitive;
mod primitive_type;
mod primitive_iter;
mod value;

pub use primitive::*;
pub use primitive_type::*;
pub use primitive_iter::*;
pub use value::*;

// use std::ops::Range;

// use crate::{cexpr_parser::data::{Parsed, Primitive, PrimitiveType}, Loc};


// #[derive(Clone,Copy)]
// pub struct BlockContainer<'a> {
//     parsed:&'a Parsed,
//     primitive_ind:usize,
// }

// impl<'a> BlockContainer<'a> {
//     fn inner_primitive(&self) -> &'a Primitive {
//         &self.parsed.primitives[self.primitive_ind]
//     }

//     fn block_ind(&self) -> usize {
//         match self.inner_primitive().primitive_type {
//             PrimitiveType::Root(x)|
//             PrimitiveType::CurlyBlock(x)|
//             PrimitiveType::SquareBlock(x)|
//             PrimitiveType::ParenthesesBlock(x)
//             => x,
//             _ => panic!(""),
//         }
//     }

//     fn block_range(&self) -> Range<usize> {
//         self.parsed.blocks[self.block_ind()].primitives.clone()
//     }

//     pub fn is_root(&self) -> bool {
//         if let PrimitiveType::Root(_)=self.inner_primitive().primitive_type {
//             true
//         } else {
//             false
//         }
//     }
//     pub fn is_square(&self) -> bool {
//         if let PrimitiveType::SquareBlock(_)=self.inner_primitive().primitive_type {
//             true
//         } else {
//             false
//         }
//     }
//     pub fn is_curly(&self) -> bool {
//         if let PrimitiveType::CurlyBlock(_)=self.inner_primitive().primitive_type {
//             true
//         } else {
//             false
//         }
//     }

//     pub fn is_parentheses(&self) -> bool {
//         if let PrimitiveType::ParenthesesBlock(_)=self.inner_primitive().primitive_type {
//             true
//         } else {
//             false
//         }
//     }

//     pub fn size(&self) -> usize {
//         self.block_range().len()
//     }
//     pub fn children(&self) -> PrimitiveIterContainer<'a> {
//         let r=self.block_range();
//         PrimitiveIterContainer { last_loc:self.inner_primitive().end_loc ,start: r.start, end: r.end, parsed: self.parsed }
//     }

//     pub fn primitive(&self) -> PrimitiveContainer<'a> {
//         PrimitiveContainer { parsed: self.parsed, primitive_ind: self.primitive_ind,  }
//     }
// }
