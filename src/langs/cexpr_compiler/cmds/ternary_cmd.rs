use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;
use super::super::super::super::common::JmpCond;



pub fn ternary_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() != 4 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    let cond=record.param(1).unwrap().as_primitive();
    let x=record.param(2).unwrap().as_primitive();
    let y=record.param(3).unwrap().as_primitive();

    //
    builder
        .block_start(None)
            .block_start(None)
                .eval(cond)
                .param_push()
                .call_method("not", 1)
                .to_block_end(JmpCond::True //False
                    ,0)
                .eval(x)
                .to_block_end(JmpCond::None,1)
            .block_end()
            .eval(y)
        .block_end();


    //
    Ok(())
}
