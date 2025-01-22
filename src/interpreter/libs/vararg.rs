use super::super::super::common::*;

use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;
use super::super::data::*;



// #[derive(Debug,Clone)]
// struct CustomVararg;

fn vararg_len(context:FuncContext2) -> Result<Value,MachineError> {
    let Some(stack_frame) = context.stack_frame() else {
        return Ok(Value::Int(0));
    };
    
    let vararg_len=stack_frame.stack_params_num-stack_frame.func_params_num;
    Ok(Value::Int(vararg_len as IntT))
}

fn vararg_get_field(context:FuncContext2) -> Result<Value,MachineError> {
    //0 vararg, 1 index

    //
    let Some(stack_frame) = context.stack_frame() else {
        return Ok(Value::Nil);
    }; //err instead?


    let n=stack_frame.stack_params_num-stack_frame.func_params_num;
    let i=context.param(1).as_int();
    let x = (((i % (n as IntT)) + if i<0{n as IntT}else{0}) % (n as IntT)) as usize;

    //
    let stack_ind = stack_frame.stack_params_start+n-x-1;

    //
    // let val = context.stack().get(stack_ind).unwrap();
    let val=context.stack_val(stack_ind)?;

    //
    Ok(val.clone())
}

// fn vararg_set_field(mut context:FuncContext2) -> Result<Value,MachineError> {
//     //0 = vararg, 1 = index, 2 = value

//     //
//     let Some(stack_frame) = context.last_stack_frame() else {
//         return Ok(Value::Nil);
//     }; //err instead?

//     let n=stack_frame.stack_params_num-stack_frame.func_params_num;
//     let i=context.param(1).as_int();
//     let x = (((i % (n as IntT)) + if i<0{n as IntT}else{0}) % (n as IntT)) as usize;
    
//     //
//     let stack_ind = stack_frame.stack_params_start+n-x-1;

//     //
//     let val=context.param(2).clone();
//     // *context.stack_mut().get_mut(stack_ind).unwrap()=val;
//     context.set_stack_val(stack_ind, val)?;
//     //
//     Ok(Value::Void)
// }

fn custom_vararg_to_string(_:FuncContext2) -> Result<Value,MachineError> {
    Ok(Value::string(format!("Vararg")))
}

fn custom_vararg_copy(mut context:FuncContext2) -> Result<Value,MachineError> {
    let Some(stack_frame) = context.stack_frame() else {
        return Ok(Value::Nil);
    };

    let vararg_len=stack_frame.stack_params_num-stack_frame.func_params_num;
    let stack_params_start = stack_frame.stack_params_start;
    let stack_params_end=stack_params_start+vararg_len;

    let data=(stack_params_start..stack_params_end)
        .rev()
        // .map(|stack_ind|context.stack().get(stack_ind).unwrap().clone())
        .map(|stack_ind|context.stack_val(stack_ind).unwrap().clone())
        .collect::<Vec<_>>();
    
    // let array = context.new_custom_managed(Array(data));

    // Ok(Value::Custom(array))

    
    // Ok(context.custom_managed_mut(Array(data)))
    Ok(Value::custom_managed_mut(Array(data), context.gc_scope()))
    // Ok(Value::custom_managed_mut(context.gc_scope(), Array(data)))
}

pub fn register<X>(func_scope : &mut LibScope<X>) {
    func_scope.method("len", vararg_len)
        .custom_ref::<Vararg>()
        .end();

    func_scope.method("get_field", vararg_get_field)
        .custom_ref::<Vararg>()
        .int()
        .end();

    // func_scope.method("set_field", vararg_set_field)
    //     .custom_ref::<Vararg>()
    //     .int()
    //     .any()
    //     .end();

    func_scope.method("string", custom_vararg_to_string)
        .custom_ref::<Vararg>()
        .end();

    func_scope.method("copy", custom_vararg_copy)
        .custom_ref::<Vararg>()
        .end();
}

