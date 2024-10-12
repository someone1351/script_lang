// use super::super::super::common::*;

use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;



fn default_to_string<X>(context:FuncContext<X>) -> Result<Value,MachineError> {
    Ok(Value::string(context.param(0).as_string()))
}

pub fn print_func<X>(context:FuncContext<X>) -> Result<Value,MachineError> {
    print!("{}",context.param(0).as_string());
    Ok(Value::Void)
}
  
pub fn error_func<X>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let msg=if context.param_is_nil(0) {
        String::new()
    } else {
        context.param(0).as_string()
    };
    
    Err(context.error(msg))
    // Err(MachineError::bound_func_new(msg))
}

pub fn nil_len<X>(_ : FuncContext<X>) -> Result<Value,MachineError> {
    Ok(Value::Int(0))
}

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    lib_scope.method("stdout", print_func)
        .str()
        .end();

    lib_scope.method("string",default_to_string)
        .any()
        .end();
    
    lib_scope.method("error", error_func)
        .optional()
        .any()
        .end();

    lib_scope.method("len", nil_len)
        .nil()
        .end();
}