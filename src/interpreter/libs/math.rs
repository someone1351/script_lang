// use std::collections::HashSet;

// use super::super::super::common::*;

// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;

// use super::super::data::*;

pub fn register_num_misc<X>(lib_scope : &mut LibScope<X>) {

    //string(bool)
    lib_scope.method("string",|context|{
        Ok(Value::string(context.param(0).as_bool().to_string()))
    }).bool().end();


}

pub fn register_float<X>(lib_scope : &mut LibScope<X>) {
    //string(float)
    lib_scope.method("string",|context|{
        let a=context.param(0).as_float();
        Ok(a.to_string().into())
    }).float().end();

    //float(int)
    lib_scope.method("float", |context|{
        let a=context.param(0).as_float();
        Ok(a.into())
    }).int().end();

    //neg(float)
    lib_scope.method("-", |context|{
        let a=context.param(0).as_float();
        Ok(a.neg().into())
    }).float().end();

    //add(float,float), add(int,float), add(float,int)
    lib_scope.method("+", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.add(b).into())
    }).float().float().end().int().float().end().float().int().end();

    //sub(float,float), sub(int,float), sub(float,int)
    lib_scope.method("-", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.sub(b).into())
    }).float().float().end().int().float().end().float().int().end();

    //mul(float,float), mul(int,float), mul(float,int)
    lib_scope.method("*", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.mul(b).into())
    }).float().float().end().int().float().end().float().int().end();

    //div(float,float), div(int,float)
    lib_scope.method("/", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.div(b).into())
    }).float().float().end().int().float().end();

    //gt(float,float)
    lib_scope.method(">", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.gt(&b).into())
    }).float().float().end();

    //lt(float,float)
    lib_scope.method("<", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.lt(&b).into())
    }).float().float().end();

    //ge(float,float)
    lib_scope.method(">=", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.ge(&b).into())
    }).float().float().end();

    //le(float,float)
    lib_scope.method("<=", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.le(&b).into())
    }).float().float().end();

    //eq(float,float)
    lib_scope.method("=", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.eq(&b).into())
    }).float().float().end();

    //min(float,float)
    lib_scope.method("min", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.min(b).into())
    }).float().float().end();

    //max(float,float)
    lib_scope.method("max", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.max(b).into())
    }).float().float().end();

    //clamp(float,float,float)
    lib_scope.method("clamp", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();
        Ok(a.checked_clamp(b,c)?.into())
    }).float().float().float().end();

    //abs(float)
    lib_scope.method("abs", |context|{
        let a=context.param(0).as_float();
        Ok(a.abs().into())
    }).float().end();

    //sign(float)
    lib_scope.method("signum", |context|{
        let a=context.param(0).as_float();
        Ok(a.signum().into())
    }).float().end();


    //pow(float,float)
    lib_scope.method("pow", |context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.powf(b).into())
    }).float().float().end();

    //floor(float)
    lib_scope.method("floor", |context|{
        let a=context.param(0).as_float();
        Ok(Value::float(a.floor()))
    }).float().end();

    //ceil(float)
    lib_scope.method("ceil", |context|{
        let a=context.param(0).as_float();
        Ok(Value::float(a.ceil()))
    }).float().end();

    //round(float)
    lib_scope.method("round", |context|{
        let a=context.param(0).as_float();
        Ok(Value::float(a.round()))
    }).float().end();

    //trunc(float)
    lib_scope.method("trunc", |context|{
        let a=context.param(0).as_float();
        Ok(Value::float(a.trunc()))
    }).float().end();

    //fract(float)
    lib_scope.method("fract", |context|{
        let a=context.param(0).as_float();
        Ok(Value::float(a.fract()))
    }).float().end();

    //is_infinite(float)
    lib_scope.method("is_infinite", |context|{
        let a=context.param(0).as_float();
        Ok(a.is_infinite().into())
    }).float().end();

    //is_nan(float)
    lib_scope.method("is_nan", |context|{
        let a=context.param(0).as_float();
        Ok(a.is_nan().into())
    }).float().end();

    //sqrt(float)
    lib_scope.method("sqrt", |context|{
        let a=context.param(0).as_float();
        Ok(a.sqrt().into())
    }).float().end();

    //cbrt(float)
    lib_scope.method("cbrt", |context|{
        let a=context.param(0).as_float();
        Ok(a.cbrt().into())
    }).float().end();

    //cos(float)
    lib_scope.method("cos",|context|{
        let a=context.param(0).as_float();
        Ok(a.cos().into())
    }).float().end();

    //acos(float)
    lib_scope.method("acos",|context|{
        let a=context.param(0).as_float();
        Ok(a.acos().into())
    }).float().end();

    //sin(float)
    lib_scope.method("sin",|context|{
        let a=context.param(0).as_float();
        Ok(a.sin().into())
    }).float().end();

    //asin(float)
    lib_scope.method("asin",|context|{
        let a=context.param(0).as_float();
        Ok(a.asin().into())
    }).float().end();

    //tan(float)
    lib_scope.method("tan",|context|{
        let a=context.param(0).as_float();
        Ok(a.tan().into())
    }).float().end();

    //atan(float)
    lib_scope.method("atan",|context|{
        let a=context.param(0).as_float();
        Ok(a.atan().into())
    }).float().end();

    //atan2(float,float)
    lib_scope.method("atan2",|context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        Ok(a.atan2(b).into())
    }).float().float().end();

    //lerp(float,float,float)
    lib_scope.method("lerp",|context|{
        let x=context.param(0).as_float();
        let y=context.param(1).as_float();
        let a=context.param(2).as_float();

        //x*(1.0-a)+a*y)

        Ok(a.add((-1.0).into()).mul(x).add(a.mul(y)).into())
    }).float().float().float().end();

    //inv_lerp(float,float,float)

    lib_scope.method("inv_lerp",|context|{
        let x=context.param(0).as_float();
        let e0=context.param(1).as_float().to_same(x);
        let e1=context.param(2).as_float().to_same(x);

        //((x-e0)/(e1-e0)).clamp(0.0,1.0)
        Ok(x.sub(e0).div(e1.sub(e0)).checked_clamp(0.0.into(), 1.0.into())?.into())
    }).float().float().float().end();


    //smooth_step(float,float,float)
    lib_scope.method("smooth_step",|context|{
        let t=context.param(0).as_float();

        //t * t * (3.0 - 2.0 * t)
        let x=t.mul(2.0.into()).neg().add(3.0.into());
        let y = t.checked_powi(2.into())?.mul(x);

        Ok(y.into())
    }).float().float().float().end();

    //
    lib_scope.insert_constant("pi", Value::float(std::f64::consts::PI));
    lib_scope.insert_constant("pi2", Value::float(std::f64::consts::FRAC_PI_2));
    lib_scope.insert_constant("pi4", Value::float(std::f64::consts::FRAC_PI_4));
}

