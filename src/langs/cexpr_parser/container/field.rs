

use super::super::super::super::build::Loc;
use super::super::{ Field, Parsed };
use super::PrimitiveContainer;


#[derive(Copy,Clone)]
pub struct FieldContainer<'a> {
    pub parsed:&'a Parsed,
    pub field_ind:usize,
    // pub fieldless:bool,
}

impl<'a> std::fmt::Debug for FieldContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // self.primitive().fmt(f)?;

        write!(f,"Field({:?})",self.as_primitive())?;
        Ok(())
    }
}

impl<'a> FieldContainer<'a> {
    fn field(&self) -> &Field {
      self.parsed.fields.get(self.field_ind).unwrap()
    }
    pub fn as_primitive(&self)->PrimitiveContainer<'a> {
        PrimitiveContainer{ parsed: self.parsed, primitive_ind:self.field().primitive, to_string:false, fieldless:false} //a field wouldn't have a field itself
    }
    pub fn as_string_primitive(&self)->PrimitiveContainer<'a> {
        PrimitiveContainer{ parsed: self.parsed, primitive_ind:self.field().primitive, to_string:true, fieldless:false} //a field wouldn't have a field itself
    }
    pub fn start_loc(&self) -> Loc {
        self.field().start_loc
    }
    pub fn end_loc(&self) -> Loc {
        self.as_primitive().end_loc()
    }
}
