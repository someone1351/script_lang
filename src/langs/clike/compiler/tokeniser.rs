use std::{fmt::Debug, ops::{Range, RangeInclusive}, path::Path, str::{CharIndices, Chars}};

use crate::{clike::{compiler::{error::CompileError, TokeniserErrorType}, grammar::{node::GrammarNode, utils::groups_debug_print, walker::GrammarWalker, GrammarWalkError, PrimitiveTrait, TokensPrimitiveTrait, TokensTrait }}, Loc};

#[derive(Clone,Hash,PartialEq,Eq,Debug)]
pub enum MyGroup {
    Identifier,
    Int,
    Float,
    String,
    Escaped,
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
    // // NotChar(char),
    // // NotString(&'g str),
    // Chars(&'g [char]),
    // Strings(&'g [&'g str]),
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
            &Many(&And(&[
                &Or(&[&NonTerm(MyNonTerm::Ws),&Always]),
                &NonTerm(MyNonTerm::Token),
            ]), 0,0),
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
            // &NonTerm(MyNonTerm::Int),
            &Many(&Primitive(MyPrimitive::Range('0' ..= '9')), 1,0),
        ]),MyGroup::Float),

        //'\' ('s' | 't' | 'r' | 'n' | quote | '\')
        MyNonTerm::Escaped => &And(&[
            &Primitive(MyPrimitive::Char('\\')),
            &Group(&Or(&[
                // &Primitive(MyPrimitive::Chars(&['s','t','r','n', '"','\\'])),
                &Primitive(MyPrimitive::Char('s')),
                &Primitive(MyPrimitive::Char('t')),
                &Primitive(MyPrimitive::Char('r')),
                &Primitive(MyPrimitive::Char('n')),
                &Primitive(MyPrimitive::Char('"')),
                &Primitive(MyPrimitive::Char('\\')),
                &NonTerm(MyNonTerm::Eol),
                // &Always,
            ]),MyGroup::Escaped),
        ]),

        //quote (escaped | !quote)* quote
        MyNonTerm::String => &And(&[
            &Primitive(MyPrimitive::Char('"')),
            &Group(&Many(&Or(&[
                &NonTerm(MyNonTerm::Escaped),
                &And(&[&Not(&Primitive(MyPrimitive::Char('"'))), &Primitive(MyPrimitive::Any)]),
            ]), 0,0),MyGroup::String),
            &Primitive(MyPrimitive::Char('"')),
        ]),

        //
        // MyNonTerm::Symbol => &Group(&Primitive(MyPrimitive::Chars(&['&','|','<','>','+','-','*','/','(',')','[',']','{','}','.',',',';','^','%','=','!','~'])),MyGroup::Symbol),
        MyNonTerm::Symbol => &Group(
            &Or(&[
                &Primitive(MyPrimitive::Char('&')),
                &Primitive(MyPrimitive::Char('|')),
                &Primitive(MyPrimitive::Char('<')),
                &Primitive(MyPrimitive::Char('>')),
                &Primitive(MyPrimitive::Char('+')),
                &Primitive(MyPrimitive::Char('-')),
                &Primitive(MyPrimitive::Char('*')),
                &Primitive(MyPrimitive::Char('/')),
                &Primitive(MyPrimitive::Char('(')),
                &Primitive(MyPrimitive::Char(')')),
                &Primitive(MyPrimitive::Char('[')),
                &Primitive(MyPrimitive::Char(']')),
                &Primitive(MyPrimitive::Char('{')),
                &Primitive(MyPrimitive::Char('}')),
                &Primitive(MyPrimitive::Char('.')),
                &Primitive(MyPrimitive::Char(',')),
                &Primitive(MyPrimitive::Char(';')),
                &Primitive(MyPrimitive::Char('^')),
                &Primitive(MyPrimitive::Char('%')),
                &Primitive(MyPrimitive::Char('=')),
                &Primitive(MyPrimitive::Char('!')),
                &Primitive(MyPrimitive::Char('~')),
            ]),MyGroup::Symbol),

        //'_'* ('a'..'z' | 'A' .. 'Z') ('a'..'z' | 'A' .. 'Z' | '0' .. '9' | '_')
        MyNonTerm::Identifier => &Group(&And(&[

            &Or(&[
                &Primitive(MyPrimitive::Range('a'..='z')),
                &Primitive(MyPrimitive::Range('A'..='Z')),
                &Primitive(MyPrimitive::Char('_')),
            ]),
            &Many(&Or(&[
                &Primitive(MyPrimitive::Range('a'..='z')),
                &Primitive(MyPrimitive::Range('A'..='Z')),
                &Primitive(MyPrimitive::Range('0'..='9')),
                &Primitive(MyPrimitive::Char('_')),
            ]), 0,0),
        ]),MyGroup::Identifier),

        //
        // MyNonTerm::Keyword => &Group(&Primitive(MyPrimitive::Strings(&["for","in","while","continue","break","goto","label","include","true","false","nil","void","print","println","format","var","fn","return","if","elif","else"])),MyGroup::Keyword),

        MyNonTerm::Keyword => &Group(&And(&[
            &Or(&[
                &Primitive(MyPrimitive::String("for")),
                &Primitive(MyPrimitive::String("in")),
                &Primitive(MyPrimitive::String("while")),
                &Primitive(MyPrimitive::String("continue")),
                &Primitive(MyPrimitive::String("break")),
                &Primitive(MyPrimitive::String("goto")),
                &Primitive(MyPrimitive::String("label")),
                &Primitive(MyPrimitive::String("include")),
                &Primitive(MyPrimitive::String("true")),
                &Primitive(MyPrimitive::String("false")),
                &Primitive(MyPrimitive::String("nil")),
                &Primitive(MyPrimitive::String("println")),
                &Primitive(MyPrimitive::String("print")),
                &Primitive(MyPrimitive::String("format")),
                &Primitive(MyPrimitive::String("fn")),
                &Primitive(MyPrimitive::String("return")),
                &Primitive(MyPrimitive::String("if")),
                &Primitive(MyPrimitive::String("elif")),
                &Primitive(MyPrimitive::String("else")),
                &Primitive(MyPrimitive::String("void")),
                &Primitive(MyPrimitive::String("var")),
            ]),
            &Not(&Or(&[
                &Primitive(MyPrimitive::Range('a'..='z')),
                &Primitive(MyPrimitive::Range('A'..='Z')),
                &Primitive(MyPrimitive::Range('0'..='9')),
                &Primitive(MyPrimitive::Char('_')),
            ]))
        ]),MyGroup::Keyword),

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

            &And(&[
                &Many(&And(&[
                    &Not(&Primitive(MyPrimitive::String("*/"))),
                    &Primitive(MyPrimitive::Any),
                ]), 0,0),
                &Or(&[
                    &Primitive(MyPrimitive::String("*/")),
                    &Error("closing comment")
                ]),
            ]),
        ]),

        //'\r'? \n'
        MyNonTerm::Eol => &Group(&Or(&[
            &Primitive(MyPrimitive::Char('\n')),
            &Primitive(MyPrimitive::String("\r\n")),
            &And(&[&Primitive(MyPrimitive::Char('\r')),&Error("lone carriage return")]), //don't allow by itself
        ]),MyGroup::Eol),

        //(' ' | '\t')+
        // MyNonTerm::Spc => &Many(&Primitive(MyPrimitive::Chars(&[' ','\t'])), 1,0),
        MyNonTerm::Spc => &Many(&Or(&[
            &Primitive(MyPrimitive::Char(' ')),
            &Primitive(MyPrimitive::Char('\t')),
        ]), 1,0),

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
pub struct Input<'t> {
    // chars : Chars<'t>,
    chars:&'t str,
    start : usize,
    end : usize,
    start_loc:Loc,
    end_loc:Loc,
}
// impl<'a> std::fmt::Debug for TokenContainer<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}::{:?}", &self.token_ind,&self.token_type()))
//     }
// }


