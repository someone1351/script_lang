

use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;
use super::super::super::super::common::JmpCond;



pub fn if_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    //if cond0 body0
    //if cond0 body0 else body1
    //if cond0 body0 elif cond1 body1
    //if cond0 body0 elif cond1 body1 else body0

    //eg (if [COND0 BODY0*]+ [else BODYELSE?]?)

    if record.params_num() < 3 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    // || (record.len()-3)%2 !=0

    //
    let mut conds = Vec::new();
    let mut bodies = Vec::new();

    //
    {
        conds.push(record.param(1).unwrap().primitive());
        bodies.push(record.param(2).unwrap().primitive());

        let mut i=3;

        while i!=record.params_num(){
            let if_type=record.param(i).unwrap().primitive();

            match if_type.symbol() {
                Some("elif") => {
                    if i+2>=record.params_num() {
                        return Err(BuilderError::new(if_type.end_loc(), BuilderErrorType::IncorrectParamsNum));
                    }

                    let cond=record.param(i+1).unwrap().primitive();
                    let body=record.param(i+2).unwrap().primitive();
                    conds.push(cond);
                    bodies.push(body);
                    i+=3;
                }
                Some("else") => {
                    if i+1==record.params_num() {
                        return Err(BuilderError::new(if_type.end_loc(), BuilderErrorType::IncorrectParamsNum));
                    }

                    let body=record.param(i+1).unwrap().primitive();
                    bodies.push(body);
                    i+=2;

                    if i!=record.params_num() {
                        return Err(BuilderError::new(body.end_loc(), BuilderErrorType::IncorrectParamsNum));
                    }
                }
                Some(_)|None => {
                    return Err(BuilderError::new(if_type.start_loc(), BuilderErrorType::InvalidParam));
                }
            }
        }
    }


    //
    builder.block_start(None);

    for (i,&body) in bodies.iter().enumerate() {
        if let Some(&cond)=conds.get(i) {
            builder
                .block_start(None)
                    .eval(cond)
                    .param_push()
                    .call_method("not", 1)
                    .to_block_end(JmpCond::True //False
                        ,0)
                    .eval(body)
                    .to_block_end(JmpCond::None,1)
                .block_end()
            ;
        } else {
            builder.eval(body);
        }
    }

    builder.block_end();

    Ok(())
}