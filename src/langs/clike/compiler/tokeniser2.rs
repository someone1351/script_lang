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
    cur_pos:usize,
    // end_pos_rewind:usize,
    // prev_end_pos:usize,

    start_loc: Loc,
    cur_loc: Loc,
}
impl<'t> Input2<'t> {
    pub fn new(data:&'t str) -> Self {
        Self {
            data, start_pos:0, cur_pos:0,
            start_loc:Loc::one(),
            cur_loc:Loc::one(),
            // prev_end_pos:0,
            // end_pos_rewind:0,
        }
    }
    pub fn start_loc(&self) -> Loc {
        self.start_loc
    }
    pub fn cur_loc(&mut self) -> Loc {
         if self.cur_pos != self.cur_loc.byte_pos {
            self.cur_loc.calc(&self.data[self.cur_loc.byte_pos..self.cur_pos]);
        }

        self.cur_loc
    }
    pub fn slice(&self) -> &'t str {
        &self.data[self.start_pos..self.cur_pos]
    }
    pub fn rest(&self) -> &'t str {
        &self.data[self.cur_pos..]
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
        self.start_pos..self.cur_pos
    }
    pub fn is_eof(&self) -> bool {
        self.cur_pos >= self.data.len()
    }

    pub fn next(&mut self) {
        let cur_loc = self.cur_loc();
        self.start_pos = self.cur_pos;
        self.start_loc = cur_loc;
    }

    pub fn undo(&mut self) {
        self.cur_pos=self.start_pos;
        self.cur_loc = self.start_loc;
    }


    pub fn take_range(&mut self, r:RangeInclusive<char>) -> Option<char> {
        let Some(x) = self.rest().chars().next() else {return None;};
        if !r.contains(&x) {return None;}
        self.cur_pos+=x.len_utf8();
        Some(x)
    }

    pub fn take_one_of(&mut self, cs:&str) -> Option<char> {
        let Some(x) = self.rest().chars().next() else {return None;};
        if !cs.contains(x) {return None;}
        self.cur_pos+=x.len_utf8();
        Some(x)
    }

    pub fn take_char(&mut self, c:char) -> Option<char> {
        if Some(c)!=self.rest().chars().next() {return None;};
        self.cur_pos+=c.len_utf8();
        Some(c)
    }
    pub fn take_any(&mut self,) -> Option<char> {
        let Some(x) = self.rest().chars().next() else {return None;};
        self.cur_pos+=x.len_utf8();
        Some(x)
    }
    pub fn take_str(&mut self, s:&str) -> Option<&'t str> {
        let p=self.cur_pos;

        if !self.rest().starts_with(s) {return None;}
        self.cur_pos+=s.len();

        Some(&self.data[p..self.cur_pos])
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

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            // 3-character
            "..." => Some(Self::Variadic),
            "..=" => Some(Self::ForInclusiveRange),
            "&&=" => Some(Self::SetAnd),
            "||=" => Some(Self::SetOr),

            // 2-character
            ".." => Some(Self::ForExclusiveRange),
            "==" => Some(Self::Equal),
            "!=" => Some(Self::NotEqual),
            "<=" => Some(Self::LessThanEqual),
            ">=" => Some(Self::GreaterThanEqual),
            "&&" => Some(Self::And),
            "||" => Some(Self::Or),
            "+=" => Some(Self::SetAdd),
            "-=" => Some(Self::SetSub),
            "*=" => Some(Self::SetMul),
            "/=" => Some(Self::SetDiv),
            "%=" => Some(Self::SetMod),
            "^=" => Some(Self::SetBitXor),
            "&=" => Some(Self::SetBitAnd),
            "|=" => Some(Self::SetBitOr),

            // 1-character
            "&" => Some(Self::BitAnd),
            "|" => Some(Self::BitOr),
            "^" => Some(Self::BitXor),
            "~" => Some(Self::BitNot),
            "!" => Some(Self::Not),
            "=" => Some(Self::Set),
            "," => Some(Self::Comma),
            "%" => Some(Self::Mod),
            ";" => Some(Self::SemiColon),
            "." => Some(Self::Dot),
            ":" => Some(Self::Colon),
            ">" => Some(Self::GreaterThan),
            "<" => Some(Self::LessThan),
            "+" => Some(Self::Add),
            "-" => Some(Self::Sub),
            "*" => Some(Self::Mul),
            "/" => Some(Self::Div),
            "(" => Some(Self::LParen),
            ")" => Some(Self::RParen),
            "[" => Some(Self::LBracket),
            "]" => Some(Self::RBracket),
            "{" => Some(Self::LBrace),
            "}" => Some(Self::RBrace),

            _ => None,
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
    Symbol(Symbol),
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

fn tokenise_num<'t>(input:&mut Input2<'t>) -> Result<Option<Token<'t>>,(CompileErrorType<'t>,Loc)> {
    //[0] | [0-9][1-9]*

    //
    if input.eat_range('1'..='9') {
        while input.eat_range('0'..='9') {}
    } else if input.eat_char('0') {
    } else {
        input.undo();
        return Ok(None);
    }

    //
    // input.mark(); //the int
    let int_input=input.clone();

    //
    if !input.eat_char('.') || !input.eat_range('0'..='9') {
        *input=int_input;
        // input.rewind(); //back to the int

        //int
        let val = input.slice().parse::<i64>().map_err(|e|(
            CompileErrorType::Tokeniser2(TokenizerErrorType2::Int(e)),
            input.start_loc(),
        ))?;

        let start_loc=input.start_loc();
        let end_loc=input.cur_loc();
        let token_type=TokenType::Int(val);

        input.next();
        return Ok(Some(Token { start_loc, end_loc, token_type, }));
    }

    //
    while input.eat_range('0'..='9') {}

    //float
    let val = input.slice().parse::<f64>().map_err(|e|(
        CompileErrorType::Tokeniser2(TokenizerErrorType2::Float(e)),
        input.start_loc(),
    ))?;

    let start_loc=input.start_loc();
    let end_loc=input.cur_loc();
    let token_type=TokenType::Float(val);

    input.next();
    return Ok(Some(Token { start_loc, end_loc, token_type, }));
}

