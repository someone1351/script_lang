
pub use super::error::*;

use std::collections::HashMap;

use super::data::*;

use super::input::*;


// use super::super::super::build::Loc;


pub fn parse<'a>(src:&'a str, ) -> Result<Parsed,ParseError> {

    let mut input = Input::new(src);
    let mut cur_primitives = Vec::new();
    let mut text_map = HashMap::<String,usize>::new();

    text_map.insert("".to_string(), 0); //used in primitive container, when converting box primitive to string primitive

    //
    loop {
        //
        if let Some(primitive)=parse_ws(&mut input)? {
            cur_primitives.push(primitive); //end
        }

        //
        if let Some(primitive)=parse_string(&mut input,&mut text_map)? {
            cur_primitives.push(primitive);
            continue;
        } else if let Some(primitive)=parse_number(&mut input, true, &mut text_map) {
            cur_primitives.push(primitive);
            continue;
        } else if let Some(primitive)=parse_ident_symbol(&mut input, &mut text_map) {
            cur_primitives.push(primitive);
            continue;
        } else if let Some(primitive)=parse_char_symbol(&mut input, &mut text_map) { //needs to go last to catch "_"
            cur_primitives.push(primitive);
            continue;
        } else if input.is_end() {
            //not needed?
            // if cur_primitives.last().map(|p|p.primitive_type.clone())!=Some(PrimitiveType::End) {
            //     cur_primitives.push(Primitive { primitive_type: PrimitiveType::End, start_loc:input.loc(), end_loc: input.loc() });
            // }

            break;
        }

        if !input.is_end() {
            return Err(ParseError { loc: input.loc(), error_type: ParserErrorType::Unexpected });
        }
    }

    //
    //
    let mut out_texts: Vec<String>=vec![String::new();text_map.len()];

    for (s,i) in text_map {
        out_texts[i]=s;
    }

    //
    Ok(Parsed { primitives: cur_primitives, texts: out_texts })
}

fn parse_number(
    input:&mut Input,
    float_aswell:bool,
    text_map:&mut HashMap<String,usize>,
) -> Option<Primitive> {
    let mut i=0;
    let mut is_float=false;
    // let mut ok=false;

    while let Some(c)=input.get(i, 1) {
        if ("0"..="9").contains(&c) {
            i+=1;
            // ok=true;
        } else {
            break;
        }
    }

    if i==0 {
        return None;
    }

    if float_aswell {
        let mut j=i;

        if Some(".")==input.get(j, 1) {
            j+=1;
        }

        if j!=i {
            // ok=false;
            while let Some(c)=input.get(j, 1) {
                if ("0"..="9").contains(&c) {
                    j+=1;
                    // ok=true;
                } else {
                    break;
                }
            }

            if j>i+1 {
                is_float=true;
                i=j;
            }
        }
    }

    let start_loc=input.loc();
    let token=input.get(0, i).unwrap();

    let text_map_size=text_map.len();
    let text_ind=*text_map.entry(token.to_string()).or_insert(text_map_size);

    let primitive_type=if is_float {
        PrimitiveType::Float(token.parse().unwrap(),text_ind,)
    } else {
        PrimitiveType::Int(token.parse().unwrap(),text_ind,)
    };

    input.next(i);
    let end_loc=input.loc();

    Some(Primitive { start_loc, end_loc, primitive_type })
}

fn parse_string<'a>(
    input:&mut Input,
    text_map:&mut HashMap<String,usize>,
    // texts:&mut Vec<String>,
    // src:&'a str,
    // path:Option<&'a Path>,
) -> Result<Option<Primitive>,ParseError> {
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
        let primitive_type=PrimitiveType::String(text_ind);
        return Ok(Some(Primitive { primitive_type, start_loc, end_loc }))
    }

    Ok(None)
}

fn parse_char_symbol(
    input:&mut Input,
    // texts:&mut Vec<String>,
    text_map:&mut HashMap<String,usize>,
) -> Option<Primitive> {
    if let Some(x)=input.hasc(0, [
        '!','&','|','*','-','+','=','<','>','/',',','.',';',':','(',')','[',']','{','}',
        '~','`','@','#','$','%','^','?','_',
    ],[]) { //'?','^',
        let start_loc=input.loc();

        let text_map_size=text_map.len();
        let text_ind=*text_map.entry(x.to_string()).or_insert(text_map_size);

        input.next(1);
        let end_loc=input.loc();
        // Some((text_ind,start_loc,end_loc))

        let primitive_type=PrimitiveType::Symbol(text_ind);
        Some(Primitive { primitive_type, start_loc, end_loc, })
    } else {
        None
    }
}


fn parse_ident_symbol(
    input:&mut Input,
    // texts:&mut Vec<String>,
    text_map:&mut HashMap<String,usize>,
) -> Option<Primitive> {

    let mut i=0;

    if input.hasc(i,['_'],[]).is_some() {
        i+=1;
    }

    if input.hasc(i,[],['a'..='z','A'..='Z']).is_some() {
        i+=1;
    }

    if i==0 || (i==1 && input.hasc(i,['_'],[]).is_some()) {
        return None;
    }

    while input.hasc(i,['_'],['a'..='z','A'..='Z','0'..='9']).is_some() {
        i+=1;
    }

    //
    let start_loc=input.loc();
    let val=input.get(0, i).unwrap().to_string();

    let text_map_size=text_map.len();
    let text_ind=*text_map.entry(val).or_insert(text_map_size);

    input.next(i);
    let end_loc=input.loc();
    let primitive_type=PrimitiveType::Identifier(text_ind);
    Some(Primitive { primitive_type, start_loc, end_loc })
}

fn parse_eol(input:&mut Input) -> Option<Primitive> {
    if let Some(x)=input.has(0, ["\r\n","\n"]) {
        input.next(x.len());
        let primitive_type=PrimitiveType::Eol;
        Some(Primitive { primitive_type, start_loc: input.prev_loc(), end_loc: input.loc() })
    } else {
        None
    }
}

fn parse_cmnt(input:&mut Input) -> Result<bool,ParseError> {
    if let Some(x)=input.has(0, ["//"]) {
        input.next(x.len());

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

        return Ok(true);
    } else if let Some(x)=input.has(0, ["/*"]) {
        input.next(x.len());

        loop {
            if let Some(x)=input.has(0, ["*/"]) {
                input.next(x.len());
                break;
            }

            if input.is_end() {
                return Err(ParseError {
                    loc: input.loc(),
                    error_type: ParserErrorType::ClosingCommentExpected,
                });
            }

            input.next(1);
        }
    }

    Ok(false)
}

fn parse_space(input:&mut Input) -> bool {
    let mut found=false;

    while let Some(x)=input.has(0, [" ","\t"]) { //,"\\\r\n","\\\n"
        input.next(x.len());
        found=true;
    }

    found
}

fn parse_ws(input:&mut Input) -> Result<Option<Primitive>,ParseError> {
    let mut first_end : Option<Primitive> = None;

    while !input.is_end() {

        if parse_cmnt(input)? || parse_space(input) {
            continue;
        } else if parse_space(input) {
            continue;
        } else if let Some(x)=parse_eol(input) {
            if let Some(y)=&mut first_end {
                y.end_loc=x.end_loc;
            } else {
                first_end=Some(x);

            }
            // if first_end.is_none() {
            // }
            continue;
        }
        // else if input.is_end() {
        //     if first_end.is_none() {
        //         first_end=Some(Primitive { primitive_type: PrimitiveType::End, start_loc: input.loc(), end_loc: input.loc() });
        //     }
        // }

        break;

    }

    Ok(first_end)
}