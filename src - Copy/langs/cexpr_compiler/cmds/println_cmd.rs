
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

use super::print_cmd::*;

pub fn println_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    print_cmd(record,builder)?;

    builder
        .result_string("\n")
        .param_push()
        .call_method("stdout", 1)
        ;
    Ok(())
}
