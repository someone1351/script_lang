//TODO
//add scanner to block container
//instead of just getting primitives eg "1*2" "+" "3" => "1" "*" "2" "+" "3"
//can generate start/end locs for them
//ScannedContainer {parsed,primitive_ind,start_loc,end_loc,chars:Range<usize>}
//block.scanner() -> ScannerContainer
//record.scanner() -> ScannerContainer


//start => (cmd | block | val | cmnt)*

// cmnt => [#] ([^#\n]|^([\r][\n]))* (eol|eof)

//block => [{] spc? start ws [}]

// cmd => idn (sep param)*

// param => block | val

//val => idn (ws (field|index))*
//field => "." idn
//index => "[" "]"

// int => ([+-]spcs?)?([1-9][0-9]*|[0-9])
// float => int([.][0-9]*)?
// bool => "true"|"false"
// str => ["] ([^"])* ["]
//===========================

/*
TODO1
* look at start/end loc for strings eg loc of start/end quotes and also of string value
* * how is it done for blocks?

TODO2
* add option for commas in records, similar to semicolons
* * two empty commas eg ",," equals 3 empty params? no just two params
* * have comma_loc func on param (like record has semicolon_loc), so can check in compiler to give error
* * if a comma at the end of the line, then any thing before any semi colon, is accepted as a param? no

* allow fields to come after new line? eg:
abc
    .def
    .ghi

* allow fields to have spaces? eg: abc .def .ghi

* could treat symbols as separate params eg abc+def => "abc", "+", "def"
* * for !%^&*-+=<>?/|
* * maybe not @ ~
* * maybe :`
* * not ,.[]{}()#$_\;"'
* * how to handle: expr 1 +2
* * * the expr command would want to know the number had a prefix, store as part of float/int primitive has prefix?
* * * or don't have prefix for numbers, but in compiler check if prev param in front is +/- ? no hassle
* * * also currently in 1 +2, the + is taken by the char_symbol, before it gets to number,
* * * * have to do number first? if so then this: +5+6 => 5 6

* for block, add get_bracket_type -> Bracket{Curly,Parentheses,Square,}, so can use for error checking if want to force one type

*/

mod input;
mod error;
mod container;
mod parsed;