pub fn register_int<X>(lib_scope : &mut LibScope<X>) {

    //string(int)
    lib_scope.method("string",|context|{
        Ok(Value::string(context.param(0).as_int().to_string()))
    }).int().end();

    //int(float)
    lib_scope.method("int", |context|{
        Ok(Value::int(context.param(0).as_int()))
    }).float().end();

    //neg(int)
    lib_scope.method("-", |context|{
        Ok(context.param(0).as_int().checked_neg()?.into())
    }).int().end();


    //add(int,int)
    lib_scope.method("+",|context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.checked_add(b)?.into())
    }).int().int().end();

    //sub(int,int)
    lib_scope.method("-",|context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.checked_add(b)?.into())
    }).int().int().end();

    //mul(int,int)
    lib_scope.method("*",|context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.checked_mul(b)?.into())
    }).int().int().end();


    //div(int,int), div(float,int)
    lib_scope.method("/",|context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.checked_div(b)?.into())
    })
        .int().int().end()
        .float().int().end();

    //gt(int,int)
    lib_scope.method(">",|context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.gt(&b).into())
    }).int().int().end();


    //lt(int,int)
    lib_scope.method("<",|context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.lt(&b).into())
    }).int().int().end();

    //ge(int,int)
    lib_scope.method(">=",|context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.ge(&b).into())
    }).int().int().end();

    //le(int,int)
    lib_scope.method("<=",|context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.le(&b).into())
    }).int().int().end();

    //eq(int,int)
    lib_scope.method("=",|context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.eq(&b).into())
    }).int().int().end();

    //min(int,int)
    lib_scope.method("min", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.checked_min(b)?.into())
    }).int().int().end();


    //max(int,int)
    lib_scope.method("max", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.checked_max(b)?.into())
    }).int().int().end();


    //clamp(int,int,int)
    lib_scope.method("clamp", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        let c=context.param(2).as_int();
        Ok(a.checked_clamp(b,c)?.into())
    }).int().int().int().end();

    //abs(int)
    lib_scope.method("abs", |context|{
        let a=context.param(0).as_int();
        Ok(a.abs().abs().into())
    }).int().end();

    //sign(int)
    lib_scope.method("signum", |context|{
        let a=context.param(0).as_int();
        Ok(a.abs().signum().into())
    }).int().end();


	//pow(int,int)
    lib_scope.method("pow", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.checked_pow(b)?.into())
    }).int().int().end();

	//mod(int,int)
    lib_scope.method("mod", |context|{
        let a=context.param(0).as_int();
        let b=context.param(1).as_int();
        Ok(a.checked_rem(b)?.into())
    }).int().int().end();
}

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    register_float(lib_scope);
    register_int(lib_scope);
    register_num_misc(lib_scope);
}