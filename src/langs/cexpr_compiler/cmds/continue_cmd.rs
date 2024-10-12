
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;


pub fn continue_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() != 1 {
        return Err(BuilderError::new(record.start_loc(), BuilderErrorType::NoParamsAllowed));
    }

    let e=BuilderError::new(record.start_loc(), BuilderErrorType::ContinueNotInLoop);
    builder.to_block_start_label(None,"loop",Some(e));

    Ok(())
}