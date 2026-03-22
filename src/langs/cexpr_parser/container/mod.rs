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
    // End,
    Eol,Eob,
}

#[derive(Clone,Copy)]
pub struct PrimitiveContainer<'a> {
    pub parsed:&'a Parsed,
    pub primitive_ind:usize,
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
            PrimitiveType::String(_) => PrimitiveTypeContainer::String(self.get_string().unwrap()),
            PrimitiveType::Symbol(_) => PrimitiveTypeContainer::Symbol(self.get_symbol().unwrap()),
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
        PrimitiveIterContainer { start: r.start, end: r.end, parsed: self.parsed }
    }

}


#[derive(Copy,Clone)]
pub struct PrimitiveIterContainer<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub parsed :&'a Parsed,
}

impl<'a> PrimitiveIterContainer<'a> {
    pub fn pop_front(&mut self) -> Option<PrimitiveContainer<'a>> {
        if self.start < self.end {
            let primitive_ind=self.start;
            self.start+=1;
            Some(PrimitiveContainer { parsed: self.parsed, primitive_ind})
        } else {
            None
        }
    }
    pub fn pop_back(&mut self) -> Option<PrimitiveContainer<'a>> {
        if self.start < self.end {
            self.end-=1;
            let primitive_ind=self.end;
            Some(PrimitiveContainer { parsed: self.parsed, primitive_ind})
        } else {
            None
        }
    }

    pub fn pop_front_amount(&mut self,amount:usize) -> Option<PrimitiveIterContainer<'a>> {
        if self.start+amount < self.end {
            let x=self.start;
            self.start+=amount;
            Some(PrimitiveIterContainer{ start: x, end: self.start, parsed: self.parsed })
        } else {
            None
        }
    }
    pub fn pop_back_amount(&mut self,amount:usize) -> Option<PrimitiveIterContainer<'a>> {
        if self.start+amount < self.end {
            self.end+=amount;
            Some(PrimitiveIterContainer{ start: self.end-amount, end: self.end, parsed: self.parsed })
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
            Some(PrimitiveContainer { parsed: self.parsed, primitive_ind})
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
            return PrimitiveIterContainer {start: 0, end: 0, parsed: self.parsed};
        }

        let x_len=range_end-range_start;

        if x_len>self.len() {
            return PrimitiveIterContainer {start: 0, end: 0, parsed: self.parsed};
        }

        let x_start=self.start+range_start;
        let x_end = x_start+x_len;

        PrimitiveIterContainer {start: x_start, end: x_end, parsed: self.parsed}
    }

    pub fn first(&self) -> Option<PrimitiveContainer<'a>> {
        if self.start < self.end {
            Some(PrimitiveContainer { parsed: self.parsed, primitive_ind:self.start})
        } else {
            None
        }
    }
    pub fn last(&self) -> Option<PrimitiveContainer<'a>> {
        if self.start < self.end {
            Some(PrimitiveContainer { parsed: self.parsed, primitive_ind:self.end-1})
        } else {
            None
        }
    }

}

impl<'a> Iterator for PrimitiveIterContainer<'a> {
    type Item = PrimitiveContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let x=PrimitiveContainer {primitive_ind: self.start,parsed: self.parsed,};
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
            Some(PrimitiveContainer {primitive_ind: self.end,parsed: self.parsed,})
        } else {
            None
        }
    }
}

