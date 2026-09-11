// use std::ops::{Bound, Range, RangeBounds};
use std::ops::Range;
use crate::build::Loc;
use crate::clike::compiler::rules::GrammarPrimitive;
use crate::clike::grammar::{GrammarPrimitiveTrait, TokenIterGetTrait, TokenIterTrait};
use super::super::super::tokenizer::data::Tokenized;

use super::*;

#[derive(Copy,Clone)]
pub struct TokenIterContainer<'t> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub last_loc:Loc,
    pub parsed :&'t Tokenized,
    //todo add filtered:bool, // and .filtered() method for filtering out eols during iteration

    // pub prev:Option<usize>,
}

impl<'t> TokenIterContainer<'t> {
    pub fn inds(&self) -> Range<usize> {
        self.start..self.end
    }
    pub fn last_loc(&self) -> Loc {
        self.last_loc
    }
    pub fn start_loc(&self) -> Loc {
        // self.first().map(|p|p.end_loc()).unwrap_or(self.last_loc) //why was it last_loc?

        if self.is_empty() {
            if self.start==self.parsed.primitives.len() {
                self.parsed.primitives.last().map(|x|x.end_loc).unwrap_or(Loc::zero())
            } else {
                self.parsed.primitives[self.start].start_loc
            }
        } else {
            self.first().unwrap().start_loc()
        }


        // self.first().map(|p|p.start_loc())
        //     .unwrap_or(self.last_loc)
    }

