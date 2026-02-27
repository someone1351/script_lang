
// use super::super::parser::*;
// use super::error::*;
// use super::Builder;

use super::super::builder::*;
use super::SexprBuilderErrorType;

use super::super::sexpr_parser::*;
use super::super::super::common::JmpCond;

pub fn get_symbol<'a>(sexpr : SExprContainer<'a>) -> Result<&'a str,BuilderError<SexprBuilderErrorType>> {
    if let Some(symbol)=sexpr.symbol() {
        Ok(symbol)
    } else {
        // Err(ParseError { loc: sexpr.start_loc(), msg: "expected symbol".to_string() })
        Err(BuilderError::new(sexpr.start_loc(), SexprBuilderErrorType::ExpectSymbol))
    }
}

// pub fn get_special_symbol(sexpr : SExprContainer) -> Option<&str> {
//     if let SExprValContainer::Symbol(s)= sexpr.val() {
//         if s.starts_with(":") {
//             let s=&s[":".len()..]; //len not really necessary?
//             //&& s.len()>1

//             if !s.is_empty() {
//                 return Some(s);
//             }
//         }
//     }

//     None
// }

//==================

pub fn while_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() < 2 {
        // return Err(ParseError { loc: sexpr.start_loc(), msg: "Incorrect params num".to_string() });
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    let cond_expr = sexpr.get(1).unwrap();
    let body_stmts = sexpr.list_iter_from(2);

    // let c = Compiler::<'a>::new()
    builder
        // .loop_instr()
        .block_start(Some("loop"))
            .eval(cond_expr)
            .to_block_end(JmpCond::False,0);

    for x in body_stmts {
        builder
            .eval(x);
    }
    // builder
    //         .eval_sexprs(body_stmts);
    builder
            .to_block_start(JmpCond::None,0)
        .block_end()
    // .value_instr(&Value::Void)
    ;

    Ok(())
}

pub fn for_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    //(for (i 0 n) body ...)

    if sexpr.len() < 2 {
        // return Err(ParseError { loc: sexpr.start_loc(), msg: "Incorrect params num".to_string() });
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    let header = sexpr.get(1).unwrap();

    if !header.is_list() {
        return Err(BuilderError::new(header.start_loc(), SexprBuilderErrorType::ExpectList));
    }

    if header.len() != 3 {
        return Err(BuilderError::new(header.start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    //
    let idn = get_symbol(header.get(0).unwrap())?;
    let body_stmts = sexpr.list_iter_from(2);

    //

    builder
        .block_start(None)
            .decl_var_start(idn, false)

                .eval(header.get(2).unwrap())
                .decl_anon_var("n", false)
                .set_anon_var("n")
                .decl_anon_var("r", false)
                .result_void()
                .set_anon_var("r")
            .decl_var_end()

            .eval(header.get(1).unwrap())
            .set_var(idn)

            .block_start(Some("loop"))
                .block_start(None)
                    .get_anon_var("n")
                    .param_push()
                    .get_var(idn)
                    .param_push()
                    .call_method("<", 2)
                    // .to_block_end_label(Some(false),"loop", None)
                    .to_block_end(JmpCond::False, 1)

                    .result_void();
    // builder
    //                 .eval_sexprs(body_stmts);
    for x in body_stmts {
        builder
                    .eval(x);
    }
    builder
                    .set_anon_var("r")
                .block_end()

                .result_int(1 as i64)
                .param_push()
                .get_var(idn)
                .param_push()
                .call_method("+", 2)

                .set_var(idn)

                .to_block_start(JmpCond::None,0)
            .block_end()
            .get_anon_var("r")
        .block_end()
        ;

    Ok(())
}

pub fn continue_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() != 1 {
        // return Err(ParseError { loc: sexpr.start_loc(), msg: "No args".to_string() });
        return Err(BuilderError::new(sexpr.start_loc(), SexprBuilderErrorType::NoParamsAllowed));
    }

    // builder.continue_instr();

    let e = BuilderError::new(sexpr.start_loc(), SexprBuilderErrorType::ContinueNotInLoop);
    builder.to_block_start_label(JmpCond::None,"loop",Some(e));

    Ok(())
}

pub fn break_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() != 1 {
        // return Err(ParseError { loc: sexpr.start_loc(), msg: "No args".to_string() });
        return Err(BuilderError::new(sexpr.start_loc(), SexprBuilderErrorType::NoParamsAllowed));
    }

    // builder.break_instr();
    let e = BuilderError::new(sexpr.start_loc(), SexprBuilderErrorType::BreakNotInLoop);
    builder.to_block_end_label(JmpCond::None,"loop",Some(e));

    Ok(())
}

pub fn return_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() > 2 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    if sexpr.len()==2 {
        builder.eval(sexpr.get(1).unwrap());
    } else {
        // builder.result_value(&Value::Void);
        builder.result_void();
    }

    let e = BuilderError::new(sexpr.start_loc(), SexprBuilderErrorType::ReturnNotInMethodOrLambda);
    builder.to_block_end_label(JmpCond::None, "func",Some(e));

    Ok(())
}

