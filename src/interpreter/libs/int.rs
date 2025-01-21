
// use super::super::super::common::*;
// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;



pub fn register<X>(lib_scope : &mut LibScope<X>) {
    
	//
    lib_scope.method_ext("is_int", |_|{
        Ok(Value::Bool(true))
    }).int().end();

    lib_scope.method_ext("is_int", |_|{ Ok(Value::Bool(false)) }).any().end();

    lib_scope.method_ext("-", |context|{
        Ok(Value::Int(-context.param(0).as_int()))
    }).int().end();

    lib_scope.method_ext("+",|context|{
        Ok(Value::Int(context.param(0).as_int()+context.param(1).as_int()))
    }).int().int().end();

    lib_scope.method_ext("-",|context|{
        Ok(Value::Int(context.param(0).as_int()-context.param(1).as_int()))
    }).int().int().end();

    lib_scope.method_ext("*",|context|{
        Ok(Value::Int(context.param(0).as_int()*context.param(1).as_int()))
    }).int().int().end();


    lib_scope.method_ext("/",|context|{
        context.param(0).as_int().checked_div(context.param(1).as_int())
            .and_then(|x|Some(Value::Int(x)))
            .ok_or(context.error("Divide by zero".to_string()))
    }).int().or_float().int().end();


    //
    lib_scope.method_ext(">",|context|{
        Ok(Value::Bool(context.param(0).as_int()>context.param(1).as_int()))
    }).int().int().end();
    
    lib_scope.method_ext("<",|context|{
        Ok(Value::Bool(context.param(0).as_int()<context.param(1).as_int()))
    }).int().int().end();
    
    lib_scope.method_ext(">=",|context|{
        Ok(Value::Bool(context.param(0).as_int()>=context.param(1).as_int()))
    }).int().int().end();
    
    lib_scope.method_ext("<=",|context|{
        Ok(Value::Bool(context.param(0).as_int()<=context.param(1).as_int()))
    }).int().int().end();
    
    lib_scope.method_ext("=",|context|{
        Ok(Value::Bool(context.param(0).as_int()==context.param(1).as_int()))
    }).int().int().end();

    //
    lib_scope.method_ext("min", |context|{
        Ok(Value::Int(context.param(0).as_int().min(context.param(1).as_int())))
    }).int().int().end();

    lib_scope.method_ext("max", |context|{
        Ok(Value::Int(context.param(0).as_int().max(context.param(1).as_int())))
    }).int().int().end();

    lib_scope.method_ext("clamp", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        let c=context.param(2).as_int();

        if a <b || a>c {
            return Err(context.error("value not within min/max".to_string()));
        }

        Ok(Value::Int(a.clamp(b,c)))
    }).int().int().int().end();

    //
    lib_scope.method_ext("abs", |context|{
        Ok(Value::Int(context.param(0).as_int().abs()))
    }).int().end();
    
    //
    lib_scope.method_ext("sign", |context|{
        let a=context.param(0).as_int();

        let b= if a > 0 {
            1
        } else if a < 0 {
            -1
        } else {
            0
        };

        Ok(Value::Int(b))
    }).int().end();

	//
    lib_scope.method_ext("pow", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();

        if b<0 {
            return Err(context.error("exp less than 0".to_string()));
        }

        Ok(Value::Int(a.pow(b as u32)))
    }).int().int().end();

	//
    lib_scope.method_ext("mod", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();

        Ok(Value::Int(a%b))
    }).int().int().end();
}