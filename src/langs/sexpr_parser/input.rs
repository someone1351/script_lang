// use super::loc::*;
// use super::Loc;

use super::super::super::build::Loc;

#[derive(Debug,Clone)]
pub struct InputItem<'a> {
    pub chrs : std::str::Chars<'a>,
    pub buf : String,
    pub loc : Loc,
}

#[derive(Debug,Clone)]
pub struct Input<'a> {
    stk : Vec<InputItem<'a>>,
    // chrs : std::str::Chars<'a>,
    // buf : String,
    // loc : Loc,
}

impl<'a> Input<'a> {
    pub fn new(chrs :std::str::Chars<'a>) -> Input<'a> {
        Self {
            stk : vec![InputItem{
                chrs : chrs.clone(),
                buf : String::new(),
                loc : Loc::one(),

            }]
            // chrs : chrs.clone(),
            // buf : String::new(),
            // loc : Loc::default(),
        }
    }

    // pub fn push(&mut self) {
    //     let top = self.stk.last().unwrap().clone();
    //     self.stk.push(top);
    // }

    // pub fn pop_keep(&mut self) {
    //     if self.stk.len()==1 {
    //         panic!("chars input, pop discard on empty stack");
    //     }

    //     self.stk.remove(self.stk.len()-2);
    // }

    // pub fn pop_discard(&mut self) {
    //     if self.stk.len()==1 {
    //         panic!("chars input, pop discard on empty stack");
    //     }

    //     self.stk.pop();
    // }

    pub fn get(&mut self, i : usize, n : usize) -> Option<&str> {
        let cur = self.stk.last_mut().unwrap();
        let mut cur_buf_count=cur.buf.chars().count();
        let m = i+n;

        while cur_buf_count < m {
            if let Some(c) = cur.chrs.next() {
                cur.buf.push(c);
                cur_buf_count+=1;
            } else {
                break;
            }
        }

        if cur_buf_count >= m { //on n is 0, returns Some("") ............why not None? Because consume() needs to?
            Some(&(cur.buf[i..m]))
        } else {
            None
        }
    }
    pub fn getc(&mut self, i : usize) -> Option<char> {
        if let Some(s) = self.get(i,1) {
            s.chars().last()
        } else {
            None
        }
    }
    // pub fn char_range(&mut self, i : usize, a:char,b:char) -> Option<char> {
    //     if let Some(c)=self.getc(i) {
    //         if (a..=b).contains(&c) {
    //             return Some(c);
    //         }
    //     }

    //     None
    // }
    // pub fn char(&mut self, i : usize, a:char) -> Option<char> {
    //     if let Some(c)=self.getc(i) {
    //         if a==c {
    //             return Some(c);
    //         }
    //     }

    //     None
    // }
    pub fn has<const N:usize>(&mut self, i:usize,xs: [&'static str;N]) -> Option<&'static str> {
        for &x in xs.iter() {
            if Some(x)==self.get(i, x.chars().count()) {
                return Some(x);
            }
        }

        None
    }

    pub fn hasnt<const N:usize>(&mut self, i:usize,xs: [&'static str;N]) -> bool {
        // println!("b {} {}",!self.is_end(),self.has(i, xs).is_none());
        !self.get(i,1).is_none() && self.has(i, xs).is_none()
    }

    // pub fn hasc<const N:usize>(&mut self, i:usize,xs:&str,ys: [(char,char);N]) -> Option<char> {
    //     if let Some(a)=self.getc(i) {
    //         if xs.contains(a) {
    //             return Some(a);
    //         }
    //     }

    //     for &y in ys.iter() {
    //         if let Some(a)=self.char_range(i,y.0,y.1) {
    //             return Some(a);
    //         }
    //     }

    //     None
    // }


    // pub fn predicates<const N:usize>(&mut self, i:usize,xs: [fn(char)->bool;N]) -> Option<char> {
    //     if let Some(c)=self.getc(i) {
    //         for &x in xs.iter() {
    //             if x(c) {
    //                 return Some(c);
    //             }
    //         }
    //     }

    //     None
    // }

    // fn calc_loc(&mut self, n : usize) {
    //     let cur = self.stk.last().unwrap();
    //     let mut loc = cur.loc;

    //     if let Some(v) = self.get(0,n) {
    //         for c in v.chars() {
    //             loc.pos+=1;

    //             if c=='\n' {
    //                 loc.row+=1;
    //                 loc.col=0;
    //                 loc.line_start_pos = loc.pos;
    //             } else if c!='\r' {
    //                 loc.col+=1;
    //             }
    //         }

    //         // cur.loc = loc;
    //     }

    //     let cur = self.stk.last_mut().unwrap();
    //     cur.loc = loc;
    // }

    fn calc_loc(&mut self, n : usize) {
        let cur = self.stk.last().unwrap();
        let mut loc = cur.loc;

        // let mut line_inds=Vec::new();

        if let Some(v) = self.get(0,n) {
            for c in v.chars() {
                loc.pos+=1;

                if c=='\n' {
                    loc.row+=1;
                    // loc.col=0;
                    loc.col=1; //starts at 1, not 0
                    // loc.line_pos = loc.pos;
                    // line_inds.push(i);
                } else if c!='\r' {
                    loc.col+=1;
                }
            }

            loc.byte_pos+=v.len();
            // self.loc = loc;

            let cur = self.stk.last_mut().unwrap();
            cur.loc = loc;

        }
    }
    pub fn next(&mut self, n : usize) {
        self.calc_loc(n);
        let cur = self.stk.last_mut().unwrap();
        cur.buf.drain(0 .. cur.buf.chars().count().min(n));
    }

    pub fn is_end(&mut self) -> bool {
        self.get(0,1).is_none()
    }

    pub fn loc(&self) -> Loc {
        let cur = self.stk.last().unwrap();
        cur.loc
    }


}