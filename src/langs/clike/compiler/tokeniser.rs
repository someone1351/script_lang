use std::{ops::{Range, RangeInclusive}, str::{CharIndices, Chars}};

use crate::{clike::grammar::{node::GrammarNode, utils::groups_debug_print, walker::GrammarWalker, GrammarWalkError, PrimitiveTrait, TokensTrait }, Loc};

#[derive(Clone,Hash,PartialEq,Eq,Debug)]
pub enum MyGroup {
    Identifier,
    Int,
    Float,
    String,
    Symbol,
    Keyword,
    Eol,
}

#[derive(Clone,Hash,PartialEq,Eq,Debug)]
pub enum MyPrimitive<'g> {
    Range(RangeInclusive<char>),
    Any,
    Char(char),
    String(&'g str),
    // NotChar(char),
    // NotString(&'g str),
    Chars(&'g [char]),
    Strings(&'g [&'g str]),
}


impl<'g> PrimitiveTrait for MyPrimitive<'g> {
    fn is_trimmable(&self) -> bool {
        false
    }
}


#[derive(Debug,Clone,Copy,Hash,PartialEq, Eq)]
pub enum MyNonTerm {
    Start,
    Token,
    Int,
    Float,
    Escaped,
    String,
    Symbol,
    Identifier,
    Keyword,
    Cmnt,
    MlCmnt,
    Eol,
    Spc,
    Ws,
}


pub fn get_non_term<'g>(n:MyNonTerm) -> Option<&'g GrammarNode<'g,MyNonTerm,MyPrimitive<'g>,MyGroup>> {

    // use MyPrimitive::*;
    use GrammarNode::*;

    Some(match n {
        //token? (ws token)* ws?
        // MyNonTerm::Start => &And(&[
        //     &Or(&[&NonTerm(MyNonTerm::Token),&Always]),
        //     &And(&[
        //         &NonTerm(MyNonTerm::Ws),
        //         &NonTerm(MyNonTerm::Token),
        //     ]),
        //     &Or(&[&NonTerm(MyNonTerm::Ws),&Always]),
        // ]),


        //(ws? token)* ws?
        MyNonTerm::Start => &And(&[
            &And(&[
                &Or(&[&NonTerm(MyNonTerm::Ws),&Always]),
                &NonTerm(MyNonTerm::Token),
            ]),
            &Or(&[&NonTerm(MyNonTerm::Ws),&Always]),
        ]),

        //string | float | int | keyword | identifier | symbol
        MyNonTerm::Token => &Or(&[
            &NonTerm(MyNonTerm::String),
            &NonTerm(MyNonTerm::Float),
            &NonTerm(MyNonTerm::Int),
            &NonTerm(MyNonTerm::Keyword),
            &NonTerm(MyNonTerm::Identifier),
            &NonTerm(MyNonTerm::Symbol),
        ]),

        //'1' .. '9' ('0' .. '9')*
        MyNonTerm::Int => &Group(&And(&[
            &Primitive(MyPrimitive::Range('1' ..= '9')),
            &Many(&Primitive(MyPrimitive::Range('0' ..= '9')), 0,0),
        ]),MyGroup::Int),

        //int '.' int*
        MyNonTerm::Float => &Group(&And(&[
            &NonTerm(MyNonTerm::Int),
            &Primitive(MyPrimitive::Char('.')),
            &NonTerm(MyNonTerm::Int),
        ]),MyGroup::Float),

        //'\' ('s' | 't' | 'r' | 'n' | quote | '\')
        MyNonTerm::Escaped => &And(&[
            &Primitive(MyPrimitive::Char('\\')),
            &Primitive(MyPrimitive::Chars(&['s','t','r','n', '"','\\'])),
        ]),

        //quote (escaped | !quote)* quote
        MyNonTerm::String => &Group(&And(&[
            &Primitive(MyPrimitive::Char('"')),
            &Many(&Or(&[
                &NonTerm(MyNonTerm::Escaped),
                &And(&[&Not(&Primitive(MyPrimitive::Char('"'))), &Primitive(MyPrimitive::Any)]),
            ]), 0,0),
            &Primitive(MyPrimitive::Char('"')),
        ]),MyGroup::String),

        //
        MyNonTerm::Symbol => &Group(&Primitive(MyPrimitive::Chars(&['&','|','<','>','+','-','*','/','(',')','[',']','{','}','.',',',';','^','%','=','!','~'])),MyGroup::Symbol),

        //'_'* ('a'..'z' | 'A' .. 'Z') ('a'..'z' | 'A' .. 'Z' | '0' .. '9' | '_')
        MyNonTerm::Identifier => &Group(&And(&[
            &Primitive(MyPrimitive::Char('_')),
            &Or(&[
                &Primitive(MyPrimitive::Range('a'..='z')),
                &Primitive(MyPrimitive::Range('A'..='Z')),
            ]),
            &Or(&[
                &Primitive(MyPrimitive::Range('a'..='z')),
                &Primitive(MyPrimitive::Range('A'..='Z')),
                &Primitive(MyPrimitive::Range('0'..='9')),
                &Primitive(MyPrimitive::Char('_')),
            ]),
        ]),MyGroup::Identifier),

        //
        MyNonTerm::Keyword => &Group(&Primitive(MyPrimitive::Strings(&["for","in","while","continue","break","goto","label","include","true","false","nil","void","print","println","format","var","fn","return","if","elif","else"])),MyGroup::Keyword),

        //'/' '/' !eol*
        MyNonTerm::Cmnt => &And(&[
            &Primitive(MyPrimitive::String("//")),
            &Many(&And(&[
                &Not(&Or(&[
                    &Primitive(MyPrimitive::String("\r\n")),
                    &Primitive(MyPrimitive::Char('\n')),
                ])),
                &Primitive(MyPrimitive::Any),
            ]),
            0,0),
        ]),

        // '/' '*'  !mcmnt_end* mcmnt_end.expect("closing comment")
        MyNonTerm::MlCmnt => &And(&[
            &Primitive(MyPrimitive::String("/*")),
            &Expect(&And(&[
                &Many(&And(&[
                    &Not(&Primitive(MyPrimitive::String("*/"))),
                    &Primitive(MyPrimitive::Any),
                ]), 0,0),
                &Primitive(MyPrimitive::String("*/")),
            ]), "closing comment"),
        ]),

        //'\r'? \n'
        MyNonTerm::Eol => &Group(&Or(&[
            &Primitive(MyPrimitive::Char('\n')),
            &Primitive(MyPrimitive::String("\r\n")),
        ]),MyGroup::Eol),

        //(' ' | '\t')+
        MyNonTerm::Spc => &Many(&Primitive(MyPrimitive::Chars(&[' ','\t'])), 1,0),

        //(cmnt | mcmnt | spc | eol)+
        MyNonTerm::Ws => &Many(&Or(&[
            &NonTerm(MyNonTerm::Cmnt),
            &NonTerm(MyNonTerm::MlCmnt),
            &NonTerm(MyNonTerm::Spc),
            &NonTerm(MyNonTerm::Eol),
        ]), 1,0),
    })
}


