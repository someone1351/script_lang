use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

use super::get_symbol;

pub fn set_var_cmd<'a>(record : RecordContainer<'a>, builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
    //set x 123

    if record.params_num() != 3 {        
        return Err(BuilderError::new(record.last_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }

    //
    let loc = record.start_loc();

    // if let Some(symbol)=record.param(1).unwrap().primitive().symbol() {
    //     if symbol.fields_num()!=0 { //has fields
    //         builder.get_var(symbol.str());
    //     } else {

    //     }

    // } else {

    // }
    
    //idn
    let var_param=record.param(1).unwrap();

    if var_param.fields_num()==0 {

        let idn = get_symbol(var_param)?;
        let idn_loc=record.param(1).unwrap().start_loc();
    
        //val
        let val_sexpr = record.param(record.params_num()-1).unwrap().primitive();
    
        //
    
        builder.loc(loc);
    
        builder
            .loc(idn_loc)
            .eval(val_sexpr)
            .loc(idn_loc)
            .set_var(idn)
            ;
    } else {
        if let Some(idn)=var_param.primitive().symbol() {
            let idn_loc=record.param(1).unwrap().start_loc();
        
            builder
                .loc(idn_loc)
                .get_var(idn);
        } else { //a block, otherwise would be an err, due to nothing else has fields
            builder.eval(var_param.primitive());
        }

        //
        // builder.eval(var_param.primitive());

        
        let to_val=record.param(2).unwrap().primitive();

        //
        let fields_num = var_param.fields_num();

        //
        
        let mut last_start_loc=var_param.start_loc();
        let mut last_end_loc=var_param.end_loc();


        //fields
        for field_ind in 0 .. fields_num {
            // let param_ind=2+field_ind;
            // let prev_sexpr = var_param.field(field_ind); //record.param(param_ind-1).unwrap();

            //push last result

            if fields_num>1 {
                builder
                    .param_loc(last_start_loc,last_end_loc)
                    .param_push();
            }
            
            builder
                .param_loc(last_start_loc,last_end_loc)
                .param_push();

            //push last result
            if field_ind!=0 && field_ind!=fields_num-1 { //not first or last field
                builder            
                    .param_loc(last_start_loc,last_end_loc)
                    .param_push();
            }

            //on last field : push to
            if field_ind==fields_num-1 {
                //push to_val
                builder
                    .eval(to_val)
                    .param_loc(to_val.start_loc(),to_val.end_loc())
                    .param_push()
                    .swap()
                    ;
            }

            //
            // let field_primitive=record.param(param_ind).unwrap().primitive();
            let field=var_param.field(field_ind).unwrap();
            let field_primitive=field.primitive();
            //eval field
            // if let Some(s)=get_special_symbol(field) {
            //     builder.result_string(s);
            // } else {
                builder.eval(field_primitive);
            // }

            //push field, swap
            builder
                .param_loc(field_primitive.start_loc(),field_primitive.end_loc())
                .param_push()
                .swap();

            //on not last field
            if field_ind!=fields_num-1 {
                //push field, swap
                builder 
                    .param_loc(field_primitive.start_loc(),field_primitive.end_loc())
                    .param_push()
                    .swap();

                //get_field
                builder
                    .loc(field_primitive.start_loc())
                    .call_method("get_field", 2)
                    ;
            }

            
            last_start_loc=field.start_loc();
            last_end_loc=field.end_loc();
        }

        //
        let loc = record.param(1).unwrap().start_loc();

        //
        builder
            .loc(loc)
            .call_method("set_field", 3);

        //sometimes is unecessary to call, for things like arrays and dicts, since they hold "pointer" like values,
        //  and not copies, but for get_field's that return a copy and not a "pointer", then
        //  it must be modified and then copied back to its original owner
        // println!("fields num is {fields_num}");
        for _ in 0 .. fields_num-1 {
            builder
                .rot()
                .rot()
                .swap()
                .call_method("set_field", 3)
                ;
        }

    }


    //
    Ok(())
}

