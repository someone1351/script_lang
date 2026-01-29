
use super::super::super::super::common::Loc;
use super::super::{ Parsed, Primitive, PrimitiveType };
use super::{ParamContainer,FieldContainer,PrimitiveTypeContainer,BlockContainer};

#[derive(Copy,Clone)]
pub struct PrimitiveContainer<'a> {
    pub parsed:&'a Parsed,
    pub primitive_ind:usize,
    pub to_string : bool,
    pub fieldless:bool,
}

impl<'a> PrimitiveContainer<'a> {
    fn primitive(&self) -> &Primitive {
        self.parsed.primitives.get(self.primitive_ind).unwrap()
    }
    pub fn primitive_type(&self) -> PrimitiveTypeContainer<'a> {
        let primitive=self.primitive();
        if self.to_string {
            let s=match primitive.primitive_type.clone() {
                PrimitiveType::Block(_) => 0, //0 is an empty string
                PrimitiveType::Float(_,s,_)|PrimitiveType::Int(_,s,_)|PrimitiveType::String(s)|PrimitiveType::Symbol(s) => s,
            };

            PrimitiveTypeContainer::String(self.parsed.texts.get(s).unwrap().as_str())
        } else {

            match primitive.primitive_type.clone() {
                // x if self.to_string =>
                PrimitiveType::Block(block_ind) => PrimitiveTypeContainer::Block(BlockContainer {parsed:self.parsed,block_ind,fieldless:self.fieldless}),
                PrimitiveType::Float(f,_s,p) => PrimitiveTypeContainer::Float(f,p),
                PrimitiveType::Int(i,_s,p) => PrimitiveTypeContainer::Int(i,p),
                PrimitiveType::String(s) => PrimitiveTypeContainer::String(self.parsed.texts.get(s).unwrap().as_str()),
                PrimitiveType::Symbol(s) =>PrimitiveTypeContainer::Symbol(self.parsed.texts.get(s).unwrap().as_str()),
            }
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        if let PrimitiveTypeContainer::Float(x,_)=self.primitive_type() {
            Some(x)
        } else {
            None
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        if let PrimitiveTypeContainer::Int(x,_)=self.primitive_type() {
            Some(x)
        } else {
            None
        }
    }
    pub fn as_string(&self) -> Option<&'a str> {
        if let PrimitiveTypeContainer::String(s)=self.primitive_type() {
            Some(s)
        // } else if self.to_string {
        //     if let PrimitiveTypeContainer::Symbol(s)=self.primitive_type() {
        //         Some(s)
        //     } else {
        //         None
        //     }
        } else {
            None
        }
    }
    // pub fn make_string(&self) -> Option<Self> {
    //     if let PrimitiveTypeContainer::Symbol(_)=self.primitive_type() {
    //         let mut x = self.clone();
    //         x.to_string=true;
    //         Some(x)
    //     } else {
    //         None
    //     }
    // }
    // pub fn make_string(&self) -> Self {
    //     let mut x = self.clone();
    //     x.to_string=true;
    //     x
    // }
    pub fn as_symbol(&self) -> Option<&'a str> {
        if let PrimitiveTypeContainer::Symbol(s)=self.primitive_type() {
            Some(s)
        } else {
            None
        }
    }
    pub fn as_block(&self) -> Option<BlockContainer<'a>> {
        if let PrimitiveTypeContainer::Block(x)=self.primitive_type() {
            Some(x)
        } else {
            None
        }
    }
    pub fn as_param(&self) -> Option<ParamContainer<'a>> {
        self.primitive().param.map(|param_ind|ParamContainer{ parsed: self.parsed, param_ind,fieldless:self.fieldless })
    }
    pub fn as_field(&self) -> Option<FieldContainer<'a>> {
        self.primitive().field.map(|field_ind|FieldContainer{ parsed: self.parsed, field_ind, })
    }
    pub fn start_loc(&self) -> Loc {
        self.primitive().start_loc
    }
    pub fn end_loc(&self) -> Loc {
        self.primitive().end_loc
    }
}

impl<'a> std::fmt::Debug for PrimitiveContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.primitive_type().fmt(f)
    }
}