use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

use super::get_idn;

pub fn for_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    //for {i 0 n} {}
    //for i 0 n {}
    if record.params_num() != 5 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    //
    let idn = get_idn(record.param(1).unwrap())?;
    let from = record.param(2).unwrap().primitive();
    let to = record.param(3).unwrap().primitive();
    let body = record.param(4).unwrap().primitive();
    
    // let Some(body_block)=body.block() else {
    //     return Err(BuilderError::new(body.start_loc(), BuilderErrorType::ExpectBlock));
    // };

    //
    
    builder
        .block_start(None)
            .decl_var_start(idn, false)

                //why do these inside idn decl?
                .eval(to)
                .decl_anon_var("n", false)
                .set_anon_var("n")

                .decl_anon_var("i", false)
                .result_void() //why set to void?? oh "i" is the value of the body's return
                .set_anon_var("i")

            .decl_var_end()

            .eval(from)
            .set_var(idn)
            
            .block_start(Some("loop"))
                .block_start(None)
                    .get_anon_var("n")
                    .param_push()
                    .get_var(idn) //shouldn't this be anon var i?
                    .param_push()
                    .call_method("<", 2)
                    // .to_block_end_label(Some(false),"loop", None)
                    .to_block_end(Some(false), 1)

                    .result_void()
                    .eval(body)
                    .set_anon_var("i") //
                .block_end()

                .result_int(1)
                .param_push()
                .get_var(idn)
                .param_push()
                .call_method("+", 2)

                .set_var(idn)

                .to_block_start(None,0)
            .block_end()
            .get_anon_var("i")
        .block_end()
        ;

    Ok(())
}
