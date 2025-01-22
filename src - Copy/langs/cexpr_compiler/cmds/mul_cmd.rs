
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;


pub fn mul_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() == 1 {
        // return Err(ParseError { loc: sexpr.start_loc(), msg: "Incorrect params num".to_string() });
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    //a*b*c*d = a*(b*(c*d))
    
    builder.eval(record.last_param().unwrap().primitive());
    let loc = record.param(0).unwrap().start_loc();

    builder.loc(loc);

    for i in (1 .. record.params_num()-1).rev() {
        builder
            .param_push() //push last result
            .eval(record.param(i).unwrap().primitive())
            .param_push()
            // .swap()
            .call_method("*",2); //,loc
    }

    Ok(())
}