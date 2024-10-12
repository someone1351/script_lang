
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;


pub fn call_func_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() == 1 {
        return Err(BuilderError::new(record.start_loc(), BuilderErrorType::InvalidParam));
    }

    let func=record.param(1).unwrap().primitive();

    for i in (2..record.params_num()).rev() {
        let param: PrimitiveContainer<'a>=record.param(i).unwrap().primitive();
        
        builder
            .loc(param.start_loc())
            .eval(param)
            .param_push();
    }

    builder
        .loc(func.start_loc())
        .eval(func)
        .call_result(record.params_num()-2)
        ;

    Ok(())
}
