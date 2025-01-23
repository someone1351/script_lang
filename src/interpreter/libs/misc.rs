// use super::super::super::common::*;
// use super::super::func_context::*;
// use super::super::error::*;

use super::super::value::*;
use super::super::lib_scope::*;

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    lib_scope.method("stdout", |context|{
        print!("{}",context.param(0).as_string());
        Ok(Value::Void)
    })
        .str()
        .end();

    lib_scope.method("string",|context|Ok(Value::string(context.param(0).as_string())))
        .any()
        .end();
    
    lib_scope.method("error", |context|{
        let msg=if context.param(0).is_nil() {
            String::new()
        } else {
            context.param(0).as_string()
        };
        
        Err(context.error(msg))
    })
        .optional()
        .any()
        .end();

    lib_scope.method("len", |_|Ok(Value::Int(0)))
        .nil()
        .end();

    lib_scope.method("type",|context|Ok(Value::string(context.param(0).type_string())))
        .any()
        .end();

    //
    lib_scope.method("is_nil", |_|Ok(Value::Bool(true)))
        .nil()
        .end();

    lib_scope.method("is_nil", |_|Ok(Value::Bool(false)))
        .any()
        .end();

    //
    lib_scope.method("is_bool", |_|Ok(Value::Bool(true)))
        .bool()
        .end();

    lib_scope.method("is_bool", |_|Ok(Value::Bool(false)))
        .any()
        .end();

    lib_scope.method("not",|context|Ok(Value::Bool(!context.param(0).as_bool())))
        //.bool().or_nil().or_int()
        .any()
        .end();
    
    //
    lib_scope.method("=",|_|Ok(Value::Bool(false)))
        .any()
        .any()
        .end();
    
    lib_scope.method("=",|context|Ok(Value::Bool(context.param(1).is_nil())))
        .nil()
        .any()
        .end();
    
    lib_scope.method("=",|context|Ok(Value::Bool(context.param(0).is_nil())))
        .any()
        .nil()
        .end();
}