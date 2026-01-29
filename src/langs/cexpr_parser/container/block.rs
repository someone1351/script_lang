
use super::super::super::super::common::Loc;
use super::super::{ Block, Parsed };
use super::{RecordIter,ParamIter,ParamContainer,RecordContainer,PrimitiveContainer};


#[derive(Copy,Clone)]
pub struct BlockContainer<'a> {
    pub parsed:&'a Parsed,
    pub block_ind:usize,
    pub fieldless:bool,
}

impl<'a> BlockContainer<'a> {
    fn block(&self) -> &Block {
        self.parsed.blocks.get(self.block_ind).unwrap()
    }
    pub fn records(&self) -> RecordIter<'a> {
        let block=self.block();
        RecordIter {
            start: block.records.start, end: block.records.end, parsed: self.parsed,
            // len:block.records.len()
        }
    }
    pub fn records_num(&self) -> usize {
        let block=self.block();
        block.records.len()
    }
    pub fn record(&self, ind:usize) -> Option<RecordContainer<'a>>{
        let block=self.block();
        let record_ind=block.records.start+ind;

        if record_ind<block.records.end {
            Some(RecordContainer {parsed:self.parsed,record_ind,})
        } else {
            None
        }
    }
    pub fn primitive(&self) -> PrimitiveContainer<'a> {
        PrimitiveContainer { parsed: self.parsed, primitive_ind:self.block().primitive, to_string:false, fieldless:self.fieldless }
    }

    pub fn start_loc(&self) -> Loc {
        self.primitive().start_loc()
    }

    pub fn end_loc(&self) -> Loc {
        self.primitive().end_loc()
    }
    pub fn params(&self) -> ParamIter<'a> {
        let params=self.block().params.clone();
        ParamIter {
            start: params.start, end: params.end, parsed: self.parsed,
            // len:params.len()
        }
    }
    pub fn params_num(&self) -> usize {
        self.block().params.len()
    }
    pub fn param(&self, ind:usize) -> Option<ParamContainer<'a>> {
        let params=self.block().params.clone();
        let param_ind=params.start+ind;

        if param_ind<params.end {
            Some(ParamContainer { parsed: self.parsed, param_ind, fieldless:false,})
        } else {
            None
        }
    }
    pub fn first_param(&self) -> Option<ParamContainer<'a>> {
        self.param(0)
    }
    pub fn last_param(&self) -> Option<ParamContainer<'a>> {
        if self.params_num()==0 {
            None
        } else {
            self.param(self.params_num()-1)
        }
    }
}

impl<'a> std::fmt::Debug for BlockContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //

        for (i,record) in self.records().enumerate()
        {
            if i>0 {
                write!(f,", ")?;
            }

            record.fmt(f)?;
        }
        // write!(f, "BlockContainer({})", self.block_ind)?;

        Ok(())
    }
}
