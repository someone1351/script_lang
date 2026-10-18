use std::ops::{RangeInclusive};

use crate::clike::grammar::{node::GrammarNode, GrammarPrimitiveTrait};


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


impl<'g> GrammarPrimitiveTrait for MyPrimitive<'g> {
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


pub fn get_non_term<'g>(n:MyNonTerm) -> Option<&'g GrammarNode<'g,MyNonTerm,MyPrimitive<'g>>> {

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

/*

int => '1' .. '9' ('0' .. '9')*

float => int '.' int*

escape => '\' ('s' | 't' | 'r' | 'n' | quote)
quote => '"'
string => quote (escape | !quote)* quote

symbol => '|' | '&' | '>' | '<' | '/' | '*' | '+' | '-' | ',' | '.' | '(' | ')' | '[' | ']' | '{' | '}' | ';' | '^' | '%' | '=' | '!' | '~'

identifier => '_'* ('a'..'z' | 'A' .. 'Z') ('a'..'z' | 'A' .. 'Z' | '0' .. '9' | '_')

keyword => "for" | "in" | "while" | "continue" | "break" | "goto" | "label" | "include" | "true" | "false" | "nil" | "void" | "print" | "println" | "format" | "var" | "fn" | "return" | "if" | "elif" | "else"

cmnt => '/' '/' !eol*
mcmnt_end => '*' '/'
mcmnt => '/' '*'  !mcmnt_end* mcmnt_end.expect("closing comment")

eol => '\r'? \n'
spc => (' ' | '\t')+
ws => (cmnt | mcmnt | spc | eol)*

token => string | float | int | identifier | symbol
start => token? (ws token)* ws?


*/