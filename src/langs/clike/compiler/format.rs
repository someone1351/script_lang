use std::{num::ParseIntError, path::Path};

use crate::{clike::{char_input::Input2, compiler::{CompileError, CompileErrorType}}, Loc};


#[derive(Debug,Clone)]
pub enum  FormatErrorType {
    Unexpected,
    ClosingBracket,
    NoIndex,
    NoField,
    Int(ParseIntError),
}


enum FieldType<'t> {
    Index(i64),
    Name(&'t str),
}

fn parse_field<'t>(input:&mut Input2<'t>) -> Result<Option<(FieldType<'t>,Loc)>,(CompileErrorType,Loc)> {
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

     Err((CompileErrorType::Format(FormatErrorType::NoField), input.start_loc(),)) //should error be at start of '.' instead of after?

}

fn parse_index<'t>(input:&mut Input2<'t>) -> Result<Option<(i64,Loc)>,(CompileErrorType,Loc)> {
    // [\[] int [\]]

    let start_loc=input.start_loc();

    if !input.eat_char('[') {
        return Ok(None);
    }

    input.next(); //so on parse_int fail, input.start_loc is after '['

    let Some((x,_))=parse_int(input)? else {
        return Err((CompileErrorType::Format(FormatErrorType::NoIndex), input.start_loc(),));
    };

    if !input.eat_char(']') {
        return Err((CompileErrorType::Format(FormatErrorType::ClosingBracket), input.cur_loc(),));
    }

    input.next();

    Ok(Some((x,start_loc)))
}


fn parse_int<'t>(input:&mut Input2<'t>) -> Result<Option<(i64,Loc)>,(CompileErrorType,Loc)> {
    //[-]? int

    let start_loc=input.start_loc();
    let neg=input.eat_char('-');

    if let Some((x,_))=parse_uint(input)? {
        Ok(Some((if neg {-x} else {x},start_loc)))
    } else {
        input.undo();
        Ok(None)
    }
}

fn parse_uint<'t>(input:&mut Input2<'t>) -> Result<Option<(i64,Loc)>,(CompileErrorType,Loc)> {
    //[-]? ([0] | [0-9][1-9]*)

    //

    //
    if input.eat_range('1'..='9') {
        while input.eat_range('0'..='9') {}
    } else if input.eat_char('0') {
    } else {
        return Ok(None);
    }

    let val = input.slice().parse::<i64>().map_err(|e|(
        CompileErrorType::Format(FormatErrorType::Int(e)),
        input.start_loc(),
    ))?;

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

pub fn parse<'p,'t>(src:&'t str,
    src_loc:Loc,
    path:Option<&'p Path>,
) ->  Result<(), CompileError<'p,'t>>

{
    let mut input=Input2::new(src,src_loc);

    while !input.is_eof() {

        //escaped lbrace/rbrace
        if input.eat_str("{{") || input.eat_str("}}") {

            continue;
        }

        //
        if input.eat_char('{') {
            loop {

                if input.eat_char('}') {
                    break;
                }
            }
        }


        //text [^{}]+
        if input.eat_not_of("{}") {
            while input.eat_not_of("{}") {}
            continue;
        }

        //
        return Err(CompileError {
            src: Some(src),
            path,
            error_type: CompileErrorType::Format(FormatErrorType::Unexpected),
            loc:input.start_loc(),
        });
    }

    Ok(())
}