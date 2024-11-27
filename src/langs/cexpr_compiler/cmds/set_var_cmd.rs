use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

use super::get_idn;

pub fn set_var_cmd<'a>(
    record : RecordContainer<'a>, 
    builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>,
    get_var_prefix : Option<&'static str>,
) -> Result<(),BuilderError<BuilderErrorType>> {
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

        let idn = get_idn(var_param)?;
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
        builder.loc(var_param.start_loc());

        if let Some(idn)=var_param.primitive().symbol() {
            // let idn_loc=var_param.start_loc();
        
            builder
                // .loc(var_param.start_loc())
                .get_var(idn);
        } else { //a block, otherwise would be an err, due to nothing else has fields
            builder.eval(var_param.primitive());
        }

        //
        // builder.eval(var_param.primitive());

        
        let to_val=record.param(2).unwrap();

        //
        let x=var_param.fields().map(|field|{
            let s=field.primitive().symbol();
            let f=if s.is_none()||get_var_prefix.map(|x|s.unwrap().starts_with(x)).unwrap_or_default(){
                field.primitive()
            } else {
                field.string_primitive()
            };

            (f,field.start_loc())
        });

        builder.set_fields(x, (to_val.primitive(),to_val.start_loc()));
    }


    //
    Ok(())
}

