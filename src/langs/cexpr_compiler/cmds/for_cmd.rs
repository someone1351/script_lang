use super::super::super::cexpr_parser::*;
use super::super::super::super::compiler::builder::*;
use super::super::error::*;

pub fn for_cmd<'a>(primitives : &mut PrimitiveIterContainer<'a>, builder :&mut Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    let for_param = primitives.pop_front().unwrap(); //if

    let cond_param=primitives.pop_front()
        .ok_or_else(||BuilderError{ loc: for_param.end_loc(), error_type: BuilderErrorType::ExpectedParenthesis })?;

    if !cond_param.is_parentheses() {
        return Err(BuilderError{ loc: cond_param.start_loc(), error_type: BuilderErrorType::ExpectedParenthesis });
    }
    Ok(())
}