impl<'t> Input<'t> {
    pub fn new(src:&'t str) -> Self {
        // let chars=src.chars();
        // let end=chars.clone().count();

        let chars=src;
        let end=src.len();

        Self {
            chars,
            start: 0,
            end,
            start_loc: Loc::one(),
            end_loc: Loc::zero(), //uninit
        }
    }
}

impl<'t> std::fmt::Debug for Input<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f,"{}",self.chars.clone().map(|c|format!("{c:?}")).collect::<Vec<String>>().join(", "))
        write!(f,"{:?}",self.chars)
    }
}

// impl<'t> Iterator for Input<'t> {
//     type Item = char;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.start < self.end {
//             let orig_len=self.chars.as_str().len();
//             let orig_chars=self.chars.as_str();

//             let x=self.chars.next().unwrap();

//             let taken_len=orig_len-self.chars.as_str().len();
//             let taken=&orig_chars[..taken_len];

//             self.start_loc.calc(taken);

//             self.start+=1;
//             Some(x)
//         } else {
//             None
//         }
//     }
// }

impl<'t> TokensTrait for Input<'t> {


    fn trim2(&mut self) {

    }

    fn len2(&self) -> usize {
        // println!("{} {}",self.end,self.start);
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
                chars: "",
                start:self.start,
                end: self.start,
                start_loc: self.start_loc,
                end_loc: self.start_loc,
            })
        } else {
            let start2=self.start;
            let end2=self.start+size;
            self.start=end2;

            // let orig_len=self.chars.len();
            // let orig_chars=self.chars;

            // for _ in 0..size {
            //     self.chars.next().unwrap();
            // }


            // let taken_len=orig_len-self.chars.len();
            // println!("takenlen={taken_len}");

            let taken=&self.chars[0..size];
            // let taken_chars=taken.chars();

            self.chars=&self.chars[size ..];
            // self.start+=size;

            let loc_start2=self.start_loc;
            self.start_loc.calc(taken);
            // self.start+=size;


            Some(Input{
                start_loc:loc_start2,
                end_loc:self.start_loc,
                start: start2,
                end: end2,
                chars: taken,
            })
        }
    }
}

