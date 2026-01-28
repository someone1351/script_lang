use super::super::builder::*;
use super::super::super::cexpr_parser::*;
use super::super::BuilderErrorType;

use super::get_func_params;
use super::get_idn;

/*
TODO
* allow elipsis to be passed to func call for vararg param input? eg fn{a ...} {call {fn {b ...}{} a ...}}
*/

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

    if name_param.primitive().symbol().is_none() {
        return Err(BuilderError::new(name_param.start_loc(), BuilderErrorType::ExpectSymbol(0)));
    }

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
    let fields=
    if let Some(idn)=idn {
        builder
            .decl_var_start(idn,false)
            .decl_var_end();
        None
    } else { //fields
        builder.loc(name_param.start_loc());
        builder.get_var(name_param.primitive().symbol().unwrap());

        //
        let fields=name_param.fields().map(|field|{
            let s=field.primitive().symbol();
            let (f,is_field_symbol)=if s.is_none()||get_var_prefix.map(|x|s.unwrap().starts_with(x)).unwrap_or_default(){
                (field.primitive(),false)
            } else {
                (field.string_primitive(),true)
            };

            (f,is_field_symbol,field.start_loc())
        });
        //
        builder.set_fields_begin(fields.clone());
        Some(fields)
    };

    //
    builder
        .func_start(params,variadic)
            .block_start(Some("func"))
                .eval(body)
                .result_void()
            .block_end()
        .func_end();

    if let Some(idn)=idn {
        builder.set_var(idn);
    } else { //fields
        //
        builder.loc(name_param.start_loc());

        //
        builder.set_fields_end(
            // name_param.fields_num()
            fields.unwrap()
        );


    }

    //
    Ok(())
}