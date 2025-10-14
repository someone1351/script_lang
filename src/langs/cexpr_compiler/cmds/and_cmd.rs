
// use crate::JmpCond;
use super::super::super::super::common::JmpCond;
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

pub fn and_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() < 2 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    //
    builder.block_start(None);

    //
    for i in 1 .. record.params_num() {
        let cond=record.param(i).unwrap().primitive();

        builder
            .eval(cond)
            .param_push() //push result
            .call_method("not", 1)
            .to_block_end(
                JmpCond::True //False //Some(false)
                ,0)
            ;
    }

    //
    builder.block_end();

    //
    Ok(())
}