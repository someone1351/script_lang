use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;
use super::super::super::super::build::JmpCond;



pub fn or_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    if record.params_num() < 2 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    //
    builder
        .decl_anon_var("x", false)
        .block_start(None);

    //
    for i in 1 .. record.params_num() {
        let cond=record.param(i).unwrap().as_primitive();

        builder
            .eval(cond)
            .set_anon_var("x")
            .param_push()
            .call_method("not", 1)
            .to_block_end(JmpCond::False //True
                ,0)
            ;
    }

    //
    builder
        .block_end()
        .get_anon_var("x")
        ;

    //
    Ok(())
}
