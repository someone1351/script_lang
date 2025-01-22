use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

use super::get_idn;

pub fn var_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    //var x 123
    //var x

    if record.params_num() < 2 || record.params_num() > 3 {
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    // let mut i=1;

    // while i<record.params_num() {
    //     let idn = get_idn(record.param(i).unwrap())?;

    //     if Some(",")==record.param(i+1).and_then(|x|x.primitive().string()) {

    //     }


    // }

    let idn = get_idn(record.param(1).unwrap())?;

    let is_init_nil=record.params_num()==2;

    builder.decl_var_start(idn,is_init_nil);

    if !is_init_nil {
        let val_expr = record.param(2).unwrap().primitive();
        builder.eval(val_expr);
    }

    builder.decl_var_end();

    if !is_init_nil {
        builder.set_var(idn);
    }
    
    Ok(())
}
