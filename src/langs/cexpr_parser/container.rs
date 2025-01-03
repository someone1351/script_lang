
// use std::ops::Range;

/*
TODO
* make new record out of existing one by specifying param range eg param_start,param_end (make sure can accumulate eg record=>new_record1=>new_record2)
* take single record from existing block, and return in its own block
*/

use super::super::super::common::Loc;
use super::{ Block, Field, Param, Parsed, Primitive, PrimitiveType, Record };

#[derive(Copy,Clone)]
pub struct BlockContainer<'a> {
    pub parsed:&'a Parsed,
    pub block_ind:usize,
}

impl<'a> BlockContainer<'a> {
    fn block(&self) -> &Block {
        self.parsed.blocks.get(self.block_ind).unwrap()
    }
    pub fn records(&self) -> RecordIter<'a> {
        let block=self.block();
        RecordIter { start: block.records.start, end: block.records.end, parsed: self.parsed }
    }
    pub fn records_num(&self) -> usize {
        let block=self.block();
        block.records.len()
    }
    pub fn record(&self, ind:usize) -> Option<RecordContainer<'a>>{
        let block=self.block();
        let record_ind=block.records.start+ind;

        if record_ind<block.records.end {
            Some(RecordContainer {parsed:self.parsed,record_ind,})
        } else {
            None
        }
    }
    pub fn primitive(&self) -> PrimitiveContainer<'a> {
        PrimitiveContainer { parsed: self.parsed, primitive_ind:self.block().primitive, to_string:false, }
    }

    pub fn start_loc(&self) -> Loc {
        self.primitive().start_loc()
    }

    pub fn end_loc(&self) -> Loc {
        self.primitive().end_loc()
    }
    pub fn params(&self) -> ParamIter<'a> {
        let params=self.block().params.clone();
        ParamIter { start: params.start, end: params.end, parsed: self.parsed }
    }
    pub fn params_num(&self) -> usize {
        self.block().params.len()
    }
    pub fn param(&self, ind:usize) -> Option<ParamContainer<'a>> {
        let params=self.block().params.clone();
        let param_ind=params.start+ind;

        if param_ind<params.end {
            Some(ParamContainer { parsed: self.parsed, param_ind, })
        } else {
            None
        }
    }    
    pub fn first_param(&self) -> Option<ParamContainer<'a>> {
        self.param(0)
    }
    pub fn last_param(&self) -> Option<ParamContainer<'a>> {
        if self.params_num()==0 {
            None
        } else {
            self.param(self.params_num()-1)
        }
    } 
}

impl<'a> std::fmt::Debug for BlockContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // 

        for (i,record) in self.records().enumerate() 
        {
            if i>0 {
                write!(f,", ")?;
            }

            record.fmt(f)?;
        }
        // write!(f, "BlockContainer({})", self.block_ind)?;

        Ok(())
    }
}

#[derive(Copy,Clone)]
pub struct RecordContainer<'a> {
    pub parsed:&'a Parsed,
    pub record_ind:usize,
}

impl<'a> RecordContainer<'a> {
    fn record(&self) -> &Record {
        self.parsed.records.get(self.record_ind).unwrap()
    }    
    pub fn params(&self) -> ParamIter<'a> {
        let record=self.record();
        ParamIter { start: record.params.start, end: record.params.end, parsed: self.parsed }
    }
    pub fn params_num(&self) -> usize {
        let record=self.record();
        record.params.len()
    }
    pub fn param(&self, ind:usize) -> Option<ParamContainer<'a>> {
        let record=self.record();
        let param_ind=record.params.start+ind;

        if param_ind<record.params.end {
            Some(ParamContainer { parsed: self.parsed, param_ind, })
        } else {
            None
        }
    }
    pub fn first_param(&self) -> Option<ParamContainer<'a>> {
        self.param(0)
    }
    pub fn last_param(&self) -> Option<ParamContainer<'a>> {
        if self.params_num()==0 {
            None
        } else {
            self.param(self.params_num()-1)
        }
    }    
    pub fn start_loc(&self) -> Loc {
        self.first_param().map(|x|x.start_loc()).unwrap_or_else(||self.semi_colon_loc().unwrap()) //if no params, then will have a semi colon
    }

    pub fn end_loc(&self) -> Loc {
        self.last_param().map(|x|x.end_loc()).unwrap_or_else(||self.semi_colon_loc().unwrap()) //if no params, then will have a semi colon
    }

    pub fn semi_colon_loc(&self) -> Option<Loc> {
        let record=self.parsed.records.get(self.record_ind).unwrap();
        record.semi_colon_loc
    }
}

impl<'a> std::fmt::Debug for RecordContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Record(")?;
        for (i,param) in self.params().enumerate() {
            if i>0 {
                write!(f,",")?;
            }

            param.fmt(f)?;
        }
        write!(f,")")?;

        Ok(())
    }
}

