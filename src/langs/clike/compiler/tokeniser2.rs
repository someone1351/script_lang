use core::panic;
use std::{fmt::Debug, num::{IntErrorKind, ParseFloatError, ParseIntError}, ops::{Range, RangeInclusive}, path::Path, str::{CharIndices, Chars}};

use crate::{clike::{compiler::{error::CompileError, CompileErrorType, TokeniserErrorType}, grammar::{node::GrammarNode, utils::groups_debug_print, walker::GrammarWalker, GrammarWalkError, PrimitiveTrait, TokensPrimitiveTrait, TokensTrait }}, Loc};

#[derive(Debug,Clone)]
pub enum  TokenizerErrorType2 {
    Unexpected,
    ClosingQuoteExpected,
    // UnexpectedChar,
    // ClosingBracketExpected(&'static str),
    // NoMatchingOpeningBracket(&'static str),
    ClosingCommentExpected,
    Int(ParseIntError),
    Float(ParseFloatError),

    // ExpectingClosingCurlyBracket,
    // ExpectingClosingSquareBracket,
    // ExpectingClosingParentheses,
}

    // pub fn take(&mut self, amount:usize) -> Option<Self> {
    //     if amount > self.data.len() {return None;}
    //     let t= Self {data:&self.data[..amount],pos:self.pos};
    //     self.data=&self.data[amount ..];
    //     self.pos+=amount;
    //     Some(t)
    // }
    // pub fn eat(&mut self, amount:usize) -> bool {
    //     let new_end = self.end_pos + amount;
    //     if new_end > self.data.len() {return false;}
    //     debug_assert!(self.data.is_char_boundary(new_end), "eat() landed inside a multi-byte UTF-8 character");
    //     self.end_pos=new_end;
    //     true
    // }
    // pub fn get_range(&self, r:RangeInclusive<char>) -> usize {
    //     let Some(x) = self.remaining().chars().next() else {return 0;};

    //     if r.contains(&x) {
    //         x.len_utf8()
    //     } else {
    //         0
    //     }
    // }

    // pub fn get_char(&self, cs:&str) -> usize {
    //     let Some(x) = self.remaining().chars().next() else {return 0;};

    //     if cs.contains(x) {
    //         x.len_utf8()
    //     } else {
    //         0
    //     }
    // }

    // pub fn get_str(&self, s:&str) -> usize {
    //     if self.remaining().starts_with(s) {
    //         s.len()
    //     } else {
    //         0
    //     }
    // }

    // pub fn is_range(&self, r:RangeInclusive<char>) -> bool {
    //     let x=self.get_range(r);
    //     if x==0 {return false;}
    //     true
    // }

    // pub fn is_char(&self, cs:&str) -> bool {
    //     let x=self.get_char(cs);
    //     if x==0 {return false;}
    //     true
    // }

    // pub fn is_str(&self, s:&str) -> bool {
    //     let x=self.get_str(s);
    //     if x==0 {return false;}
    //     true
    // }


