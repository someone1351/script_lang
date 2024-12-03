use std::ops::Range;


use super::container::BlockContainer;

use super::super::super::common::Loc;
use super::PrimitiveContainer;

#[derive(Debug,Clone)]
pub enum PrimitiveType {
    Block(usize),
    Float(f64,usize,bool), //num,text_ind,has_prefix
    Int(i64,usize,bool),//num,text_ind,has_prefix
    // Bool(bool),
    // Nil,
    // Void,
    String(usize),//text_ind
    Symbol(usize), //text_ind
    // Param(usize),
}

#[derive(Debug,Clone)]
pub struct Primitive {
    pub primitive_type:PrimitiveType,
    pub start_loc : Loc, //for block is first brace
    pub end_loc : Loc, //for block last brace, or if has param/fields then last field
    pub param:Option<usize>,
    pub field:Option<usize>,
    // pub record_parent:usize,
}

#[derive(Debug,Clone)]
pub struct Field {
    pub primitive : usize,
    pub start_loc : Loc, //from dot
    // pub end_loc : Loc, //from primitive
}

#[derive(Debug,Clone,Default)]
pub struct Param {
    // pub owner : usize,
    pub primitive : usize,
    pub fields : Range<usize>, //field
    // pub start_loc : Loc, //from primitive
    // pub end_loc : Loc, //from primitive or last field
}

#[derive(Debug,Clone,Default)]
pub struct Record {
    // pub primitives : Range<usize>,
    pub params : Range<usize>,
    pub semi_colon_loc:Option<Loc>,
    // pub block_parent:usize,
    // pub start_loc : Loc, //from first param or semi_colon_loc
    // pub end_loc : Loc, //from last param or semi_colon_loc
}

#[derive(Debug,Clone,)]
pub struct Block {
    // pub seps_num:usize,
    // pub primitive:Option<usize>,
    pub primitive:usize, //BlockParent, // enum? Primitive(usize),Field(usize)
    pub records : Range<usize>,
    pub params : Range<usize>,
    // pub start_loc : Loc, //opening from primitive
    // pub end_loc : Loc, // from closing brace
}

pub struct Parsed {
    pub blocks : Vec<Block>, //of records
    pub records : Vec<Record>, //of primitives
    pub params : Vec<Param>,
    pub fields : Vec<Field>,
    pub primitives : Vec<Primitive>,
    pub texts : Vec<String>, //could store start/end locs as well? useful for strings, and symbols

    // pub symbols : Vec<Symbol>,
    // pub fields : Vec<Field>,
    // pub src:&'a str,
    // pub path:Option<&'a Path>,
}

impl Parsed {
    pub fn root_block(&self) -> BlockContainer {
        BlockContainer { parsed: self, block_ind: 0 }
    }
    pub fn root_block_primitive(&self) -> PrimitiveContainer {
        PrimitiveContainer { parsed: self, primitive_ind: 0, to_string:false, }
    }
    // pub fn src(&self)->&'a str {
    //     self.src
    // }
    // pub fn path(&self)->Option<&'a Path> {
    //     self.path
    // }
}