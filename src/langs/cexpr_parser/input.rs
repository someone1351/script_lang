use std::str::Chars;

use super::super::super::build::*;

#[derive(Debug,Clone)]
pub struct Input<'a> {
    chrs : Chars<'a>,
    // chrs:CharIndices<'a>,
    buf : String,
    loc : Loc,
    // prev_loc : Loc,
}

impl<'a> Input<'a> {
    pub fn new(src:&'a str) -> Input<'a> {
        Self {
            chrs:src.chars(),
            buf : String::new(),
            loc : Loc::one(),
            // prev_loc : Loc::one(),
        }
    }

    fn inner_reserve(&mut self, m : usize) {
        while self.buf.len() < m {
            if let Some(c) = self.chrs.next() {
                self.buf.push(c);
            } else {
                break;
            }
        }
    }

    fn inner_get(&self, i : usize, n : usize) -> Option<&str> {
        let m = i+n;

        if self.buf.len() >= m { //on n is 0, returns Some("") ............why not None? Because consume() needs to?
            //
            let a=self.buf.char_indices().nth(i).map(|x|x.0).unwrap_or_default();
            let b=self.buf.char_indices().nth(m).map(|x|x.0).unwrap_or(self.buf.len());

            Some(&(self.buf[a..b]))
        } else {
            None
        }
    }

    pub fn get(&mut self, i : usize, n : usize) -> Option<&str> {
        self.inner_reserve(i+n);
        self.inner_get(i, n)
    }

    pub fn getc(&mut self, i : usize) -> Option<char> {
        if let Some(s) = self.get(i,1) {
            s.chars().last()
        } else {
            None
        }
    }
    fn calc_loc(&mut self, n : usize) {
        let mut loc = self.loc;

        if let Some(v) = self.get(0,n) {
            for c in v.chars() {
                loc.pos+=1;

                if c=='\n' {
                    loc.row+=1;
                    loc.col=1; //starts at 1, not 0
                } else if c!='\r' {
                    loc.col+=1;
                }
            }

            loc.byte_pos+=v.len();
            // self.prev_loc=self.loc;
            self.loc = loc;
        }
    }

    pub fn next(&mut self, n : usize) -> bool {
        let e=self.buf.len()<n;
        self.calc_loc(n);
        let r=0 .. self.buf.len().min(n);
        self.buf.drain(r.clone());
        e
    }

    // pub fn has_buf_left(&self, n : usize) -> bool {
    //     self.buf.len()>=n
    // }

    pub fn loc(&self) -> Loc {
        self.loc
    }

    // pub fn prev_loc(&self) -> Loc {
    //     self.prev_loc
    // }


    pub fn has<'b,const N:usize>(&mut self, i:usize,xs: [&'b str;N]) -> Option<&'b str> {
        let mut res=None;

        for &x in xs.iter() {
            if Some(x)==self.get(i, x.chars().count()) {
                res=Some(x);
                break;
            }
        }

        res
    }
    pub fn hasnt<const N:usize>(&mut self, i:usize,xs: [&'static str;N]) -> bool {
        self.get(i,1).is_some() && self.has(i, xs).is_none()
    }
    pub fn is_end(&mut self) -> bool {
        self.get(0,1).and_then(|s|s.chars().last()).is_none() //what is the and_then for?
    }
}
