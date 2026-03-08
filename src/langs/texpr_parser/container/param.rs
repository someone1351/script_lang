
use std::ops::{Bound, RangeBounds};

use super::super::super::super::build::Loc;
use super::super::{ Param, Parsed };
use super::{FieldContainer,PrimitiveContainer,FieldIter};


#[derive(Copy,Clone)]
pub struct ParamContainer<'a> {
    pub parsed:&'a Parsed,
    pub param_ind:usize,
    pub fieldless:bool,
}

impl<'a> std::fmt::Debug for ParamContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Param({:?}",self.as_primitive())?;

        if self.fields_num()!=0 {
            write!(f, ", ")?;
            write!(f, "{}",self.fields().map(|x|format!("{x:?}")).collect::<Vec<_>>().join(", "))?;
        }

        write!(f, ")")?;

        Ok(())
    }
}

impl<'a> ParamContainer<'a> {
    fn param(&self) -> &Param {
      self.parsed.params.get(self.param_ind).unwrap()
    }

    pub fn as_primitive(&self)->PrimitiveContainer<'a> {
        let param=self.param();
        PrimitiveContainer{ parsed: self.parsed, primitive_ind:param.primitive, to_string:false, fieldless:self.fieldless}
    }
    pub fn as_string_primitive(&self)->PrimitiveContainer<'a> {
        let param=self.param();
        PrimitiveContainer{ parsed: self.parsed, primitive_ind:param.primitive, to_string:true, fieldless:self.fieldless}
    }
    pub fn fields_num(&self) -> usize {
        if self.fieldless { return 0;}
        self.param().fields.len()
    }
    pub fn field(&self,ind:usize)->Option<FieldContainer<'a>> {
        if self.fieldless { return None;}

        let param=self.param();
        let field_ind=param.fields.start+ind;
        if field_ind>=param.fields.end {return None;}
        Some(FieldContainer{ parsed: self.parsed, field_ind, })
    }
    pub fn fields(&self) -> FieldIter<'a> {
        if self.fieldless {
            return FieldIter {start:0,end:0,parsed:self.parsed,};
        }

        let param=self.param();

        FieldIter {
            start: param.fields.start, end: param.fields.end, parsed: self.parsed,
            // len:param.fields.len()
        }
    }

    pub fn fields_range<R:RangeBounds<usize>>(&self,r:R) -> FieldIter<'a> {
        if self.fieldless {
            return FieldIter {start:0,end:0,parsed:self.parsed,};
        }

        let param=self.param();

        let range_start=match r.start_bound() {
            Bound::Included(x)=>*x,
            Bound::Excluded(_)=>panic!(""),
            Bound:: Unbounded=>0,
        };

        let range_end=match r.start_bound() {
            Bound::Included(x)=>*x+1,
            Bound::Excluded(x)=>*x,
            Bound:: Unbounded=>param.fields.len(),
        };

        if range_start>range_end {
            return FieldIter {
                start: 0, end: 0, parsed: self.parsed,
                // len:0
            };
        } //if range start==end will return some empty iter

        let x_len=range_end-range_start;

        if x_len>param.fields.len() {
            return FieldIter {
                start: 0, end: 0, parsed: self.parsed,
                // len:0,
            };
        }

        let x_start=param.fields.start+range_start;
        let x_end = x_start+x_len;

        FieldIter {
            start: x_start,
            end: x_end,
            parsed: self.parsed,
            // len:x_len,
        }
    }

    pub fn start_loc(&self) -> Loc {
        self.as_primitive().start_loc()
    }
    pub fn end_loc(&self) -> Loc {
        if self.fields_num()==0 {
            self.as_primitive().end_loc()
        } else {
            self.field(self.fields_num()-1).unwrap().end_loc()
        }
    }

    pub fn to_fieldless(&self) -> Self {
        ParamContainer { fieldless: true, .. self.clone()}
    }
}
