



use std::ops::Range;

use crate::cexpr_parser::PrimitiveIterContainer;

use super::super::super::build::Loc;

#[derive(Debug,Clone)]
pub enum PrimitiveType {
    Root(usize), //block_ind
    CurlyBlock(usize), //block_ind
    SquareBlock(usize), //block_ind
    ParenthesesBlock(usize), //block_ind
    // Block(usize), //block_ind
    Float(f64,usize,), //num,text_ind,
    Int(i64,usize,),//num,text_ind,
    String(usize),//text_ind
    Symbol(usize), //text_ind
    Identifier(usize), //text_ind
    // Param(usize),

    // Undefined,
    // Bool(bool),
    // Nil,
    // Void,

    // End(bool), //is_semicolon
    // Eof,
    Eol,
    Eob,
    // End, //eof,eol, eob (end of block)
    // Semicolon,
}

#[derive(Debug,Clone)]
pub struct Primitive {
    pub primitive_type:PrimitiveType,
    pub start_loc : Loc, //for block is first brace
    pub end_loc : Loc, //for block last brace, or if has param/fields then last field
}

// #[derive(Debug,Clone,)]
// pub enum BlockType {
//     Curly,
//     Square,
//     Bracket,
// }

#[derive(Debug,Clone,)]
pub struct Block {
    // pub block_type:BlockType,
    // pub self_primitive:usize, //BlockParent, // enum? Primitive(usize),Field(usize)
    pub primitives : Range<usize>,
    pub inner_start_loc:Loc,
    pub inner_end_loc:Loc,
}

// #[derive (Default)]
pub struct Parsed {
    pub blocks : Vec<Block>,
    pub primitives : Vec<Primitive>,
    pub texts : Vec<String>, //could store start/end locs as well? useful for strings, and symbols
}

impl Parsed {
//     pub fn root_block(&'_ self) -> BlockContainer<'_> {
//         BlockContainer { parsed: self, block_ind: 0, fieldless:false, }
//     }
    // pub fn root_block_primitive(&self) -> PrimitiveContainer<'_> {
    //     PrimitiveContainer { parsed: self, primitive_ind: 0, }
    // }
    pub fn root_primitives(&self) -> PrimitiveIterContainer<'_> {
        let r=&self.blocks[0].primitives;
        PrimitiveIterContainer { last_loc:Loc::one(),start: r.start, end: r.end, parsed: self }
    }
//     // pub fn src(&self)->&'a str {
//     //     self.src
//     // }
//     // pub fn path(&self)->Option<&'a Path> {
//     //     self.path
//     // }

    pub fn print(&self) {
        // let mut work: Vec<(usize, usize)> = self.blocks[0].primitives.clone().map(|i|(i,0)).collect();

        let mut work = vec![(0,0)];

        while let Some((cur,depth))=work.pop() {
            let indent="    ".repeat(depth);
            let primitive=self.primitives.get(cur).unwrap();

            match primitive.primitive_type {
                PrimitiveType::Root(b) => {
                    println!("{indent}{cur} root");
                    for i in self.blocks[b].primitives.clone().rev() { work.push((i,depth+1)); }
                }
                PrimitiveType::CurlyBlock(b) => {
                    println!("{indent}{cur} curly_block");
                    for i in self.blocks[b].primitives.clone().rev() { work.push((i,depth+1)); }
                }
                PrimitiveType::SquareBlock(b) => {
                    println!("{indent}{cur} square_block");
                    for i in self.blocks[b].primitives.clone().rev() { work.push((i,depth+1)); }
                }
                PrimitiveType::ParenthesesBlock(b) => {
                    println!("{indent}{cur} parenth_block");
                    for i in self.blocks[b].primitives.clone().rev() { work.push((i,depth+1)); }
                }
                PrimitiveType::Float(_, s) => {
                    println!("{indent}{cur} float({})",self.texts[s]);
                }
                PrimitiveType::Int(_, s) => {
                    println!("{indent}{cur} int({})",self.texts[s]);
                }
                PrimitiveType::String(s) => {
                    println!("{indent}{cur} string({})",self.texts[s]);
                }
                PrimitiveType::Symbol(s) => {
                    println!("{indent}{cur} symbol({})",self.texts[s]);
                }
                PrimitiveType::Identifier(s) => {
                    println!("{indent}{cur} identifier({})",self.texts[s]);
                }
                // PrimitiveType::End => {
                //     println!("{indent}{cur} end");
                // }
                PrimitiveType::Eol => {
                    println!("{indent}{cur} eol");
                }
                PrimitiveType::Eob => {
                    println!("{indent}{cur} eob");
                }
            }
        }
    }
}