use std::collections::HashMap;
use super::super::common::Loc;
use input::*;
pub use error::*;
pub use container::*;
pub use parsed::*;


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
pub fn parse<'a>(src:&'a str,
    // path:Option<&'a Path>
) -> Result<Parsed,ParseError> {

    pub enum TempWork {
        Block{block : TempBlock, block_start_loc : Loc, field_start_loc : Option<Loc>,bracket:Option<BlockBracket>},
        Param{param:TempParam,
            // start_loc : Loc,
        }

    }


    //
    let mut input = Input::new(src);
    let mut text_map = HashMap::<String,usize>::new();
    text_map.insert("".to_string(), 0); //used in primitive container, when converting box primitive to string primitive

    // let mut temp_elements_stk=vec![TempElement{block:TempBlock{records:Vec::new()},start_loc:Loc::one()}];
    let mut temp_works_stk=vec![TempWork::Block{
        block:TempBlock{records:Vec::new(),},
        block_start_loc:Loc::one(),
        field_start_loc : None,
        bracket:None,
    }]; // end_loc: Loc::zero()

    // let mut bracket_stk=Vec::new();
    //
    loop {

        //when temp_works_stk.last is a symbol
        if let TempWork::Param { param: cur_work_param, .. } = temp_works_stk.last_mut().unwrap() {

            if let Some(field_start_loc)=parse_field_sep(&mut input) {
                let loc=input.loc();

                //parse symbol,string,int or block
                if let Some(primitive)=parse_number(&mut input,false,&mut text_map) {
                    cur_work_param.fields.push(TempField{ primitive, start_loc: field_start_loc });
                } else if let Some((text_ind,start_loc,end_loc))=parse_ident_symbol(&mut input,&mut text_map) {
                    // let primitive_type=TempPrimitiveType::String(text_ind);
                    let primitive_type=TempPrimitiveType::Symbol(text_ind); //fixed
                    let primitive=TempPrimitive { primitive_type, start_loc, end_loc };
                    cur_work_param.fields.push(TempField{ primitive, start_loc: field_start_loc });
                } else if let Some(primitive)=parse_string(&mut input,&mut text_map)? {
                    cur_work_param.fields.push(TempField{ primitive, start_loc: field_start_loc });
                } else if let Some(bracket)=parse_block_begin(&mut input) {
                    temp_works_stk.push(TempWork::Block {
                        block: TempBlock { records: Vec::new(),},
                        block_start_loc:loc,
                        field_start_loc: Some(field_start_loc),
                        bracket:Some(bracket)
                    }); // end_loc: Loc::zero(),
                } else {
                    return Err(ParseError{
                        // path:path.map(|p|p.to_path_buf()),
                        loc:input.loc(),
                        error_type:ParserErrorType::ExpectedField,
                    });
                }

                continue;
            } else {
                let TempWork::Param {
                    param:popped_work_param,
                    // start_loc: popped_start_loc
                }=temp_works_stk.pop().unwrap() else{panic!("");};

                let TempWork::Block { block:cur_work_block, ..}=temp_works_stk.last_mut().unwrap() else{panic!("");};

                //
                if cur_work_block.records.last().map(|r|r.ended).unwrap_or(true) {
                    cur_work_block.records.push(TempRecord { params: Vec::new(), ended: false, semi_colon_end_loc:None, });
                }

                let cur_record=cur_work_block.records.last_mut().unwrap();
                // let loc=input.loc();
                // TempPrimitive {
                //     primitive_type: TempPrimitiveType::Symbol(popped_work_param),
                //     start_loc:popped_start_loc, end_loc:loc,
                // }
                // popped_work_param
                cur_record.params.push(popped_work_param);
            }
        }

        //when temp_works_stk.last is a block

        //
        let loc=input.loc();
        let spc=parse_space(&mut input);

        //on end (of record)
        if parse_cmnt(&mut input) || parse_end(&mut input) || input.is_end() {
            let TempWork::Block { block:cur_block, .. } = temp_works_stk.last_mut().unwrap() else {
                panic!("");
            };

            // let cur_element=temp_elements_stk.last_mut().unwrap();

            if let Some(cur_record)=cur_block.records.last_mut() {
                cur_record.ended=true; //might already be true //?? not true?? //only when this has run multiple times eg multiple newlines
            }

            //
            if input.is_end() {
                break;
            } else {
                continue;
            }
        }

        if parse_semi_colon(&mut input) {
            // let cur_element=temp_elements_stk.last_mut().unwrap();

            let TempWork::Block { block:cur_block, .. } = temp_works_stk.last_mut().unwrap() else {
                panic!("");
            };

            if let Some(cur_record)=cur_block.records.last_mut() {
                cur_record.ended=true; //might already be true

                if cur_record.semi_colon_end_loc.is_some() {
                    cur_block.records.push(TempRecord { params: Vec::new(), ended: true, semi_colon_end_loc:Some(loc), });
                } else {
                    cur_record.semi_colon_end_loc=Some(loc);
                }
            } else {
                cur_block.records.push(TempRecord { params: Vec::new(), ended: true, semi_colon_end_loc:Some(loc), });
            }

            //
            if input.is_end() {
                break;
            } else {
                continue;
            }
        }

        //on non root block begin
        if let Some(bracket)=parse_block_begin(&mut input) {
            temp_works_stk.push(TempWork::Block {
                block: TempBlock { records: Vec::new(),},
                block_start_loc:loc,
                field_start_loc:None,
                bracket:Some(bracket),
            }); //, end_loc:Loc::zero()
            continue;
        }

        //on block end
        if let Some(close_bracket)=parse_block_end(&mut input) {
            //error
            if temp_works_stk.len()==1 {
                return Err(ParseError{
                    // path:path.map(|p|p.to_path_buf()),
                    loc,
                    error_type:ParserErrorType::UnmatchedClosingBlock,
                });
            }

            let block_end_loc = input.loc();

            //
            // let popped_block=temp_elements_stk.pop().unwrap();
            // let cur_element=temp_elements_stk.last_mut().unwrap();
            let TempWork::Block {
                block:popped_work_block,
                block_start_loc:popped_work_block_start_loc,
                field_start_loc,
                bracket,
            } = temp_works_stk.pop().unwrap() else {
                panic!("");
            };

            if bracket.unwrap()!=close_bracket {
                return Err(ParseError{
                    // path:path.map(|p|p.to_path_buf()),
                    loc:input.loc(),
                    error_type:match bracket.unwrap() {
                        BlockBracket::Curly => ParserErrorType::ExpectingClosingCurlyBracket,
                        BlockBracket::Square => ParserErrorType::ExpectingClosingSquareBracket,
                        BlockBracket::Parentheses => ParserErrorType::ExpectingClosingParentheses,
                    },
                });
            }

            // popped_work_block.end_loc=input.loc();

            let cur_work=temp_works_stk.last_mut().unwrap();

            match cur_work {
                TempWork::Param { param, .. } => {
                    let primitive=TempPrimitive {
                        primitive_type: TempPrimitiveType::Block(popped_work_block),
                        start_loc: popped_work_block_start_loc,
                        end_loc: block_end_loc,
                    };

                    param.fields.push(TempField { primitive, start_loc: field_start_loc.unwrap() });
                }
                TempWork::Block { block:cur_block, .. } => {
                    if cur_block.records.last().map(|r|r.ended).unwrap_or(true) {
                        cur_block.records.push(TempRecord { params: Vec::new(), ended: false, semi_colon_end_loc:None, });
                    }

                    // let cur_record=cur_block.records.last_mut().unwrap();

                    let primitive=TempPrimitive {
                        primitive_type: TempPrimitiveType::Block(popped_work_block), //popped_block.block
                        start_loc: popped_work_block_start_loc, //popped_block.start_loc,
                        end_loc: loc,
                    };

                    let param=TempParam {
                        primitive,
                        // start_loc: cur_block.,
                        // end_loc: (),
                        fields: Vec::new(),
                    };

                    temp_works_stk.push(TempWork::Param { param,  });
                    // cur_record.params.push();
                }

            }
            // let TempElement::Block { block:cur_block, .. } = temp_elements_stk.last_mut().unwrap() else {
            //     continue;
            // };


            //
            continue;
        }

        //
        //parse num
        if let Some(primitive)=parse_number(&mut input,true,&mut text_map) {
            let TempWork::Block { block:cur_block, .. } = temp_works_stk.last_mut().unwrap() else {panic!("");};

            if cur_block.records.last().map(|r|r.ended).unwrap_or(true) {
                cur_block.records.push(TempRecord { params: Vec::new(), ended: false, semi_colon_end_loc:None, });
            }

            let cur_record=cur_block.records.last_mut().unwrap();

            //no fields allowed for string, float or int, so pushed directly on record
            cur_record.params.push(TempParam { primitive, fields: Vec::new() });

            continue;
        }

        // println!("{:?}, {}",input.getc(0),input.loc());
        let mut had_char_symbol = false;

        //char symbols don't need spaces between them and other params (also don't have fields like ident symbols)
        if let Some((text_ind,start_loc,end_loc))=parse_char_symbol(&mut input,&mut text_map)  {
            let primitive=TempPrimitive{primitive_type: TempPrimitiveType::Symbol(text_ind), start_loc, end_loc};

            let TempWork::Block { block:cur_block, .. } = temp_works_stk.last_mut().unwrap() else {panic!("");};

            if cur_block.records.last().map(|r|r.ended).unwrap_or(true) {
                cur_block.records.push(TempRecord { params: Vec::new(), ended: false, semi_colon_end_loc:None, });
            }

            let cur_record=cur_block.records.last_mut().unwrap();
            cur_record.params.push(TempParam { primitive, fields: Vec::new() });
            had_char_symbol=true;
            // continue;
        }

        //make sure there are spaces between primitives
        else if !input.is_end() && !spc {
            // let cur_element=temp_elements_stk.last().unwrap();
            let TempWork::Block { block:cur_block, .. } = temp_works_stk.last_mut().unwrap() else {panic!("");};

            if !cur_block.records.last().map(|r|r.ended).unwrap_or(true) {
                return Err(ParseError{loc:input.loc(),error_type:ParserErrorType::UnexpectedChar,});
            }
        }

        //parse primitives
        if let Some(primitive)=parse_string(&mut input,&mut text_map,
            // src,
            // path
        )?
            // .or_else(||parse_number(&mut input,true,&mut text_map))
            // // .or_else(||parse_symbol(&mut input,&mut text_map))
        {
            // let cur_element=temp_elements_stk.last_mut().unwrap();
            let TempWork::Block { block:cur_block, .. } = temp_works_stk.last_mut().unwrap() else {panic!("");};

            if cur_block.records.last().map(|r|r.ended).unwrap_or(true) {
                cur_block.records.push(TempRecord { params: Vec::new(), ended: false, semi_colon_end_loc:None, });
            }

            let cur_record=cur_block.records.last_mut().unwrap();

            //no fields allowed for string, float or int, so pushed directly on record
            cur_record.params.push(TempParam { primitive, fields: Vec::new() });

            continue;
        }

        //parse symbol
        if let Some((text_ind,start_loc,end_loc))=parse_ident_symbol(&mut input,&mut text_map)  {
            // let cur_element=temp_elements_stk.last_mut().unwrap();
            // let TempElement::Block { block:cur_block, .. } = temp_elements_stk.last_mut().unwrap() else {
            //     panic!("");
            // };

            let primitive=TempPrimitive{
                primitive_type: TempPrimitiveType::Symbol(text_ind),
                start_loc,
                end_loc
            };

            let param=TempParam {
                // text_ind,
                // start_loc,
                // end_loc,
                fields: Vec::new(),
                primitive,
            };

            temp_works_stk.push(TempWork::Param { param,   }); //, end_loc:Loc::zero() start_loc:loc

            continue;
        }

        //
        if !had_char_symbol {
            break;
        }
    }

    //errors
    if !input.is_end() {
        return Err(ParseError{
            // path:path.map(|p|p.to_path_buf()),
            loc:input.loc(),
            error_type:ParserErrorType::Unknown,
        });
    } else if temp_works_stk.len()!=1 {
        return Err(ParseError{
            // path:path.map(|p|p.to_path_buf()),
            loc:input.loc(),
            error_type:ParserErrorType::ClosingBlockExpected,
        });
    }

    // let temp_work_root=temp_works_stk.first().unwrap();
    let temp_root_block=if let TempWork::Block { block, .. } = temp_works_stk.first().unwrap() {
        block
    } else {
        panic!("");
    };

    // temp_root_block.end_loc=input.loc();

   Ok(generate_parsed(temp_root_block,input.loc(),text_map))
}

