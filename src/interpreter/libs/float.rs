
use super::super::super::common::*;
// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    
    lib_scope.method("string", |context|{
        Ok(Value::string(format!("{:?}",context.param(0).as_float())))
    })
        .float()
        .end();

    lib_scope.method("is_float", |_|Ok(Value::Bool(true)))
        .float()
        .end();

    lib_scope.method("is_float", |_|Ok(Value::Bool(false)))
        .any()
        .end();

    //

}