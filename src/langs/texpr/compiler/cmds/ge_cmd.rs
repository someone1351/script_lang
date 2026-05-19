
use super::super::builder::*;
use super::super::super::parser::*;
use super::super::BuilderErrorType;

pub fn ge_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() != 3 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    let loc = record.param(0).unwrap().start_loc();

    builder
        .eval(record.param(2).unwrap().as_primitive())
        .param_push()
        .eval(record.param(1).unwrap().as_primitive())
        .param_push()
        .loc(loc)
        .call_method("ge", 2)
        ;

    Ok(())
}