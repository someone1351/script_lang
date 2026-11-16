use std::ops::{Range, RangeInclusive};

use crate::Loc;


#[derive(Clone,Copy,)]
pub struct Input2<'t> {
    data:&'t str,
    start_pos:usize,
    cur_pos:usize,
    // end_pos_rewind:usize,
    // prev_end_pos:usize,

    start_loc: Loc,
    cur_loc: Loc,

    init_byte_pos: usize,
}
impl<'t> Input2<'t> {
    pub fn new(data:&'t str, init_loc:Loc,) -> Self {
        Self {
            data, start_pos:0, cur_pos:0,
            start_loc:init_loc,
            cur_loc:init_loc,
            init_byte_pos: init_loc.byte_pos,
        }
    }
    pub fn start_loc(&self) -> Loc {
        self.start_loc
    }
    pub fn cur_loc(&mut self) -> Loc {
        let synced_pos = self.cur_loc.byte_pos - self.init_byte_pos;

        if self.cur_pos != synced_pos  //self.cur_loc.byte_pos
        {
            // self.cur_loc.calc(&self.data[self.cur_loc.byte_pos..self.cur_pos]);
            self.cur_loc.calc(&self.data[synced_pos ..self.cur_pos]);
        }

        self.cur_loc
    }
    pub fn slice(&self) -> &'t str {
        &self.data[self.start_pos..self.cur_pos]
    }
    pub fn rest(&self) -> &'t str {
        &self.data[self.cur_pos..]
    }
    pub fn peek(&self,chars_num:usize) -> Option<&'t str> {
        let rest=self.rest();

        let mut cs=rest.chars();

        for _ in 0..chars_num {
            if cs.next().is_none() {
                return None;
            }
        }

        let dif=rest.len()-cs.as_str().len();

        Some(&rest[..dif])
    }
    pub fn peek_char(&self,) -> Option<char> {
        self.rest().chars().next()
    }
    pub fn pos_range(&self) -> Range<usize> {
        self.start_pos..self.cur_pos
    }
    pub fn is_eof(&self) -> bool {
        self.cur_pos >= self.data.len()
    }

    pub fn next(&mut self) {
        let cur_loc = self.cur_loc();
        self.start_pos = self.cur_pos;
        self.start_loc = cur_loc;
    }

    pub fn undo(&mut self) {
        self.cur_pos=self.start_pos;
        self.cur_loc = self.start_loc;
    }


    pub fn take_range(&mut self, r:RangeInclusive<char>) -> Option<char> {
        let Some(x) = self.rest().chars().next() else {return None;};
        if !r.contains(&x) {return None;}
        self.cur_pos+=x.len_utf8();
        Some(x)
    }

    pub fn take_one_of(&mut self, cs:&str) -> Option<char> {
        let Some(x) = self.rest().chars().next() else {return None;};
        if !cs.contains(x) {return None;}
        self.cur_pos+=x.len_utf8();
        Some(x)
    }

    pub fn take_not_of(&mut self, cs:&str) -> Option<char> {
        let Some(x) = self.rest().chars().next() else {return None;};
        if cs.contains(x) {return None;}
        self.cur_pos+=x.len_utf8();
        Some(x)
    }
    pub fn take_char(&mut self, c:char) -> Option<char> {
        if Some(c)!=self.rest().chars().next() {return None;};
        self.cur_pos+=c.len_utf8();
        Some(c)
    }
    pub fn take_any(&mut self,) -> Option<char> {
        let Some(x) = self.rest().chars().next() else {return None;};
        self.cur_pos+=x.len_utf8();
        Some(x)
    }
    pub fn take_str(&mut self, s:&str) -> Option<&'t str> {
        let p=self.cur_pos;

        if !self.rest().starts_with(s) {return None;}
        self.cur_pos+=s.len();

        Some(&self.data[p..self.cur_pos])
    }


    pub fn eat_range(&mut self, r:RangeInclusive<char>) -> bool {
        self.take_range(r).is_some()
    }
    pub fn eat_one_of(&mut self, cs:&str) -> bool {
        self.take_one_of(cs).is_some()
    }
    pub fn eat_not_of(&mut self, cs:&str) -> bool {
        self.take_not_of(cs).is_some()
    }
    pub fn eat_char(&mut self, c:char) -> bool {
        self.take_char(c).is_some()
    }
    pub fn eat_any(&mut self,) -> bool {
        self.take_any().is_some()
    }
    pub fn eat_str(&mut self, s:&str) -> bool {
        self.take_str(s).is_some()
    }

}

impl<'t> std::fmt::Debug for Input2<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self.data)
    }
}