    pub fn pop_front(&mut self) -> Result<TokenContainer<'t>,Loc> {
        if self.start < self.end {
            let primitive_ind=self.start;
            self.last_loc=self.parsed.primitives[self.start].end_loc;
            // self.last_loc=Loc::zero();
            self.start+=1;
            // self.last_loc=self.loc();
            // self.last_loc=self.first().map(|p|p.end_loc()).unwrap_or(self.last_loc);
            Ok(TokenContainer { parsed: self.parsed, token_ind: primitive_ind,}) //last_loc:self.last_loc,
        } else {
            Err(self.last_loc)
        }
    }
    pub fn pop_back(&mut self) -> Result<TokenContainer<'t>,Loc> {
        if self.start < self.end {
            self.end-=1;
            let primitive_ind=self.end;

            // let last_loc=if self.start==self.end {
            //     self.last_loc
            // } else {
            //     self.parsed.primitives[self.end-1].end_loc
            // };

            Ok(TokenContainer { parsed: self.parsed, token_ind: primitive_ind,}) //last_loc
        } else {
            Err(self.last_loc)
        }
    }

    pub fn pop_front_amount(&mut self,amount:usize) -> Option<TokenIterContainer<'t>> {
        if self.start+amount > self.end { //|| amount==0
            // println!("{}+{} > {}", self.start,amount,self.end);
            None
        } else {
            let start2=self.start;
            self.start+=amount;
            let end2=self.start;

            Some(TokenIterContainer{last_loc:self.last_loc, start: start2, end: end2, parsed: self.parsed })
        }
    }
    pub fn pop_back_amount(&mut self,amount:usize) -> Option<TokenIterContainer<'t>> {
        if self.start+amount > self.end
        // || amount==0
        {
            println!("{}+{} > {}", self.start,amount,self.end);

            None
        } else {
            let end2=self.end;
            self.end-=amount;
            let start2=self.end;

            // let last_loc=if self.start==self.end {
            //     self.last_loc
            // } else {
            //     self.parsed.primitives[self.end-1].start_loc
            // };

            let last_loc=self.last_loc;

            Some(TokenIterContainer{last_loc, start: start2, end: end2, parsed: self.parsed })
        }
    }

    pub fn len(&self) -> usize {
        self.end-self.start
    }

    // pub fn get(&self, ind:usize) -> Result<TokenContainer<'a>,Loc> {
    //     let primitive_ind= self.start+ind;

    //     if primitive_ind < self.end {
    //         // let last_loc=if ind==0 {
    //         //     self.last_loc
    //         // } else {
    //         //     self.parsed.primitives[primitive_ind-1].end_loc
    //         // };

    //         Ok(TokenContainer { parsed: self.parsed, primitive_ind,}) //last_loc
    //     } else {
    //         // let last_loc=if self.len()==0 {
    //         //     self.last_loc
    //         // } else {
    //         //     self.get(self.len()-1).unwrap().end_loc()
    //         // };

    //         let last_loc=self.last().map(|x|x.end_loc()).unwrap_or(self.last_loc);
    //         Err(last_loc)
    //     }
    // }

    // pub fn get_range<R:RangeBounds<usize>>(&self,r:R) ->
    // // Result<TokenIterContainer<'a>,Loc>
    // Option<TokenIterContainer<'a>>
    // {

    //     let range_start=match r.start_bound().cloned() {
    //         Bound::Included(x)=>x,
    //         Bound::Excluded(_)=>panic!(""),
    //         Bound:: Unbounded=>0,
    //     };

    //     let range_end=match r.end_bound().cloned() {
    //         Bound::Included(x)=>x+1,
    //         Bound::Excluded(x)=>x,
    //         Bound::Unbounded=>self.len(),
    //     };

    //     let last_loc=self.last().map(|x|x.end_loc()).unwrap_or(self.last_loc);

    //     if range_start>range_end { //if range start==end is same as empty iter
    //         // return TokenIterContainer {last_loc:Loc::zero(),start: 0, end: 0, parsed: self.parsed};
    //         // return Err(last_loc);
    //         println!("rs>re {:?} {:?}",r.start_bound(),r.end_bound());
    //         return None;
    //     }

    //     let x_len=range_end-range_start;

    //     if x_len>self.len() {
    //         // return TokenIterContainer {last_loc:Loc::zero(),start: 0, end: 0, parsed: self.parsed};
    //         // return Err(last_loc);
    //         println!("xl>l {:?} {:?}",r.start_bound(),r.end_bound());
    //         return  None;
    //     }

    //     let x_start=self.start+range_start;
    //     let x_end = x_start+x_len;

    //     // if x_start<self.start || x_end>self.end
    //     //     || x_end<self.start || x_start > self.end
    //     // {
    //     //     return None
    //     // }

    //     // let last_loc=if range_start==0 {
    //     //     self.last_loc
    //     // } else if range_start==self.parsed.primitives.len() {
    //     //     self.parsed.primitives[range_start-1].start_loc
    //     // } else {
    //     //     self.parsed.primitives[range_start].start_loc
    //     // };

    //     // println!("~~~~ x_len={x_len} x_start={x_start} x_end={x_end}, len={}, range_start={range_start}, range_end={range_end}",self.len());

    //     Some(TokenIterContainer {last_loc,start: x_start, end: x_end, parsed: self.parsed})
    // }

    pub fn get_amount(&self,amount:usize) -> Option<TokenIterContainer<'t>> {
        if amount > self.len() {
            return None;
        }

        // None

        Some(TokenIterContainer {
            last_loc:self.last_loc,
            start: self.start,
            end: self.start+amount,
            parsed: self.parsed,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.start==self.end

        || (self.start+1==self.end && self.last().unwrap().is_eol())


    }

    pub fn first(&self) -> Result<TokenContainer<'t>,Loc> {
        // self.get(0)

        if self.is_empty() {
            Err(self.last_loc)
        } else {
            Ok(TokenContainer { parsed: self.parsed, token_ind:self.start,})
        }
    }
    // pub fn last(&self) -> Result<TokenContainer<'a>,Loc> {
    //     // self.get(if self.is_empty() {0} else{self.len()-1})

    //     if self.is_empty() {
    //         Err(self.last_loc)
    //     } else {
    //         Ok(TokenContainer { parsed: self.parsed, primitive_ind:self.end-1,})
    //     }
    // }
    fn pop_get<T,F>(&mut self,skip_eols:bool,func:F) -> Result<ValueContainer<'t,T>,Loc>
    where
        F:FnOnce(TokenContainer<'t>)->Result<ValueContainer<'t,T>,Loc>,
    {
        let mut tmp=self.clone();
        let v=tmp.first().and_then(func)?;
        tmp.pop_front().unwrap();
        *self=tmp;
        Ok(v)
    }
    pub fn pop_eol(&mut self) -> Result<ValueContainer<'t,()>,Loc> {
         self.pop_get(false,|p|p.get_eol())
    }
    pub fn pop_float(&mut self) -> Result<ValueContainer<'t,f64>,Loc> {
        self.pop_get(true,|p|p.get_float())
    }

    pub fn pop_int(&mut self) -> Result<ValueContainer<'t,i64>,Loc> {
        self.pop_get(true,|p|p.get_int())
    }

    pub fn pop_string(&mut self) -> Result<ValueContainer<'t,&'t str>,Loc> {
        self.pop_get(true,|p|p.get_string())
    }

    pub fn pop_symbol(&mut self) -> Result<ValueContainer<'t,&'t str>,Loc> {
        self.pop_get(true,|p|p.get_symbol())
    }

    pub fn pop_identifier(&mut self) -> Result<ValueContainer<'t,&'t str>,Loc> {
        self.pop_get(true,|p|p.get_identifier())
    }

    pub fn pop_keyword(&mut self) -> Result<ValueContainer<'t,&'t str>,Loc> {
        self.pop_get(true,|p|p.get_keyword())
    }
    // pub fn pop_with_identifiers<'b,I>(&mut self,idns:I) -> Result<ValueContainer<'a,&'a str>,Loc>
    // where
    //     I:IntoIterator<Item = &'b str>,
    // {
    //     self.pop_get(true,move|p|p.has_identifiers(idns))
    // }

    // pub fn pop_with_symbols<'b,I>(&mut self,symbols:I) -> Result<ValueContainer<'a,&'a str>,Loc>
    // where
    //     I:IntoIterator<Item = &'b str>,
    // {
    //     self.pop_get(true,move|p|p.has_symbols(symbols))
    // }


    pub fn pop_with_keyword<'b>(&mut self,keyword:&'b str) -> Result<ValueContainer<'t,&'t str>,Loc>
    {
        self.pop_get(true,move|p|p.has_keyword(keyword))
    }

    pub fn pop_with_symbol<'b,>(&mut self,symbol:&'b str) -> Result<ValueContainer<'t,&'t str>,Loc>
    {
        self.pop_get(true,move|p|p.has_symbol(symbol))
    }

    pub fn trim(&mut self) {
        // while self.next()
        //     .map(|x|!x.is_eol())
        //     .unwrap_or_default()
        // {
        // }
        while self.pop_eol().is_ok() {}
    }
    pub fn trimmed(self) -> Self {
        let mut t=self;
        t.trim();
        t
    }

}

