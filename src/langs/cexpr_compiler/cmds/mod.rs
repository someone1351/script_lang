pub mod while_cmd;
pub mod for_cmd;
pub mod continue_cmd;
pub mod break_cmd;
pub mod return_cmd;
pub mod var_cmd;
pub mod set_var_cmd;
// pub mod set_field_cmd;
// pub mod get_field_cmd;
pub mod if_cmd;
pub mod and_cmd;
pub mod or_cmd;
pub mod add_cmd;
pub mod sub_cmd;
pub mod mul_cmd;
pub mod div_cmd;
pub mod include_cmd;
pub mod format_cmd;
pub mod print_cmd;
pub mod println_cmd;
pub mod func_cmd;
pub mod lambda_cmd;
pub mod call_func_cmd;
pub mod ternary_cmd;


pub use while_cmd::*;
pub use for_cmd::*;
pub use continue_cmd::*;
pub use break_cmd::*;
pub use return_cmd::*;
pub use var_cmd::*;
pub use set_var_cmd::*;
// pub use set_field_cmd::*;
// pub use get_field_cmd::*;
pub use if_cmd::*;
pub use and_cmd::*;
pub use or_cmd::*;
pub use add_cmd::*;
pub use sub_cmd::*;
pub use mul_cmd::*;
pub use div_cmd::*;
pub use include_cmd::*;
pub use format_cmd::*;
pub use print_cmd::*;
pub use println_cmd::*;
pub use func_cmd::*;
pub use lambda_cmd::*;
pub use call_func_cmd::*;
pub use ternary_cmd::*;

use super::super::builder::*;
use super::{super::cexpr_parser::*, BuilderErrorType};





pub fn get_symbol<'a>(param : ParamContainer<'a>) -> Result<&'a str,BuilderError<BuilderErrorType>> {
    if let Some(symbol)=param.primitive().symbol() {
        if param.fields_num()==0 {
            Ok(symbol)
        } else {
            Err(BuilderError::new(param.field(0).unwrap().start_loc(), BuilderErrorType::NoFieldsAllowed))
        }
    } else {
        Err(BuilderError::new(param.start_loc(), BuilderErrorType::ExpectSymbol))
    }
}

/*
only use blocks for code?
eg 
    for i 0 10 {} 
instead of 
    for {i 0 10} {}
?
*/

//mark loops?

//only need builder.loc() for param_push? could just check the primitive pushed for the loc instead?





// pub fn ternary_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a>) -> Result<(),BuilderError> {
//     //eg (if cond then else)
//     //{? cond x y}
//     //{if cond x else y}

//     if sexpr.len() < 2 {
//         return Err(BuilderError::new(sexpr.last().unwrap().start_loc(), BuilderErrorType::IncorrectParamsNum));
//     }

//     let cond_sexpr = sexpr.get(1).unwrap();
//     let then_sexpr = sexpr.get(2);
//     let else_sexpr = sexpr.get(3);

//     builder
//         .eval_sexpr(cond_sexpr)
//         .block_start();

//     if let Some(then_sexpr)=then_sexpr {
//         builder
//             .block_start()
//                 .to_block_end(Some(false),0)
//                 .eval_sexpr(then_sexpr)
//                 .to_block_end(None,1)
//             .block_end()
//             ;

//         if let Some(else_sexpr)=else_sexpr {
//             builder
//                 .eval_sexpr(else_sexpr)
//                 ;
//         } else {
//             builder.result_nil();
//         }
//     } else {
//         builder.result_nil();
//     }

//     builder.block_end();
    
//     Ok(())
// }


pub fn get_func_params<'a>(record : RecordContainer<'a>, params_start:usize, params_end:usize) -> Result<(Vec<&'a str>,bool),BuilderError<BuilderErrorType>> {
    //(a b c)
    //(a b c ...)

    // if !record.is_list() {
    //     return Err(BuilderError::new(record.start_loc(), BuilderErrorType::IncorrectParamsNum));
    // }

    let mut params=Vec::<&str>::new();
    let mut variadic=false;

    for i in params_start .. params_end {
        let x=record.param(i).unwrap();
        // (i,param_sexpr) record.list_iter().enumerate()   
        // println!("i {i}, len {}",params_sexpr.len()); 
        let idn = get_symbol(x)?;

        if idn=="..." { //variadic
            if i!=record.params_num()-1 {
                return Err(BuilderError::new(x.start_loc(), BuilderErrorType::VariadicMustBeAtEnd));
            }
            
            variadic=true;
        } else { //param  
            params.push(idn);           
        }
    }

    Ok((params,variadic))
}


pub fn get_func_params2<'a>(block : BlockContainer<'a>) -> Result<(Vec<&'a str>,bool),BuilderError<BuilderErrorType>> {
    // if block.has_semi_colon_ends() {

    // }

    let mut params=Vec::<&str>::new();
    let mut variadic=false;

    //need block.values_iter
    if block.primitive().param().unwrap().fields_num()!=0 {
        return Err(BuilderError::new(block.end_loc(), BuilderErrorType::NoFieldsAllowed));
    }
    
    if let Some(loc)=block.records().find_map(|x|x.semi_colon_loc()) {
        return Err(BuilderError::new(loc, BuilderErrorType::NoSemiColonsAllowed));
    }

    if let Some(loc)=block.params().find_map(|x|x.primitive().symbol().is_none().then_some(x.start_loc())) {
        return Err(BuilderError::new(loc, BuilderErrorType::ExpectSymbol));
    }

    if let Some(loc)=block.params().find_map(|x|(x.fields_num()!=0).then(||x.field(0).unwrap().start_loc())) {
        return Err(BuilderError::new(loc, BuilderErrorType::NoFieldsAllowed));
    }

    // if let Some(loc)=block.first_value_block_loc() {
    //     return Err(BuilderError::new(loc, BuilderErrorType::NoBlocksAllowed));
    // }

    for (param_ind,param) in block.params().enumerate() {

        let idn = get_symbol(param)?;

        if idn=="..." { //variadic
            if param_ind!=block.params_num()-1 {
                return Err(BuilderError::new(param.start_loc(), BuilderErrorType::VariadicMustBeAtEnd));
            }
            
            variadic=true;
        } else { //param  
            params.push(idn);           
        }
    }

    Ok((params,variadic))
}
// pub fn decl_method_cmd<'a>(sexpr : SExprContainer<'a>, builder :&mut Builder<'a>) -> Result<(),BuilderError> {
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
// //             .block_start_label("func")
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