fn generate_parsed(temp_root_block:&TempBlock,last_loc:Loc,text_map:HashMap<String, usize>) -> Parsed {
    let mut parsed=Parsed {
        blocks:vec![Block{primitive:0,records:0..0,params:0..0}], // end_loc: Loc::zero()
        records:Vec::new(),
        params:Vec::new(),
        fields:Vec::new(),
        primitives:vec![Primitive{
            primitive_type:PrimitiveType::Block(0),
            start_loc:Loc::one(),
            end_loc:last_loc,
            param: None, field: None
        }],
        texts:Vec::new(),
        // src,
        // path,
    };

    //
    struct ParsedWork<'a> {
        cur_temp_block:&'a TempBlock,
        cur_block_ind:usize,
    }

    //
    let mut work_stk = vec![ParsedWork{cur_block_ind:0,cur_temp_block: temp_root_block}];

    //
    while let Some(parsed_work)=work_stk.pop() {
        match parsed_work {
            ParsedWork { cur_temp_block, cur_block_ind } => {
                let new_records_start=parsed.records.len();
                let new_records_end=new_records_start+cur_temp_block.records.len();
                let new_block_params_start=parsed.params.len();
                parsed.blocks.get_mut(cur_block_ind).unwrap().records=new_records_start..new_records_end;

                //push records
                for temp_record in cur_temp_block.records.iter() {
                    let new_params_start=parsed.params.len();
                    let new_params_end=new_params_start+temp_record.params.len();

                    parsed.records.push(Record {
                        params : new_params_start..new_params_end,
                        semi_colon_loc:temp_record.semi_colon_end_loc,
                    });

                    //push params
                    for temp_param in temp_record.params.iter() {
                        let new_fields_start = parsed.fields.len();
                        let new_fields_end = new_fields_start+temp_param.fields.len();
                        let new_param_ind=parsed.params.len();
                        let new_param_primitive_ind = parsed.primitives.len();

                        //
                        parsed.params.push(Param { primitive: new_param_primitive_ind, fields: new_fields_start..new_fields_end });

                        //push primitive
                        {
                            let primitive_type=match &temp_param.primitive.primitive_type {
                                TempPrimitiveType::Block(temp_block)=> {
                                    let new_block_ind=parsed.blocks.len();
                                    parsed.blocks.push(Block { primitive: new_param_primitive_ind, records: 0..0, params:0..0}); //end_loc: temp_block.end_loc
                                    work_stk.push(ParsedWork { cur_temp_block: temp_block, cur_block_ind: new_block_ind });
                                    PrimitiveType::Block(new_block_ind)
                                },
                                TempPrimitiveType::Float(x,s,p)=>PrimitiveType::Float(*x,*s,*p),
                                TempPrimitiveType::Int(x,s,p)=>PrimitiveType::Int(*x,*s,*p),
                                TempPrimitiveType::String(s)=>PrimitiveType::String(*s),
                                TempPrimitiveType::Symbol(s)=>PrimitiveType::Symbol(*s),
                            };

                            parsed.primitives.push(Primitive {
                                primitive_type,
                                start_loc: temp_param.start_loc(), end_loc: temp_param.end_loc(),
                                param: Some(new_param_ind), field: None,
                            });
                        }

                        //push fields
                        for temp_field in temp_param.fields.iter() {
                            let new_field_primitive_ind = parsed.primitives.len();

                            //
                            parsed.fields.push(Field { primitive: new_field_primitive_ind, start_loc: temp_field.start_loc, });

                            //
                            let primitive_type=match &temp_field.primitive.primitive_type {
                                TempPrimitiveType::Block(temp_block)=> {
                                    let new_block_ind=parsed.blocks.len();
                                    parsed.blocks.push(Block { params:0..0, primitive: new_field_primitive_ind, records: 0..0, }); //end_loc: temp_block.end_loc
                                    work_stk.push(ParsedWork { cur_temp_block: temp_block, cur_block_ind: new_block_ind });
                                    PrimitiveType::Block(new_block_ind)
                                },
                                TempPrimitiveType::Int(x,s,p)=>PrimitiveType::Int(*x,*s,*p),
                                TempPrimitiveType::String(s)=>PrimitiveType::String(*s),
                                TempPrimitiveType::Symbol(s)=>PrimitiveType::Symbol(*s), //fixed
                                _ => {panic!("")},
                            };

                            parsed.primitives.push(Primitive {
                                primitive_type,
                                start_loc: temp_field.primitive.start_loc, end_loc: temp_field.primitive.end_loc,
                                param: None, field: Some(new_field_primitive_ind),
                            });
                        }
                    }
                }

                let new_block_params_end=parsed.params.len();
                // let new_block_params_range=if new_block_params_start==new_block_params_end {0..0} else {new_block_params_start..new_block_params_end};
                // parsed.blocks.get_mut(cur_block_ind).unwrap().params=new_block_params_range;
                parsed.blocks.get_mut(cur_block_ind).unwrap().params=new_block_params_start..new_block_params_end;

            }
        }
    }

    //text
    parsed.texts.resize(text_map.len(), String::new());

    for (k,v) in text_map {
        parsed.texts[v]=k;
    }

    //
    parsed
}

