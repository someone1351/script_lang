use super::super::super::cexpr_parser::*;
use super::super::super::super::compiler::builder::*;
use super::super::error::*;

pub fn for_cmd<'a>(primitives : &mut PrimitiveIterContainer<'a>, builder :&mut Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {

    let cond_param=primitives.pop_front().and_then(|p|p.get_parenthesis());
    let cond_param=cond_param.or_else(|loc|Err(BuilderError{loc,error_type:BuilderErrorType::ExpectedParenthesis}))?;

    let body_param=primitives.pop_front().and_then(|p|p.get_curly());
    let body_param=body_param.or_else(|loc|Err(BuilderError{loc,error_type:BuilderErrorType::ExpectedCurlyBraces}))?;

    //todo cond_param
    //todo body_param

    Ok(())
}