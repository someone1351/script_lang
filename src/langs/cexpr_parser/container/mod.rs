use std::ops::Range;

use crate::{cexpr_parser::data::{Parsed, Primitive, PrimitiveType}, Loc};



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
    pub fn get_end(&self) -> bool {
        if let PrimitiveType::End=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
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
    pub fn primitives(&self) -> PrimitiveIter<'a> {
        let r=self.block_range();
        PrimitiveIter { start: r.start, end: r.end, parsed: self.parsed }
    }

}


#[derive(Copy,Clone)]
pub struct PrimitiveIter<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub parsed :&'a Parsed,
}

impl<'a> Iterator for PrimitiveIter<'a> {
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

impl<'a> DoubleEndedIterator for PrimitiveIter<'a> {
    fn next_back(&mut self) -> Option<PrimitiveContainer<'a>> {
        if self.end > self.start {
            self.end-=1;
            Some(PrimitiveContainer {primitive_ind: self.end,parsed: self.parsed,})
        } else {
            None
        }
    }
}