pub fn decl_var_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    //eg (var x 123) or (var x)

    if sexpr.len() < 2 || sexpr.len() > 3 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    let idn = get_symbol(sexpr.get(1).unwrap())?;

    let is_init_nil=sexpr.len()==2;
    // let is_global=sexpr.depth()==0 ;

    builder.decl_var_start(idn,is_init_nil);

    if !is_init_nil {
        let val_expr = sexpr.get(2).unwrap();
        builder.eval(val_expr);
    }

    builder.decl_var_end();

    if !is_init_nil {
        builder.set_var(idn);
    }

    // else if !is_global {
    //     builder.result_nil();
    // }

    // if is_global {
    //     builder.decl_global_var(idn,init);
    // } else {

    // }

    // builder
    //     .def_var_instr(idn)
    //     // .value_instr(&Value::Void)
    //     ;

    Ok(())
}

pub fn set_var_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    //eg (set x 123)

    if sexpr.len() != 3 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    //
    let loc = sexpr.start_loc();

    //idn
    let idn = get_symbol(sexpr.get(1).unwrap())?;
    let idn_loc=sexpr.get(1).unwrap().start_loc();

    //val
    let val_sexpr = sexpr.get(sexpr.len()-1).unwrap();

    //

    builder.loc(loc);

    builder
        .loc(idn_loc)
        .eval(val_sexpr)
        .loc(idn_loc)
        .set_var(idn) //,loc
        // .value_instr(&Value::Void)
        ;
    //
    Ok(())
}

pub fn set_field_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    //(set self "color" "r" 123)
    //(set self :color :r 123)
    //(set array i 123)

    if sexpr.len() < 4 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    //push idn
    builder.eval(sexpr.get(1).unwrap());

    //
    let fields_num = sexpr.len()-3;

    let fields=(0 .. fields_num).map(|field_ind|{
        let param_ind=2+field_ind;
        let field=sexpr.get(param_ind).unwrap();
        (field,false,field.start_loc(),)
    });

    //
    builder.set_fields_begin(fields.clone());

    //
    let to_val_sexpr = sexpr.get(sexpr.len()-1).unwrap();

    //push to_val
    builder
        .loc(to_val_sexpr.start_loc())
        .eval(to_val_sexpr)
        ;

    //
    builder.set_fields_end(
        fields
        // fields_num
    );

    //
    Ok(())
}

pub fn get_field_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    //(get self "color" "r")
    //(get self :color :r)
    //(get self i)

    if sexpr.len() < 3 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    //
    builder.eval(sexpr.get(1).unwrap());

    builder.get_fields((2 .. sexpr.len()).map(|field_ind|{
        let field=sexpr.get(field_ind).unwrap();
        (field,false,field.start_loc())
    }));

    //
    Ok(())
}


