
use super::super::super::common::*;
// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    
    //
    lib_scope.method("min", |context|{
        Ok(Value::float(context.param(0).as_float().min(context.param(1).as_float())))
    }).float().float().end();

    lib_scope.method("max", |context|{
        Ok(Value::float(context.param(0).as_float().min(context.param(1).as_float())))
    }).float().float().end();

    lib_scope.method("clamp", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();

        if b > c {
            return Err(context.error("min>max".to_string()));
        }

        Ok(Value::float(a.clamp(b,c)))
    }).float().float().float().end();

    //
    lib_scope.method("abs", |context|{
        Ok(Value::float(context.param(0).as_float().abs()))
    }).float().end();
    
    lib_scope.method("floor", |context|{
        Ok(Value::float(context.param(0).as_float().floor()))
    }).float().end();

    lib_scope.method("ceil", |context|{
        Ok(Value::float(context.param(0).as_float().ceil()))
    }).float().end();

    lib_scope.method("round", |context|{
        Ok(Value::float(context.param(0).as_float().round()))
    }).float().end();

    lib_scope.method("trunc", |context|{
        Ok(Value::float(context.param(0).as_float().trunc()))
    }).float().end();

    lib_scope.method("fract", |context|{
        Ok(Value::float(context.param(0).as_float().fract()))
    }).float().end();
    
    //
    lib_scope.method("is_infinite", |context|{
        Ok(Value::Bool(context.param(0).as_float().is_infinite()))
    }).float().end();

    lib_scope.method("is_nan", |context|{
        Ok(Value::Bool(context.param(0).as_float().is_nan()))
    }).float().end();
    
    lib_scope.method("sign", |context|{
        let a=context.param(0).as_float();

        let b= if a.is_sign_positive() {
            1.0
        } else if a.is_sign_negative() {
            -1.0
        } else {
            0.0
        };

        Ok(Value::float(b))
    }).float().end();

    //
    lib_scope.method("sqrt", |context|{
        Ok(Value::float(context.param(0).as_float().sqrt()))
    }).float().end();
    
    lib_scope.method("cbrt", |context|{
        Ok(Value::float(context.param(0).as_float().cbrt()))
    }).float().end();

    lib_scope.method("pow", |context|{
        Ok(Value::float(context.param(0).as_float().powf(context.param(1).as_float())))
    }).float().float().end();

    //
    lib_scope.insert_constant("pi", Value::float(std::f64::consts::PI as FloatT));
    lib_scope.insert_constant("pi2", Value::float(std::f64::consts::FRAC_PI_2 as FloatT));
    lib_scope.insert_constant("pi4", Value::float(std::f64::consts::FRAC_PI_4 as FloatT));

    //
    lib_scope.method("cos",|context|{
        Ok(Value::float(context.param(0).as_float().cos()))
    }).float().end();
    
    lib_scope.method("acos",|context|{
        Ok(Value::float(context.param(0).as_float().acos()))
    }).float().end();
    
    lib_scope.method("sin",|context|{
        Ok(Value::float(context.param(0).as_float().sin()))
    }).float().end();

    lib_scope.method("asin",|context|{
        Ok(Value::float(context.param(0).as_float().asin()))
    }).float().end();

    lib_scope.method("tan",|context|{
        Ok(Value::float(context.param(0).as_float().tan()))
    }).float().end();
    
    lib_scope.method("atan",|context|{
        Ok(Value::float(context.param(0).as_float().atan()))
    }).float().end();
    
    lib_scope.method("atan2",|context|{
        Ok(Value::float(context.param(0).as_float().atan2(context.param(1).as_float())))
    }).float().float().end();

    //
    lib_scope.method("lerp",|context|{
        let x=context.param(0).as_float();
        let y=context.param(1).as_float();
        let a=context.param(2).as_float();
        Ok(Value::float(x*(1.0-a)+y*a))
    }).float().float().float().end();

    
    lib_scope.method("smooth_step",|context|{
        let edge0=context.param(0).as_float();
        let edge1=context.param(1).as_float();
        let x=context.param(2).as_float();
        let t=((x-edge0)/(edge1-edge0)).clamp(0.0,1.0);
        let q= t * t * (3.0 - 2.0 * t);

        Ok(Value::float(q))
    }).float().float().float().end();
}