#[derive(Copy,Clone)]
pub struct PrimitiveContainer<'a> {
    pub parsed:&'a Parsed,
    pub primitive_ind:usize,
    pub to_string : bool,
}

impl<'a> PrimitiveContainer<'a> {
    fn primitive(&self) -> &Primitive {
        self.parsed.primitives.get(self.primitive_ind).unwrap()
    }
    pub fn primitive_type(&self) -> PrimitiveTypeContainer<'a> {
        let primitive=self.primitive();
        if self.to_string {
            let s=match primitive.primitive_type.clone() {
                PrimitiveType::Block(_) => 0, //0 is an empty string
                PrimitiveType::Float(_,s,_)|PrimitiveType::Int(_,s,_)|PrimitiveType::String(s)|PrimitiveType::Symbol(s) => s,
            };

            PrimitiveTypeContainer::String(self.parsed.texts.get(s).unwrap().as_str())
        } else {

            match primitive.primitive_type.clone() {
                // x if self.to_string => 
                PrimitiveType::Block(block_ind) => PrimitiveTypeContainer::Block(BlockContainer {parsed:self.parsed,block_ind}),
                PrimitiveType::Float(f,_s,p) => PrimitiveTypeContainer::Float(f,p),
                PrimitiveType::Int(i,_s,p) => PrimitiveTypeContainer::Int(i,p),
                PrimitiveType::String(s) => PrimitiveTypeContainer::String(self.parsed.texts.get(s).unwrap().as_str()),
                PrimitiveType::Symbol(s) =>PrimitiveTypeContainer::Symbol(self.parsed.texts.get(s).unwrap().as_str()),
            }
        }
    }
    pub fn float(&self) -> Option<f64> {
        if let PrimitiveTypeContainer::Float(x,_)=self.primitive_type() {
            Some(x)
        } else {
            None
        }
    }
    pub fn int(&self) -> Option<i64> {
        if let PrimitiveTypeContainer::Int(x,_)=self.primitive_type() {
            Some(x)
        } else {
            None
        }
    }
    pub fn string(&self) -> Option<&'a str> {
        if let PrimitiveTypeContainer::String(s)=self.primitive_type() {
            Some(s)
        // } else if self.to_string {
        //     if let PrimitiveTypeContainer::Symbol(s)=self.primitive_type() {
        //         Some(s)
        //     } else {
        //         None
        //     }
        } else {
            None
        }
    }
    // pub fn make_string(&self) -> Option<Self> {
    //     if let PrimitiveTypeContainer::Symbol(_)=self.primitive_type() {
    //         let mut x = self.clone();
    //         x.to_string=true;
    //         Some(x)
    //     } else {
    //         None
    //     }
    // }
    // pub fn make_string(&self) -> Self {
    //     let mut x = self.clone();
    //     x.to_string=true;
    //     x
    // }
    pub fn symbol(&self) -> Option<&'a str> {
        if let PrimitiveTypeContainer::Symbol(s)=self.primitive_type() {
            Some(s)
        } else {
            None
        }
    }
    pub fn block(&self) -> Option<BlockContainer<'a>> {
        if let PrimitiveTypeContainer::Block(x)=self.primitive_type() {
            Some(x)
        } else {
            None
        }
    }
    pub fn param(&self) -> Option<ParamContainer<'a>> {
        self.primitive().param.map(|param_ind|ParamContainer{ parsed: self.parsed, param_ind })
    }
    pub fn field(&self) -> Option<FieldContainer<'a>> {
        self.primitive().field.map(|field_ind|FieldContainer{ parsed: self.parsed, field_ind })
    }
    pub fn start_loc(&self) -> Loc {
        self.primitive().start_loc
    }
    pub fn end_loc(&self) -> Loc {
        self.primitive().end_loc
    }
}

impl<'a> std::fmt::Debug for PrimitiveContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.primitive_type().fmt(f)
    }
}

#[derive(Copy,Clone)]
pub enum PrimitiveTypeContainer<'a> {
    Block(BlockContainer<'a>),
    Float(f64,bool),
    Int(i64,bool),
    String(&'a str),
    Symbol(&'a str),
}

impl<'a> std::fmt::Debug for PrimitiveTypeContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // PrimitiveTypeContainer::Block(b) => write!(f, "Block([{:?}])",b.block_ind),
            PrimitiveTypeContainer::Block(b) => write!(f, "Block({b:?})"),
            PrimitiveTypeContainer::Float(x,_) => write!(f, "Float({x})"),
            PrimitiveTypeContainer::Int(x,_) => write!(f, "Int({x})"),
            PrimitiveTypeContainer::String(x) => write!(f, "String({x:?})"),
            PrimitiveTypeContainer::Symbol(x) => write!(f, "Symbol({x})"),
        }
    }
}

