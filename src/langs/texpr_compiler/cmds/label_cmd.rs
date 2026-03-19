
use crate::texpr_compiler::cmds::get_idn;

use super::super::builder::*;
use super::super::super::texpr_parser::*;
use super::super::BuilderErrorType;

pub fn label_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() != 2{
        return Err(BuilderError::new(record.start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    let label=get_idn(record.param(1).unwrap())?;
    builder.label(label);

    Ok(())
}