// pub struct Input<'a> {
//     chars : CharIndices<'a>,

// }

#[derive(Clone)]
pub struct Input<'a> {
    chars : Chars<'a>,
    start : usize,
    end : usize,
    loc:Loc,
}
// impl<'a> std::fmt::Debug for TokenContainer<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}::{:?}", &self.token_ind,&self.token_type()))
//     }
// }


impl<'t> Input<'t> {
    pub fn new(src:&'t str) -> Self {
        let chars=src.chars();
        let end=chars.clone().count();

        Self {
            chars,
            start: 0,
            end,
            loc: Loc::zero(),
        }
    }
}

impl<'t> std::fmt::Debug for Input<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.chars.as_str())
    }
}

impl<'t> Iterator for Input<'t> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let orig_len=self.chars.as_str().len();

            let x=self.chars.next().unwrap();
            self.start+=1;

            let taken_len=orig_len-self.chars.as_str().len();
            let taken=&self.chars.as_str()[0..taken_len];
            self.loc.calc(taken);

            Some(x)
        } else {
            None
        }
    }
}

impl<'t,'g> TokensTrait<MyPrimitive<'g>> for Input<'t> {


    fn trim2(&mut self) { }

    fn len2(&self) -> usize {
        self.end-self.start
    }


    fn inds2(&self) -> Range<usize> {
        self.start..self.end
    }


