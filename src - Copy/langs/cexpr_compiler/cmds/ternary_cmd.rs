use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;



pub fn ternary_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() != 4 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    let cond=record.param(1).unwrap().primitive();
    let x=record.param(2).unwrap().primitive();
    let y=record.param(3).unwrap().primitive();
    
    //
    builder
        .block_start(None)
            .block_start(None)
                .eval(cond)
                .to_block_end(Some(false),0)
                .eval(x)
                .to_block_end(None,1)
            .block_end()
            .eval(y)
        .block_end();

    
    //
    Ok(())
}
