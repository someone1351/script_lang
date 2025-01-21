// use super::super::super::common::*;

use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;
use super::utils::*;

fn string_len<X>(context:FuncContext<X>) -> Result<Value,MachineError> {
    Ok(Value::int(context.param(0).as_string().chars().count()))
}

fn string_contains<X>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let string=context.param(0).as_string();
    let val=context.param(1).as_string();
    Ok(Value::Bool(string.contains(&val)))
}


// fn string_insert(mut context:FuncContext) -> Result<Value,MachineError> {
//     //0 str, 1 ind, 2 val

//     let mut string=context.param(0).as_string();
//     let val = context.param(2).as_string();

//     let len = string.chars().count();
//     let ind=context.param(1).as_int();

//     // let len2:IntT=len.try_into().unwrap_or_default();

//     // if ind==len2  {
//     //     string.push_str(val.as_str());
//     // } else 
//     {
//         let Some(ind2) = calc_ind(ind,len) else {
//             return Err(MachineError::method(format!("Invalid index ({})",context.param(1).as_int())));
//         };
    
//         if ind2==len  {
//             string.push_str(val.as_str());
//         } else {
//             string.insert_str(ind2, val.as_str());
//         }
//     }
    
//     Ok(Value::string(string))
// }

fn string_remove<X>(context:FuncContext<X>) -> Result<Value,MachineError> {
    //0 str, 1 from, 2 to,

    let string=context.param(0).as_string();
    // let amount = context.param(2).as_string();

    let len = string.chars().count();
    let from = calc_ind(context.param(1).as_int(),len);

    let to=if context.params_len()==3 {
        calc_ind(context.param(2).as_int(),len)
    } else {
        from.map(|x|x+1)
    };

    if len==0 {
        return Err(MachineError::method("string length is 0"));
    }

    let Some(from)=from else {
        return Err(MachineError::method(format!("Invalid from index ({})",context.param(1).as_int())));
    };

    let Some(to)=to else {
        return Err(MachineError::method(format!("Invalid to index ({})",context.param(2).as_int())));
    };

    if from>=len {
        return Err(MachineError::method(format!("Invalid from index ({})",context.param(1).as_int())));
    }

    if to>len {
        return Err(MachineError::method(format!("Invalid to index ({})",context.param(2).as_int())));
    }

    if from>to {
        return Err(MachineError::method("from greater than to"));
    }

    let mut cs=string.chars().collect::<Vec<_>>();

    for _ in from .. to {
        cs.remove(from);
    }

    let x:String= cs.into_iter().collect();
    Ok(Value::string(x))
}

fn string_to_string<X>(context:FuncContext<X>) -> Result<Value,MachineError> {
    Ok(context.param(0).clone())
}

// fn string_push(mut context:FuncContext) -> Result<Value,MachineError> {
//     //string
//     //val
//     let mut s=context.param(0).as_string();

//     s.push_str(context.param(1).as_string().as_str());

//     Ok(())
// }

// fn string_pop(mut context:FuncContext) -> Result<Value,MachineError> {
// }{
fn string_append<X>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let s0=context.param(0).as_string();
    let s1=context.param(1).as_string();
    Ok(Value::string(format!("{s0}{s1}")))
}

fn string_eq<X>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let s0=context.param(0).as_string();
    let s1=context.param(1).as_string();
    Ok(Value::Bool(s0.eq(&s1)))
}


fn string_repeat<X>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let s=context.param(0).as_string();
    let r=context.param(1).as_int() as usize;
    Ok(Value::string(s.repeat(r)))
}


pub fn register<X>(lib_scope : &mut LibScope<X>) {
    lib_scope.method_ext("len", string_len)
        .str().end();
    
    lib_scope.method_ext("contains", string_contains)
        .str().str().end();

    // lib_scope.method("insert", string_insert)
    //     .str().int().str().end();

    lib_scope.method_ext("remove", string_remove)
        .str().int().optional().int().end();

    lib_scope.method_ext("is_string", |_|{
        Ok(Value::Bool(true))
    }).str().end();

    lib_scope.method_ext("is_string", |_|{
        Ok(Value::Bool(false))
    }).any().end();

    lib_scope.method_ext("+",string_append).str().or_any().str().end();
    lib_scope.method_ext("+",string_append).str().str().or_any().end();
    
    lib_scope.method_ext("=",string_eq).str().or_any().str().end();
    lib_scope.method_ext("=",string_eq).str().str().or_any().end();
    lib_scope.method_ext("string",string_to_string).str().end();

    lib_scope.method_ext("repeat",string_repeat).str().int().end();
    
}