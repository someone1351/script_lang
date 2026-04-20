
use crate::build::Loc;
use super::super::super::tokenizer::data::{Parsed, Primitive, PrimitiveType};

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
    pub fn ind(&self) -> usize {
        self.primitive_ind
    }

    pub fn start_loc(&self) -> Loc {
        self.primitive().start_loc
    }
    pub fn end_loc(&self) -> Loc {
        self.primitive().end_loc
    }

    pub fn primitive_type(&self) -> PrimitiveTypeContainer<'a> {
        match self.primitive().primitive_type {
            PrimitiveType::Float(x, _) => PrimitiveTypeContainer::Float(x),
            PrimitiveType::Int(x, _) => PrimitiveTypeContainer::Int(x),
            PrimitiveType::String(x) => PrimitiveTypeContainer::String(self.parsed.texts[x].as_str()),
            PrimitiveType::Symbol(x) => PrimitiveTypeContainer::Symbol(self.parsed.texts[x].as_str()),
            PrimitiveType::Identifier(x) => PrimitiveTypeContainer::Identifier(self.parsed.texts[x].as_str()),
            // PrimitiveType::End => PrimitiveTypeContainer::End,
            PrimitiveType::Eol => PrimitiveTypeContainer::Eol,
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
    pub fn get_eol(&self) -> Result<ValueContainer<'a,()>,Loc> {
        if let PrimitiveType::Eol=self.primitive().primitive_type {
            Ok(ValueContainer{ primitive: self.clone(), value: () })
        } else {
            Err(self.start_loc())
        }
    }

    pub fn has_identifiers<'b,I>(&self,idns:I) -> Result<ValueContainer<'a,&'a str>,Loc>
    where
        I:IntoIterator<Item = &'b str>,
    {
        let g=self.get_identifier()?;

        for idn in idns.into_iter() {
            if idn.eq(g.value) {
                return Ok(g);
            }
        }

        Err(self.start_loc())
    }


    pub fn has_symbols<'b,I>(&self,symbols:I) -> Result<ValueContainer<'a,&'a str>,Loc>
    where
        I:IntoIterator<Item = &'b str>,
    {
        let g=self.get_symbol()?;

        for idn in symbols.into_iter() {
            if idn.eq(g.value) {
                return Ok(g);
            }
        }

        Err(self.start_loc())
    }

    pub fn is_eol(&self) -> bool {
        match self.primitive().primitive_type {
            PrimitiveType::Eol => true,
            _ => false,

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
}


impl<'a> std::fmt::Debug for PrimitiveContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::{:?}", &self.primitive_ind,&self.primitive_type()))
        // f.wr
        // f.debug_struct("Primitive")
        // // .field("parsed", &self.parsed)
        // .field("primitive_ind", &self.primitive_ind)
        // .field("loc", &self.start_loc())
        // .field("primitive_type", &format!("{:?}",self.primitive_type()))
        // .finish()
    }
}