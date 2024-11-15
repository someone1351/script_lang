
use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

use super::get_func_params;

pub fn lambda_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    //(fn (a b c) (something a) (+ b c))
    //(fn (a b c ...) (something c) (+ b a))
    //(fn () (something))
    //(fn () )

    //{fn a b c {}}
    //lambda a b c {}
    //{lambda a b c {}}
    /*
    
    var a {lambda a b {
        + a b
    }}

    var a {fn a b {
        + a b
    }}
    
    var a {fn {a b} {
        + a b
    }}
    
    var a {fn {a 
               b} {
        + a b
    }}
    var a {fn a b ... {
        + a b
    }}
    
    var a {fn {a b ...} {
        + a b
    }}

    var a {lambda x 1}
    var a {lambda 1}
    */

    // if record.primitives_num() < 2 {
    //     return Err(BuilderError::new(record.last().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    // }

    //fn {a} {body}

    if record.params_num() != 3 {
        // println!("==== is {}",record.primitives_num());
        return Err(BuilderError::new(record.first_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    //params
    let params_primitive=record.param(1).unwrap().primitive();
    
    let Some(params_block)=params_primitive.block() else {
        return Err(BuilderError::new(params_primitive.start_loc(), BuilderErrorType::IncorrectParamsNum));
    };

    // let (params,variadic)=get_func_typed_params(sexpr.get(2).unwrap(), builder)?;
    let (params,variadic)=get_func_params(params_block)?;

    //
    let body=record.last_param().unwrap().primitive();

    //params
    // let (params,variadic)=get_func_params(record,1, record.primitives_num()-1)?;

    //
    builder
        .func_start(params,variadic)
        // .result_void()
        ;

    //body
    // if record.len() > 2 {
    //     let body_sexprs = record.list_iter_from(2);

        builder
            .block_start(Some("func"))
            .eval(body)
            .block_end();
    // }

    //
    builder.func_end();

    //
    Ok(())
}