pub fn parse_block_begin(input:&mut Input) -> Option<BlockBracket> {
    let x=input.has(0, ["{","[","("]);

    if x.is_some() {
        input.next(x.unwrap().len());
    }

    match x.unwrap_or_default() {
        "{" => Some(BlockBracket::Curly),
        "[" => Some(BlockBracket::Square),
        "(" => Some(BlockBracket::Parentheses),
        _ => None,
    }

//     if Some("{")==input.get(0, 1) {
//         input.next(1);
//         true
//     } else {
//         false
//     }
}

pub fn parse_block_end(input:&mut Input) -> Option<BlockBracket> {
    let x=input.has(0, ["}","]",")"]);

    if x.is_some() {
        input.next(x.unwrap().len());
    }

    match x.unwrap_or_default() {
        "}" => Some(BlockBracket::Curly),
        "]" => Some(BlockBracket::Square),
        ")" => Some(BlockBracket::Parentheses),
        _ => None,
    }

    // if Some("}")==input.get(0, 1) {
    //     input.next(1);
    //     true
    // } else {
    //     false
    // }
}

pub fn parse_cmnt(input:&mut Input) -> bool {
    if Some("#")!=input.get(0, 1) {
        return false;
    }

    loop {
        if let Some(x)=input.has(0, ["\r\n","\n"]) {
            input.next(x.len());
            break;
        }

        if input.is_end() {
            break;
        }

        input.next(1);
    }

    true
}

