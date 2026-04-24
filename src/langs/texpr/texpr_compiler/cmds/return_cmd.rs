
use super::super::builder::*;
use super::super::super::texpr_parser::*;
use super::super::BuilderErrorType;
use crate::build::JmpCond;



pub fn return_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() > 2 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    if record.params_num()==2 {
        builder.eval(record.param(1).unwrap().as_primitive());
    } else {
        builder.result_void();
    }

    let e = BuilderError::new(record.start_loc(), BuilderErrorType::ReturnNotInMethodOrLambda);
    builder.to_block_end_label(JmpCond::None, "func",0,Some(e));

    Ok(())
}