#[derive(Clone,Copy,)]
pub struct Input2<'t> {
    data:&'t str,
    start_pos:usize,
    end_pos:usize,
    end_pos_rewind:usize,
}
impl<'t> Input2<'t> {
    pub fn new(data:&'t str) -> Self {
        Self { data, start_pos:0, end_pos:0,end_pos_rewind:0, }
    }
    pub fn slice(&self) -> &'t str {
        &self.data[self.start_pos..self.end_pos]
    }
    pub fn rest(&self) -> &'t str {
        &self.data[self.end_pos..]
    }
    pub fn peek(&self,chars_num:usize) -> Option<&'t str> {
        let rest=self.rest();

        let mut cs=rest.chars();

        for _ in 0..chars_num {
            if cs.next().is_none() {
                return None;
            }
        }

        let dif=rest.len()-cs.as_str().len();

        Some(&rest[..dif])
    }
    pub fn peek_char(&self,) -> Option<char> {
        self.rest().chars().next()
    }
    pub fn pos_range(&self) -> Range<usize> {
        self.start_pos..self.end_pos
    }
    pub fn is_eof(&self) -> bool {
        self.end_pos >= self.data.len()
    }

    pub fn commit(&mut self) {
        self.start_pos=self.end_pos;
        self.end_pos_rewind=self.end_pos;
    }

    pub fn reset(&mut self) {
        self.end_pos=self.start_pos;
        self.end_pos_rewind=self.start_pos;
    }

    pub fn mark(&mut self) {
        self.end_pos_rewind=self.end_pos;
    }

    pub fn rewind(&mut self) {
        self.end_pos=self.end_pos_rewind;
    }

    pub fn take_range(&mut self, r:RangeInclusive<char>) -> Option<char> {
        let Some(x) = self.rest().chars().next() else {return None;};
        if !r.contains(&x) {return None;}
        self.end_pos+=x.len_utf8();
        Some(x)
    }

    pub fn take_one_of(&mut self, cs:&str) -> Option<char> {
        let Some(x) = self.rest().chars().next() else {return None;};
        if !cs.contains(x) {return None;}
        self.end_pos+=x.len_utf8();
        Some(x)
    }

    pub fn take_char(&mut self, c:char) -> Option<char> {
        if Some(c)!=self.rest().chars().next() {return None;};
        self.end_pos+=c.len_utf8();
        Some(c)
    }
    pub fn take_any(&mut self,) -> Option<char> {
        let Some(x) = self.rest().chars().next() else {return None;};
        self.end_pos+=x.len_utf8();
        Some(x)
    }
    pub fn take_str(&mut self, s:&str) -> Option<&'t str> {
        let p=self.end_pos;

        if !self.rest().starts_with(s) {return None;}
        self.end_pos+=s.len();

        Some(&self.data[p..self.end_pos])
    }


    pub fn eat_range(&mut self, r:RangeInclusive<char>) -> bool {
        self.take_range(r).is_some()
    }
    pub fn eat_one_of(&mut self, cs:&str) -> bool {
        self.take_one_of(cs).is_some()
    }
    pub fn eat_char(&mut self, c:char) -> bool {
        self.take_char(c).is_some()
    }
    pub fn eat_any(&mut self,) -> bool {
        self.take_any().is_some()
    }
    pub fn eat_str(&mut self, s:&str) -> bool {
        self.take_str(s).is_some()
    }

}

impl<'t> std::fmt::Debug for Input2<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self.data)
    }
}

#[derive(Copy,Clone,Debug)]
pub enum Symbol {
    Variadic, // ...
    ForInclusiveRange, // ..=
    SetAnd, // &&=
    SetOr, // ||=

    ForExclusiveRange, // ..
    Equal, // ==
    NotEqual, // !=
    LessThanEqual, // <=
    GreaterThanEqual, // >=
    And, // &&
    Or, // ||

    SetAdd, // +=
    SetSub, // -=
    SetMul, // *=
    SetDiv, // /=
    SetMod, // %=
    // SetNot, // !=

    SetBitXor, // ^=
    SetBitAnd, // &=
    SetBitOr, // |=
    // SetBitNot, // ~=


    BitAnd, // &
    BitOr, // |
    BitXor, // ^
    BitNot, // ~

    Not, // !
    Set, // =
    Comma, // ,
    Mod, // %
    SemiColon, // ;

    Dot, // .
    Colon, // :

    GreaterThan, // >
    LessThan, // <

    Add, // +
    Sub, // -
    Mul, // *
    Div, // /

    LParen, // (
    RParen, // )
    LBracket, // [
    RBracket, // ]
    LBrace, // {
    RBrace, // }
}