pub fn parse_end(input:&mut Input) -> bool {
    if let Some(x)=input.has(0, ["\r\n","\n"]) { //";",
        input.next(x.len());
        true
    } else {
        false
    }
}
pub fn parse_semi_colon(input:&mut Input) -> bool {
    if let Some(x)=input.has(0, [";"]) {
        input.next(x.len());
        true
    } else {
        false
    }
}
pub fn parse_space(input:&mut Input) -> bool {
    let mut found=false;

    while let Some(x)=input.has(0, [" ","\t","\\\r\n","\\\n"]) {
        input.next(x.len());
        found=true;
    }

    found
}

pub fn parse_string<'a>(
    input:&mut Input,
    text_map:&mut HashMap<String,usize>,
    // texts:&mut Vec<String>,
    // src:&'a str,
    // path:Option<&'a Path>,
) -> Result<Option<TempPrimitive>,ParseError> {
    let quotes=["\"\"\"","'''","\"","'"];

    for quote in quotes {
        //
        let start_loc=input.loc();

        //opening quote
        if Some(quote)!=input.get(0, quote.len()) {
            continue;
        }

        input.next(quote.len());

        //
        let mut s=String::new();

        //body
        loop {
            //closing quote
            if Some(quote)==input.get(0, quote.len()) {
                input.next(quote.len());
                break;
            }

            //escapes
            if let Some(x)=input.get(0, 2) {
                let xs=x.chars().collect::<Vec<_>>();

                if xs[0]=='\\' {
                    if quote.starts_with("\"") {
                        s.push(match xs[1] {
                            's'=>' ',
                            't'=>'\t',
                            'r'=>'\r',
                            'n'=>'\n',
                            _=>xs[1],
                        });
                    } else if quote=="'" {
                        s+=quote;
                    } else {
                        s+=x;
                    }

                    input.next(2);
                    continue;
                }
            }

            //char
            if let Some(x)=input.get(0, 1) {
                s+=x;
                input.next(1);
                continue;
            }

            //
            return Err(ParseError {
                // path:path.map(|p|p.to_path_buf()),
                loc: input.loc(),
                error_type: ParserErrorType::ClosingQuoteExpected(quote),
            });

        }

        //
        let text_map_size=text_map.len();
        let text_ind=*text_map.entry(s).or_insert(text_map_size);

        //
        let end_loc=input.loc();
        let primitive_type=TempPrimitiveType::String(text_ind);
        return Ok(Some(TempPrimitive { primitive_type, start_loc, end_loc }))
    }

    Ok(None)
}

