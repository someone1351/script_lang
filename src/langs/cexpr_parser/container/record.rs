

use super::super::super::super::common::Loc;
use super::super::{ Parsed, Record };
use super::{ParamIter,ParamContainer};

#[derive(Copy,Clone)]
pub struct RecordContainer<'a> {
    pub parsed:&'a Parsed,
    pub record_ind:usize,
}

impl<'a> RecordContainer<'a> {
    fn record(&self) -> &Record {
        self.parsed.records.get(self.record_ind).unwrap()
    }
    pub fn params(&self) -> ParamIter<'a> {
        let record=self.record();
        ParamIter {
            start: record.params.start, end: record.params.end, parsed: self.parsed,
            // len:record.params.len()
        }
    }
    pub fn params_num(&self) -> usize {
        let record=self.record();
        record.params.len()
    }
    pub fn param(&self, ind:usize) -> Option<ParamContainer<'a>> {
        let record=self.record();
        let param_ind=record.params.start+ind;

        if param_ind<record.params.end {
            Some(ParamContainer { parsed: self.parsed, param_ind, fieldless:false, })
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
    pub fn start_loc(&self) -> Loc {
        self.first_param().map(|x|x.start_loc()).unwrap_or_else(||self.semi_colon_loc().unwrap()) //if no params, then will have a semi colon
    }

    pub fn end_loc(&self) -> Loc {
        self.last_param().map(|x|x.end_loc()).unwrap_or_else(||self.semi_colon_loc().unwrap()) //if no params, then will have a semi colon
    }

    pub fn semi_colon_loc(&self) -> Option<Loc> {
        let record=self.parsed.records.get(self.record_ind).unwrap();
        record.semi_colon_loc
    }
}

impl<'a> std::fmt::Debug for RecordContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Record(")?;
        for (i,param) in self.params().enumerate() {
            if i>0 {
                write!(f,",")?;
            }

            param.fmt(f)?;
        }
        write!(f,")")?;

        Ok(())
    }
}