impl<'t,'g> TokensPrimitiveTrait<MyPrimitive<'g>> for Input<'t> {
    fn pop2(&mut self, p:&MyPrimitive<'g>) -> bool {
        match p {
            MyPrimitive::Range(x) => {
                let mut cs=self.chars.chars();
                let Some(c)=cs.next() else {return false};
                if !x.contains(&c) {return false;}

                let char_len=self.chars.len()-cs.as_str().len();
                self.start_loc.calc(&self.chars[..char_len]);
                self.start+=char_len;
                self.chars=cs.as_str();
                return true;
            }
            MyPrimitive::Any => {
                let mut cs=self.chars.chars();
                let Some(_)=cs.next() else {return false};


                let char_len=self.chars.len()-cs.as_str().len();
                self.start_loc.calc(&self.chars[..char_len]);
                self.start+=char_len;
                self.chars=cs.as_str();
                return true;
            }
            MyPrimitive::Char(x) => {
                let mut cs=self.chars.chars();
                let Some(c)=cs.next() else {return false};
                if *x!=c {return false;}

                let char_len=self.chars.len()-cs.as_str().len();
                self.start_loc.calc(&self.chars[..char_len]);
                self.start+=char_len;
                self.chars=cs.as_str();

                return true;
            }
            MyPrimitive::String(s) => {
                if !self.chars.starts_with(s) { return false; }

                self.start_loc.calc(s);
                self.start+=s.len();
                self.chars=&self.chars[s.len()..];

                return true;
            }

            // MyPrimitive::Chars(xs) => {
            //     let Some(c)=self.clone().next() else {return false};

            //     for &x in xs.iter() {
            //         if x!=c {continue;}
            //         self.next().unwrap();
            //         return true;
            //     }

            //     return false;
            // }
            // MyPrimitive::Strings(xs) => {
            //     for x in xs.iter() {
            //         if !self.chars.as_str().starts_with(x) {
            //             continue;
            //         }

            //         for _ in 0 .. x.chars().count() {
            //             self.next().unwrap();
            //         }

            //         return true;
            //     }

            //     return false;


            // }
        }
    }




}


// pub struct Tokens<'a> {
//     tokens:Vec<Token<'a>>,
// }

#[derive(Copy,Clone,Debug)]
pub enum Keyword {
    For,In, //To,
    While,Continue,Break,
    Goto,Label,
    Include,
    True,False,Nil,Void,
    Print,Println,Format,
    Var,Fn,Return,
    If,Elif,Else,
}

