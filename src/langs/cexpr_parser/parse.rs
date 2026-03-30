
pub use super::error::*;

use std::collections::HashMap;

use super::data::*;

use super::input::*;


use super::super::super::build::Loc;

#[derive(Clone, Copy, PartialEq,Eq)]
enum BlockBracket {
    Curly,Parentheses,Square,
}

pub fn parse<'a>(src:&'a str, ) -> Result<Parsed,ParseError> {

    let mut input = Input::new(src);
    let mut text_map = HashMap::<String,usize>::new();
    text_map.insert("".to_string(), 0); //used in primitive container, when converting box primitive to string primitive


    // let mut blocks : Vec<Block> = vec![Block{ self_primitive: todo!(), primitives: todo!() }];
    // let mut primitives : Vec<Primitive> = Default::default();

    let mut block_primitives : Vec<Vec<Primitive>> = vec![Vec::new()]; //starts with root prims pushed
    let mut block_inner_locs : Vec<(Loc,Loc)> = vec![(Loc::zero(),Loc::zero())]; //init for root, but not used, just for simplicity
    let mut block_stk: Vec<(usize,BlockBracket,Loc)> = vec![]; //(0,None,Loc::one())

    let mut some=false;

    //
    loop {
        let cur_block_ind=block_stk.last().map(|x|x.0).unwrap_or(0);

        let cur_block_primitives=block_primitives.get_mut(cur_block_ind).unwrap();

        //
        if let Some(primitive)=parse_ws(&mut input)? {
            cur_block_primitives.push(primitive); //end
            some=true; // println!("yes1");

        }

        //
        if let Some(primitive)=parse_string(&mut input,&mut text_map)? {
            cur_block_primitives.push(primitive);
            some=true; // println!("yes2");
        } else if let Some(primitive)=parse_number(&mut input, true, &mut text_map) {
            cur_block_primitives.push(primitive);
            some=true; // println!("yes3");
        } else if let Some(primitive)=parse_char_symbol(&mut input, &mut text_map) {
            cur_block_primitives.push(primitive);
            some=true; // println!("yes4");
        } else if let Some(primitive)=parse_ident_symbol(&mut input, &mut text_map) {
            cur_block_primitives.push(primitive);
            some=true; // println!("yes5");
        } else if let Some((bracket,start_loc,end_loc))=parse_block_begin(&mut input) {
            // let cur_block_primitives=block_primitives.get_mut(cur_block_ind).unwrap();


            // cur_block_primitives.push(Primitive { primitive_type, start_loc, end_loc: Loc::zero() });

            block_stk.push((block_primitives.len(),bracket,start_loc));
            block_inner_locs.push((end_loc,Loc::zero()));
            block_primitives.push(Vec::new());
            some=true; // println!("yes6");

        } else if let Some((bracket,start_loc,end_loc))=parse_block_end(&mut input) {
            //
            if block_stk.is_empty() {
                let error_type=match bracket {
                    BlockBracket::Curly => ParserErrorType::NoMatchingOpeningBracket("{"),
                    BlockBracket::Parentheses => ParserErrorType::NoMatchingOpeningBracket("("),
                    BlockBracket::Square => ParserErrorType::NoMatchingOpeningBracket("["),
                };

                return Err(ParseError { loc: start_loc, error_type});
            }

            //
            let last_bracket=block_stk.last().unwrap().1;

            //
            if last_bracket!=bracket {
                let error_type=match last_bracket {
                    BlockBracket::Curly => ParserErrorType::ClosingBracketExpected("}"),
                    BlockBracket::Parentheses => ParserErrorType::ClosingBracketExpected(")"),
                    BlockBracket::Square => ParserErrorType::ClosingBracketExpected("]"),
                };

                return Err(ParseError { loc: start_loc, error_type });
            }

            // //
            // if let Some(PrimitiveType::End)=cur_block_primitives.last().map(|p|p.primitive_type.clone()) {
            //     //
            // } else {
            //     cur_block_primitives.push(Primitive { primitive_type: PrimitiveType::End, start_loc, end_loc: start_loc });
            // }

            //
            cur_block_primitives.push(Primitive { primitive_type: PrimitiveType::Eob, start_loc, end_loc: start_loc });

            //
            block_stk.pop();

            //
            let block_ind=cur_block_ind;

            //
            let primitive_type=match bracket {
                BlockBracket::Curly => PrimitiveType::CurlyBlock(block_ind),
                BlockBracket::Parentheses => PrimitiveType::ParenthesesBlock(block_ind),
                BlockBracket::Square => PrimitiveType::SquareBlock(block_ind),
            };

            //
            let start_loc=block_stk.last().map(|x|x.2).unwrap_or(Loc::one());

            //
            let cur_block_ind=block_stk.last().map(|x|x.0).unwrap_or(0);
            let cur_block_primitives=block_primitives.get_mut(cur_block_ind).unwrap();

            cur_block_primitives.push(Primitive { primitive_type, start_loc, end_loc });
            some=true; // println!("yes7");


            let cur_block_inner_locs = block_inner_locs.get_mut(cur_block_ind).unwrap();
            cur_block_inner_locs.1=start_loc;

        } else if input.is_end() {
            // match cur_block_primitives.last().map(|p|p.primitive_type.clone()) {
            //     Some(PrimitiveType::Eob) => {

            //     }
            //     Some(PrimitiveType::Eol) => {
            //         cur_block_primitives.push(Primitive { primitive_type: PrimitiveType::Eob, start_loc:input.loc(), end_loc: input.loc() });
            //     }
            //     _ => {
            //         cur_block_primitives.push(Primitive { primitive_type: PrimitiveType::Eob, start_loc:input.loc(), end_loc: input.loc() });
            //     }
            // }
            // // if let Some(PrimitiveType::End)=cur_block_primitives.last().map(|p|p.primitive_type.clone()) {
            // //     //
            // // } else {
            // //     cur_block_primitives.push(Primitive { primitive_type: PrimitiveType::End, start_loc:input.loc(), end_loc: input.loc() });
            // // }

            cur_block_primitives.push(Primitive { primitive_type: PrimitiveType::Eob, start_loc:input.loc(), end_loc: input.loc() });

            break;
        }

        if !some && !input.is_end() {
            return Err(ParseError { loc: input.loc(), error_type: ParserErrorType::Unknown });
        }


        // println!("loc {}, {some}",input.loc());
        some=false;
    }

    //
    let mut out_texts: Vec<String>=vec![String::new();text_map.len()];

    for (s,i) in text_map {
        out_texts[i]=s;
    }

    //
    let mut out_primitives = vec![Primitive{
        primitive_type: PrimitiveType::Root(0),
        start_loc: Loc::one(),
        end_loc: block_primitives[0].last().map(|p|p.end_loc).unwrap_or(Loc::one()),
    }];

    let mut out_blocks=vec![];

    for (block_ind,primitives) in block_primitives.into_iter().enumerate() {
        let primitive_start=out_primitives.len();
        out_primitives.extend(primitives);
        let primitive_end=out_primitives.len();

        let (inner_start_loc,inner_end_loc)=block_inner_locs[block_ind];

        out_blocks.push(Block{ primitives: primitive_start..primitive_end,inner_start_loc,inner_end_loc });
    }

    //
    Ok(Parsed {
        blocks: out_blocks,
        primitives: out_primitives,
        texts: out_texts,
    })
}



