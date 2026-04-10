use std::ops::{Bound, Range, RangeBounds};

use crate::{ccexpr_parser::data::Parsed, Loc};

use super::*;

#[derive(Copy,Clone)]
pub struct PrimitiveIterContainer<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub last_loc:Loc,
    pub parsed :&'a Parsed,

}

impl<'a> PrimitiveIterContainer<'a> {
    pub fn inds(&self) -> Range<usize> {
        self.start..self.end
    }
    pub fn last_loc(&self) -> Loc {
        self.last_loc
    }
    pub fn loc(&self) -> Loc {
        self.first().map(|p|p.start_loc()).unwrap_or(self.last_loc)
    }

    pub fn pop_front(&mut self) -> Result<PrimitiveContainer<'a>,Loc> {
        if self.start < self.end {
            let primitive_ind=self.start;
            self.start+=1;
            Ok(PrimitiveContainer { parsed: self.parsed, primitive_ind,}) //last_loc:self.last_loc,
        } else {
            Err(self.last_loc)
        }
    }
    pub fn pop_back(&mut self) -> Result<PrimitiveContainer<'a>,Loc> {
        if self.start < self.end {
            self.end-=1;
            let primitive_ind=self.end;

            // let last_loc=if self.start==self.end {
            //     self.last_loc
            // } else {
            //     self.parsed.primitives[self.end-1].end_loc
            // };

            Ok(PrimitiveContainer { parsed: self.parsed, primitive_ind,}) //last_loc
        } else {
            Err(self.last_loc)
        }
    }

    pub fn pop_front_amount(&mut self,amount:usize) -> Option<PrimitiveIterContainer<'a>> {
        if self.start+amount > self.end || amount==0 {
            None
        } else {
            let start2=self.start;
            self.start+=amount;
            let end2=self.start;

            Some(PrimitiveIterContainer{last_loc:self.last_loc, start: start2, end: end2, parsed: self.parsed })
        }
    }
    pub fn pop_back_amount(&mut self,amount:usize) -> Option<PrimitiveIterContainer<'a>> {
        if self.start+amount > self.end || amount==0 {
            None
        } else {
            let end2=self.end;
            self.end-=amount;
            let start2=self.end;

            let last_loc=if self.start==self.end {
                self.last_loc
            } else {
                self.parsed.primitives[self.end-1].start_loc
            };

            Some(PrimitiveIterContainer{last_loc, start: start2, end: end2, parsed: self.parsed })
        }
    }

    pub fn len(&self) -> usize {
        self.end-self.start
    }

    pub fn get(&self, ind:usize) -> Result<PrimitiveContainer<'a>,Loc> {
        let primitive_ind= self.start+ind;

        if primitive_ind < self.end {
            // let last_loc=if ind==0 {
            //     self.last_loc
            // } else {
            //     self.parsed.primitives[primitive_ind-1].end_loc
            // };

            Ok(PrimitiveContainer { parsed: self.parsed, primitive_ind,}) //last_loc
        } else {
            let last_loc=if self.len()==0 {
                self.last_loc
            } else {
                self.get(self.len()-1).unwrap().end_loc()
            };

            Err(last_loc)
        }
    }

    pub fn get_range<R:RangeBounds<usize>>(&self,r:R) -> PrimitiveIterContainer<'a> {

        let range_start=match r.start_bound() {
            Bound::Included(x)=>*x,
            Bound::Excluded(_)=>panic!(""),
            Bound:: Unbounded=>0,
        };

        let range_end=match r.start_bound() {
            Bound::Included(x)=>*x+1,
            Bound::Excluded(x)=>*x,
            Bound::Unbounded=>self.len(),
        };

        if range_start>range_end { //if range start==end is same as empty iter
            return PrimitiveIterContainer {last_loc:Loc::zero(),start: 0, end: 0, parsed: self.parsed};
        }

        let x_len=range_end-range_start;

        if x_len>self.len() {
            return PrimitiveIterContainer {last_loc:Loc::zero(),start: 0, end: 0, parsed: self.parsed};
        }

        let x_start=self.start+range_start;
        let x_end = x_start+x_len;

        let last_loc=if range_start==0 {
            self.last_loc
        } else {
            self.parsed.primitives[range_start].start_loc
        };

        PrimitiveIterContainer {last_loc,start: x_start, end: x_end, parsed: self.parsed}
    }

    pub fn is_empty(&self) -> bool {
        self.start!=self.end
    }

    pub fn first(&self) -> Result<PrimitiveContainer<'a>,Loc> {
        self.get(0)
    }
    pub fn last(&self) -> Result<PrimitiveContainer<'a>,Loc> {
        self.get(if self.is_empty() {0} else{self.len()-1})

    }

    fn pop_get<T,F>(&mut self,skip_eols:bool,func:F) -> Result<ValueContainer<'a,T>,Loc>
    where
        F:FnOnce(PrimitiveContainer<'a>)->Result<ValueContainer<'a,T>,Loc>,
    {
        if skip_eols {
            while let Ok(x)=self.first() {
                if !x.is_eol() {
                    break;
                }

                self.pop_front().unwrap();
            }
        }


        let v=self.first().and_then(func);

        if v.is_ok() {
            self.pop_front().unwrap();
        }

        v
    }

    pub fn pop_eol(&mut self) -> Result<ValueContainer<'a,()>,Loc> {
         self.pop_get(false,|p|p.get_eol())
    }
    pub fn pop_float(&mut self) -> Result<ValueContainer<'a,f64>,Loc> {
        self.pop_get(true,|p|p.get_float())
    }

    pub fn pop_int(&mut self) -> Result<ValueContainer<'a,i64>,Loc> {
        self.pop_get(true,|p|p.get_int())
    }

    pub fn pop_string(&mut self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        self.pop_get(true,|p|p.get_string())
    }

    pub fn pop_symbol(&mut self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        self.pop_get(true,|p|p.get_symbol())
    }

    pub fn pop_identifier(&mut self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        self.pop_get(true,|p|p.get_identifier())
    }

    pub fn pop_with_identifiers<'b,I>(&mut self,idns:I) -> Result<ValueContainer<'a,&'a str>,Loc>
    where
        I:IntoIterator<Item = &'b str>,
    {
        self.pop_get(true,move|p|p.has_identifiers(idns))
    }
    pub fn pop_with_symbols<'b,I>(&mut self,symbols:I) -> Result<ValueContainer<'a,&'a str>,Loc>
    where
        I:IntoIterator<Item = &'b str>,
    {
        self.pop_get(true,move|p|p.has_symbols(symbols))
    }
}

impl<'a> Iterator for PrimitiveIterContainer<'a> {
    type Item = PrimitiveContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            // let last_loc2=self.last_loc;
            self.last_loc=self.parsed.primitives[self.start].end_loc;

            let x=PrimitiveContainer {primitive_ind: self.start,parsed: self.parsed,}; //last_loc:last_loc2
            self.start+=1;

            Some(x)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for PrimitiveIterContainer<'a> {
    fn next_back(&mut self) -> Option<PrimitiveContainer<'a>> {
        if self.end > self.start {
            self.end-=1;
            let primitive_ind=self.end;

            // let last_loc=if self.len()==1 {
            //     self.last_loc
            // } else {
            //     self.parsed.primitives[primitive_ind-1].end_loc
            // };

            Some(PrimitiveContainer {primitive_ind,parsed: self.parsed,}) //last_loc
        } else {
            None
        }
    }
}

impl<'a> std::fmt::Debug for PrimitiveIterContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        f.write_fmt(format_args!("[{}]", self.clone().map(|p|format!("{p:?}")).collect::<Vec<String>>().join(", ")))

    }
}