use std::ops::Range;

use crate::clike::grammar::GrammarPrimitiveTrait;


#[derive(Clone,Hash,PartialEq,Eq,Debug)]
pub enum MyPrimitive<'g> {
    Range(Range<char>),
    Char(char),
    String(&'g char),
    NotChar(char),
    NotString(&'g char),
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
    Escape,
    Quote,
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