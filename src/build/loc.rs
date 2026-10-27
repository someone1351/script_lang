use std::str::Chars;


#[derive(Default,Debug,Copy,Clone, Hash,Eq,Ord)]
pub struct Loc {
    pub pos : usize,
    pub row : usize,
    pub col : usize,
    // pub line_pos : usize,
    pub byte_pos:usize,
}

impl Loc {
    pub fn one() -> Self {
        Loc { pos: 1, row: 1, col: 1, byte_pos:0 }
    }
    pub fn zero() -> Self {
        Loc { pos: 0, row: 0, col: 0, byte_pos:0 }
    }
    pub fn is_zero(&self) -> bool {
        self.pos==0
    }
    pub fn calc(&mut self, s:&str) {
        self.byte_pos+=s.len();

        for c in s.chars() {
            self.pos+=1;

            if c=='\n' {
                self.row+=1;
                self.col=1; //starts at 1, not 0
            } else if c!='\r' {
                self.col+=1;
            }
        }
    }
}

impl PartialEq for Loc {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"line {}, column {}, position {}",self.row,self.col,self.pos)
    }
}

// impl<'a> From<&'a str> for Loc {
//     fn from(value: &'a str) -> Self {
//         todo!()
//     }
// }