#[derive(Copy,Clone)]
pub struct ParamContainer<'a> {
    pub parsed:&'a Parsed,
    pub param_ind:usize,
}

impl<'a> std::fmt::Debug for ParamContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Param({:?}",self.primitive())?;

        if self.fields_num()!=0 {
            write!(f, ", ")?;
            write!(f, "{}",self.fields().map(|x|format!("{x:?}")).collect::<Vec<_>>().join(", "))?;
        }

        write!(f, ")")?;

        Ok(())
    }
}

impl<'a> ParamContainer<'a> {
    fn param(&self) -> &Param {
      self.parsed.params.get(self.param_ind).unwrap()
    }
    pub fn primitive(&self)->PrimitiveContainer<'a> {
        let param=self.param();
        PrimitiveContainer{ parsed: self.parsed, primitive_ind:param.primitive, to_string:false, }
    }
    pub fn string_primitive(&self)->PrimitiveContainer<'a> {
        let param=self.param();
        PrimitiveContainer{ parsed: self.parsed, primitive_ind:param.primitive, to_string:true, }
    }
    pub fn fields_num(&self) -> usize {
        self.param().fields.len()
    }
    pub fn field(&self,ind:usize)->Option<FieldContainer<'a>> {
        let param=self.param();
        let field_ind=param.fields.start+ind;
        if field_ind>=param.fields.end {return None;}
        Some(FieldContainer{ parsed: self.parsed, field_ind })
    }
    pub fn fields(&self) -> FieldIter<'a> {
        let param=self.param();
        FieldIter { start: param.fields.start, end: param.fields.end, parsed: self.parsed } 
    }
    pub fn start_loc(&self) -> Loc {
        self.primitive().start_loc()
    }
    pub fn end_loc(&self) -> Loc {
        if self.fields_num()==0 {
            self.primitive().end_loc()
        } else {
            self.field(self.fields_num()-1).unwrap().end_loc()
        }
    }
}


#[derive(Copy,Clone)]
pub struct FieldContainer<'a> {
    pub parsed:&'a Parsed,
    pub field_ind:usize,
}

impl<'a> std::fmt::Debug for FieldContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // self.primitive().fmt(f)?;
        
        write!(f,"Field({:?})",self.primitive())?;
        Ok(())
    }
}

impl<'a> FieldContainer<'a> {
    fn field(&self) -> &Field {
      self.parsed.fields.get(self.field_ind).unwrap()
    }
    pub fn primitive(&self)->PrimitiveContainer<'a> {
        PrimitiveContainer{ parsed: self.parsed, primitive_ind:self.field().primitive, to_string:false, }
    }
    pub fn string_primitive(&self)->PrimitiveContainer<'a> {
        PrimitiveContainer{ parsed: self.parsed, primitive_ind:self.field().primitive, to_string:true, }
    }
    pub fn start_loc(&self) -> Loc {
        self.field().start_loc
    }
    pub fn end_loc(&self) -> Loc {
        self.primitive().end_loc()
    }
}
#[derive(Copy,Clone)]
pub struct RecordIter<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub parsed :&'a Parsed,
}

impl<'a> Iterator for RecordIter<'a> {
    type Item = RecordContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let x=RecordContainer {record_ind: self.start,parsed: self.parsed,};
            self.start+=1;
            Some(x)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for RecordIter<'a> {
    fn next_back(&mut self) -> Option<RecordContainer<'a>> {
        if self.end > self.start {
            self.end-=1;
            Some(RecordContainer {record_ind: self.end,parsed: self.parsed,})
        } else {
            None
        }
    }
}

#[derive(Copy,Clone)]
pub struct ParamIter<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub parsed :&'a Parsed,
}

impl<'a> Iterator for ParamIter<'a> {
    type Item = ParamContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let x=ParamContainer {param_ind: self.start,parsed: self.parsed,};
            self.start+=1;
            Some(x)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for ParamIter<'a> {
    fn next_back(&mut self) -> Option<ParamContainer<'a>> {
        if self.end > self.start {
            self.end-=1;
            Some(ParamContainer {param_ind: self.end,parsed: self.parsed,})
        } else {
            None
        }
    }
}

#[derive(Copy,Clone)]
pub struct FieldIter<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub parsed :&'a Parsed,
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = FieldContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let x=FieldContainer {field_ind: self.start,parsed: self.parsed,};
            self.start+=1;
            Some(x)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for FieldIter<'a> {
    fn next_back(&mut self) -> Option<FieldContainer<'a>> {
        if self.end > self.start {
            self.end-=1;
            Some(FieldContainer {field_ind: self.end,parsed: self.parsed,})
        } else {
            None
        }
    }
}
