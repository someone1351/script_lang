
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;
use super::super::super::super::common::JmpCond;

pub fn while_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    //while true {}
    //while {true} {}

    if record.params_num() != 3 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    let cond_expr = record.param(1).unwrap().primitive();
    let body=record.param(2).unwrap().primitive();

    if body.block().is_none() {
        return Err(BuilderError::new(body.start_loc(), BuilderErrorType::ExpectBlock));
    }

    // let body_stmts = record.list_iter_from(2);

    // let c = Compiler::<'a>::new()
    builder
        // .loop_instr()
        .block_start(Some("loop"))
            .eval(cond_expr)
            .param_push()
            .call_method("not", 1)
            .to_block_end(JmpCond::True //False
                ,0)
            // .eval_sexprs(body_stmts)
            .eval(body)
            .to_block_start(JmpCond::None,0)
        .block_end()
    // .value_instr(&Value::Void)
    ;

    Ok(())
}