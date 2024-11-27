use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

use super::get_func_params;
use super::get_idn;


pub fn func_cmd<'a>(
    record : RecordContainer<'a>, 
    builder :&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>,
    get_var_prefix : Option<&'static str>,
) -> Result<(),BuilderError<BuilderErrorType>> {
    //eg
    //(fn ab (a b c) (something a) (+ b c))
    //(fn ab (a b c ...) (something a) (+ b c))

    //fn abc {a b c} {}
    //fn abc a b c {}

    /*
    fn abc a b c {
        + a b;
    }
    fn abc {a b c} {
        + a b;
    }
    fn abc {a b 
            c} {
        + a b;
    }
    fn abc a b c ... {
        + a b;
    }
    fn abc {a b c ...} {
        + a b;
    }

    fn abc 1
    fn abc {1}
    */
    //having block for params
    //  will have semicolons allowed
    //  newlines will cause multiple records
    //  could have get primitives from block, though to return an iter might need to make all those primitives adjacent

    // if record.len() < 3 {
    //     return Err(BuilderError::new(record.last().start_loc(), BuilderErrorType::IncorrectParamsNum));
    // }

    //fn abc {} {x}

    if record.params_num()!=4 {
        return Err(BuilderError::new(record.first_param().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
    }
    // if sexpr.depth()!=0 {
    //     return Err(BuilderError::new(sexpr.start_loc(), BuilderErrorType::DeclFuncNotRoot));
    // }


    //idn
    let name_param=record.param(1).unwrap();
    let idn = if name_param.fields_num()==0 {Some(get_idn(name_param)?)}else{None};
    // let idn_loc=record.primitive(1).unwrap().start_loc();

    //
    let body=record.last_param().unwrap().primitive();

    //params
    let params_primitive=record.param(2).unwrap().primitive();
    
    let Some(params_block)=params_primitive.block() else {
        return Err(BuilderError::new(params_primitive.start_loc(), BuilderErrorType::IncorrectParamsNum));
    };

    // let (params,variadic)=get_func_typed_params(sexpr.get(2).unwrap(), builder)?;
    let (params,variadic)=get_func_params(params_block)?;

    //initialise var decl, so it can be captured and used for recursion if necessary
    // builder.result_nil();

    // if sexpr.depth()==0 {
    //     builder.decl_global_var(idn, true);
    // } else {
    //     builder.decl_local_var(idn);
    // }

    //body

    // if var_param.fields_num()==0 {
    //
    if let Some(idn)=idn {
        builder
            .decl_var_start(idn,false)
            .decl_var_end();
    }

    builder.func_start(params,variadic);

        // .result_void()
    // if record.len() > 3 {
    //     let body_sexprs = record.list_iter_from(3);

        builder
            .block_start(Some("func"))
            .eval(body)
            .block_end();
    // }
    
    builder
        .func_end();
        // .decl_global_var(idn)
        // // .decl_local_var(idn) //should make this a global... since inner_globals no longer used
        // .set_var(idn, idn_loc)
        // .decl_global_var(idn, true)
        
    if let Some(idn)=idn {
        builder.set_var(idn);
    } else {
        // builder.call_method(name, params_num)

        builder.loc(name_param.start_loc());

        if let Some(idn)=name_param.primitive().symbol() {
            // let idn_loc=var_param.start_loc();
        
            builder
                // .loc(var_param.start_loc())
                .get_var(idn);
        } else { //a block, otherwise would be an err, due to nothing else has fields
            builder.eval(name_param.primitive());
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