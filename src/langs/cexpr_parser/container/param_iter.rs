
use super::super:: Parsed ;
use super::ParamContainer;


impl<'a> ExactSizeIterator for ParamIter<'a> {
    fn len(&self) -> usize {
        // self.len
        self.end-self.start
    }
}

#[derive(Copy,Clone)]
pub struct ParamIter<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub parsed :&'a Parsed,
    // pub len:usize,
}

impl<'a> Iterator for ParamIter<'a> {
    type Item = ParamContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let x=ParamContainer {param_ind: self.start,parsed: self.parsed,fieldless:false};
            self.start+=1;
            Some(x)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for ParamIter<'a> {
    fn next_back(&mut self) -> Option<ParamContainer<'a>> {
        if self.end > self.start {
            self.end-=1;
            Some(ParamContainer {param_ind: self.end,parsed: self.parsed,fieldless:false})
        } else {
            None
        }
    }
}
