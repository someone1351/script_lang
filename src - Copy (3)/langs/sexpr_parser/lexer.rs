
use super::super::super::common::Loc;

use super::error::*;
use super::input::Input;



#[derive(Debug,Clone,PartialEq)]
pub enum TokenBracketType {
    Brace,Curly,Square,
}

#[derive(Debug,Clone)]
pub enum TokenType {
    Symbol(String),
    String(String),
    Float(String),
    Int(String),
    // Bool(bool),
    LBracket(TokenBracketType),
    RBracket(TokenBracketType),
}

#[derive(Debug,Clone)]
pub struct Token {
    pub start_loc : Loc,
    pub end_loc : Loc,
    pub token_type : TokenType,
}


pub struct Lexer<'a> {
    input : Input<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(chrs :std::str::Chars<'a>) -> Self {
        Self {
            input : Input::new(chrs),
        }
    }

    fn whitespace(&mut self) -> bool {
        let mut b=false;

        while let Some(x)=self.input.has(0, [" ","\t","\n","\r\n",";"]) {
            b=true;
            self.input.next(x.chars().count());

            if x==";" {
                while self.input.hasnt(0, ["\n","\r\n"]) {
                    self.input.next(1);
                }
            }
        }

        b
    }

    fn bracket(&mut self) -> Result<Option<Token>,ParserError> {
        if let Some(x)=self.input.has(0, ["(",")","[","]","{","}"]) { //brace/bracket
            let start_loc = self.input.loc();
            self.input.next(x.chars().count());
            let end_loc = self.input.loc();

            let token=match x {
                "(" => Token{start_loc,end_loc,token_type:TokenType::LBracket(TokenBracketType::Brace)},
                ")" => Token{start_loc,end_loc,token_type:TokenType::RBracket(TokenBracketType::Brace)},
                "[" => Token{start_loc,end_loc,token_type:TokenType::LBracket(TokenBracketType::Square)},
                "]" => Token{start_loc,end_loc,token_type:TokenType::RBracket(TokenBracketType::Square)},
                "{" => Token{start_loc,end_loc,token_type:TokenType::LBracket(TokenBracketType::Curly)},
                _ => Token{start_loc,end_loc,token_type:TokenType::RBracket(TokenBracketType::Curly)},
            };

            return Ok(Some(token));
        }

        return Ok(None);
    }

    fn string(&mut self) -> Result<Option<Token>,ParserError> {

        // [\][s] => ' '
        // [\][t] => '\t'
        // [\][r] => '\r'
        // [\][n] => '\n'
        // [\][\n] => ' ' 
        // [\][\r][\n] =>' '
        // [\]. => '.' 
        // 

        let quote_start_loc = self.input.loc();

        if let Some('"')=self.input.getc(0) {
            self.input.next(1);                    
        } else {
            return Ok(None);
        }

        let start_loc = self.input.loc();
        let mut s = String::new();

        loop {
            if let Some(x)=self.input.has(0, ["\\b","\\s","\\t","\\r","\\n","\\\r\n","\\\n","\\"]) {
                self.input.next(x.chars().count());

                if x=="\\" && self.input.is_end() {
                    return Err(ParserError { loc: self.input.loc(), error_type: ParserErrorType::ExpectedEscapedChar });
                }

                //only escape b,s,t,n,rn,spc n,spc rn

                let c=match x {
                    "\\b" => '\u{8}',
                    
                    "\\s" => ' ',
                    "\\t" => '\t',
                    "\\r" => '\r',
                    "\\n" => '\n',
                    "\\\r\n" => ' ',
                    "\\\n" => ' ',
                    
                    _=> '\\', //self.input.getc(0).unwrap()
                };

                // if x=="\\" {
                //     self.input.next(1);
                //     // s.push('\\');
                // }

                s.push(c);
            } else if let Some('"')=self.input.getc(0) {
                break;
            } else if let Some(x)=self.input.getc(0) {
                s.push(x);
                self.input.next(1);

            } else {
                break;
            }
        }

// println!("{}",s);

        if let Some('"')=self.input.getc(0) {
            let end_loc = self.input.loc();
            self.input.next(1);
            
            Ok(Some(Token{start_loc,end_loc,token_type:TokenType::String(s)}))
        } else {
            Err(ParserError { loc: quote_start_loc, error_type: ParserErrorType::ExpectedClosingDoubleQuote })
        }

    }

    fn symbol(&mut self) -> usize { //Result<Option<Token>,ParseError>
        //[^ \t\r\n"`',;()[]{}]+
        let mut i=0;

        while self.input.hasnt(i, [" ","\t","\r","\n","\"","`","'",",",";","(",")","[","]","{","}"]) {
            if self.input.getc(i)==Some('\\') {
                if //let Some(x)=
                    self.input.getc(i+1)
                        .is_some()
                {
                    // match x {
                    //     ' '
                    //     _=>{i+=2}
                    // }
                    i+=2;
                } else {
                    i+=1;
                }
            } else {
                i+=1;

            }
            // i+=1;
        }

        // if i>0 {
        //     let symbol = self.input.get(0, i).unwrap().to_string();
        //     let start_loc = self.input.loc();
        //     self.input.next(i);
        //     let end_loc = self.input.loc();
            
        //     Ok(Some(Token{start_loc,end_loc,token_type:TokenType::Symbol(symbol)}))
        // } else {
        //     Ok(None)
        // }

        i
    }

    // fn number(&mut self) -> Result<Option<Token>,ParseError> {
    // }

    fn number(&mut self) -> (char,usize) { //Result<Option<Token>,ParseError>
        //[+-]? ( [0-9]+ ([.][0-9]*)? | [.][0-9]+ )

        // let start_loc = self.input.loc();
        let mut i=0;

        // let mut has_sign=false;

        if "+-".contains(self.input.getc(i).unwrap_or(' ')) {
            i+=1;
            // has_sign=true;
        }

        let mut first=false;

        while self.input.getc(i).unwrap_or(' ').is_ascii_digit() {
            first=true;
            i+=1;
        }

        if Some('.')==self.input.getc(i) {
            i+=1;
            let mut second=false;

            while self.input.getc(i).unwrap_or(' ').is_ascii_digit() {
                i+=1;
                second=true;
            }

            if first||second { //return float
                // let s = self.input.get(0, i).unwrap().to_string();
                // self.input.next(i);
                // let end_loc = self.input.loc();
                // Ok(Some(Token{start_loc,end_loc,token_type:TokenType::Float(s)}))
                // println!("a");
                ('f',i)
            } else { //fail
                // Ok(None)
                // println!("b");
                ('_',i)
            }
        } else if first { //return int
            // let s = self.input.get(0, i).unwrap().to_string();
            // self.input.next(i);
            // let end_loc = self.input.loc();
            // Ok(Some(Token{start_loc,end_loc,token_type:TokenType::Int(s)}))
            // println!("c");
                ('i',i)
        } else { //fail
            // Ok(None)
            // println!("d");
            ('_',i)
        }
    }

    // fn bool(&mut self) -> usize {
    //     if let Some(x)=self.input.has(0, ["true","false"]) {
    //         x.len()
    //     } e
    //     if "+-".contains(self.input.getc(i).unwrap_or(' ')) {
    // }

    pub fn next(&mut self) -> Result<Option<Token>,ParserError> {
        // println!("{:?}","001".parse::<IntType>());

        if self.whitespace() {
            // self.sep_prev=true;
        }

        if let Some(token) = self.bracket()? {            
            // self.sep_prev=true;
            return Ok(Some(token));
        }

        if let Some(token) = self.string()? {            
            // self.sep_prev=true;
            return Ok(Some(token));
        }

        // if self.sep_prev {
        //     if let Some(token) = self.number()? {
        //         self.sep_prev=false;
        //         return Ok(Some(token));
        //     }

        //     if let Some(token) = self.symbol()? {
        //         self.sep_prev=false;
        //         return Ok(Some(token));
        //     }
        // }

        let symbol_len = self.symbol();
        let (num_type,num_len) = self.number();
        let i = symbol_len.max(num_len);

        if i >0 {
            
            let start_loc = self.input.loc();
            let s = self.input.get(0, i).unwrap().to_string();
            self.input.next(i);
            let end_loc = self.input.loc();

            if num_len<symbol_len {
                Ok(Some(Token{start_loc,end_loc,token_type:TokenType::Symbol(s)}))
            } else {
                if num_type=='f' {
                    Ok(Some(Token{start_loc,end_loc,token_type:TokenType::Float(s)}))
        
                } else if num_type =='i' {
                    Ok(Some(Token{start_loc,end_loc,token_type:TokenType::Int(s)}))
                } else {

                    Ok(Some(Token{start_loc,end_loc,token_type:TokenType::Symbol(s)}))
                }
            }
        } else {

            if self.input.is_end() {
                Ok(None)
            } else {
                Err(ParserError {loc : self.input.loc(), error_type : ParserErrorType::FailedToLexToken})
            }
        }


    }
}