fn tokenise_symbol<'t>(input: &mut Input2<'t>) -> Option<Token<'t>> {
    for len in (1..=3).rev() {
        if let Some(s) = input.peek(len) {
            if let Some(sym) = Symbol::from_str(s) {
                input.eat_str(s);

                let start_loc=input.start_loc();
                let end_loc=input.cur_loc();
                let token_type=TokenType::Symbol(sym);

                input.next();
                return Some(Token { start_loc, end_loc, token_type, });
            }
        }
    }
    None
}


fn tokenise_word<'t>(input: &mut Input2<'t>) -> Option<Token<'t>> {
    //[_]*[a-zA-Z][_a-zA-Z90-9]*

    while input.eat_char('_') {}

    if !input.eat_range('a'..='z') && !input.eat_range('A'..='Z') {
        // input.rewind();
        input.undo();
        return None;
    }

    while input.eat_range('a'..='z')
        || input.eat_range('A'..='Z')
        || input.eat_range('0'..='9')
        || input.eat_char('_')
    {}

    let slice = input.slice();
    let token_type = match Keyword::from_str(slice) {
        Some(kw) => TokenType::Keyword(kw),
        None => TokenType::Identifier(slice),
    };

    let start_loc=input.start_loc();
    let end_loc=input.cur_loc();

    input.next();
    Some(Token { start_loc, end_loc, token_type, })



}

fn tokenise_string<'t>(input:&mut Input2<'t>) -> Result<Option<Token<'t>>,(CompileErrorType<'t>,Loc)> {
    //["]([\\]([strn"\\\n][\r][\n])|[^"])*["]
    //
    if !input.eat_char('"') {
        input.undo();
        return Ok(None);
    }

    //

    let mut s=String::new();

    //
    loop {
        //
        // let p=input.pos_range();

        //
        if input.eat_char('"') {
            let start_loc=input.start_loc();
            let end_loc=input.cur_loc();
            let token_type=TokenType::String(s);

            input.next();
            return Ok(Some(Token { start_loc, end_loc, token_type, }));
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

    Err((CompileErrorType::Tokeniser2(TokenizerErrorType2::ClosingQuoteExpected),input.start_loc()))

}


fn tokenise_ws<'t>(input:&mut Input2<'t>) -> Result<Option<Token<'t>>,(CompileErrorType<'t>,Loc)> {
    // smlcmnt => [/][/][^\r\n]*([\n]|[\r][\n]|eof)
    // mlcmnt => [/][*] (mlcmnt|[^*]|[*][^/])* [*][/]
    // space => [ \t]
    // eol => [\r][\n]|[\n]

    let mut eol_start: Option<Loc>=None;
    let mut eol_end= Loc::zero();

    loop {
        //space
        if input.eat_one_of(" \t") {
            continue;
        }

        //eol
        {
            let new_start=eol_start.unwrap_or(input.cur_loc());

            if input.eat_char('\n') || input.eat_str("\r\n") {
                eol_start=Some(new_start);
                eol_end=input.cur_loc();
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
        let comment_start_loc = input.cur_loc();

        if input.eat_str("/*") {
            let mut depth=1;

            while depth>0 {
                if input.eat_str("/*") {
                    depth+=1;
                } else if input.eat_str("*/") {
                    depth-=1;
                } else if !input.eat_any() {
                    return Err((CompileErrorType::Tokeniser2(TokenizerErrorType2::ClosingCommentExpected),comment_start_loc));
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
        let start_loc=eol_start;
        let end_loc=eol_end;
        let token_type=TokenType::Eol;

        input.next();
        return Ok(Some(Token { start_loc, end_loc, token_type, }));
    }

    //
    input.next();
    Ok(None)
}

pub fn tokenise2<'p,'t,'g>(
    src:&'t str,
    path:Option<&'p Path>,

) -> Result<Vec<Token<'t>>, CompileError<'p,'t>>
{ //(Loc,&'g str)

    let mut input=Input2::new(src);
    let mut tokens=Vec::new();


    loop {

        if let Some(token)=tokenise_ws(&mut input)
            .map_err(|(error_type,loc)|{
                CompileError{src:Some(src),path,error_type,loc,}
            })?
        {
            tokens.push(token);
        }

        //
        if input.is_eof() {
            break;
        }

        //

        if let Some(token)=tokenise_num(&mut input)
            .map_err(|(error_type,loc)|{
                CompileError{src:Some(src),path,error_type,loc,}
            })?
        {
            tokens.push(token);
            continue;
        }

        //


        if let Some(token)=tokenise_string(&mut input)
            .map_err(|(error_type,loc)|{
                CompileError{src:Some(src),path,error_type,loc,}
            })?
        {
            tokens.push(token);
            continue;
        }

        //
        if let Some(token)=tokenise_word(&mut input) {
            tokens.push(token);
            continue;
        }

        //
        if let Some(token)=tokenise_symbol(&mut input) {
            tokens.push(token);
            continue;
        }

        //
        return Err(CompileError {
            src: Some(src),
            path,
            error_type: CompileErrorType::Tokeniser2(TokenizerErrorType2::Unexpected),
            loc:input.start_loc(),
        });
    }


    Ok(tokens)
}