pub fn if_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    //eg (if [COND0 BODY0*]+ [else BODYELSE?]?)

    if sexpr.len() < 2 {
        // return Err(ParseError { loc: sexpr.start_loc(), msg: "Incorrect params num".to_string() });
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    builder.block_start(None);

    for case_sexpr in sexpr.list_iter_from(1) {
        if case_sexpr.len()==0 {
            return Err(BuilderError::new(case_sexpr.start_loc(), SexprBuilderErrorType::ExpectExpr));
        }

        let cond_sexpr = case_sexpr.get(0).unwrap();
        let body_sexprs = case_sexpr.list_iter_from(1);

        if let Some("else")=cond_sexpr.symbol() {
            if case_sexpr.child_ind()+1!=sexpr.len() {
                return Err(BuilderError::new(cond_sexpr.start_loc(), SexprBuilderErrorType::ElseMustBeAtEnd));
            } else {
                // builder.eval_sexprs(body_sexprs);
                for x in body_sexprs {
                    builder.eval(x);
                }
            }
        } else {
            builder
                .eval(cond_sexpr)
                .block_start(None)
                    .to_block_end(JmpCond::False,0);
            // builder
            //         .eval_sexprs(body_sexprs);

            for x in body_sexprs {
                builder
                    .eval(x);
            }

            builder
                    .to_block_end(JmpCond::None,1)
                .block_end();
        }
    }

    builder.block_end();

    Ok(())
}

pub fn ternary_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    //eg (if cond then else)

    if sexpr.len() < 2 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    let cond_sexpr = sexpr.get(1).unwrap();
    let then_sexpr = sexpr.get(2);
    let else_sexpr = sexpr.get(3);

    builder
        .eval(cond_sexpr)
        .block_start(None);

    if let Some(then_sexpr)=then_sexpr {
        builder
            .block_start(None)
                .to_block_end(JmpCond::False,0)
                .eval(then_sexpr)
                .to_block_end(JmpCond::None,1)
            .block_end()
            ;

        if let Some(else_sexpr)=else_sexpr {
            builder
                .eval(else_sexpr)
                ;
        } else {
            builder.result_nil();
        }
    } else {
        builder.result_nil();
    }

    builder.block_end();

    Ok(())
}

pub fn and_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() < 2 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    // let mut last_val=Value::Bool(false);
    builder.block_start(None);

    for cond_sexpr in sexpr.list_iter_from(1) {
        builder
            .eval(cond_sexpr)
            .to_block_end(JmpCond::False,0)
            ;
    }

    builder.block_end();
    Ok(())
}

pub fn or_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() < 2 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    // let mut last_val=Value::Bool(false);
    builder.block_start(None);

    for cond_sexpr in sexpr.list_iter_from(1) {
        builder
            .eval(cond_sexpr)
            .to_block_end(JmpCond::True,0)
            ;
    }

    builder.block_end();
    Ok(())
}


