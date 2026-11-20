use std::{num::ParseIntError, path::Path};

use crate::{builder::BuilderError, clike::{char_input::Input2, compiler::{BuilderErrorType, CompileError, CompileErrorType}}, Loc};


#[derive(Debug,Clone)]
pub enum  FormatErrorType {
    Unexpected,
    ClosingBracket,
    ClosingBrace,
    NoIndex,
    NoField,
    // NoVariable,
    Int(ParseIntError),
}


#[derive(Debug, Clone)]
pub enum FieldType<'t> {
    Index(i64),
    Name(&'t str),
}


#[derive(Debug, Clone)]
pub enum Access<'t> {
    Field(FieldType<'t>, Loc),
    Index(i64, Loc),
}

#[derive(Debug, Clone)]
pub enum Placeholder<'t> {
    Positional(Loc),
    Var {
        name: &'t str,
        loc: Loc,
        chain: Vec<Access<'t>>,
    },
}

#[derive(Debug, Clone)]
pub enum FormatItem<'t> {
    Text(&'t str),
    Placeholder(Placeholder<'t>),
}


fn parse_field<'t>(input:&mut Input2<'t>) -> Result<Option<(FieldType<'t>,Loc)>,BuilderError<BuilderErrorType>> {
    // [.] (ident | uint)

    let start_loc=input.start_loc();

    if !input.eat_char('.') {
        return Ok(None);
    }

    input.next(); //so undo from parse_identifier doesn't reset input.start_loc

    if let Some((x,_))=parse_identifier(input) {
        return Ok(Some((FieldType::Name(x),start_loc)));
    }

    if let Some((x,_))=parse_uint(input)? {
        return Ok(Some((FieldType::Index(x),start_loc)));
    }

    //  Err((CompileErrorType::Format(FormatErrorType::NoField), input.start_loc(),)) //should error be at start of '.' instead of after?

     Err(BuilderError{ loc: input.start_loc(), error_type: BuilderErrorType::Format(FormatErrorType::NoField) })


}

fn parse_index<'t>(input:&mut Input2<'t>) -> Result<Option<(i64,Loc)>,BuilderError<BuilderErrorType>> {
    // [\[] int [\]]

    let start_loc=input.start_loc();

    if !input.eat_char('[') {
        return Ok(None);
    }

    input.next(); //so on parse_int fail, input.start_loc is after '['

    let Some((x,_))=parse_int(input)? else {
        // return Err((CompileErrorType::Format(FormatErrorType::NoIndex), input.start_loc(),));

        return Err(BuilderError{ loc: input.start_loc(), error_type: BuilderErrorType::Format(FormatErrorType::NoIndex) });

    };

    if !input.eat_char(']') {
        // return Err((CompileErrorType::Format(FormatErrorType::ClosingBracket), input.cur_loc(),));
        return Err(BuilderError{ loc: input.cur_loc(), error_type: BuilderErrorType::Format(FormatErrorType::ClosingBracket) });
    }

    input.next();

    Ok(Some((x,start_loc)))
}


fn parse_int<'t>(input:&mut Input2<'t>) -> Result<Option<(i64,Loc)>,BuilderError<BuilderErrorType>> {
    //[-]? int

    let start_loc=input.start_loc();
    // let neg=
    input.eat_char('-');

    // //so parse_uint doesn't use it
    // input.next();

    //

    if let Some((x,_))=parse_uint(input)? {
        Ok(Some((x,start_loc))) //if neg {-x} else {x}
    } else {
        input.undo();
        Ok(None)
    }
}

fn parse_uint<'t>(input:&mut Input2<'t>) -> Result<Option<(i64,Loc)>,BuilderError<BuilderErrorType>> {
    //[-]? ([0] | [0-9][1-9]*)

    //

    //
    if input.eat_range('1'..='9') {
        while input.eat_range('0'..='9') {}
    } else if input.eat_char('0') {
    } else {
        return Ok(None);
    }

    let val = input.slice().parse::<i64>()
        // .map_err(|e|(
        //     CompileErrorType::Format(FormatErrorType::Int(e)),
        //     input.start_loc(),
        // ))
        .map_err(|e|BuilderError{
            loc: input.start_loc(), error_type: BuilderErrorType::Format(FormatErrorType::Int(e))
        })
        ?;

    let start_loc=input.start_loc();
    // let end_loc=input.cur_loc();

    input.next();
    return Ok(Some((val,start_loc)));


}


