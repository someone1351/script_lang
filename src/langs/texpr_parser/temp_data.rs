
use super::super::super::build::Loc;

pub struct ParsedWork<'a> {
    pub cur_temp_block:&'a TempBlock,
    pub cur_block_ind:usize,
}


pub enum TempWork {
    Block{block : TempBlock, block_start_loc : Loc, field_start_loc : Option<Loc>,bracket:Option<BlockBracket>},
    Param{param:TempParam,
        // start_loc : Loc,
    }

}


pub enum TempPrimitiveType {
    Block(TempBlock),
    Float(f64,usize,bool), //num,text_ind,has_prefix
    Int(i64,usize,bool),//num,text_ind,has_prefix
    String(usize),
    Symbol(usize),
}

pub struct TempPrimitive {
    pub primitive_type:TempPrimitiveType,
    pub start_loc : Loc,
    pub end_loc : Loc,
}

pub struct TempField {
    pub primitive : TempPrimitive,
    pub start_loc : Loc, //the dot
}

pub struct TempParam {
    pub primitive:TempPrimitive,
    // pub start_loc : Loc,
    // pub end_loc : Loc,
    pub fields : Vec<TempField>,
}

impl TempParam {

    pub fn start_loc(&self) -> Loc {
        self.primitive.start_loc
    }
    pub fn end_loc(&self) -> Loc {
        self.fields.last().map(|f|f.primitive.end_loc).unwrap_or(self.primitive.end_loc)
    }
}

pub struct TempRecord {
    pub params:Vec<TempParam>,
    pub ended:bool,
    pub semi_colon_end_loc:Option<Loc>,
}

pub struct TempBlock {
    pub records : Vec<TempRecord>,
    // pub end_loc : Loc, // from closing brace
}

#[derive(Clone, Copy, PartialEq,Eq)]
pub enum BlockBracket {
    Curly,Parentheses,Square,
}