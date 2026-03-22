



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
    // Param(usize),

    // Undefined,
    // Bool(bool),
    // Nil,
    // Void,

    // End(bool), //is_semicolon
    // Eof,
    // Eol,
    End, //eof,eol, eob (end of block)
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
        PrimitiveIterContainer { start: r.start, end: r.end, parsed: self }
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
                    println!("{indent}root");
                    for i in self.blocks[b].primitives.clone().rev() { work.push((i,depth+1)); }
                }
                PrimitiveType::CurlyBlock(b) => {
                    println!("{indent}curly_block");
                    for i in self.blocks[b].primitives.clone().rev() { work.push((i,depth+1)); }
                }
                PrimitiveType::SquareBlock(b) => {
                    println!("{indent}square_block");
                    for i in self.blocks[b].primitives.clone().rev() { work.push((i,depth+1)); }
                }
                PrimitiveType::ParenthesesBlock(b) => {
                    println!("{indent}parenth_block");
                    for i in self.blocks[b].primitives.clone().rev() { work.push((i,depth+1)); }
                }
                PrimitiveType::Float(_, s) => {
                    println!("{indent}float({})",self.texts[s]);
                }
                PrimitiveType::Int(_, s) => {
                    println!("{indent}int({})",self.texts[s]);
                }
                PrimitiveType::String(s) => {
                    println!("{indent}string({})",self.texts[s]);
                }
                PrimitiveType::Symbol(s) => {
                    println!("{indent}symbol({})",self.texts[s]);
                }
                PrimitiveType::End => {
                    println!("{indent}end");
                }
            }
        }
    }
}