use std::ops::{Bound, Range, RangeBounds};

use crate::{cexpr_parser::data::{Parsed, Primitive, PrimitiveType}, Loc};



#[derive(Clone,Copy)]
pub enum PrimitiveTypeContainer<'a> {
    // Root(BlockContainer<'a>),
    CurlyBlock(BlockContainer<'a>),
    SquareBlock(BlockContainer<'a>),
    ParenthesesBlock(BlockContainer<'a>),
    Float(f64),
    Int(i64),
    String(&'a str),
    Symbol(&'a str),
    Identifier(&'a str),
    // End,
    Eol,Eob,
}

#[derive(Clone,Copy)]
pub struct PrimitiveContainer<'a> {
    pub parsed:&'a Parsed,
    pub primitive_ind:usize,
    pub last_loc:Loc,
}


#[derive(Clone,Copy)]
pub struct BlockContainer<'a> {
    parsed:&'a Parsed,
    primitive_ind:usize,
}


impl<'a> PrimitiveContainer<'a> {
    fn primitive(&self) -> &'a Primitive {
        &self.parsed.primitives[self.primitive_ind]
    }

    pub fn start_loc(&self) -> Loc {
        self.primitive().start_loc
    }
    pub fn end_loc(&self) -> Loc {
        self.primitive().end_loc
    }

    pub fn primitive_type(&self) -> PrimitiveTypeContainer<'a> {
        match self.primitive().primitive_type {
            PrimitiveType::Root(_) => panic!("shouldn't be able to get"),
            PrimitiveType::CurlyBlock(_) => PrimitiveTypeContainer::CurlyBlock(self.get_block().unwrap()),
            PrimitiveType::SquareBlock(_) => PrimitiveTypeContainer::SquareBlock(self.get_block().unwrap()),
            PrimitiveType::ParenthesesBlock(_) => PrimitiveTypeContainer::ParenthesesBlock(self.get_block().unwrap()),
            PrimitiveType::Float(x, _) => PrimitiveTypeContainer::Float(x),
            PrimitiveType::Int(x, _) => PrimitiveTypeContainer::Int(x),
            PrimitiveType::String(x) => PrimitiveTypeContainer::String(self.parsed.texts[x].as_str()),
            PrimitiveType::Symbol(x) => PrimitiveTypeContainer::Symbol(self.parsed.texts[x].as_str()),
            PrimitiveType::Identifier(x) => PrimitiveTypeContainer::Symbol(self.parsed.texts[x].as_str()),
            // PrimitiveType::End => PrimitiveTypeContainer::End,
            PrimitiveType::Eol => PrimitiveTypeContainer::Eol,
            PrimitiveType::Eob => PrimitiveTypeContainer::Eob,
        }
    }

    pub fn get_float(&self) -> Option<f64> {
        if let PrimitiveType::Float(x, _)=self.primitive().primitive_type {
            Some(x)
        } else {
            None
        }
    }
    pub fn get_int(&self) -> Option<i64> {
        if let PrimitiveType::Int(x, _)=self.primitive().primitive_type {
            Some(x)
        } else {
            None
        }
    }
    pub fn get_string(&self) -> Option<&'a str> {
        if let PrimitiveType::String(x)=self.primitive().primitive_type {
            Some(self.parsed.texts[x].as_str())
        } else {
            None
        }
    }

    pub fn get_symbol(&self) -> Option<&'a str> {
        if let PrimitiveType::Symbol(x)=self.primitive().primitive_type {
            Some(self.parsed.texts[x].as_str())
        } else {
            None
        }
    }
    pub fn get_identifier(&self) -> Option<&'a str> {
        if let PrimitiveType::Identifier(x)=self.primitive().primitive_type {
            Some(self.parsed.texts[x].as_str())
        } else {
            None
        }
    }
    // pub fn get_end(&self) -> bool {
    //     if let PrimitiveType::End=self.primitive().primitive_type {
    //         true
    //     } else {
    //         false
    //     }
    // }

    pub fn get_block(&self) -> Option<BlockContainer<'a>> {
        match self.primitive().primitive_type {
            PrimitiveType::Root(_)|
            PrimitiveType::CurlyBlock(_)|
            PrimitiveType::SquareBlock(_)|
            PrimitiveType::ParenthesesBlock(_)
            => Some(BlockContainer{ parsed: self.parsed, primitive_ind: self.primitive_ind }),
            _ => None,
        }
    }



    pub fn is_eol(&self) -> bool {
        if let PrimitiveType::Eol=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_eob(&self) -> bool {
        if let PrimitiveType::Eob=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_end(&self) -> bool {
        match self.primitive().primitive_type {
            PrimitiveType::Eob => true,
            PrimitiveType::Eol => true,
            _ => false,

        }
    }

    pub fn is_parentheses(&self) -> bool {
        if let PrimitiveType::ParenthesesBlock(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_square(&self) -> bool {
        if let PrimitiveType::SquareBlock(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }

    pub fn is_curly(&self) -> bool {
        if let PrimitiveType::CurlyBlock(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let PrimitiveType::String(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }

    pub fn is_symbol(&self) -> bool {
        if let PrimitiveType::Symbol(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_float(&self) -> bool {
        if let PrimitiveType::Float(..)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_int(&self) -> bool {
        if let PrimitiveType::Int(..)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
}

impl<'a> BlockContainer<'a> {
    fn primitive(&self) -> &'a Primitive {
        &self.parsed.primitives[self.primitive_ind]
    }

    fn block_ind(&self) -> usize {
        match self.primitive().primitive_type {
            PrimitiveType::Root(x)|
            PrimitiveType::CurlyBlock(x)|
            PrimitiveType::SquareBlock(x)|
            PrimitiveType::ParenthesesBlock(x)
            => x,
            _ => panic!(""),
        }
    }

    fn block_range(&self) -> Range<usize> {
        self.parsed.blocks[self.block_ind()].primitives.clone()
    }

    pub fn is_root(&self) -> bool {
        if let PrimitiveType::Root(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_square(&self) -> bool {
        if let PrimitiveType::SquareBlock(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_curly(&self) -> bool {
        if let PrimitiveType::CurlyBlock(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }

    pub fn is_parentheses(&self) -> bool {
        if let PrimitiveType::ParenthesesBlock(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }

    pub fn size(&self) -> usize {
        self.block_range().len()
    }
    pub fn primitives(&self) -> PrimitiveIterContainer<'a> {
        let r=self.block_range();
        PrimitiveIterContainer { last_loc:self.primitive().end_loc ,start: r.start, end: r.end, parsed: self.parsed }
    }

}


#[derive(Copy,Clone)]
pub struct PrimitiveIterContainer<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub last_loc:Loc,
    pub parsed :&'a Parsed,

}

impl<'a> PrimitiveIterContainer<'a> {
    pub fn pop_front(&mut self) -> Option<PrimitiveContainer<'a>> {
        if self.start < self.end {
            let primitive_ind=self.start;
            self.start+=1;
            Some(PrimitiveContainer { parsed: self.parsed, primitive_ind,last_loc:self.last_loc,})
        } else {
            None
        }
    }
    pub fn pop_back(&mut self) -> Option<PrimitiveContainer<'a>> {
        if self.start < self.end {
            self.end-=1;
            let primitive_ind=self.end;

            let last_loc=if self.start==self.end {
                self.last_loc
            } else {
                self.parsed.primitives[self.end-1].end_loc
            };

            Some(PrimitiveContainer { parsed: self.parsed, primitive_ind,last_loc})
        } else {
            None
        }
    }

    pub fn pop_front_amount(&mut self,amount:usize) -> Option<PrimitiveIterContainer<'a>> {
        if self.start+amount < self.end {
            let start2=self.start;
            self.start+=amount;
            let end2=self.start;

            Some(PrimitiveIterContainer{last_loc:self.last_loc, start: start2, end: end2, parsed: self.parsed })
        } else {
            None
        }
    }
    pub fn pop_back_amount(&mut self,amount:usize) -> Option<PrimitiveIterContainer<'a>> {
        if self.start+amount < self.end {
            let end2=self.end;
            self.end-=amount;
            let start2=self.end;

            let last_loc=if self.start==self.end {
                self.last_loc
            } else {
                self.parsed.primitives[self.end-1].start_loc
            };

            Some(PrimitiveIterContainer{last_loc, start: start2, end: end2, parsed: self.parsed })
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.end-self.start
    }

    pub fn get(&self, ind:usize) -> Option<PrimitiveContainer<'a>> {
        let primitive_ind= self.start+ind;

        if primitive_ind < self.end {
            let last_loc=if ind==0 {
                self.last_loc
            } else {
                self.parsed.primitives[primitive_ind-1].end_loc
            };

            Some(PrimitiveContainer { parsed: self.parsed, primitive_ind,last_loc})
        } else {
            None
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

    pub fn first(&self) -> Option<PrimitiveContainer<'a>> {
        if self.start < self.end {
            Some(PrimitiveContainer { parsed: self.parsed, primitive_ind:self.start,last_loc:self.last_loc})
        } else {
            None
        }
    }
    pub fn last(&self) -> Option<PrimitiveContainer<'a>> {
        if self.start < self.end {
            let primitive_ind=self.end-1;

            let last_loc=if self.len()==1 {
                self.last_loc
            } else {
                self.parsed.primitives[primitive_ind-1].end_loc
            };

            Some(PrimitiveContainer { parsed: self.parsed, primitive_ind,last_loc})
        } else {
            None
        }
    }

}

impl<'a> Iterator for PrimitiveIterContainer<'a> {
    type Item = PrimitiveContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let last_loc2=self.last_loc;
            self.last_loc=self.parsed.primitives[self.start].end_loc;

            let x=PrimitiveContainer {primitive_ind: self.start,parsed: self.parsed,last_loc:last_loc2};
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

            let last_loc=if self.len()==1 {
                self.last_loc
            } else {
                self.parsed.primitives[primitive_ind-1].end_loc
            };
            Some(PrimitiveContainer {primitive_ind,parsed: self.parsed,last_loc})
        } else {
            None
        }
    }
}

