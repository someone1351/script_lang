
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;


pub fn sub_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() == 1 {
        // return Err(ParseError { loc: sexpr.start_loc(), msg: "Incorrect params num".to_string() });
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    builder.eval(record.param(1).unwrap().primitive());
    let loc = record.param(0).unwrap().start_loc();
    
    builder.loc(loc);

    if record.params_num()==2 {
        builder
            .param_push()
            .call_method("-",1); //,loc       
    } else {
        for i in 2 .. record.params_num() {
            builder
                .param_push() //push last result
                .eval(record.param(i).unwrap().primitive())
                .param_push()
                .swap()
                .call_method("-",2); //,loc
        }
    }

    Ok(())
}