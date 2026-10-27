use std::{ops::{Range, RangeInclusive}, str::CharIndices};

use crate::{clike::grammar::{node::GrammarNode, walker::GrammarWalker, GrammarWalkError, PrimitiveTrait, TokensTrait }, Loc};

#[derive(Clone,Hash,PartialEq,Eq,Debug)]
pub enum MyGroup {}

#[derive(Clone,Hash,PartialEq,Eq,Debug)]
pub enum MyPrimitive<'g> {
    Range(RangeInclusive<char>),
    Char(char),
    String(&'g str),
    NotChar(char),
    NotString(&'g str),
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
        MyNonTerm::Int => &And(&[
            &Primitive(MyPrimitive::Range('1' ..= '9')),
            &Many(&Primitive(MyPrimitive::Range('0' ..= '9')), 0,0),
        ]),

        //int '.' int*
        MyNonTerm::Float => &And(&[
            &NonTerm(MyNonTerm::Int),
            &Primitive(MyPrimitive::Char('.')),
            &NonTerm(MyNonTerm::Int),
        ]),

        //'\' ('s' | 't' | 'r' | 'n' | quote | '\')
        MyNonTerm::Escaped => &And(&[
            &Primitive(MyPrimitive::Char('\\')),
            &Primitive(MyPrimitive::Chars(&['s','t','r','n', '"','\\'])),
        ]),

        //quote (escaped | !quote)* quote
        MyNonTerm::String => &And(&[
            &Primitive(MyPrimitive::Char('"')),
            &Many(&Or(&[
                &NonTerm(MyNonTerm::Escaped),
                &Primitive(MyPrimitive::NotChar('"')),
            ]), 0,0),
            &Primitive(MyPrimitive::Char('"')),
        ]),

        //
        MyNonTerm::Symbol => &Primitive(MyPrimitive::Chars(&['&','|','<','>','+','-','*','/','(',')','[',']','{','}','.',',',';','^','%','=','!','~'])),

        //'_'* ('a'..'z' | 'A' .. 'Z') ('a'..'z' | 'A' .. 'Z' | '0' .. '9' | '_')
        MyNonTerm::Identifier => &And(&[
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
        ]),

        //
        MyNonTerm::Keyword => &Primitive(MyPrimitive::Strings(&["for","in","while","continue","break","goto","label","include","true","false","nil","void","print","println","format","var","fn","return","if","elif","else"])),

        //'/' '/' !eol*
        MyNonTerm::Cmnt => &And(&[
            &Primitive(MyPrimitive::String("//")),
            &Many(&And(&[
                &Primitive(MyPrimitive::NotChar('\n')),
                &Primitive(MyPrimitive::NotString("\r\n")),
            ]), 0,0),
        ]),

        // '/' '*'  !mcmnt_end* mcmnt_end.expect("closing comment")
        MyNonTerm::MlCmnt => &And(&[
            &Primitive(MyPrimitive::String("/*")),
            &Expect(&And(&[
                &Many(&Primitive(MyPrimitive::NotString("*/")), 0,0),
                &Primitive(MyPrimitive::String("*/")),
            ]), "closing comment"),
        ]),

        //'\r'? \n'
        MyNonTerm::Eol => &Or(&[
            &Primitive(MyPrimitive::Char('\n')),
            &Primitive(MyPrimitive::String("\r\n")),
        ]),

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

pub struct Tokens<'a> {
    chars : CharIndices<'a>,
    loc:Loc,
}
// impl<'a> std::fmt::Debug for TokenContainer<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}::{:?}", &self.token_ind,&self.token_type()))
//     }
// }


// impl<'t,'g> TokensTrait<MyPrimitive<'g>> for Tokens<'t> {


//     fn trim2(&mut self) { }

//     fn truncate2(&mut self,size:usize,) {
//         let end2=self.start+size;

//         if self.end > end2 {
//             self.end=end2;
//         }
//     }

//     fn eat2(&mut self,size:usize,) {
//         // if self.len()>=size {

//         // }
//         // self.take(size);

//         for _ in 0..size.min(self.len()) {
//             self.next().unwrap();
//         }
//     }
//     fn len2(&self) -> usize {
//         self.len()
//     }

//     fn is_empty2(&self) -> bool {
//         self.is_empty()
//     }

//     fn inds2(&self) -> Range<usize> {
//         self.inds()
//     }



//     fn pop_primitive(&mut self, p:&MyPrimitive<'g>) -> bool {
//         match p {
//             MyPrimitive::String => self.pop_string().is_ok(),
//             MyPrimitive::Identifier => self.pop_identifier().is_ok(),
//             MyPrimitive::Int => self.pop_int().is_ok(),
//             MyPrimitive::Float => self.pop_float().is_ok(),
//             MyPrimitive::Symbol(s) => self.pop_with_symbol(s).is_ok(),
//             MyPrimitive::Keyword(s) => self.pop_with_keyword(s).is_ok(),
//             MyPrimitive::Eol => self.pop_eol().is_ok(),
//         }
//     }


// }

// pub fn tokenise(src:&str,) {

//         //
//         let mut walker=GrammarWalker::new(tokens, get_non_term,);
//         walker.set_debug(true);

//         let start_time = std::time::Instant::now();
//         let result=walker.run(MyNonTerm::Start) ;


//         let time_elapsed=start_time.elapsed().as_secs_f64();
//         //
//         if let Err(e)=result {


//             println!("Time elapsed: {time_elapsed:?} {}" ,walker.step_count());

//             match e {
//                 GrammarWalkError::FailedParse => {
//                     let expecteds=walker.expects_string();

//                     let error_type= if expecteds.is_empty() {
//                         CompileErrorType::ParserUnexpected
//                     } else {
//                         CompileErrorType::ParserExpected(expecteds)
//                     };
//                     // let error_type=CompileErrorType::ParserExpected(expecteds);

//                     return Err(CompileError{path:pathbuf,src,loc:
//                         // walker.last_loc()
//                         walker.last_loc2().start_loc()
//                         ,error_type});

//                     // return Err(CompileError{path:pathbuf,src,loc:walker.last_loc(),error_type:CompileErrorType::ParserExpected(walker.expects_string())});
//                 }
//                 // GrammarWalkError::Unfinished => todo!(),
//                 // GrammarWalkError::RecursiveNonTerm(_) => todo!(),
//                 // GrammarWalkError::MissingNonTerm(_) => todo!(),
//                 _ => {
//                     // println!("{:?} {:?}",walker.expecteds_string(),walker.last_loc());
//                     panic!("{e:?}");
//                 }
//             }
//         }

//         //
//         println!("-----------------");
//         if true {

//             let walk2=walker.get_walk2(false);

//             let mut stk=vec![(walk2.root(),0)];

//             while let Some((cur,depth))=stk.pop() {
//                 let indent="    ".repeat(depth);
//                 // let xx= cur.tokens().map(|x|format!("{x:?}")).collect::<Vec<_>>().join(", ");
//                 // println!("{indent}group: {:?} : [{}]",cur.name2(),xx);
//                 if let Some(name)=cur.name2() {
//                     // let xx= cur.tokens().map(|x|format!("{x:?}")).collect::<Vec<_>>().join(", ");
//                     println!("{indent}group: {name:?}",);
//                     stk.extend(cur.children().rev().map(|c|(c,depth+1)));

//                     if cur.children().len()==0 {
//                         for t in cur.tokens() {
//                             println!("{indent}    {t:?}");
//                         }
//                     }
//                 } else  {
//                     for t in cur.tokens() {
//                         println!("{indent}{t:?}");
//                     }

//                 }
//             }

//             println!("1_____");
//             for (i,g) in walk2.groups.iter().enumerate() {
//                 println!("{i}: {:?} : {:?} : {:?}",g.name,g.children,g.tokens);
//             }
//         }

//         let walk=walker.get_walk2(false);

//             println!("2_____");
//             for (i,g) in walk.groups.iter().enumerate() {
//                 println!("{i}: {:?} : {:?} : {:?}",g.name,g.children,g.tokens);
//             }
//         println!("===---===");
// }