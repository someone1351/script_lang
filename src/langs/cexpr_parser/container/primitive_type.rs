
use super::BlockContainer;



#[derive(Copy,Clone)]
pub enum PrimitiveTypeContainer<'a> {
    Block(BlockContainer<'a>),
    Float(f64,bool),
    Int(i64,bool),
    String(&'a str),
    Symbol(&'a str),
}

impl<'a> std::fmt::Debug for PrimitiveTypeContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // PrimitiveTypeContainer::Block(b) => write!(f, "Block([{:?}])",b.block_ind),
            PrimitiveTypeContainer::Block(b) => write!(f, "Block({b:?})"),
            PrimitiveTypeContainer::Float(x,_) => write!(f, "Float({x})"),
            PrimitiveTypeContainer::Int(x,_) => write!(f, "Int({x})"),
            PrimitiveTypeContainer::String(x) => write!(f, "String({x:?})"),
            PrimitiveTypeContainer::Symbol(x) => write!(f, "Symbol({x})"),
        }
    }
}