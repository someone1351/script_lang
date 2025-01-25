
// use super::super::super::common::*;
// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;



pub fn register<X>(lib_scope : &mut LibScope<X>) {
    
    //
    lib_scope.method("min", |context|{
        Ok(Value::int(context.param(0).as_int().min(context.param(1).as_int())))
    }).int().int().end();

    lib_scope.method("max", |context|{
        Ok(Value::int(context.param(0).as_int().max(context.param(1).as_int())))
    }).int().int().end();

    lib_scope.method("clamp", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        let c=context.param(2).as_int();

        if a <b || a>c {
            return Err(context.error("value not within min/max".to_string()));
        }

        Ok(Value::int(a.clamp(b,c)))
    }).int().int().int().end();

    //
    lib_scope.method("abs", |context|{
        Ok(Value::int(context.param(0).as_int().abs()))
    }).int().end();
    
    //
    lib_scope.method("sign", |context|{
        let a=context.param(0).as_int();

        let b= if a > 0 {
            1
        } else if a < 0 {
            -1
        } else {
            0
        };

        Ok(Value::int(b))
    }).int().end();

	//
    lib_scope.method("pow", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();

        if b<0 {
            return Err(context.error("exp less than 0".to_string()));
        }

        Ok(Value::int(a.pow(b as u32)))
    }).int().int().end();

	//
    lib_scope.method("mod", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(Value::int(a%b))
    }).int().int().end();
}