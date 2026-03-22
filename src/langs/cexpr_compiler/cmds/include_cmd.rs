use super::super::super::cexpr_parser::*;
use super::super::super::super::compiler::builder::*;
use super::super::error::*;

pub fn include_cmd<'a>(primitives : &mut PrimitiveIterContainer<'a>, builder :&mut Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    let include_param = primitives.pop_front().unwrap(); //include

    let path_param=primitives.pop_front()
        .ok_or_else(||BuilderError{ loc: include_param.end_loc(), error_type: BuilderErrorType::ExpectedString })?;

    let Some(path_param_str) = path_param.get_string() else {
        return Err(BuilderError{ loc: path_param.end_loc(), error_type: BuilderErrorType::ExpectedString });
    };

    //todo path_param_str

    Ok(())
}