
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

use super::format_cmd::*;

pub fn print_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    format_cmd(record,builder)?;
    builder
        .param_push()
        .call_method("stdout", 1);
    Ok(())
}