pub fn get_func_params<'a>(params_sexpr : SExprContainer<'a>, ) -> Result<(Vec<&'a str>,bool),BuilderError<SexprBuilderErrorType>> {
    //(a b c)
    //(a b c ...)

    if !params_sexpr.is_list() {
        return Err(BuilderError::new(params_sexpr.start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    let mut params=Vec::<&str>::new();
    let mut variadic=false;

    for (i,param_sexpr) in params_sexpr.list_iter().enumerate() {
        // println!("i {i}, len {}",params_sexpr.len());
        let idn = get_symbol(param_sexpr)?;

        if idn=="..." { //variadic
            if i!=params_sexpr.len()-1 {
                return Err(BuilderError::new(param_sexpr.start_loc(), SexprBuilderErrorType::VariadicMustBeAtEnd));
            }

            variadic=true;
        } else { //param
            params.push(idn);
        }
    }

    Ok((params,variadic))
}

pub fn decl_func_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    //eg
    //(fn ab (a b c) (something a) (+ b c))
    //(fn ab (a b c ...) (something a) (+ b c))

    if sexpr.len() < 3 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    // if sexpr.depth()!=0 {
    //     return Err(BuilderError::new(sexpr.start_loc(), BuilderErrorType::DeclFuncNotRoot));
    // }

    //idn
    let idn = get_symbol(sexpr.get(1).unwrap())?;
    // let idn_loc=sexpr.get(1).unwrap().start_loc();

    //params
    // let (params,variadic)=get_func_typed_params(sexpr.get(2).unwrap(), builder)?;
    let (params,variadic)=get_func_params(sexpr.get(2).unwrap(), )?;

    //initialise var decl, so it can be captured and used for recursion if necessary
    // builder.result_nil();

    // if sexpr.depth()==0 {
    //     builder.decl_global_var(idn, true);
    // } else {
    //     builder.decl_local_var(idn);
    // }

    //body

    //
    builder
        .decl_var_start(idn,false)
        .decl_var_end()
        .func_start(params,variadic)
        // .result_void()
        ;

    if sexpr.len() > 3 {
        let body_sexprs = sexpr.list_iter_from(3);

        builder
            .block_start(Some("func"));
        // builder.eval_sexprs(body_sexprs);

        for x in body_sexprs {
            builder.eval(x);
        }

        builder
            .block_end();
    }

    builder
        .func_end()
        // .decl_global_var(idn)
        // // .decl_local_var(idn) //should make this a global... since inner_globals no longer used
        // .set_var(idn, idn_loc)
        // .decl_global_var(idn, true)
        .set_var(idn)
        ;



    //
    Ok(())
}

pub fn lambda_func_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    //(fn (a b c) (something a) (+ b c))
    //(fn (a b c ...) (something c) (+ b a))
    //(fn () (something))
    //(fn () )

    if sexpr.len() < 2 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    //params
    let (params,variadic)=get_func_params(sexpr.get(1).unwrap(), )?;

    //
    builder
        .func_start(params,variadic)
        // .result_void()
        ;

    //body
    if sexpr.len() > 2 {
        let body_sexprs = sexpr.list_iter_from(2);

        builder.block_start(Some("func"));

        for x in body_sexprs {
            builder.eval(x);
        }

        // builder.eval_sexprs(body_sexprs);
        builder.block_end();
    }

    //
    builder.func_end();

    //
    Ok(())
}

// pub fn decl_method_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> {
//     //eg
//     //(method ab (a :int b :float c) (something a) (+ b c))
//     //(method ab (a :int b :float ? c) (something a) (+ b c))
//     //(method ab (a b ? c : i32                                                                                                                                                                          ...) (something a) (+ b c))
//     //(method ab (? a b c ...) (something a) (+ b c))

//     if sexpr.len() < 3 {
//         return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
//     }

//     if sexpr.depth()!=0 {
//         return Err(BuilderError::new(sexpr.start_loc(), BuilderErrorType::DeclFuncNotRoot));
//     }

//     //idn
//     let idn = get_symbol(sexpr.get(1).unwrap())?;

//     //params
// //     // let (params,variadic)=get_func_typed_params(sexpr.get(2).unwrap(), builder)?;
// //     let (params,variadic)=get_func_params(sexpr.get(2).unwrap(), builder)?;

//     //body

// //     //
// //     builder
// //         .func_start(params,variadic)
// //         // .result_void()
// //         ;

// //     if sexpr.len() > 3 {
// //         let body_sexprs = sexpr.list_iter_from(3);

// //         builder
// //             .block_start("func")
// //             .eval_sexprs(body_sexprs)
// //             .block_end();
// //     }

// //     builder
// //         .func_end()
// //         .decl_global_var(idn)
// //         // .decl_local_var(idn) //should make this a global... since inner_globals no longer used
// //         ;

//     //
//     Ok(())
// }

pub fn add_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() == 1 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    //a+b+c+d = ((a+b)+c)+d

    builder.eval(sexpr.get(1).unwrap());
    let loc = sexpr.get(0).unwrap().start_loc();

    for i in 2 .. sexpr.len() {

        builder.loc(loc);

        builder
            .param_push() //push last result
            .eval(sexpr.get(i).unwrap())
            .param_push()
            .swap()
            .call_method("+",2); //,loc
    }

    Ok(())
}

pub fn sub_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() == 1 {
        // return Err(ParseError { loc: sexpr.start_loc(), msg: "Incorrect params num".to_string() });
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    builder.eval(sexpr.get(1).unwrap());
    let loc = sexpr.get(0).unwrap().start_loc();

    builder.loc(loc);

    if sexpr.len()==2 {
        builder
            .param_push()
            .call_method("-",1); //,loc
    } else {
        for i in 2 .. sexpr.len() {
            builder
                .param_push() //push last result
                .eval(sexpr.get(i).unwrap())
                .param_push()
                .swap()
                .call_method("-",2); //,loc
        }
    }

    Ok(())
}

pub fn mul_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() == 1 {
        // return Err(ParseError { loc: sexpr.start_loc(), msg: "Incorrect params num".to_string() });
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    //a*b*c*d = a*(b*(c*d))

    builder.eval(sexpr.last().unwrap());
    let loc = sexpr.get(0).unwrap().start_loc();

    builder.loc(loc);

    for i in (1 .. sexpr.len()-1).rev() {
        builder
            .param_push() //push last result
            .eval(sexpr.get(i).unwrap())
            .param_push()
            // .swap()
            .call_method("*",2); //,loc
    }

    Ok(())
}

