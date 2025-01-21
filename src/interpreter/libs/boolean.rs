// use super::super::super::common::*;

// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;




pub fn register<X>(lib_scope : &mut LibScope<X>) {
    
    lib_scope.method_ext("is_nil", |_|{
        Ok(Value::Bool(true))
    }).nil().end();

    lib_scope.method_ext("is_nil", |_|{
        Ok(Value::Bool(false))
    }).any().end();

    //
    lib_scope.method_ext("is_bool", |_|{
        Ok(Value::Bool(true))
    }).bool().end();

    lib_scope.method_ext("is_bool", |_|{
        Ok(Value::Bool(false))
    }).any().end();

    lib_scope.method_ext("not",|context|{
        Ok(Value::Bool(!context.param(0).as_bool()))
    })
        //.bool().or_nil().or_int()
        .any()
    .end();

    
    lib_scope.method_ext("=",|_|{
        // println!("eq0 {} {}",context.param(0).type_string(),context.param(1).type_string());
        Ok(Value::Bool(false))
    }).any().any().end();
    
    lib_scope.method_ext("=",|context|{
        // println!("eq1 {} {}",context.param(0).type_string(),context.param(1).type_string());
        Ok(Value::Bool(context.param(1).is_nil()))
    }).nil().any().end();
    
    lib_scope.method_ext("=",|context|{
        // println!("eq2 {} {}",context.param(0).type_string(),context.param(1).type_string());
        Ok(Value::Bool(context.param(0).is_nil()))
    }).any().nil().end();

    lib_scope.method_ext("type",|context|{
        Ok(Value::string(context.param(0).type_string()))
    }).any().end();
}