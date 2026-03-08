
use super::super:: Parsed ;
use super::FieldContainer;


impl<'a> ExactSizeIterator for FieldIter<'a> {
    fn len(&self) -> usize {
        // self.len
        self.end-self.start
    }
}

#[derive(Copy,Clone)]
pub struct FieldIter<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub parsed :&'a Parsed,
    // pub len:usize,
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = FieldContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let x=FieldContainer {field_ind: self.start,parsed: self.parsed,};
            self.start+=1;
            Some(x)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for FieldIter<'a> {
    fn next_back(&mut self) -> Option<FieldContainer<'a>> {
        if self.end > self.start {
            self.end-=1;
            Some(FieldContainer {field_ind: self.end,parsed: self.parsed,})
        } else {
            None
        }
    }
}
