
use crate::build::Loc;
use super::super::super::tokenizer::data::{Tokenized, Token, TokenType};

use super::*;



#[derive(Clone,Copy)]
pub struct TokenContainer<'a> {
    pub parsed:&'a Tokenized,
    pub token_ind:usize,
    // pub last_loc:Loc,
}

impl<'a> TokenContainer<'a> {
    fn primitive(&self) -> &'a Token {
        &self.parsed.primitives[self.token_ind]
    }
    pub fn ind(&self) -> usize {
        self.token_ind
    }

    pub fn start_loc(&self) -> Loc {
        self.primitive().start_loc
    }
    pub fn end_loc(&self) -> Loc {
        self.primitive().end_loc
    }

    pub fn token_type(&self) -> TokenTypeContainer<'a> {
        match self.primitive().token_type {
            TokenType::Float(x, _) => TokenTypeContainer::Float(x),
            TokenType::Int(x, _) => TokenTypeContainer::Int(x),
            TokenType::String(x) => TokenTypeContainer::String(self.parsed.texts[x].as_str()),
            TokenType::Symbol(x) => TokenTypeContainer::Symbol(self.parsed.texts[x].as_str()),
            TokenType::Identifier(x) => TokenTypeContainer::Identifier(self.parsed.texts[x].as_str()),
            TokenType::Keyword(x) => TokenTypeContainer::Keyword(self.parsed.texts[x].as_str()),
            // PrimitiveType::End => PrimitiveTypeContainer::End,
            TokenType::Eol => TokenTypeContainer::Eol,
        }
    }

    pub fn get_float(&self) -> Result<ValueContainer<'a,f64>,Loc> {
        if let TokenType::Float(value, _)=self.primitive().token_type {
            Ok(ValueContainer{ token: self.clone(), value })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_int(&self) -> Result<ValueContainer<'a,i64>,Loc> {
        if let TokenType::Int(value, _)=self.primitive().token_type {
            Ok(ValueContainer{ token: self.clone(), value })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_string(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        if let TokenType::String(x)=self.primitive().token_type {
            Ok(ValueContainer{ token: self.clone(), value: self.parsed.texts[x].as_str() })
        } else {
            Err(self.start_loc())
        }
    }

    pub fn get_symbol(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        if let TokenType::Symbol(x)=self.primitive().token_type {
            Ok(ValueContainer{ token: self.clone(), value: self.parsed.texts[x].as_str() })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_identifier(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        if let TokenType::Identifier(x)=self.primitive().token_type {
            Ok(ValueContainer{ token: self.clone(), value: self.parsed.texts[x].as_str() })
        } else {
            Err(self.start_loc())
        }
    }

    pub fn get_keyword(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        if let TokenType::Keyword(x)=self.primitive().token_type {
            Ok(ValueContainer{ token: self.clone(), value: self.parsed.texts[x].as_str() })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_eol(&self) -> Result<ValueContainer<'a,()>,Loc> {
        if let TokenType::Eol=self.primitive().token_type {
            Ok(ValueContainer{ token: self.clone(), value: () })
        } else {
            Err(self.start_loc())
        }
    }

    // pub fn has_identifiers<'b,I>(&self,idns:I) -> Result<ValueContainer<'a,&'a str>,Loc>
    // where
    //     I:IntoIterator<Item = &'b str>,
    // {
    //     let g=self.get_identifier()?;

    //     for idn in idns.into_iter() {
    //         if idn.eq(g.value) {
    //             return Ok(g);
    //         }
    //     }

    //     Err(self.start_loc())
    // }


    // pub fn has_symbols<'b,I>(&self,symbols:I) -> Result<ValueContainer<'a,&'a str>,Loc>
    // where
    //     I:IntoIterator<Item = &'b str>,
    // {
    //     let g=self.get_symbol()?;

    //     for idn in symbols.into_iter() {
    //         if idn.eq(g.value) {
    //             return Ok(g);
    //         }
    //     }

    //     Err(self.start_loc())
    // }

    pub fn is_eol(&self) -> bool {
        match self.primitive().token_type {
            TokenType::Eol => true,
            _ => false,

        }
    }

    pub fn is_string(&self) -> bool {
        if let TokenType::String(_)=self.primitive().token_type {
            true
        } else {
            false
        }
    }

    pub fn is_symbol(&self,) -> bool {
        if let TokenTypeContainer::Symbol(_)=self.token_type() {
            true
        } else {
            false
        }
    }

    pub fn is_keyword(&self, ) -> bool {
        if let TokenTypeContainer::Keyword(_)=self.token_type() {
            true
        } else {
            false
        }
    }
    pub fn is_identifier(&self) -> bool {
        if let TokenType::Identifier(_)=self.primitive().token_type {
            true
        } else {
            false
        }
    }
    pub fn is_float(&self) -> bool {
        if let TokenType::Float(..)=self.primitive().token_type {
            true
        } else {
            false
        }
    }
    pub fn is_int(&self) -> bool {
        if let TokenType::Int(..)=self.primitive().token_type {
            true
        } else {
            false
        }
    }

    pub fn has_symbol(&self,symbol:& str) -> Result<ValueContainer<'a,&'a str>,Loc> {
        if let TokenTypeContainer::Symbol(s)=self.token_type() {
            if symbol==s {
                return self.get_symbol();
            }
        }

        Err(self.start_loc())
    }

    pub fn has_keyword(&self, keyword:&str) -> Result<ValueContainer<'a,&'a str>,Loc> {
        if let TokenTypeContainer::Keyword(k)=self.token_type() {
            if k==keyword {
                return self.get_keyword();
            }
        }

        Err(self.start_loc())
    }
}


impl<'a> std::fmt::Debug for TokenContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::{:?}", &self.token_ind,&self.token_type()))
        // f.wr
        // f.debug_struct("Primitive")
        // // .field("parsed", &self.parsed)
        // .field("primitive_ind", &self.primitive_ind)
        // .field("loc", &self.start_loc())
        // .field("primitive_type", &format!("{:?}",self.primitive_type()))
        // .finish()
    }
}