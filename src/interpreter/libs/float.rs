
use super::super::super::common::*;
// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    
    lib_scope.method_ext("string", |context|{
        let x = context.param(0).as_float();
        Ok(Value::string(format!("{x:?}")))
    }).float().end();

    lib_scope.method_ext("is_float", |_|{
        Ok(Value::Bool(true))
    }).float().end();

    lib_scope.method_ext("is_float", |_|{
        Ok(Value::Bool(false))
    }).any().end();


    //
    lib_scope.method_ext("-", |context|{
        Ok(Value::Float(-context.param(0).as_float()))
    }).float().end();

    //
    lib_scope.method_ext("+", |context|{
        Ok(Value::Float(context.param(0).as_float()+context.param(1).as_float()))
    })
    .float().or_int().float().end()
    .float().int().end();

    lib_scope.method_ext("-", |context|{
        Ok(Value::Float(context.param(0).as_float()-context.param(1).as_float()))
    })
    .float().or_int().float().end()
    .float().int().end();

    lib_scope.method_ext("*", |context|{
        Ok(Value::Float(context.param(0).as_float()*context.param(1).as_float()))
    })
    .float().or_int().float().end()
    .float().int().end();


    lib_scope.method_ext("/", |context|{
        Ok(Value::Float(context.param(0).as_float()/context.param(1).as_float()))  
    })
    .float().or_int().float().end();



    //
    lib_scope.method_ext(">", |context|{
        Ok(Value::Bool(context.param(0).as_float()>context.param(1).as_float()))
    }).float().float().end();
    
    lib_scope.method_ext("<", |context|{
        Ok(Value::Bool(context.param(0).as_float()<context.param(1).as_float()))
    }).float().float().end();
    
    lib_scope.method_ext(">=", |context|{
        Ok(Value::Bool(context.param(0).as_float()>=context.param(1).as_float()))
    }).float().float().end();
    
    lib_scope.method_ext("<=", |context|{
        Ok(Value::Bool(context.param(0).as_float()<=context.param(1).as_float()))
    }).float().float().end();

    lib_scope.method_ext("=", |context|{
        Ok(Value::Bool(context.param(0).as_float()==context.param(1).as_float()))
    }).float().float().end();

    //
    lib_scope.method_ext("min", |context|{
        Ok(Value::Float(context.param(0).as_float().min(context.param(1).as_float())))
    }).float().float().end();

    lib_scope.method_ext("max", |context|{
        Ok(Value::Float(context.param(0).as_float().min(context.param(1).as_float())))
    }).float().float().end();

    lib_scope.method_ext("clamp", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();

        if b > c {
            return Err(context.error("min>max".to_string()));
        }

        Ok(Value::Float(a.clamp(b,c)))
    }).float().float().float().end();

    //
    lib_scope.method_ext("abs", |context|{
        Ok(Value::Float(context.param(0).as_float().abs()))
    }).float().end();
    
    lib_scope.method_ext("floor", |context|{
        Ok(Value::Float(context.param(0).as_float().floor()))
    }).float().end();

    lib_scope.method_ext("ceil", |context|{
        Ok(Value::Float(context.param(0).as_float().ceil()))
    }).float().end();

    lib_scope.method_ext("round", |context|{
        Ok(Value::Float(context.param(0).as_float().round()))
    }).float().end();

    lib_scope.method_ext("trunc", |context|{
        Ok(Value::Float(context.param(0).as_float().trunc()))
    }).float().end();

    lib_scope.method_ext("fract", |context|{
        Ok(Value::Float(context.param(0).as_float().fract()))
    }).float().end();
    
    //

    lib_scope.method_ext("is_infinite", |context|{
        Ok(Value::Bool(context.param(0).as_float().is_infinite()))
    }).float().end();

    lib_scope.method_ext("is_nan", |context|{
        Ok(Value::Bool(context.param(0).as_float().is_nan()))
    }).float().end();
    
    lib_scope.method_ext("sign", |context|{
        let a=context.param(0).as_float();

        let b= if a.is_sign_positive() {
            1.0
        } else if a.is_sign_negative() {
            -1.0
        } else {
            0.0
        };

        Ok(Value::Float(b))
    }).float().end();

    //
    lib_scope.method_ext("sqrt", |context|{
        Ok(Value::Float(context.param(0).as_float().sqrt()))
    }).float().end();
    
    lib_scope.method_ext("cbrt", |context|{
        Ok(Value::Float(context.param(0).as_float().cbrt()))
    }).float().end();

    lib_scope.method_ext("pow", |context|{
        Ok(Value::Float(context.param(0).as_float().powf(context.param(1).as_float())))
    }).float().float().end();

    //
    lib_scope.insert_constant("pi", Value::Float(std::f64::consts::PI as FloatT));
    lib_scope.insert_constant("pi2", Value::Float(std::f64::consts::FRAC_PI_2 as FloatT));
    lib_scope.insert_constant("pi4", Value::Float(std::f64::consts::FRAC_PI_4 as FloatT));

    //
    lib_scope.method_ext("cos",|context|{
        Ok(Value::Float(context.param(0).as_float().cos()))
    }).float().end();
    
    lib_scope.method_ext("acos",|context|{
        Ok(Value::Float(context.param(0).as_float().acos()))
    }).float().end();
    
    lib_scope.method_ext("sin",|context|{
        Ok(Value::Float(context.param(0).as_float().sin()))
    }).float().end();

    lib_scope.method_ext("asin",|context|{
        Ok(Value::Float(context.param(0).as_float().asin()))
    }).float().end();

    lib_scope.method_ext("tan",|context|{
        Ok(Value::Float(context.param(0).as_float().tan()))
    }).float().end();
    
    lib_scope.method_ext("atan",|context|{
        Ok(Value::Float(context.param(0).as_float().atan()))
    }).float().end();
    
    lib_scope.method_ext("atan2",|context|{
        Ok(Value::Float(context.param(0).as_float().atan2(context.param(1).as_float())))
    }).float().float().end();

    //

    lib_scope.method_ext("lerp",|context|{
        let x=context.param(0).as_float();
        let y=context.param(1).as_float();
        let a=context.param(2).as_float();
        Ok(Value::Float(x*(1.0-a)+y*a))
    }).float().float().float().end();

    
    lib_scope.method_ext("smooth_step",|context|{
        let edge0=context.param(0).as_float();
        let edge1=context.param(1).as_float();
        let x=context.param(2).as_float();
        let t=((x-edge0)/(edge1-edge0)).clamp(0.0,1.0);
        let q= t * t * (3.0 - 2.0 * t);

        Ok(Value::Float(q))
    }).float().float().float().end();
}