fn parse_number(
    input:&mut Input,
    float_aswell:bool,
    text_map:&mut HashMap<String,usize>,
) -> Option<Primitive> {
    let mut i=0;
    let mut is_float=false;
    let mut ok=false;

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
    if let Some(x)=input.has(0, ["!","^","&","|","*","-","+","=","<",">","?","/",",",".",";",":"]) {
        let start_loc=input.loc();

        let text_map_size=text_map.len();
        let text_ind=*text_map.entry(x.to_string()).or_insert(text_map_size);

        input.next(x.len());
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

    if input.hasc(i,['_'],['a'..='z','A'..='Z']).is_some() {
        i+=1;
    }

    while input.hasc(i,['_'],['a'..='z','A'..='Z','0'..='9']).is_some() {
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
        let primitive_type=PrimitiveType::Identifier(text_ind);
        Some(Primitive { primitive_type, start_loc, end_loc })
    }

}


fn parse_block_begin(input:&mut Input) -> Option<(BlockBracket,Loc,Loc)> {
    if let Some(x)=input.has(0, ["{","[","("]) {
        input.next(x.len());

        let b=match x {
            "{" => BlockBracket::Curly,
            "[" => BlockBracket::Square,
            "(" => BlockBracket::Parentheses,
            _ => panic!(""),
        };

        Some((b,input.prev_loc(),input.loc()))
    } else {
        None
    }
}

fn parse_block_end(input:&mut Input) -> Option<(BlockBracket,Loc,Loc)> {

    if let Some(x)=input.has(0, ["}","]",")"]) {
        input.next(x.len());

        let b=match x {
            "}" => BlockBracket::Curly,
            "]" => BlockBracket::Square,
            ")" => BlockBracket::Parentheses,
            _ => panic!(""),
        };

        Some((b,input.prev_loc(),input.loc()))
    } else {
        None
    }
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

// fn parse_semi_colon(input:&mut Input) -> Option<Primitive> {
//     if let Some(x)=input.has(0, [";"]) {
//         input.next(x.len());
//         Some(Primitive { primitive_type: PrimitiveType::Semicolon, start_loc: input.prev_loc(), end_loc: input.loc() })
//     } else {
//         None
//     }
// }



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

    while let Some(x)=input.has(0, [" ","\t",]) { //"\\\r\n","\\\n"
        input.next(x.len());
        found=true;
    }

    found
}

fn parse_ws(input:&mut Input) -> Result<Option<Primitive>,ParseError> {
    let mut first_end : Option<Primitive> = None;
    let mut found=true;

    while found && !input.is_end() {
        found=false;

        if parse_cmnt(input)? || parse_space(input) {
            found=true;
        } else if parse_space(input) {
            found=true;
        } else if let Some(x)=parse_eol(input) {
            found=true;

            if first_end.is_none() {
                first_end=Some(x);
            }
        } else if input.is_end() {
            if first_end.is_none() {
                first_end=Some(Primitive { primitive_type: PrimitiveType::Eob, start_loc: input.loc(), end_loc: input.loc() });
            }
        }

    }

    Ok(first_end)
}