// use std::collections::HashSet;

// use super::super::super::common::*;

// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;

// use super::super::data::*;

pub fn register<X>(lib_scope : &mut LibScope<X>) {

    //string(float)
    lib_scope.method("string",|context|{
        Ok(Value::string(context.param(0).as_float().to_string()))
    }).float().end();

    //string(int)
    lib_scope.method("string",|context|{
        Ok(Value::string(context.param(0).as_int().to_string()))
    }).int().end();

    //string(bool)
    lib_scope.method("string",|context|{
        Ok(Value::string(context.param(0).as_bool().to_string()))
    }).bool().end();
    //string(float)

    //float(int)
    lib_scope.method("float", |context|{
        Ok(Value::float(context.param(0).as_float()))
    }).int().end();

    //int(float)
    lib_scope.method("int", |context|{
        Ok(Value::int(context.param(0).as_int()))
    }).float().end();

    //neg(int)
    lib_scope.method("-", |context|{
        Ok(Value::int(-context.param(0).as_int()))
    }).int().end();

    //neg(float)
    lib_scope.method("-", |context|{
        Ok(Value::float(-context.param(0).as_float()))
    }).float().end();

    //add(float,float), add(int,float), add(float,int)
    lib_scope.method("+", |context|{
        Ok(Value::float(context.param(0).as_float()+context.param(1).as_float()))
    })
        .float().float().end()
        .int().float().end()
        .float().int().end();

    //add(int,int)
    lib_scope.method("+",|context|{
        Ok(Value::int(context.param(0).as_int()+context.param(1).as_int()))
    }).int().int().end();

    //sub(float,float), sub(int,float), sub(float,int)
    lib_scope.method("-", |context|{
        Ok(Value::float(context.param(0).as_float()-context.param(1).as_float()))
    })
        .float().float().end()
        .int().float().end()
        .float().int().end();

    //sub(int,int)
    lib_scope.method("-",|context|{
        Ok(Value::int(context.param(0).as_int()-context.param(1).as_int()))
    }).int().int().end();

    //mul(float,float), mul(int,float), mul(float,int)
    lib_scope.method("*", |context|{
        Ok(Value::float(context.param(0).as_float()*context.param(1).as_float()))
    })
        .float().float().end()
        .int().float().end()
        .float().int().end();

    //mul(int,int)
    lib_scope.method("*",|context|{
        Ok(Value::int(context.param(0).as_int()*context.param(1).as_int()))
    }).int().int().end();

    //div(float,float), div(int,float)
    lib_scope.method("/", |context|{
        Ok(Value::float(context.param(0).as_float()/context.param(1).as_float()))
    })
        .float().float().end()
        .int().float().end();

    //div(int,int), div(float,int)
    lib_scope.method("/",|context|{
        context.param(0).as_int().checked_div(context.param(1).as_int())
            .and_then(|x|Some(Value::int(x)))
            .ok_or(context.error("Divide by zero".to_string()))
    })
        .int().int().end()
        .float().int().end();

    //gt(float,float)
    lib_scope.method(">", |context|{
        Ok(Value::Bool(context.param(0).as_float()>context.param(1).as_float()))
    }).float().float().end();

    //gt(int,int)
    lib_scope.method(">",|context|{
        Ok(Value::Bool(context.param(0).as_int()>context.param(1).as_int()))
    }).int().int().end();

    //lt(float,float)
    lib_scope.method("<", |context|{
        Ok(Value::Bool(context.param(0).as_float()<context.param(1).as_float()))
    }).float().float().end();

    //lt(int,int)
    lib_scope.method("<",|context|{
        Ok(Value::Bool(context.param(0).as_int()<context.param(1).as_int()))
    }).int().int().end();

    //ge(float,float)
    lib_scope.method(">=", |context|{
        Ok(Value::Bool(context.param(0).as_float()>=context.param(1).as_float()))
    }).float().float().end();

    //ge(int,int)
    lib_scope.method(">=",|context|{
        Ok(Value::Bool(context.param(0).as_int()>=context.param(1).as_int()))
    }).int().int().end();

    //le(float,float)
    lib_scope.method("<=", |context|{
        Ok(Value::Bool(context.param(0).as_float()<=context.param(1).as_float()))
    }).float().float().end();

    //le(int,int)
    lib_scope.method("<=",|context|{
        Ok(Value::Bool(context.param(0).as_int()<=context.param(1).as_int()))
    }).int().int().end();

    //eq(float,float)
    lib_scope.method("=", |context|{
        Ok(Value::Bool(context.param(0).as_float()==context.param(1).as_float()))
    }).float().float().end();

    //eq(int,int)
    lib_scope.method("=",|context|{
        Ok(Value::Bool(context.param(0).as_int()==context.param(1).as_int()))
    }).int().int().end();

    //min(float,float)
    lib_scope.method("min", |context|{
        Ok(Value::float(context.param(0).as_float().min(context.param(1).as_float())))
    }).float().float().end();

    //min(int,int)
    lib_scope.method("min", |context|{
        Ok(Value::int(context.param(0).as_int().min(context.param(1).as_int())))
    }).int().int().end();

    //max(float,float)
    lib_scope.method("max", |context|{
        Ok(Value::float(context.param(0).as_float().min(context.param(1).as_float())))
    }).float().float().end();

    //max(int,int)
    lib_scope.method("max", |context|{
        Ok(Value::int(context.param(0).as_int().max(context.param(1).as_int())))
    }).int().int().end();

    //clamp(float,float,float)
    lib_scope.method("clamp", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();
        if b > c { return Err(context.error("min>max".to_string())); }
        Ok(Value::float(a.clamp(b,c)))
    }).float().float().float().end();

    //clamp(int,int,int)
    lib_scope.method("clamp", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        let c=context.param(2).as_int();
        if b > c { return Err(context.error("min>max".to_string())); }
        // if a <b || a>c { return Err(context.error("value not within min/max".to_string())); }
        Ok(Value::int(a.clamp(b,c)))
    }).int().int().int().end();

    //abs(float)
    lib_scope.method("abs", |context|{
        Ok(Value::float(context.param(0).as_float().abs()))
    }).float().end();

    //abs(int)
    lib_scope.method("abs", |context|{
        Ok(Value::int(context.param(0).as_int().abs()))
    }).int().end();

    //sign(float)
    lib_scope.method("sign", |context|{
        let a=context.param(0).as_float();
        let b= if a.is_sign_positive() {1.0} else if a.is_sign_negative() {-1.0} else {0.0};
        Ok(Value::float(b))
    }).float().end();

    //sign(int)
    lib_scope.method("sign", |context|{
        let a=context.param(0).as_int();
        let b= if a > 0 {1} else if a < 0 {-1} else {0};
        Ok(Value::int(b))
    }).int().end();

    //pow(float,float)
    lib_scope.method("pow", |context|{
        Ok(Value::float(context.param(0).as_float().powf(context.param(1).as_float())))
    }).float().float().end();

	//pow(int,int)
    lib_scope.method("pow", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        if b<0 {return Err(context.error("exp less than 0".to_string()));}
        Ok(Value::int(a.pow(b as u32)))
    }).int().int().end();

	//mod(int,int)
    lib_scope.method("mod", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(Value::int(a%b))
    }).int().int().end();

    //floor(float)
    lib_scope.method("floor", |context|{
        Ok(Value::float(context.param(0).as_float().floor()))
    }).float().end();

    //ceil(float)
    lib_scope.method("ceil", |context|{
        Ok(Value::float(context.param(0).as_float().ceil()))
    }).float().end();

    //round(float)
    lib_scope.method("round", |context|{
        Ok(Value::float(context.param(0).as_float().round()))
    }).float().end();

    //trunc(float)
    lib_scope.method("trunc", |context|{
        Ok(Value::float(context.param(0).as_float().trunc()))
    }).float().end();

    //fract(float)
    lib_scope.method("fract", |context|{
        Ok(Value::float(context.param(0).as_float().fract()))
    }).float().end();

    //is_infinite(float)
    lib_scope.method("is_infinite", |context|{
        Ok(Value::Bool(context.param(0).as_float().is_infinite()))
    }).float().end();

    //is_nan(float)
    lib_scope.method("is_nan", |context|{
        Ok(Value::Bool(context.param(0).as_float().is_nan()))
    }).float().end();

    //sqrt(float)
    lib_scope.method("sqrt", |context|{
        Ok(Value::float(context.param(0).as_float().sqrt()))
    }).float().end();

    //cbrt(float)
    lib_scope.method("cbrt", |context|{
        Ok(Value::float(context.param(0).as_float().cbrt()))
    }).float().end();

    //cos(float)
    lib_scope.method("cos",|context|{
        Ok(Value::float(context.param(0).as_float().cos()))
    }).float().end();

    //acos(float)
    lib_scope.method("acos",|context|{
        Ok(Value::float(context.param(0).as_float().acos()))
    }).float().end();

    //sin(float)
    lib_scope.method("sin",|context|{
        Ok(Value::float(context.param(0).as_float().sin()))
    }).float().end();

    //asin(float)
    lib_scope.method("asin",|context|{
        Ok(Value::float(context.param(0).as_float().asin()))
    }).float().end();

    //tan(float)
    lib_scope.method("tan",|context|{
        Ok(Value::float(context.param(0).as_float().tan()))
    }).float().end();

    //atan(float)
    lib_scope.method("atan",|context|{
        Ok(Value::float(context.param(0).as_float().atan()))
    }).float().end();

    //atan2(float,float)
    lib_scope.method("atan2",|context|{
        Ok(Value::float(context.param(0).as_float().atan2(context.param(1).as_float())))
    }).float().float().end();

    //lerp(float,float,float)
    lib_scope.method("lerp",|context|{
        let x=context.param(0).as_float();
        let y=context.param(1).as_float();
        let a=context.param(2).as_float();
        Ok(Value::float(x*(1.0-a)+y*a))
    }).float().float().float().end();

    //smooth_step(float,float,float)
    lib_scope.method("smooth_step",|context|{
        let edge0=context.param(0).as_float();
        let edge1=context.param(1).as_float();
        let x=context.param(2).as_float();
        let t=((x-edge0)/(edge1-edge0)).clamp(0.0,1.0);
        let q= t * t * (3.0 - 2.0 * t);
        Ok(Value::float(q))
    }).float().float().float().end();

    //
    lib_scope.insert_constant("pi", Value::float(std::f64::consts::PI));
    lib_scope.insert_constant("pi2", Value::float(std::f64::consts::FRAC_PI_2));
    lib_scope.insert_constant("pi4", Value::float(std::f64::consts::FRAC_PI_4));

}