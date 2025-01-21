
// use super::super::super:Builder;

// use super::super::error::*;

use super::super::builder::*;
use super::super::BuilderErrorType;

use super::super::super::cexpr_parser::*;

pub fn add_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    //add 1
    //add 1 2
    //+ 1
    //+ 1 2
    //{+ 1 2}
    
    if record.params_num() == 1 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    //
    builder.loc(record.param(1).unwrap().start_loc());
    builder.eval(record.param(1).unwrap().primitive());
    
    for i in 2 .. record.params_num() {
        let x=record.param(i).unwrap().primitive();
        
        builder.loc(x.start_loc());

        builder
            .param_push() //push last result
            .eval(x)
            .param_push()
            .swap()
            .call_method("+",2); //,loc
    }

    Ok(())
}
