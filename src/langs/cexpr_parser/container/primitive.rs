
use crate::{cexpr_parser::data::{Parsed, Primitive, PrimitiveType}, Loc};

use super::*;



#[derive(Clone,Copy)]
pub struct PrimitiveContainer<'a> {
    pub parsed:&'a Parsed,
    pub primitive_ind:usize,
    // pub last_loc:Loc,
}



impl<'a> PrimitiveContainer<'a> {
    fn primitive(&self) -> &'a Primitive {
        &self.parsed.primitives[self.primitive_ind]
    }

    pub fn start_loc(&self) -> Loc {
        self.primitive().start_loc
    }
    pub fn end_loc(&self) -> Loc {
        self.primitive().end_loc
    }

    pub fn primitive_type(&self) -> PrimitiveTypeContainer<'a> {
        match self.primitive().primitive_type {
            PrimitiveType::Root(_) => panic!("shouldn't be able to get"),
            PrimitiveType::CurlyBlock(_) => PrimitiveTypeContainer::CurlyBlock(self.get_block().unwrap()),
            PrimitiveType::SquareBlock(_) => PrimitiveTypeContainer::SquareBlock(self.get_block().unwrap()),
            PrimitiveType::ParenthesesBlock(_) => PrimitiveTypeContainer::ParenthesesBlock(self.get_block().unwrap()),
            PrimitiveType::Float(x, _) => PrimitiveTypeContainer::Float(x),
            PrimitiveType::Int(x, _) => PrimitiveTypeContainer::Int(x),
            PrimitiveType::String(x) => PrimitiveTypeContainer::String(self.parsed.texts[x].as_str()),
            PrimitiveType::Symbol(x) => PrimitiveTypeContainer::Symbol(self.parsed.texts[x].as_str()),
            PrimitiveType::Identifier(x) => PrimitiveTypeContainer::Symbol(self.parsed.texts[x].as_str()),
            // PrimitiveType::End => PrimitiveTypeContainer::End,
            PrimitiveType::Eol => PrimitiveTypeContainer::Eol,
            PrimitiveType::Eob => PrimitiveTypeContainer::Eob,
        }
    }

    pub fn get_float(&self) -> Result<ValueContainer<'a,f64>,Loc> {
        if let PrimitiveType::Float(value, _)=self.primitive().primitive_type {
            Ok(ValueContainer{ primitive: self.clone(), value })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_int(&self) -> Result<ValueContainer<'a,i64>,Loc> {
        if let PrimitiveType::Int(value, _)=self.primitive().primitive_type {
            Ok(ValueContainer{ primitive: self.clone(), value })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_string(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        if let PrimitiveType::String(x)=self.primitive().primitive_type {
            Ok(ValueContainer{ primitive: self.clone(), value: self.parsed.texts[x].as_str() })
        } else {
            Err(self.start_loc())
        }
    }

    pub fn get_symbol(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        if let PrimitiveType::Symbol(x)=self.primitive().primitive_type {
            Ok(ValueContainer{ primitive: self.clone(), value: self.parsed.texts[x].as_str() })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn get_identifier(&self) -> Result<ValueContainer<'a,&'a str>,Loc> {
        if let PrimitiveType::Identifier(x)=self.primitive().primitive_type {
            Ok(ValueContainer{ primitive: self.clone(), value: self.parsed.texts[x].as_str() })
        } else {
            Err(self.start_loc())
        }
    }
    pub fn identifier_eq(&self,idn:&str) -> Result<(),Loc> {
        if idn == self.get_identifier()?.value {
            Ok(())
        } else {
            Err(self.start_loc())
        }

    }
    // pub fn get_end(&self) -> bool {
    //     if let PrimitiveType::End=self.primitive().primitive_type {
    //         true
    //     } else {
    //         false
    //     }
    // }

    pub fn get_block(&self) -> Option<BlockContainer<'a>> {
        match self.primitive().primitive_type {
            PrimitiveType::Root(_)| //primtive not provided for root?
            PrimitiveType::CurlyBlock(_)|
            PrimitiveType::SquareBlock(_)|
            PrimitiveType::ParenthesesBlock(_)
            => Some(BlockContainer{ parsed: self.parsed, primitive_ind: self.primitive_ind }),
            _ => None,
        }
    }

    pub fn get_curly(&self) -> Result<BlockContainer<'a>,Loc> {
        if let PrimitiveType::CurlyBlock(_)=self.primitive().primitive_type {
            Ok(BlockContainer{ parsed: self.parsed, primitive_ind: self.primitive_ind })
        } else {
            Err(self.start_loc())
        }
    }

    pub fn get_parenthesis(&self) -> Result<BlockContainer<'a>,Loc> {
        if let PrimitiveType::ParenthesesBlock(_)=self.primitive().primitive_type {
            Ok(BlockContainer{ parsed: self.parsed, primitive_ind: self.primitive_ind })
        } else {
            Err(self.start_loc())
        }
    }

    pub fn get_square(&self) -> Result<BlockContainer<'a>,Loc> {
        if let PrimitiveType::SquareBlock(_)=self.primitive().primitive_type {
            Ok(BlockContainer{ parsed: self.parsed, primitive_ind: self.primitive_ind })
        } else {
            Err(self.start_loc())
        }
    }

    pub fn is_eol(&self) -> bool {
        if let PrimitiveType::Eol=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_eob(&self) -> bool {
        if let PrimitiveType::Eob=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_end(&self) -> bool {
        match self.primitive().primitive_type {
            PrimitiveType::Eob => true,
            PrimitiveType::Eol => true,
            _ => false,

        }
    }

    pub fn is_parentheses(&self) -> bool {
        if let PrimitiveType::ParenthesesBlock(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_square(&self) -> bool {
        if let PrimitiveType::SquareBlock(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }

    pub fn is_curly(&self) -> bool {
        if let PrimitiveType::CurlyBlock(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let PrimitiveType::String(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }

    pub fn is_symbol(&self) -> bool {
        if let PrimitiveType::Symbol(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_identifier(&self) -> bool {
        if let PrimitiveType::Identifier(_)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_float(&self) -> bool {
        if let PrimitiveType::Float(..)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    pub fn is_int(&self) -> bool {
        if let PrimitiveType::Int(..)=self.primitive().primitive_type {
            true
        } else {
            false
        }
    }
    // pub fn expect_int(&self) -> Result<i64,Loc> {
    //     self.get_int().ok_or_else(||self.start_loc())
    // }
    // pub fn expect_float(&self) -> Result<f64,Loc> {
    //     self.get_float().ok_or_else(||self.start_loc())
    // }
    // pub fn expect_string(&self) -> Result<&'a str,Loc> {
    //     self.get_string().ok_or_else(||self.start_loc())
    // }
    // pub fn expect_symbol(&self) -> Result<&'a str,Loc> {
    //     self.get_symbol().ok_or_else(||self.start_loc())
    // }
    // pub fn expect_identifier(&self) -> Result<&'a str,Loc> {
    //     self.get_identifier().ok_or_else(||self.start_loc())
    // }
    // pub fn expect_curly(&self) -> Result<BlockContainer<'a>,Loc> {
    //     self.get_curly().ok_or_else(||self.start_loc())
    // }
    // pub fn expect_parenthesis(&self) -> Result<BlockContainer<'a>,Loc> {
    //     self.get_parenthesis().ok_or_else(||self.start_loc())
    // }
    // pub fn expect_square(&self) -> Result<BlockContainer<'a>,Loc> {
    //     self.get_square().ok_or_else(||self.start_loc())
    // }
}