fn parse_char_symbol(
    input:&mut Input,
    // texts:&mut Vec<String>,
    text_map:&mut HashMap<String,usize>,
) -> Option<(usize,Loc,Loc)> {
    if let Some(x)=input.has(0, ["~","!","@","%","^","&","*","-","+","=","<=",">=","<",">","?","/","|",",",]) {
        let start_loc=input.loc();

        let text_map_size=text_map.len();
        let text_ind=*text_map.entry(x.to_string()).or_insert(text_map_size);

        input.next(x.len());
        let end_loc=input.loc();
        // println!("got {x:?}, {start_loc}:{end_loc}");
        Some((text_ind,start_loc,end_loc))
    } else {
        None
    }
}

fn parse_ident_symbol(
    input:&mut Input,
    // texts:&mut Vec<String>,
    text_map:&mut HashMap<String,usize>,
) -> Option<(usize,Loc,Loc)> {
    //[^ \t\r\n"',;{}]+

    //symbol_str=[^ \t\r\n"',;{}]+
    //symbol=symbol_str ([.](symbol_str|block))*

/*
* * for ~!@%^&*-+=<>?/|
* * maybe :`
* * not ,.[]{}()#$_\; "'
*/
    let mut i=0;

    while input.hasnt(i, [
        "\\"," ","\t","\r","\n",
        "\"","'",
        ".",",",";",
        "{","}","[","]","(",")",
        ":","`",
        "~","!","@","%","^","&","*","-","+","=","<",">","?","/","|",
    ]) {
        i+=1;
    }

    if i==0 {
        None
    } else {
        let start_loc=input.loc();
        let val=input.get(0, i).unwrap().to_string();

        let text_map_size=text_map.len();
        let text_ind=*text_map.entry(val).or_insert(text_map_size);



        input.next(i);
        let end_loc=input.loc();
        Some((text_ind,start_loc,end_loc))
    }

}

