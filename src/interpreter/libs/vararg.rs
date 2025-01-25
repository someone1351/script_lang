use super::super::super::common::*;

use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;
use super::super::data::*;



fn vararg_get_field(context:FuncContext) -> Result<Value,MachineError> {
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
    let val=context.stack_val(stack_ind)?;

    //
    Ok(val.clone())
}

fn custom_vararg_copy(mut context:FuncContext) -> Result<Value,MachineError> {
    let Some(stack_frame) = context.stack_frame() else {
        return Ok(Value::Nil);
    };

    let vararg_len=stack_frame.stack_params_num-stack_frame.func_params_num;
    let stack_params_start = stack_frame.stack_params_start;
    let stack_params_end=stack_params_start+vararg_len;

    let data=(stack_params_start..stack_params_end)
        .rev()
        .map(|stack_ind|context.stack_val(stack_ind).unwrap().clone())
        .collect::<Vec<_>>();
    
    Ok(Value::custom_managed_mut(Array(data), context.gc_scope()))
}

pub fn register<X>(func_scope : &mut LibScope<X>) {
    func_scope.method("len", |context|{
        Ok(Value::int(if let Some(stack_frame) = context.stack_frame() {
            stack_frame.stack_params_num-stack_frame.func_params_num
        } else {
            0
        }))
    })
        .custom_ref::<Vararg>().end();

    func_scope.method("get_field", vararg_get_field)
        .custom_ref::<Vararg>()
        .int()
        .end();

    func_scope.method("string", |_|{
        Ok(Value::string(format!("Vararg")))
    })
        .custom_ref::<Vararg>()
        .end();

    func_scope.method("copy", custom_vararg_copy)
        .custom_ref::<Vararg>()
        .end();
}

