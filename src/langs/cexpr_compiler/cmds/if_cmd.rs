use super::super::super::cexpr_parser::*;
use super::super::super::super::compiler::builder::*;
use super::super::error::*;

pub fn if_cmd<'a>(primitives : &mut PrimitiveIterContainer<'a>, builder :&mut Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {

    let cond0=primitives.pop_front().and_then(|p|p.get_parenthesis());
    let cond0=cond0.or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedParenthesis }))?;


    let body0= primitives.pop_front().and_then(|p|p.get_curly());
    let body0=body0.or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedCurlyBraces }))?;

    loop {
        let t=primitives.get(0);
        let t=t.and_then(|p|p.identifier_eq("elif"));

        let cond0= primitives.pop_front();
        let cond0=cond0.and_then(|p|p.get_parenthesis());
        let cond0=cond0.or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedParenthesis }))?;

        let body0_primitive= primitives.pop_front();
        let body0=body0_primitive.and_then(|p|p.get_curly());
        let body0=body0.or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedCurlyBraces }))?;

    }

    // let if_param = primitives.pop_front().unwrap(); //if

    // let cond_param=primitives.pop_front()
    //     .ok_or_else(||BuilderError{ loc: if_param.end_loc(), error_type: BuilderErrorType::ExpectedParenthesis })?;

    // if !cond_param.is_parentheses() {
    //     return Err(BuilderError{ loc: cond_param.start_loc(), error_type: BuilderErrorType::ExpectedParenthesis });
    // }

    // //todo cond

    // let body0_param=primitives.pop_front()
    //     .ok_or_else(||BuilderError{ loc: if_param.end_loc(), error_type: BuilderErrorType::ExpectedCurlyBraces })?;

    // if !body0_param.is_curly() {
    //     return Err(BuilderError{ loc: body0_param.start_loc(), error_type: BuilderErrorType::ExpectedCurlyBraces });
    // }

    // //todo body0

    // loop {
    //     let Some(rest_cond_param) = primitives.first() else {break;};

    //     if rest_cond_param.get_symbol()==Some("elif") {
    //         primitives.pop_front();

    //         let body1_param=primitives.pop_front()
    //             .ok_or_else(||BuilderError{ loc: if_param.end_loc(), error_type: BuilderErrorType::ExpectedCurlyBraces })?;

    //         if !body1_param.is_curly() {
    //             return Err(BuilderError{ loc: if_param.start_loc(), error_type: BuilderErrorType::ExpectedCurlyBraces });
    //         }
    //         //todo body1
    //     } else if rest_cond_param.get_symbol()==Some("else") {
    //         primitives.pop_front();

    //         let body1_param=primitives.pop_front()
    //             .ok_or_else(||BuilderError{ loc: if_param.end_loc(), error_type: BuilderErrorType::ExpectedCurlyBraces })?;

    //         if !body1_param.is_curly() {
    //             return Err(BuilderError{ loc: if_param.end_loc(), error_type: BuilderErrorType::ExpectedCurlyBraces });
    //         }

    //         //todo body1

    //         break;
    //     } else {
    //         break;
    //     }
    // }

    Ok(())
}