fn parse_identifier<'t>(input: &mut Input2<'t>) -> Option<(&'t str,Loc)> {
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

    let start_loc=input.start_loc();
    // let end_loc=input.cur_loc();

    input.next();
    Some((slice,start_loc))



}

fn parse_placeholder<'t>(input:&mut Input2<'t>) -> Result<Option<Placeholder<'t>>,BuilderError<BuilderErrorType>> {
    // placeholder    => [{] (var_chain)? [}]
    // var_chain      => ident (field_access | index_access)*

    //

    let start_loc=input.start_loc();

    //
    if !input.eat_char('{') {
        return Ok(None);
    }

    //because of parse_identifier's undo
    input.next();

    //
    let placeholder = if let Some((name,var_loc))=parse_identifier(input) {
        let mut chain = Vec::new();

        loop {
            if let Some((x,loc))=parse_field(input)? {
                chain.push(Access::Field(x, loc));
                continue;
            }
            if let Some((x,loc))=parse_index(input)? {
                chain.push(Access::Index(x, loc));
                continue;
            }

            break;
        }
        Placeholder::Var { name, loc: var_loc, chain, }
    } else {
        Placeholder::Positional(start_loc)
    };

    //
    if !input.eat_char('}') {
        // return Err((CompileErrorType::Format(FormatErrorType::ClosingBrace), input.cur_loc(),));
        return Err(BuilderError { loc: input.cur_loc(), error_type: BuilderErrorType::Format(FormatErrorType::ClosingBrace), })
    }

    //
    input.next();

    //
    Ok(Some(placeholder))
}

pub fn parse<'p,'t>(
    decoded_src: &'t str,
    raw_src:&'t str,
    raw_loc:Loc,

    path:Option<&'p Path>,
) ->  Result<Vec<FormatItem<'t>>, BuilderError<BuilderErrorType>>

{
    let mut input=Input2::new(raw_src,raw_loc);
    let mut decoded_pos = 0;
    let mut items = Vec::new();

    while !input.is_eof() {

        //escaped lbrace/
        if input.eat_str("{{"){
            items.push(FormatItem::Text("{"));
            input.next();
            decoded_pos += "{{".len();
            continue;
        }

        //escaped rbrace
        if input.eat_str("}}") {
            items.push(FormatItem::Text("}"));
            input.next();
            decoded_pos += "}}".len();
            continue;
        }

        //placeholder => [{] (var_chain)? [}]
        {
            let raw_before = input.pos_range().start;
            if let Some(p)=parse_placeholder(&mut input)?
            {
                let consumed_len = input.pos_range().start - raw_before;
                decoded_pos += consumed_len;

                items.push(FormatItem::Placeholder(p));
                continue;
            }
        }

        //text => [^{}]+
        if input.peek_char().is_some_and(|c| c != '{' && c != '}') //input.eat_not_of("{}")
        {
            let text_decoded_start = decoded_pos;

            // while input.eat_not_of("{}") {}

            while let Some(c) = input.peek_char() {
                if c == '{' || c == '}' {break;}

                if input.eat_char('\\') {
                    if input.take_one_of("strn\n\"\\").is_some() || input.eat_str("\r\n") {
                        // decoded_pos+=decoded_src[decoded_pos..].chars().next().unwrap().len_utf8();
                        decoded_pos += 1;
                    } else if let Some(cc) = input.take_any() { //other "non escaped" escaped
                        // decoded_pos += '\\'.len_utf8() + cc.len_utf8();
                        decoded_pos += 1 + cc.len_utf8();
                    }
                } else if let Some(cc) = input.take_any() {
                    decoded_pos += cc.len_utf8();
                }
            }

            // items.push(FormatItem::Text(input.slice()));
            items.push(FormatItem::Text(&decoded_src[text_decoded_start..decoded_pos]));
            input.next();
            continue;
        }

        //
        return Err(BuilderError {
            // src: Some(raw_src),
            // path,
            error_type: BuilderErrorType::Format(FormatErrorType::Unexpected),
            loc:input.start_loc(),
        });
    }

    Ok(items)
}