impl<'t> Iterator for TokenIterContainer<'t> {
    type Item = TokenContainer<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            // let last_loc2=self.last_loc;
            self.last_loc=self.parsed.primitives[self.start].end_loc;

            let x=TokenContainer {token_ind: self.start,parsed: self.parsed,}; //last_loc:last_loc2
            self.start+=1;

            Some(x)
        } else {
            None
        }
    }
}

// impl<'t> DoubleEndedIterator for TokenIterContainer<'t> {
//     fn next_back(&mut self) -> Option<TokenContainer<'t>> {
//         if self.end > self.start {
//             self.end-=1;
//             let primitive_ind=self.end;

//             // let last_loc=if self.len()==1 {
//             //     self.last_loc
//             // } else {
//             //     self.parsed.primitives[primitive_ind-1].end_loc
//             // };

//             Some(TokenContainer {token_ind: primitive_ind,parsed: self.parsed,}) //last_loc
//         } else {
//             None
//         }
//     }
// }

impl<'t> std::fmt::Debug for TokenIterContainer<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        f.write_fmt(format_args!("[{}]", self.clone().map(|p|format!("{p:?}")).collect::<Vec<String>>().join(", ")))

    }
}

impl<'t> TokenIterTrait for TokenIterContainer<'t> {
    fn is_trimmable(&self) -> bool {
        false

    }
    // fn get<F:Fn(GrammarPrimitive<'g>)->Option<TokenContainer<'t>>>(&self, func:F) -> Option<TokenContainer<'t>> {
    //     None
    // }

    fn index(&self) -> usize {
        self.inds().start

    }


    fn trim2(&mut self) {
        self.trim();
    }

    fn truncate(&mut self,size:usize,) {
        let end2=self.start+size;

        if self.end > end2 {
            self.end=end2;
        }
    }

    fn take2(&mut self,size:usize,) {
        // if self.len()>=size {

        // }
        self.take(size);
    }
    fn len2(&self) -> usize {
        self.len()
    }

    fn is_empty2(&self) -> bool {
        self.is_empty()
    }

    fn inds2(&self) -> Range<usize> {
        self.inds()
    }





}

impl<'g,'t> TokenIterGetTrait<GrammarPrimitive<'g>> for TokenIterContainer<'t> {

    fn pop_primitive(&mut self, p:&GrammarPrimitive<'g>) -> bool {
        match p {
            GrammarPrimitive::String => self.pop_string().is_ok(),
            GrammarPrimitive::Identifier => self.pop_identifier().is_ok(),
            GrammarPrimitive::Int => self.pop_int().is_ok(),
            GrammarPrimitive::Float => self.pop_float().is_ok(),
            GrammarPrimitive::Symbol(s) => self.pop_with_symbol(s).is_ok(),
            GrammarPrimitive::Keyword(s) => self.pop_with_keyword(s).is_ok(),
            GrammarPrimitive::Eol => self.pop_eol().is_ok(),
        }
    }
}