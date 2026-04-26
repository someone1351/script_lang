
pub mod rules;
pub mod error;

use std::collections::HashSet;

use crate::clike::grammar::data::Walk;
use crate::clike::grammar::GrammarWalkError;
use crate::clike::parser::error::ParserErrorType;
use crate::clike::parser::rules::is_keyword;
use crate::clike::tokenizer::{self, tokenize, Tokenized, TokenizerErrorType};
use crate::Loc;
use error::ParserError;

use super::parser::rules::get_non_term;
use super::grammar::walker::GrammarWalker;
use super::tokenizer::TokenIterContainer;

pub struct Parsed<'g> {
    tokenized:Tokenized,
    walk:Walk<'g>,
}

pub fn parse<'t,'g>( src:&'t str) -> Result<Parsed<'g>,ParserError>{

    /*
    abc|ab with "ab" => "" //abc will fail, but then tries ab, which succeeds
    ab|abc with "abc" => "c" //will consume ab, and then fail to consume c, there is no backtracking
     */

    //expects not completely correct,
    //  if succeeds to a certain point, need to clear the expecteds,
    //  currently it just clears everything after any success
    //  could just add expects that are ==, and replace ones that are >
    //  on success, only clear if >= than loc





    // // if let Err(e)=parsed {
    // //     return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::Tokenizer(e.error_type)});
    // // }

    let tokenized=tokenize(src, is_keyword );

    let Ok(tokenized)=tokenized else {
        let e=tokenized.err().unwrap();

        match e.error_type {
            TokenizerErrorType::Unexpected => {
                panic!("TokenizerErrorType::Unexpected");
            }
            _ => {
                return Err(ParserError { loc: e.loc, error_type: ParserErrorType::Tokenizer(e.error_type) });
            }
        }
    };

    //
    let mut walker=GrammarWalker::new(tokenized.tokens(), get_non_term,);
    // // walker.set_debug(true);

    if let Err(e)=walker.run("start") {
        match e {
            GrammarWalkError::FailedParse => {
                return Err(ParserError{ loc: walker.last_loc(), error_type: ParserErrorType::Expected(walker.expecteds_string()) });
            }
            _ => {
                // println!("{:?} {:?}",walker.expecteds_string(),walker.last_loc());
                panic!("{e:?}");
            }
        }
    }
    let walk=walker.get_walk();

    Ok(Parsed{ tokenized, walk })



    // println!("{}",walk.root());

    // let mut top_primitives=top_primitives;
    // println!("====");
    // println!("{top_primitives:?}",);

    // let a=top_primitives.pop_front_amount(10);

    // println!("=\n{top_primitives:?}\n=\n{a:?}");

    // let a=top_primitives.pop_front_amount(10);
    // println!("=\n{top_primitives:?}\n=\n{a:?}");

    // Err(ParserError { loc: Loc::zero(), error_type: ParserErrorType::Expected(String::new()) })
}
