
use super::super:: Parsed ;
use super::RecordContainer;

impl<'a> ExactSizeIterator for RecordIter<'a> {
    fn len(&self) -> usize {
        // self.len
        self.end-self.start
    }

    // fn is_empty(&self) -> bool {
    //     self.len == 0
    // }
}
#[derive(Copy,Clone)]
pub struct RecordIter<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub parsed :&'a Parsed,
    // pub len:usize,
}

impl<'a> Iterator for RecordIter<'a> {
    type Item = RecordContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let x=RecordContainer {record_ind: self.start,parsed: self.parsed,};
            self.start+=1;
            Some(x)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for RecordIter<'a> {
    fn next_back(&mut self) -> Option<RecordContainer<'a>> {
        if self.end > self.start {
            self.end-=1;
            Some(RecordContainer {record_ind: self.end,parsed: self.parsed,})
        } else {
            None
        }
    }
}