impl Symbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Variadic => "...",
            Self::ForInclusiveRange => "..=",
            Self::SetAnd => "&&=",
            Self::SetOr => "||=",

            Self::ForExclusiveRange => "..",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::LessThanEqual => "<=",
            Self::GreaterThanEqual => ">=",
            Self::And => "&&",
            Self::Or => "||",

            Self::SetAdd => "+=",
            Self::SetSub => "-=",
            Self::SetMul => "*=",
            Self::SetDiv => "/=",
            Self::SetMod => "%=",

            Self::SetBitXor => "^=",
            Self::SetBitAnd => "&=",
            Self::SetBitOr => "|=",

            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::BitNot => "~",

            Self::Not => "!",
            Self::Set => "=",
            Self::Comma => ",",
            Self::Mod => "%",
            Self::SemiColon => ";",

            Self::Dot => ".",
            Self::Colon => ":",

            Self::GreaterThan => ">",
            Self::LessThan => "<",

            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",

            Self::LParen => "(",
            Self::RParen => ")",
            Self::LBracket => "[",
            Self::RBracket => "]",
            Self::LBrace => "{",
            Self::RBrace => "}",
        }
    }
}


#[derive(Copy,Clone,Debug)]
pub enum Keyword {
    Break,
    Continue,
    Elif,
    Else,
    False,
    Fn,
    For,
    Format,
    Goto,
    If,
    In,
    Include,
    Label,
    Nil,
    Print,
    Println,
    Return,
    True,
    Var,
    Void,
    While,

}

impl Keyword {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Break => "break",
            Self::Continue => "continue",
            Self::Elif => "elif",
            Self::Else => "else",
            Self::False => "false",
            Self::Fn => "fn",
            Self::For => "for",
            Self::Format => "format",
            Self::Goto => "goto",
            Self::If => "if",
            Self::In => "in",
            Self::Include => "include",
            Self::Label => "label",
            Self::Nil => "nil",
            Self::Print => "print",
            Self::Println => "println",
            Self::Return => "return",
            Self::True => "true",
            Self::Var => "var",
            Self::Void => "void",
            Self::While => "while",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "break" => Some(Self::Break),
            "continue" => Some(Self::Continue),
            "elif" => Some(Self::Elif),
            "else" => Some(Self::Else),
            "false" => Some(Self::False),
            "fn" => Some(Self::Fn),
            "for" => Some(Self::For),
            "format" => Some(Self::Format),
            "goto" => Some(Self::Goto),
            "if" => Some(Self::If),
            "in" => Some(Self::In),
            "include" => Some(Self::Include),
            "label" => Some(Self::Label),
            "nil" => Some(Self::Nil),
            "print" => Some(Self::Print),
            "println" => Some(Self::Println),
            "return" => Some(Self::Return),
            "true" => Some(Self::True),
            "var" => Some(Self::Var),
            "void" => Some(Self::Void),
            "while" => Some(Self::While),
            _ => None,
        }
    }
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

fn tokenise_num<'t>(input:&mut Input2<'t>) -> Result<Option<(TokenType<'t>,Range<usize>)>,(CompileErrorType<'t>,usize)> {
    //[0] | [0-9][1-9]*

    //
    if input.eat_range('1'..='9') {
        while input.eat_range('0'..='9') {}
    } else if input.eat_one_of("0") {
    } else {
        return Ok(None);
    }

    //
    input.mark(); //the int

    //
    if !input.eat_one_of(".") || !input.eat_range('0'..='9') {
        input.rewind(); //back to the int

        //int
        let val = input.slice().parse::<i64>().map_err(|e|(
            CompileErrorType::Tokeniser2(TokenizerErrorType2::Int(e)),
            input.pos_range().start,
        ))?;

        return Ok(Some((TokenType::Int(val),input.pos_range())));
    }

    //
    while input.eat_range('0'..='9') {}

    //float
    let val = input.slice().parse::<f64>().map_err(|e|(
        CompileErrorType::Tokeniser2(TokenizerErrorType2::Float(e)),
        input.pos_range().start,
    ))?;

    return Ok(Some((TokenType::Float(val),input.pos_range())));
}

fn tokenise_symbol<'t>(input:&mut Input2<'t>) -> Option<(TokenType<'t>,Range<usize>)> {
    //
    if !input.eat_one_of("[&|<>+-*/()[]{}.;^%=!~") {return None;}

    //
    let val = input.slice().chars().next().unwrap();
    Some((TokenType::Symbol(val),input.pos_range()))
}

// fn tokenise_keyword<'t>(input:&mut Input2<'t>) -> Option<(TokenType<'t>,Range<usize>)> {

