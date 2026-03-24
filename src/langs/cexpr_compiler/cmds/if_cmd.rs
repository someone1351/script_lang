use super::super::super::cexpr_parser::*;
use super::super::super::super::compiler::builder::*;
use super::super::error::*;

pub fn if_cmd<'a>(primitives : &mut PrimitiveIterContainer<'a>, builder :&mut Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {

    let cond0=primitives.pop_front().and_then(|p|p.get_parenthesis())
        .or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedParenthesis }))?;
    let body0= primitives.pop_front().and_then(|p|p.get_curly())
        .or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedCurlyBraces }))?;

    while let Ok(pre)=primitives.first().and_then(|p|p.has_identifiers(["elif","else"])).map(|v|v.value) {
        match pre {
            "elif" => {
                let cond1=primitives.pop_front().and_then(|p|p.get_parenthesis())
                    .or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedParenthesis }))?;
                let body1=primitives.pop_front().and_then(|p|p.get_curly())
                    .or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedCurlyBraces }))?;

            }
            "else" => {
                let body2=primitives.pop_front().and_then(|p|p.get_curly())
                    .or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedCurlyBraces }))?;

                break;
            }
            _ => {panic!("");}
        }

    }

    Ok(())
}