pub fn div_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() == 1 {
        // return Err(ParseError { loc: sexpr.start_loc(), msg: "Incorrect params num".to_string() });
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    builder.eval(sexpr.last().unwrap());
    let loc = sexpr.get(0).unwrap().start_loc();

    builder.loc(loc);

    for i in (1 .. sexpr.len()-1).rev() {
        builder
            .param_push() //push last result
            .eval(sexpr.get(i).unwrap())
            .param_push()
            // .swap()
            .call_method("/",2); //,loc
    }

    Ok(())
}

pub fn block_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    builder
        .block_start(None);
    // builder
    //     .eval_sexprs(sexpr.list_iter_from(1));

    for x in sexpr.list_iter_from(1) {
        builder.eval(x);
    }

    builder
        .block_end();

    Ok(())
}
pub fn include_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    if sexpr.len() != 2 {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::IncorrectParamsNum));
    }

    let v = sexpr.get(1).unwrap();

    let Some(s)=v.string() else {
        return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), SexprBuilderErrorType::ExpectString));
    };

    builder.include(s, v.start_loc());

    Ok(())
}


pub fn print_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    format_cmd(sexpr,builder)?;
    builder
        .param_push()
        .call_method("stdout", 1);
    Ok(())
}
pub fn println_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    print_cmd(sexpr,builder)?;

    builder
        .result_string("\n")
        .param_push()
        .call_method("stdout", 1)
        ;
    Ok(())
}

pub fn format_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>> {
    //
    builder
        .result_string("")
        .param_push();

    //
    let mut j=1;

    if sexpr.get(1).unwrap().is_string() {
        j+=1;


        //parse format string

        let s =sexpr.get(1).unwrap().string().unwrap();
        let mut cs=s.chars();
        let mut char_ind=0;

        let mut texts: Vec<(usize, usize)>= vec![(0,0)];
        let mut vars: Vec<(usize, usize)> =Vec::new();

        while let Some(c)=cs.next() {
            char_ind+=1;

            match c {
                '{' => {
                    vars.push((char_ind,char_ind));

                    while let Some(c)=cs.next() {
                        char_ind+=1;

                        match c {
                            '}' => {
                                vars.last_mut().unwrap().1=char_ind-1;
                                break;
                            }
                            '\\' => {
                                if cs.next().is_some() {
                                    char_ind+=1;
                                }

                                continue;
                            }
                            _=>{
                            }
                        }
                    }

                    texts.push((char_ind,char_ind));
                }
                '\\' => {
                    if cs.next().is_some() {
                        char_ind+=1;
                        texts.last_mut().unwrap().1=char_ind;
                    }

                    continue;
                }
                _=>{
                    texts.last_mut().unwrap().1=char_ind;
                }
            }
        }

        //

        for i in 0 .. texts.len() {
            let (text_start,text_end) = texts[i];
            let text_str=&s[text_start..text_end];

            // println!("a: {text_str:?}");

            if !text_str.is_empty() {
                builder
                    .result_string(text_str)
                    .param_push()
                    .swap()
                    .call_method("+", 2)
                    .param_push()
                    ;
            }

            if let Some((var_start,var_end)) = vars.get(i).cloned() {
                let var_str=&s[var_start..var_end];
                // println!("v: {var_str:?}");

                if var_str.is_empty() {
                    builder.eval(sexpr.get(j).unwrap());

                    j+=1;
                } else {
                    builder.get_var(var_str);
                }

                builder
                    .param_push()
                    .call_method("string", 1)

                    .param_push()
                    .swap()
                    .call_method("+", 2)

                    .param_push();
            }
        }

    }

    for k in j .. sexpr.len() {
        if k>=2 {
            builder
                .result_string(" ")

                .param_push()
                .swap()
                .call_method("+", 2)
                .param_push();
        }


        builder

            .eval(sexpr.get(k).unwrap())

            .param_push()
            .call_method("string", 1)

            .param_push()
            .swap()
            .call_method("+", 2)

            .param_push();
    }


    builder.pop();
    // builder.call_method("console_out", 1);
//
    return Ok(());

}