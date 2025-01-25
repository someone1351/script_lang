
// use super::super::super::common::*;
// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;



pub fn register<X>(lib_scope : &mut LibScope<X>) {
    
	//
    lib_scope.method("is_int", |_|Ok(Value::Bool(true)))
        .int().end();

    lib_scope.method("is_int", |_|Ok(Value::Bool(false)))
        .any().end();

    lib_scope.method("-", |context|{
        Ok(Value::int(-context.param(0).as_int()))
    }).int().end();

    lib_scope.method("+",|context|{
        Ok(Value::int(context.param(0).as_int()+context.param(1).as_int()))
    }).int().int().end();

    lib_scope.method("-",|context|{
        Ok(Value::int(context.param(0).as_int()-context.param(1).as_int()))
    }).int().int().end();

    lib_scope.method("*",|context|{
        Ok(Value::int(context.param(0).as_int()*context.param(1).as_int()))
    }).int().int().end();

    lib_scope.method("/",|context|{
        context.param(0).as_int().checked_div(context.param(1).as_int())
            .and_then(|x|Some(Value::int(x)))
            .ok_or(context.error("Divide by zero".to_string()))
    }).int().or_float().int().end();


    //
    lib_scope.method(">",|context|{
        Ok(Value::Bool(context.param(0).as_int()>context.param(1).as_int()))
    }).int().int().end();
    
    lib_scope.method("<",|context|{
        Ok(Value::Bool(context.param(0).as_int()<context.param(1).as_int()))
    }).int().int().end();
    
    lib_scope.method(">=",|context|{
        Ok(Value::Bool(context.param(0).as_int()>=context.param(1).as_int()))
    }).int().int().end();
    
    lib_scope.method("<=",|context|{
        Ok(Value::Bool(context.param(0).as_int()<=context.param(1).as_int()))
    }).int().int().end();
    
    lib_scope.method("=",|context|{
        Ok(Value::Bool(context.param(0).as_int()==context.param(1).as_int()))
    }).int().int().end();

}