
use crate::compiler::builder::*;
use super::super::super::parser::*;
use super::super::BuilderErrorType;
use crate::build::JmpCond;

pub fn break_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() != 1 {
        return Err(BuilderError::new(record.start_loc(), BuilderErrorType::NoParamsAllowed));
    }

    let e = BuilderError::new(record.start_loc(), BuilderErrorType::BreakNotInLoop);
    let skip=builder.get_flag("in_loop_cond").is_some();
    let skip = if skip {1} else {0};
    builder.to_block_end_label(JmpCond::None,"loop",skip,Some(e));

    Ok(())
}