fn parse_field_sep(
    input:&mut Input,
) -> Option<Loc> {
    //[.]

    if input.getc(0)==Some('.') {
        let start_loc=input.loc();
        input.next(1);
        Some(start_loc)
    } else {
        None
    }
}
fn parse_number(
    input:&mut Input,
    float_aswell:bool,
    text_map:&mut HashMap<String,usize>,
) -> Option<TempPrimitive> {
    let mut i=0;
    let mut is_float=false;
    let mut ok=false;

    let mut has_prefix=false;

    if let Some(c)=input.get(i, 1) {
        if "+-".contains(c) {
            has_prefix=true;
            i+=1;
        }
    }

    while let Some(c)=input.get(i, 1) {
        if ("0"..="9").contains(&c) {
            i+=1;
            ok=true;
        } else {
            break;
        }
    }

    if float_aswell {
        if Some(".")==input.get(i, 1) {
            i+=1;
            is_float=true;
        }

        while let Some(c)=input.get(i, 1) {
            if ("0"..="9").contains(&c) {
                i+=1;
                ok=true;
            } else {
                break;
            }
        }
    }

    if !ok {
        return None;
    }

    let start_loc=input.loc();
    let token=input.get(0, i).unwrap();

    let text_map_size=text_map.len();
    let text_ind=*text_map.entry(token.to_string()).or_insert(text_map_size);

    let primitive_type=if is_float {
        TempPrimitiveType::Float(token.parse().unwrap(),text_ind,has_prefix)
    } else {
        TempPrimitiveType::Int(token.parse().unwrap(),text_ind,has_prefix)
    };

    input.next(i);
    let end_loc=input.loc();

    Some(TempPrimitive { start_loc, end_loc, primitive_type })
}