//     let keywords = [
//         Keyword::Break, Keyword::Continue, Keyword::Elif, Keyword::Else, Keyword::False,
//         Keyword::Fn, Keyword::For, Keyword::Format, Keyword::Goto, Keyword::If, Keyword::In,
//         Keyword::Include, Keyword::Label, Keyword::Nil, Keyword::Print, Keyword::Println,
//         Keyword::Return, Keyword::True, Keyword::Var, Keyword::Void, Keyword::While,
//     ];

//     for keyword in keywords {
//         if !input.eat_str(keyword.as_str()) {
//             continue;
//         }

//         if input.eat_range('a'..='z') || input.eat_range('A'..='Z') || input.eat_range('0'..='9') || input.eat_one_of("_") {
//             while input.eat_range('a'..='z') || input.eat_range('A'..='Z') || input.eat_range('0'..='9') || input.eat_one_of("_") {}

//             return Some((TokenType::Identifier(input.slice()),input.pos_range()));
//         } else {
//             return Some((TokenType::Keyword(keyword),input.pos_range()));
//         }
//     }

//     None
// }

// fn tokenise_identifier<'t>(input:&mut Input2<'t>) -> Option<(TokenType<'t>,Range<usize>)> {
//     //[_]*[a-zA-Z][_a-zA-Z90-9]*

//     //
//     input.mark();

//     //
//     while input.eat_one_of("_") {}

//     //
//     if !input.eat_range('a'..='z') && !input.eat_range('A'..='Z') && !input.eat_one_of("_") {
//         input.rewind();
//         return None;
//     }

//     //
//     while input.eat_range('a'..='z') || input.eat_range('A'..='Z') || input.eat_range('0'..='9') || input.eat_one_of("_") {}

//     //
//     Some((TokenType::Identifier(input.slice()),input.pos_range()))
// }
fn tokenise_word<'t>(input: &mut Input2<'t>) -> Option<(TokenType<'t>, Range<usize>)> {
    input.mark();

    while input.eat_char('_') {}

    if !input.eat_range('a'..='z') && !input.eat_range('A'..='Z') {
        input.rewind();
        return None;
    }

    while input.eat_range('a'..='z')
        || input.eat_range('A'..='Z')
        || input.eat_range('0'..='9')
        || input.eat_char('_')
    {}

    let slice = input.slice();
    let token = match Keyword::from_str(slice) {
        Some(kw) => TokenType::Keyword(kw),
        None => TokenType::Identifier(slice),
    };

    Some((token, input.pos_range()))
}

fn tokenise_string<'t>(input:&mut Input2<'t>) -> Result<Option<(TokenType<'t>,Range<usize>)>,(CompileErrorType<'t>,usize)> {
    //["]([\\]([strn"\\\n][\r][\n])|[^"])*["]
    //
    if !input.eat_char('"') {
        return Ok(None);
    }

    //


    //
    // let quote_start=input.pos_range().end;
    // let mut quote_end=quote_start;

    //
    // input.commit();

    // let mut input2=input.clone();

    let mut s=String::new();

    //
    loop {
        //
        // let p=input.pos_range();

        //
        if input.eat_char('"') {
            return Ok(Some((TokenType::String(s),input.pos_range())));
        }

        //
        if input.eat_char('\\') {
            if let Some(c)=input.take_one_of("strn\n\"\\") {
                match c {
                    's' => {s.push(' ');}
                    't' => {s.push('\t');}
                    'r' => {s.push('\r');}
                    'n' => {s.push('\n');}
                    '\\' => {s.push('\\');}
                    '\n' => {s.push(' ');}
                    '"' => {s.push('"');}
                    _ => {panic!("");}
                }
            } else if input.eat_str("\r\n") {
                s.push(' ');
            } else {
                s.push('\\');
            }
        } else if let Some(c)=input.take_any() {
            s.push(c);
        } else {
            break;
        }

        // quote_end=input.pos_range().end;
    }


    Err((CompileErrorType::Tokeniser2(TokenizerErrorType2::ClosingQuoteExpected),input.pos_range().start))

}


