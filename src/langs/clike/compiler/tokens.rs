use std::ops::Range;

use crate::{clike::{compiler::{rules::MyPrimitive, tokeniser2::{MyKeyword, Token, TokenType}}, grammar::{TokensPrimitiveTrait, TokensTrait}}, Loc};

#[derive(Copy,Clone)]
pub struct Tokens<'t> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub last_loc:Loc,
    pub tokens :&'t Vec<Token<'t>>,
    //todo add filtered:bool, // and .filtered() method for filtering out eols during iteration

    // pub prev:Option<usize>,
}

impl<'t> Tokens<'t> {
    pub fn new(tokens:&'t Vec<Token<'t>>) -> Self {
        Self {
            start: 0, end: tokens.len(),
            last_loc: tokens.first().map(|t|t.start_loc).unwrap_or(Loc::one()),
            tokens,
        }
    }
    pub fn first(&self) -> Option<&'t Token<'t>> {
        self.tokens.get(self.start)
    }
    pub fn start_loc(&self) -> Loc {
        self.first().map(|t|t.start_loc).unwrap_or(self.last_loc)
    }
    pub fn identifier(&self) -> Option<&'t str> {
        if let Some(TokenType::Identifier(x))=self.first().map(|t|&t.token_type) {
            Some(*x)
        } else {
            None
        }
    }
    pub fn string(&self) -> Option<&'t str> {
        if let Some(TokenType::String(x,_))=self.first().map(|t|&t.token_type) {
            Some(x.as_str())
        } else {
            None
        }
    }
    pub fn string_raw(&self) -> Option<&'t str> {
        if let Some(TokenType::String(_,x))=self.first().map(|t|&t.token_type) {
            Some(x)
        } else {
            None
        }
    }
    pub fn keyword(&self, k:MyKeyword) -> bool {
        if let Some(TokenType::Keyword(x))=self.first().map(|t|&t.token_type) {
            k.eq(x)
        } else {
            false
        }
    }
    pub fn int(&self) -> Option<i64> {
        if let Some(TokenType::Int(x))=self.first().map(|t|&t.token_type) {
            Some(*x)
        } else {
            None
        }
    }
    pub fn float(&self) -> Option<f64> {
        if let Some(TokenType::Float(x))=self.first().map(|t|&t.token_type) {
            Some(*x)
        } else {
            None
        }
    }
}

impl<'t> Iterator for Tokens<'t> {
    type Item = &'t Token<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            // let last_loc2=self.last_loc;
            let x=&self.tokens[self.start];
            self.last_loc=x.end_loc;

            // let x=Token {
            //     start_loc: todo!(),
            //     end_loc: todo!(),
            //     token_type: todo!(),
            //     // token_ind: self.start,parsed: self.parsed,
            // }; //last_loc:last_loc2
            self.start+=1;

            Some(x)
        } else {
            None
        }
    }
}

impl<'t> DoubleEndedIterator for Tokens<'t> {
    fn next_back(&mut self) -> Option<&'t Token<'t>> {
        if self.end > self.start {
            self.end-=1;
            // let primitive_ind=self.end;

            // let last_loc=if self.len()==1 {
            //     self.last_loc
            // } else {
            //     self.parsed.primitives[primitive_ind-1].end_loc
            // };



            Some(&self.tokens[self.end]) //last_loc
        } else {
            None
        }
    }
}

impl<'t> std::fmt::Debug for Tokens<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v=Vec::new();

        for token_ind in self.start .. self.end {
            let token=&self.tokens[token_ind];
            v.push(format!("{token_ind}:{:?}",&token.token_type));
        }

        f.write_fmt(format_args!("[{}]", v.join(", ")))

    }
}

impl<'t,'g> TokensTrait for Tokens<'t> {
    fn trim2(&mut self) {
        while let Some(TokenType::Eol) = self.first().map(|t|&t.token_type) {
            self.next().unwrap();
        }
    }

    fn take2(&mut self,amount:usize,) -> Option<Tokens<'t>> {
        if self.len2()<amount {
            None
        } else {
            let start=self.start;
            self.start+=amount;
            let end=self.start;

            if amount!=0 {
                self.last_loc=self.tokens[end-1].end_loc;
            }

            Some(Tokens{
                last_loc:self.last_loc,
                start,
                end,
                tokens: self.tokens,
            })

        }
    }


    fn len2(&self) -> usize {
        self.end-self.start
    }

    // fn is_empty2(&self) -> bool {
    //     self.is_empty()
    // }

    fn inds2(&self) -> Range<usize> {
        self.start..self.end
    }





}

impl<'t> TokensPrimitiveTrait<MyPrimitive> for Tokens<'t> {
    fn pop2(&mut self, p:&MyPrimitive) -> bool {
        let mut ts=self.clone();
        if let MyPrimitive::Eol=p {} else {ts.trim2();}
        let Some(first)=ts.next() else {return false;};

        if match p {
            MyPrimitive::String => if let TokenType::String(_,_)= &first.token_type {true}else{false},
            MyPrimitive::Identifier => if let TokenType::Identifier(_)= &first.token_type {true}else{false},
            MyPrimitive::Int => if let TokenType::Int(_)= &first.token_type {true}else{false},
            MyPrimitive::Float => if let TokenType::Float(_)= &first.token_type {true}else{false},
            MyPrimitive::Symbol(s1) => if let TokenType::Symbol(s2)= &first.token_type {s1.eq(s2)}else{false},
            MyPrimitive::Keyword(k1) => if let TokenType::Keyword(k2)= &first.token_type {k1.eq(k2)}else{false},
            MyPrimitive::Eol => if let TokenType::Eol = &first.token_type {true}else{false},
        } {
            *self=ts;
            true
        } else {
            false
        }
    }
}