#[derive(Debug)]
enum TokenType<'a> {
    Identifier(&'a str),
    Keyword(Keyword),
    Symbol(char),
    String(String), //&'a str
    Int(i64),
    Float(f64),
    Eol,
}

pub struct Token<'t> {
    pub start_loc:Loc,
    pub end_loc:Loc,
    pub token_type:TokenType<'t>,
}

impl<'t> Debug for Token<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.token_type.fmt(f)
    }
}
pub fn tokenise<'p,'t,'g>(
    src:&'t str,
    path:Option<&'p Path>,

) -> Result<Vec<Token<'t>>, CompileError<'p,'t>> { //(Loc,&'g str)

    let input=Input::new(src);
    //
    let mut walker=GrammarWalker::new(input, get_non_term,);
    // walker.set_debug(true);

    let start_time = std::time::Instant::now();
    let result=walker.run(MyNonTerm::Start) ;


    let time_elapsed=start_time.elapsed().as_secs_f64();

    println!("tokeniser Time elapsed: {time_elapsed:?} {}" ,walker.step_count());

    //
    if let Err(e)=result {


        let loc=walker.remaining_tokens().start_loc;

        match e {
            GrammarWalkError::FailedParse => {
                // println!("---111");
                return Err(CompileError{ src:Some(src), path, error_type: crate::clike::compiler::CompileErrorType::Tokeniser(""), loc, });
            }
            GrammarWalkError::Error(e) => {
                // println!("---222");
                return Err(CompileError{ src:Some(src), path, error_type: crate::clike::compiler::CompileErrorType::Tokeniser(e), loc, });
            }
            _ => {
                panic!("{e:?}");
            }
        }
    }

    //
    println!("-----------------");

    let walk2=walker.get_walk2();
    // groups_debug_print(walk2.root(),true);


    let mut out_tokens: Vec<Token<'t>>=Vec::new();
    for x in walk2.root().children() {
        let ts=x.tokens();
        let s=ts.chars;

        let token_type=match x.name2().unwrap() {
            MyGroup::Identifier => TokenType::Identifier(s),
            MyGroup::Escaped => {panic!("");},
            MyGroup::String => {
                let mut ss=String::new();

                for y in x.children_ext(true) {
                    match y.name2() {
                        Some(MyGroup::Escaped) => {
                            ss.push(match y.tokens().chars {
                                "s" => ' ',
                                "t" => '\t',
                                "r" => '\r',
                                "n" => '\n',
                                "\\" => '\\',
                                "\"" => '"',
                                x => {panic!("'{x}'");},
                            });
                        }
                        None => {
                            ss.push_str(y.tokens().chars);
                        }
                        _ => {panic!("");}
                    }
                }
                TokenType::String(ss)
            },
            MyGroup::Symbol => TokenType::Symbol(s.chars().next().unwrap()),
            MyGroup::Keyword => TokenType::Keyword(match s {
                "for" => Keyword::For,
                "in" => Keyword::In,
                "while" => Keyword::While,
                "continue" => Keyword::Continue,
                "break" => Keyword::Break,
                "goto" => Keyword::Goto,
                "label" => Keyword::Label,
                "include" => Keyword::Include,
                "true" => Keyword::True,
                "false" => Keyword::False,
                "nil" => Keyword::Nil,
                "void" => Keyword::Void,
                "print" => Keyword::Print,
                "println" => Keyword::Println,
                "format" => Keyword::Format,
                "var" => Keyword::Var,
                "fn" => Keyword::Fn,
                "return" => Keyword::Return,
                "if" => Keyword::If,
                "elif" => Keyword::Elif,
                "else" => Keyword::Else,
                _ => {panic!("");}
            }),
            MyGroup::Int => TokenType::Int(s.parse::<i64>().unwrap()),
            MyGroup::Float => TokenType::Float(s.parse::<f64>().unwrap()),
            MyGroup::Eol => TokenType::Eol,
        };

        if let TokenType::Eol = &token_type {

            if let Some(last_token)=out_tokens.last_mut() {
                if let TokenType::Eol=&last_token.token_type {
                    last_token.end_loc.calc(x.tokens().chars);
                    continue;
                }
            }
        }

        out_tokens.push(Token{
            start_loc:x.tokens().start_loc,
            end_loc:x.tokens().end_loc,
            token_type,
        });
    }

    Ok(out_tokens)
}