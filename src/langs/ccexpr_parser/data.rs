

use crate::ccexpr_parser::PrimitiveIterContainer;

use super::super::super::build::Loc;

#[derive(Debug,Clone,PartialEq)]
pub enum PrimitiveType {
    Float(f64,usize,), //num,text_ind,
    Int(i64,usize,),//num,text_ind,
    String(usize),//text_ind
    Symbol(usize), //text_ind
    Identifier(usize), //text_ind
    Eol,
}

#[derive(Debug,Clone)]
pub struct Primitive {
    pub primitive_type:PrimitiveType,
    pub start_loc : Loc, //for block is first brace
    pub end_loc : Loc, //for block last brace, or if has param/fields then last field
}

pub struct Parsed {
    pub primitives : Vec<Primitive>,
    pub texts : Vec<String>,
}

impl Parsed {
    pub fn primitives(&self) -> PrimitiveIterContainer<'_> {
        PrimitiveIterContainer { last_loc:Loc::one(),start: 0, end: self.primitives.len(), parsed: self }
    }
    pub fn print(&self) {
        // println!("prims {:?}",self.primitives);

        for p in self.primitives() {
            println!("{p:?}");
        }
    }
}