fn tokenise_ws<'t>(input:&mut Input2<'t>) -> Result<Option<(TokenType<'t>,Range<usize>)>,(CompileErrorType<'t>,usize)> {
    // smlcmnt => [/][/][^\r\n]*([\n]|[\r][\n]|eof)
    // mlcmnt => [/][*] (mlcmnt|[^*]|[*][^/])* [*][/]
    // space => [ \t]
    // eol => [\r][\n]|[\n]

    let mut eol_start: Option<usize>=None;
    let mut eol_end= 0;

    loop {
        //space
        if input.eat_one_of(" \t") {
            continue;
        }

        //eol
        {
            let new_start=eol_start.unwrap_or(input.pos_range().end);

            if input.eat_char('\n') || input.eat_str("\r\n") {
                eol_start=Some(new_start);
                eol_end=input.pos_range().end;
                continue;
            }
        }

        //single line comment
        if input.eat_str("//") {
            while let Some(x)=input.peek_char() {
                if x=='\r' || x=='\n' {break;}
                input.eat_any();
            }

            continue;
        }

        //multi line comment
        let comment_start = input.pos_range().end;

        if input.eat_str("/*") {
            let mut depth=1;

            while depth>0 {
                if input.eat_str("/*") {
                    depth+=1;
                } else if input.eat_str("*/") {
                    depth-=1;
                } else if !input.eat_any() {
                    return Err((CompileErrorType::Tokeniser2(TokenizerErrorType2::ClosingCommentExpected),comment_start));
                }
            }

            //
            continue;
        }

        //
        break;
    }

    //
    if let Some(eol_start)=eol_start {
        return Ok(Some((TokenType::Eol,eol_start..eol_end)))
    }

    //
    Ok(None)
}

pub fn tokenise2<'p,'t,'g>(
    src:&'t str,
    path:Option<&'p Path>,

) -> Result<Vec<Token<'t>>, CompileError<'p,'t>>
{ //(Loc,&'g str)

    let mut input=Input2::new(src);
    let mut cur_loc=Loc::one();
    let mut tokens=Vec::new();

    loop {

        if let Some((token_type,p))=tokenise_ws(&mut input)
            .map_err(|(error_type,p)|{
                CompileError{src:Some(src),path,error_type,loc:cur_loc,}
            })?
        {
            let start_loc=Loc::one();
            let end_loc=Loc::one();
            tokens.push(Token{ start_loc, end_loc, token_type, });
        }

        //
        if let Some((token_type,p))=tokenise_num(&mut input)
            .map_err(|(error_type,p)|{
                CompileError{src:Some(src),path,error_type,loc:cur_loc,}
            })?
        {
            let start_loc=Loc::one();
            let end_loc=Loc::one();
            tokens.push(Token{ start_loc, end_loc, token_type, });
            continue;

        } else if let Some((token_type,p))=tokenise_string(&mut input)
            .map_err(|(error_type,p)|{
                CompileError{src:Some(src),path,error_type,loc:cur_loc,}
            })?
        {
            let start_loc=Loc::one();
            let end_loc=Loc::one();
            tokens.push(Token{ start_loc, end_loc, token_type, });
            continue;
        // } else if let Some((token_type,p))=tokenise_keyword(&mut input) {
        //     let start_loc=Loc::one();
        //     let end_loc=Loc::one();
        //     tokens.push(Token{ start_loc, end_loc, token_type, });
        //     continue;
        // } else if let Some((token_type,p))=tokenise_identifier(&mut input) {
        //     let start_loc=Loc::one();
        //     let end_loc=Loc::one();
        //     tokens.push(Token{ start_loc, end_loc, token_type, });
        //     continue;
        } else if let Some((token_type,p))=tokenise_word(&mut input) {
            let start_loc=Loc::one();
            let end_loc=Loc::one();
            tokens.push(Token{ start_loc, end_loc, token_type, });
            continue;
        } else if let Some((token_type,p))=tokenise_symbol(&mut input) {
            let start_loc=Loc::one();
            let end_loc=Loc::one();
            tokens.push(Token{ start_loc, end_loc, token_type, });
            continue;
        }

        break;
    }


    Ok(tokens)
}