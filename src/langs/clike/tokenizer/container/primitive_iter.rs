use std::ops::{Bound, Range, RangeBounds};

use crate::build::Loc;
use super::super::super::tokenizer::data::Tokenized;

use super::*;

#[derive(Copy,Clone)]
pub struct TokenIterContainer<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub last_loc:Loc,
    pub parsed :&'a Tokenized,

}

impl<'a> TokenIterContainer<'a> {
    pub fn inds(&self) -> Range<usize> {
        self.start..self.end
    }
    pub fn last_loc(&self) -> Loc {
        self.last_loc
    }
    pub fn loc(&self) -> Loc {
        self.first().map(|p|p.start_loc()).unwrap_or(self.last_loc)
    }

    pub fn pop_front(&mut self) -> Result<TokenContainer<'a>,Loc> {
        if self.start < self.end {
            let primitive_ind=self.start;
            self.start+=1;
            self.last_loc=self.loc();
            Ok(TokenContainer { parsed: self.parsed, primitive_ind,}) //last_loc:self.last_loc,
        } else {
            Err(self.last_loc)
        }
    }
    pub fn pop_back(&mut self) -> Result<TokenContainer<'a>,Loc> {
        if self.start < self.end {
            self.end-=1;
            let primitive_ind=self.end;

            // let last_loc=if self.start==self.end {
            //     self.last_loc
            // } else {
            //     self.parsed.primitives[self.end-1].end_loc
            // };

            Ok(TokenContainer { parsed: self.parsed, primitive_ind,}) //last_loc
        } else {
            Err(self.last_loc)
        }
    }

    pub fn pop_front_amount(&mut self,amount:usize) -> Option<TokenIterContainer<'a>> {
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
    pub fn pop_back_amount(&mut self,amount:usize) -> Option<TokenIterContainer<'a>> {
        if self.start+amount > self.end
        // || amount==0
        {
            println!("{}+{} > {}", self.start,amount,self.end);

            None
        } else {
            let end2=self.end;
            self.end-=amount;
            let start2=self.end;

            let last_loc=if self.start==self.end {
                self.last_loc
            } else {
                self.parsed.primitives[self.end-1].start_loc
            };

            Some(TokenIterContainer{last_loc, start: start2, end: end2, parsed: self.parsed })
        }
    }

    pub fn len(&self) -> usize {
        self.end-self.start
    }

    pub fn get(&self, ind:usize) -> Result<TokenContainer<'a>,Loc> {
        let primitive_ind= self.start+ind;

        if primitive_ind < self.end {
            // let last_loc=if ind==0 {
            //     self.last_loc
            // } else {
            //     self.parsed.primitives[primitive_ind-1].end_loc
            // };

            Ok(TokenContainer { parsed: self.parsed, primitive_ind,}) //last_loc
        } else {
            // let last_loc=if self.len()==0 {
            //     self.last_loc
            // } else {
            //     self.get(self.len()-1).unwrap().end_loc()
            // };

            let last_loc=self.last().map(|x|x.end_loc()).unwrap_or(self.last_loc);
            Err(last_loc)
        }
    }

    pub fn get_range<R:RangeBounds<usize>>(&self,r:R) ->
    // Result<TokenIterContainer<'a>,Loc>
    Option<TokenIterContainer<'a>>
    {

        let range_start=match r.start_bound().cloned() {
            Bound::Included(x)=>x,
            Bound::Excluded(_)=>panic!(""),
            Bound:: Unbounded=>0,
        };

        let range_end=match r.end_bound().cloned() {
            Bound::Included(x)=>x+1,
            Bound::Excluded(x)=>x,
            Bound::Unbounded=>self.len(),
        };

        let last_loc=self.last().map(|x|x.end_loc()).unwrap_or(self.last_loc);

        if range_start>range_end { //if range start==end is same as empty iter
            // return TokenIterContainer {last_loc:Loc::zero(),start: 0, end: 0, parsed: self.parsed};
            // return Err(last_loc);
            println!("rs>re {:?} {:?}",r.start_bound(),r.end_bound());
            return None;
        }

        let x_len=range_end-range_start;

        if x_len>self.len() {
            // return TokenIterContainer {last_loc:Loc::zero(),start: 0, end: 0, parsed: self.parsed};
            // return Err(last_loc);
            println!("xl>l {:?} {:?}",r.start_bound(),r.end_bound());
            return  None;
        }

        let x_start=self.start+range_start;
        let x_end = x_start+x_len;

        // if x_start<self.start || x_end>self.end
        //     || x_end<self.start || x_start > self.end
        // {
        //     return None
        // }

        // let last_loc=if range_start==0 {
        //     self.last_loc
        // } else if range_start==self.parsed.primitives.len() {
        //     self.parsed.primitives[range_start-1].start_loc
        // } else {
        //     self.parsed.primitives[range_start].start_loc
        // };

        // println!("~~~~ x_len={x_len} x_start={x_start} x_end={x_end}, len={}, range_start={range_start}, range_end={range_end}",self.len());

        Some(TokenIterContainer {last_loc,start: x_start, end: x_end, parsed: self.parsed})
    }

    pub fn is_empty(&self) -> bool {
        self.start==self.end
    }

    pub fn first(&self) -> Result<TokenContainer<'a>,Loc> {
        // self.get(0)

        if self.is_empty() {
            Err(self.last_loc)
        } else {
            Ok(TokenContainer { parsed: self.parsed, primitive_ind:self.start,})
        }
    }
    pub fn last(&self) -> Result<TokenContainer<'a>,Loc> {
        // self.get(if self.is_empty() {0} else{self.len()-1})

        if self.is_empty() {
            Err(self.last_loc)
        } else {
            Ok(TokenContainer { parsed: self.parsed, primitive_ind:self.end-1,})
        }
    }

    fn pop_get<T,F>(&mut self,skip_eols:bool,func:F) -> Result<ValueContainer<'a,T>,Loc>
    where
        F:FnOnce(TokenContainer<'a>)->Result<ValueContainer<'a,T>,Loc>,
    {
        if skip_eols {
            while let Ok(x)=self.first() {
                if !x.is_eol() {
                    break;
                }

                self.pop_front().unwrap();
            }
        }


        let v=self.first().and_then(func)?;

        // if v.is_ok() {
        self.pop_front().unwrap();
        // self.last_loc=v.as_ref().unwrap().primitive.end_loc();
        // self.last_loc=v.primitive.end_loc();
        // }

        Ok(v)
    }

    pub fn pop_eol(&mut self) -> Result<ValueContainer<'a,()>,Loc> {
         self.pop_get(false,|p|p.get_eol())
    }
    pub fn pop_float(&mut self) -> Result<ValueContainer<'a,f64>,Loc> {
        self.pop_get(true,|p|p.get_float())
    }

    pub fn pop_int(&mut self) -> Result<ValueContainer<'a,i64>,Loc> {
        self.pop_get(true,|p|p.get_int())
    }

    pub fn pop_string(&mut self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        self.pop_get(true,|p|p.get_string())
    }

    pub fn pop_symbol(&mut self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        self.pop_get(true,|p|p.get_symbol())
    }

    pub fn pop_identifier(&mut self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        self.pop_get(true,|p|p.get_identifier())
    }

    pub fn pop_keyword(&mut self) -> Result<ValueContainer<'a,&'a str>,Loc> {
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


    pub fn pop_with_keyword<'b>(&mut self,keyword:&'b str) -> Result<ValueContainer<'a,&'a str>,Loc>
    {
        self.pop_get(true,move|p|p.has_keyword(keyword))
    }

    pub fn pop_with_symbol<'b,>(&mut self,symbol:&'b str) -> Result<ValueContainer<'a,&'a str>,Loc>
    {
        self.pop_get(true,move|p|p.has_symbol(symbol))
    }
}

impl<'a> Iterator for TokenIterContainer<'a> {
    type Item = TokenContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            // let last_loc2=self.last_loc;
            self.last_loc=self.parsed.primitives[self.start].end_loc;

            let x=TokenContainer {primitive_ind: self.start,parsed: self.parsed,}; //last_loc:last_loc2
            self.start+=1;

            Some(x)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for TokenIterContainer<'a> {
    fn next_back(&mut self) -> Option<TokenContainer<'a>> {
        if self.end > self.start {
            self.end-=1;
            let primitive_ind=self.end;

            // let last_loc=if self.len()==1 {
            //     self.last_loc
            // } else {
            //     self.parsed.primitives[primitive_ind-1].end_loc
            // };

            Some(TokenContainer {primitive_ind,parsed: self.parsed,}) //last_loc
        } else {
            None
        }
    }
}

impl<'a> std::fmt::Debug for TokenIterContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        f.write_fmt(format_args!("[{}]", self.clone().map(|p|format!("{p:?}")).collect::<Vec<String>>().join(", ")))

    }
}