    fn take2(&mut self,size:usize,) -> Option<Self> {
         // println!("front: {}+{} > {}",self.start,amount, self.end);
        if self.start+size > self.end { //|| amount==0
            None
        } else if size==0 {
            return Some(Input {
                chars: "".chars(),
                start:self.start,
                end: self.start,
                loc: self.loc,
            })
        } else {
            let start2=self.start;
            let end2=self.start+size;
            self.start=end2;

            let orig_len=self.chars.as_str().len();

            for _ in 0..size {
                self.chars.next().unwrap();
            }

            let taken_len=orig_len-self.chars.as_str().len();
            let taken=&self.chars.as_str()[0..taken_len];
            let taken_chars=taken.chars();

            self.loc.calc(taken);


            Some(Input{
                loc:self.loc,
                start: start2,
                end: end2,
                chars: taken_chars,
            })
        }
    }

    fn pop2(&mut self, p:&MyPrimitive<'g>) -> bool {
        match p {
            MyPrimitive::Range(r) => {
                let mut cs=self.chars.clone();
                let Some(c)=cs.next() else {return false};
                if !r.contains(&c) {return false;}
                let c_len=self.chars.as_str().len()-cs.as_str().len();
                self.chars=cs;
                self.loc.calc(&self.chars.as_str()[..c_len]);
                return true;
            }
            MyPrimitive::Any => {
                let mut cs=self.chars.clone();
                if cs.next().is_none() {return false};
                let c_len=self.chars.as_str().len()-cs.as_str().len();
                self.chars=cs;
                self.loc.calc(&self.chars.as_str()[..c_len]);
                return true;
            }
            MyPrimitive::Char(cc) => {
                let mut cs=self.chars.clone();
                let Some(c)=cs.next() else {return false};
                if *cc!=c {return false;}
                let c_len=self.chars.as_str().len()-cs.as_str().len();

                self.loc.calc(&self.chars.as_str()[..c_len]);
                self.chars=cs;
                return true;
            }
            MyPrimitive::String(s) => {
                if !self.chars.as_str().starts_with(s) {
                    return false;
                }

                for _ in 0 .. s.chars().count() {
                    self.chars.next().unwrap();
                }

                self.loc.calc(s);

                return true;
            }
            // MyPrimitive::NotChar(cc) => {
            //     let mut cs=self.chars.clone();
            //     let Some(c)=cs.next() else {return false};
            //     if *cc==c {return false;}
            //     let c_len=self.chars.as_str().len()-cs.as_str().len();
            //     self.chars=cs;
            //     self.loc.calc(&self.chars.as_str()[..c_len]);
            //     return true;
            // }
            // MyPrimitive::NotString(_) => {
            //     return true;
            // }
            MyPrimitive::Chars(ccs) => {
                let mut cs=self.chars.clone();
                let Some(c)=cs.next() else {return false};
                let c_len=self.chars.as_str().len()-cs.as_str().len();

                for cc in ccs.iter() {
                    if *cc!=c {continue;}
                    self.chars=cs;
                    self.loc.calc(&self.chars.as_str()[..c_len]);
                    return true;
                }

                return false;
            }
            MyPrimitive::Strings(ss) => {
                for s in ss.iter() {
                    if !self.chars.as_str().starts_with(s) {
                        continue;
                    }

                    for _ in 0 .. s.chars().count() {
                        self.chars.next().unwrap();
                    }

                    self.loc.calc(s);
                    return true;
                }

                return false;


            }
        }
    }




}

pub struct Output {

}
pub fn tokenise(src:&str,) -> Result<Output, Loc> {

    let input=Input::new(src);
    //
    let mut walker=GrammarWalker::new(input, get_non_term,);
    walker.set_debug(true);

    let start_time = std::time::Instant::now();
    let result=walker.run(MyNonTerm::Start) ;


    let time_elapsed=start_time.elapsed().as_secs_f64();
    //
    if let Err(e)=result {


        println!("Time elapsed: {time_elapsed:?} {}" ,walker.step_count());

        match e {
            GrammarWalkError::FailedParse => {
                return Err(walker.tokens().loc);
            }
            _ => {
                panic!("{e:?}");
            }
        }
    }

    //
    println!("-----------------");

    let walk2=walker.get_walk2(false);
    groups_debug_print(walk2.root());
    let mut output = Output {};

    Ok(output)
}