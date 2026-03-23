use super::super::super::cexpr_parser::*;
use super::super::super::super::compiler::builder::*;
use super::super::error::*;

pub fn func_cmd<'a>(primitives : &mut PrimitiveIterContainer<'a>, builder :&mut Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    //fn idn(a,b ...) {}

    let idn=primitives.pop_front().and_then(|p|p.get_identifier());
    let idn=idn.or_else(|loc|Err(BuilderError{loc,error_type:BuilderErrorType::ExpectedIdentifier}))?;

    let params=primitives.pop_front().and_then(|p|p.get_parenthesis());
    let params=params.or_else(|loc|Err(BuilderError{loc,error_type:BuilderErrorType::ExpectedParenthesis}))?;

    let body=primitives.pop_front().and_then(|p|p.get_parenthesis());
    let body=body.or_else(|loc|Err(BuilderError{loc,error_type:BuilderErrorType::ExpectedCurlyBraces}))?;

    Ok(())
}