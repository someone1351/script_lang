
use crate::build::Loc;
use crate::clike::compiler::rules::GrammarPrimitive;
use crate::clike::grammar::TokenTrait;
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
        let token=self.primitive();
        let text=token.text_ind.map(|x|self.parsed.texts[x].as_str());

        match token.token_type {
            TokenType::Float(x, ) => TokenTypeContainer::Float(x),
            TokenType::Int(x, ) => TokenTypeContainer::Int(x),
            TokenType::String => TokenTypeContainer::String(text.unwrap()),
            TokenType::Symbol => TokenTypeContainer::Symbol(text.unwrap()),
            TokenType::Identifier => TokenTypeContainer::Identifier(text.unwrap()),
            TokenType::Keyword => TokenTypeContainer::Keyword(text.unwrap()),
            // PrimitiveType::End => PrimitiveTypeContainer::End,
            TokenType::Eol => TokenTypeContainer::Eol,
        }
    }

    pub fn get_float(&self) -> Result<ValueContainer<'a,f64>,Loc> {
        if let TokenType::Float(value)=self.primitive().token_type {
            Ok(ValueContainer{ token: self.clone(), value })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_int(&self) -> Result<ValueContainer<'a,i64>,Loc> {
        if let TokenType::Int(value)=self.primitive().token_type {
            Ok(ValueContainer{ token: self.clone(), value })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_string(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        let token=self.primitive();

        if let TokenType::String=token.token_type {
            let value=self.parsed.texts[token.text_ind.unwrap()].as_str();
            Ok(ValueContainer{ token: self.clone(), value,  })
        } else {
            Err(self.start_loc())
        }
    }

    pub fn get_symbol(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        let token=self.primitive();

        if let TokenType::Symbol=token.token_type {
            let value=self.parsed.texts[token.text_ind.unwrap()].as_str();
            Ok(ValueContainer{ token: self.clone(), value, })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_identifier(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        let token=self.primitive();

        if let TokenType::Identifier=token.token_type {
            let value=self.parsed.texts[token.text_ind.unwrap()].as_str();
            Ok(ValueContainer{ token: self.clone(), value, })
        } else {
            Err(self.start_loc())
        }
    }

    pub fn get_keyword(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        let token=self.primitive();

        if let TokenType::Keyword=token.token_type {
            let value=self.parsed.texts[token.text_ind.unwrap()].as_str();
            Ok(ValueContainer{ token: self.clone(), value, })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_eol(&self) -> Result<ValueContainer<'a,()>,Loc> {
        let token=self.primitive();

        if let TokenType::Eol=token.token_type {
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
        if let TokenType::String=self.primitive().token_type {
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
        if let TokenType::Identifier=self.primitive().token_type {
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

    pub fn prev(&self,) ->Option<Self> {
        if self.token_ind==0 {
            None
        } else {
            Some(Self{ parsed: self.parsed, token_ind: self.token_ind-1 })
        }
    }
    pub fn prevs(&self,) ->TokenIterContainer<'a> {
        TokenIterContainer{ start: 0, end: self.token_ind, last_loc: Loc::zero(), parsed: self.parsed }
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

impl<'t,'g> TokenTrait<GrammarPrimitive<'g>> for TokenContainer<'t> {
    fn index(&self) -> usize {
        self.ind()
    }
}
