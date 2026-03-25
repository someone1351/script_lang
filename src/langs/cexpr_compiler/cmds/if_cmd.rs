use crate::JmpCond;

use super::super::super::cexpr_parser::*;
use super::super::super::super::compiler::builder::*;
use super::super::error::*;

pub fn if_cmd<'a>(primitives : &mut PrimitiveIterContainer<'a>, builder :&mut Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    let mut vs = Vec::new();

    //
    {
        let cond0=primitives.pop_front().and_then(|p|p.get_parenthesis())
            .or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedParenthesis }))?;
        let body0= primitives.pop_front().and_then(|p|p.get_curly())
            .or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedCurlyBraces }))?;

        vs.push((Some(cond0),body0));
    }

    //
    while let Ok(pre)=primitives.first().and_then(|p|p.has_identifiers(["elif","else"])).map(|v|v.value) {
        match pre {
            "elif" => {
                let cond1=primitives.pop_front().and_then(|p|p.get_parenthesis())
                    .or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedParenthesis }))?;
                let body1=primitives.pop_front().and_then(|p|p.get_curly())
                    .or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedCurlyBraces }))?;

                vs.push((Some(cond1),body1));
            }
            "else" => {
                let body2=primitives.pop_front().and_then(|p|p.get_curly())
                    .or_else(|loc|Err(BuilderError{ loc, error_type: BuilderErrorType::ExpectedCurlyBraces }))?;

                vs.push((None,body2));
                break;
            }
            _ => {panic!("");}
        }
    }

    //
    builder.block_start(None);

    for (cond,body) in vs {
        if let Some(cond)=cond {
            builder
                .block_start(None)
                    .eval(cond.value)
                    .to_block_end(JmpCond::False,0)
                    .eval(body.value)
                    .to_block_end(JmpCond::None,1)
                .block_end()
            ;
        } else {
            builder.eval(body.value);
        }
    }

    builder.block_end();

    //
    Ok(())
}