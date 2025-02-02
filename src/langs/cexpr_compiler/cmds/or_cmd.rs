use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;
use super::super::super::super::common::JmpCond;



pub fn or_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
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
            .to_block_end(JmpCond::True,0)
            ;
    }

    //
    builder.block_end();
    
    //
    Ok(())
}
