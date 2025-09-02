
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;


pub fn dict_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {


    //
    builder
        .block_start(None)
        .call_method("dict", 0)
        .decl_anon_var("d", false)
        .set_anon_var("d")
        // .param_push()
        ;

    //
    for i in 1..record.params_num() {
        let param=record.param(i).unwrap();

        if param.fields_num() != 0 {
            return Err(BuilderError::new(param.start_loc(), BuilderErrorType::NoFieldsAllowed));
        }

        let Some(block)=param.primitive().block() else {
            return Err(BuilderError::new(param.start_loc(), BuilderErrorType::ExpectBlock));
        };

        let mut c=0;

        for record in block.records() {
            if let Some(x)=record.semi_colon_loc() {
                return Err(BuilderError::new(x, BuilderErrorType::NoSemiColonsAllowed));
            }

            for param2 in record.params() {
                if c > 1 {
                    return Err(BuilderError::new(param2.start_loc(), BuilderErrorType::IncorrectParamsNum));
                }

                builder
                    .eval(param2.primitive())
                    .param_push()
                    ;
                c+=1;
            }
        }

        //
        if c==0 {
            return Err(BuilderError::new(block.start_loc(), BuilderErrorType::IncorrectParamsNum));
        }

        if c==1 {
            builder
                .result_nil()
                .param_push()
                ;
        }

        //
        builder
            .swap()
            .get_anon_var("d")
            .param_push()
            .call_method("insert", 3)
            ;
    }

    //
    builder
        .get_anon_var("d")
        .block_end();

    Ok(())
}