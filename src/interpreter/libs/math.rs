use std::collections::HashSet;

use super::super::super::common::*;

// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;

// use super::super::data::*;

#[derive(Clone)]
pub struct Vec2(pub [FloatT;2]);
#[derive(Clone)]
pub struct Vec3(pub [FloatT;3]);
#[derive(Clone)]
pub struct Vec4(pub [FloatT;4]);


#[derive(Clone)]
pub struct IVec2(pub [IntT;2]);
#[derive(Clone)]
pub struct IVec3(pub [IntT;3]);
#[derive(Clone)]
pub struct IVec4(pub [IntT;4]);

fn get_field_inds(fields:&str,) -> Vec<(char, Option<usize>)> {
    fields.chars().map(|c|(c,match c {
        'r'|'x'=>Some(0),
        'g'|'y'=>Some(1),
        'b'|'z'=>Some(2),
        'a'|'w'=>Some(3),
        _=>None,
    })).collect()
}

fn get_field_util<T:Copy,const FROM_N: usize>(fields:&str,from:&[T;FROM_N]) -> Result<Vec<T>, String> {
    let field_inds = get_field_inds(fields);

    if field_inds.len()>4 {
        return Err("too many fields".to_string());
    }

    for &(c,field) in field_inds.iter() {
        if field.is_none() || field.unwrap()>FROM_N {
            return Err(format!("invalid field '{c}'"));
        }
    }

    Ok(field_inds.iter().map(|x|from[x.1.unwrap()]).collect())
}

fn get_field_util_float<const FROM_N: usize>(fields:&str,from:&[FloatT;FROM_N]) -> Result<Value, String> {
    let v=get_field_util::<FloatT,FROM_N>(fields,from)?;
    Ok(match v.len() {
        1 => Value::float(v[0]),
        2 => Value::custom_unmanaged(Vec2(v.try_into().unwrap())),
        3 => Value::custom_unmanaged(Vec3(v.try_into().unwrap())),
        _ => Value::custom_unmanaged(Vec4(v.try_into().unwrap())),
    })
}

fn get_field_util_int<const FROM_N: usize>(fields:&str,from:&[IntT;FROM_N]) -> Result<Value, String> {
    let v=get_field_util::<IntT,FROM_N>(fields,from)?;
    Ok(match v.len() {
        1 => Value::int(v[0]),
        2 => Value::custom_unmanaged(IVec2(v.try_into().unwrap())),
        3 => Value::custom_unmanaged(IVec3(v.try_into().unwrap())),
        _ => Value::custom_unmanaged(IVec4(v.try_into().unwrap())),
    })
}

fn set_field_util<T:Copy,const FROM_N: usize,const TO_N: usize>(fields:&str,from:&mut [T;FROM_N],to:&[T;TO_N]) -> Result<(), String> {

    let field_inds = get_field_inds(fields);

    if field_inds.len()>FROM_N {
        return Err("too many fields".to_string());
    }

    if HashSet::<char>::from_iter(field_inds.iter().map(|x|x.0)).len()!=field_inds.len() {
        return Err("duplicate fields".to_string());
    }

    for &(c,field) in field_inds.iter() {
        if field.is_none() || field.unwrap()>FROM_N {
            return Err(format!("invalid field '{c}'"));
        }
    }

    for (i,&(_,field)) in field_inds.iter().enumerate() {
        from[field.unwrap()]=to[i];
    }

    Ok(())
}

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    //string(float)
    lib_scope.method("string", |context|{
        Ok(Value::string(format!("{:?}",context.param(0).as_float())))
    }).float().end();

    //string(int)
    lib_scope.method("string", |context|{
        Ok(Value::string(format!("{:?}",context.param(0).as_int())))
    }).int().end();

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

    //vec2(ivec2)
    lib_scope.method("vec2",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged_mut(Vec2(v.0.map(|x|x as FloatT))))
    }).custom_ref::<IVec2>().end();

    //vec3(ivec3)
    lib_scope.method("vec3",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged_mut(Vec3(v.0.map(|x|x as FloatT))))
    }).custom_ref::<IVec3>().end();

    //vec4(ivec4)
    lib_scope.method("vec4",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec4>()?;
        Ok(Value::custom_unmanaged_mut(Vec4(v.0.map(|x|x as FloatT))))
    }).custom_ref::<IVec4>().end();

    //ivec2(vec2)
    lib_scope.method("ivec2",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged_mut(IVec2(v.0.map(|x|x as IntT))))
    }).custom_ref::<Vec2>().end();

    //ivec3(vec3)
    lib_scope.method("ivec3",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged_mut(IVec3(v.0.map(|x|x as IntT))))
    }).custom_ref::<Vec3>().end();

    //ivec4(vec4)
    lib_scope.method("ivec4",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec4>()?;
        Ok(Value::custom_unmanaged_mut(IVec4(v.0.map(|x|x as IntT))))
    }).custom_ref::<Vec4>().end();

    //vec2(), vec2(f), vec2(f,f)
    lib_scope.method("vec2",|context|{
        let x=context.param(0).as_float();
        let y=context.get_param(1).map(|q|q.as_float()).unwrap_or(x);
        Ok(Value::custom_unmanaged_mut(Vec2([x,y])))
    })
        .end()
        .float().end()
        .float().float().end();

    //vec3(), vec3(f), vec3(f,f,f)
    lib_scope.method("vec3",|context|{
        let x=context.param(0).as_float();
        let y=context.get_param(1).map(|q|q.as_float()).unwrap_or(x);
        let z=context.get_param(2).map(|q|q.as_float()).unwrap_or(x);
        Ok(Value::custom_unmanaged_mut(Vec3([x,y,z])))
    })
        .end()
        .float().end()
        .float().float().float().end();

    //vec4(), vec4(f), vec4(f,f,f,f)
    lib_scope.method("vec4",|context|{
        let x=context.param(0).as_float();
        let y=context.get_param(1).map(|q|q.as_float()).unwrap_or(x);
        let z=context.get_param(2).map(|q|q.as_float()).unwrap_or(x);
        let w=context.get_param(3).map(|q|q.as_float()).unwrap_or(x);
        Ok(Value::custom_unmanaged_mut(Vec4([x,y,z,w])))
    })
        .end()
        .float().end()
        .float().float().float().float().end();

    //vec3(v2,f)
    lib_scope.method("vec3",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        let f=context.param(1).as_float();
        Ok(Value::custom_unmanaged_mut(Vec3([v.0[0],v.0[1],f])))
    })
        .custom_ref::<Vec2>().float().end();

    //vec3(f,v2)
    lib_scope.method("vec3",|context|{
        let f=context.param(0).as_float();
        let v=context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged_mut(Vec3([f,v.0[0],v.0[1]])))
    })
        .float().custom_ref::<Vec2>().end();

    //vec4(v3,f)
    lib_scope.method("vec4",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        let f=context.param(1).as_float();
        Ok(Value::custom_unmanaged_mut(Vec4([v.0[0],v.0[1],v.0[2],f])))
    })
        .custom_ref::<Vec3>().float().end();

    //vec4(v3,f)
    lib_scope.method("vec4",|context|{
        let f=context.param(0).as_float();
        let v=context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged_mut(Vec4([f,v.0[0],v.0[1],v.0[2]])))
    })
        .float().custom_ref::<Vec3>().end();

    //vec4(v2,v2)
    lib_scope.method("vec4",|context|{
        let v0=context.param(0).as_custom().data_clone::<Vec2>()?;
        let v1=context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged_mut(Vec4([v0.0[0],v0.0[1],v1.0[0],v1.0[1]])))
    })
        .custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //vec4(v2,f,f)
    lib_scope.method("vec4",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        let f0=context.param(1).as_float();
        let f1=context.param(2).as_float();
        Ok(Value::custom_unmanaged_mut(Vec4([v.0[0],v.0[1],f0,f1])))
    })
        .custom_ref::<Vec2>().float().float().end();

    //vec4(f,f,v2)
    lib_scope.method("vec4",|context|{
        let f0=context.param(0).as_float();
        let f1=context.param(1).as_float();
        let v=context.param(2).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged_mut(Vec4([f0,f1,v.0[0],v.0[1],])))
    })
        .float().float().custom_ref::<Vec2>().end();

    //vec4(f,v2,f)
    lib_scope.method("vec4",|context|{
        let f0=context.param(0).as_float();
        let v=context.param(1).as_custom().data_clone::<Vec2>()?;
        let f1=context.param(2).as_float();
        Ok(Value::custom_unmanaged_mut(Vec4([f0,v.0[0],v.0[1],f1])))
    })
        .float().custom_ref::<Vec2>().float().end();


    //ivec2(), ivec2(f), ivec2(f,f)
    lib_scope.method("ivec2",|context|{
        let x=context.param(0).as_int();
        let y=context.get_param(1).map(|q|q.as_int()).unwrap_or(x);
        Ok(Value::custom_unmanaged_mut(IVec2([x,y])))
    })
        .end()
        .int().end()
        .int().int().end();

    //ivec3(), ivec3(f), ivec3(f,f,f)
    lib_scope.method("ivec3",|context|{
        let x=context.param(0).as_int();
        let y=context.get_param(1).map(|q|q.as_int()).unwrap_or(x);
        let z=context.get_param(2).map(|q|q.as_int()).unwrap_or(x);
        Ok(Value::custom_unmanaged_mut(IVec3([x,y,z])))
    })
        .end()
        .int().end()
        .int().int().int().end();

    //ivec4(), ivec4(f), ivec4(f,f,f,f)
    lib_scope.method("ivec4",|context|{
        let x=context.param(0).as_int();
        let y=context.get_param(1).map(|q|q.as_int()).unwrap_or(x);
        let z=context.get_param(2).map(|q|q.as_int()).unwrap_or(x);
        let w=context.get_param(3).map(|q|q.as_int()).unwrap_or(x);
        Ok(Value::custom_unmanaged_mut(IVec4([x,y,z,w])))
    })
        .end()
        .int().end()
        .int().int().int().int().end();

    //ivec3(v2,f)
    lib_scope.method("ivec3",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec2>()?;
        let f=context.param(1).as_int();
        Ok(Value::custom_unmanaged_mut(IVec3([v.0[0],v.0[1],f])))
    })
        .custom_ref::<IVec2>().int().end();

    //ivec3(f,v2)
    lib_scope.method("ivec3",|context|{
        let f=context.param(0).as_int();
        let v=context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged_mut(IVec3([f,v.0[0],v.0[1]])))
    })
        .int().custom_ref::<IVec2>().end();

    //ivec4(v3,f)
    lib_scope.method("ivec4",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec3>()?;
        let f=context.param(1).as_int();
        Ok(Value::custom_unmanaged_mut(IVec4([v.0[0],v.0[1],v.0[2],f])))
    })
        .custom_ref::<IVec3>().int().end();

    //ivec4(v3,f)
    lib_scope.method("ivec4",|context|{
        let f=context.param(0).as_int();
        let v=context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged_mut(IVec4([f,v.0[0],v.0[1],v.0[2]])))
    })
        .int().custom_ref::<IVec3>().end();

    //ivec4(v2,v2)
    lib_scope.method("ivec4",|context|{
        let v0=context.param(0).as_custom().data_clone::<IVec2>()?;
        let v1=context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged_mut(IVec4([v0.0[0],v0.0[1],v1.0[0],v1.0[1]])))
    })
        .custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //ivec4(v2,f,f)
    lib_scope.method("ivec4",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec2>()?;
        let f0=context.param(1).as_int();
        let f1=context.param(2).as_int();
        Ok(Value::custom_unmanaged_mut(IVec4([v.0[0],v.0[1],f0,f1])))
    })
        .custom_ref::<IVec2>().int().int().end();

    //ivec4(f,f,v2)
    lib_scope.method("ivec4",|context|{
        let f0=context.param(0).as_int();
        let f1=context.param(1).as_int();
        let v=context.param(2).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged_mut(IVec4([f0,f1,v.0[0],v.0[1],])))
    })
        .int().int().custom_ref::<IVec2>().end();

    //ivec4(f,v2,f)
    lib_scope.method("ivec4",|context|{
        let f0=context.param(0).as_int();
        let v=context.param(1).as_custom().data_clone::<IVec2>()?;
        let f1=context.param(2).as_int();
        Ok(Value::custom_unmanaged_mut(IVec4([f0,v.0[0],v.0[1],f1])))
    })
        .int().custom_ref::<IVec2>().int().end();

    //string(vec2)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        Ok(Value::string(format!("Vec2({}, {})",v.0[0],v.0[1])))
    }).custom_ref::<Vec2>().end();

    //string(vec3)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        Ok(Value::string(format!("Vec3({}, {}, {})",v.0[0],v.0[1],v.0[2])))
    }).custom_ref::<Vec3>().end();

    //string(vec4)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec4>()?;
        Ok(Value::string(format!("Vec4({}, {}, {}, {})",v.0[0],v.0[1],v.0[2],v.0[3])))
    }).custom_ref::<Vec4>().end();

    //string(ivec2)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec2>()?;
        Ok(Value::string(format!("IVec2({}, {})",v.0[0],v.0[1])))
    }).custom_ref::<IVec2>().end();

    //string(ivec3)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec3>()?;
        Ok(Value::string(format!("IVec3({}, {}, {})",v.0[0],v.0[1],v.0[2])))
    }).custom_ref::<IVec3>().end();

    //string(ivec4)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec4>()?;
        Ok(Value::string(format!("IVec4({}, {}, {}, {})",v.0[0],v.0[1],v.0[2],v.0[3])))
    }).custom_ref::<IVec4>().end();

    //get_field(vec2,ind)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..2 => Value::float(v.0[ind as usize]), _ => Value::Nil,})
    })
        .custom_ref::<Vec2>().int().end();

    //get_field(vec3,ind)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..3 => Value::float(v.0[ind as usize]), _ => Value::Nil,})
    })
        .custom_ref::<Vec3>().int().end();

    //get_field(vec4,ind)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec4>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..4 => Value::float(v.0[ind as usize]), _ => Value::Nil,})
    })
        .custom_ref::<Vec4>().int().end();

    //get_field(vec2,str)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        let fields = context.param(1).as_string();
        get_field_util_float(fields.as_str(), & v.0).map_err(|e|context.error(e))
    })
        .custom_ref::<Vec2>().str().end();

    //get_field(vec3,str)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        let fields = context.param(1).as_string();
        get_field_util_float(fields.as_str(), & v.0).map_err(|e|context.error(e))
    })
        .custom_ref::<Vec3>().str().end();

    //get_field(vec4,str)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec4>()?;
        let fields = context.param(1).as_string();
        get_field_util_float(fields.as_str(), & v.0).map_err(|e|context.error(e))
    })
        .custom_ref::<Vec4>().str().end();

    //get_field(ivec2,ind)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec2>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..2 => Value::int(v.0[ind as usize]), _ => Value::Nil,})
    })
        .custom_ref::<IVec2>().int().end();

    //get_field(ivec3,ind)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec3>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..3 => Value::int(v.0[ind as usize]), _ => Value::Nil,})
    })
        .custom_ref::<IVec3>().int().end();

    //get_field(ivec4,ind)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec4>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..4 => Value::int(v.0[ind as usize]), _ => Value::Nil,})
    })
        .custom_ref::<IVec4>().int().end();

    //get_field(ivec2,str)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec2>()?;
        let fields = context.param(1).as_string();
        get_field_util_int(fields.as_str(), & v.0).map_err(|e|context.error(e))
    })
        .custom_ref::<IVec2>().str().end();

    //get_field(ivec3,str)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec3>()?;
        let fields = context.param(1).as_string();
        get_field_util_int(fields.as_str(), & v.0).map_err(|e|context.error(e))
    })
        .custom_ref::<IVec3>().str().end();

    //get_field(ivec4,str)
    lib_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec4>()?;
        let fields = context.param(1).as_string();
        get_field_util_int(fields.as_str(), & v.0).map_err(|e|context.error(e))
    })
        .custom_ref::<IVec4>().str().end();

    //set_field(vec2,str,f)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_float()];

        this.as_custom().with_data_mut(|data:&mut Vec2|{
            set_field_util(fields.as_str(),&mut data.0,&to).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec2>().str().float().end();

    //set_field(vec3,str,f)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_float()];

        this.as_custom().with_data_mut(|data:&mut Vec3|{
            set_field_util(fields.as_str(),&mut data.0,&to).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec3>().str().float().end();

    //set_field(vec4,str,f)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_float()];

        this.as_custom().with_data_mut(|data:&mut Vec4|{
            set_field_util(fields.as_str(),&mut data.0,&to).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec4>().str().float().end();

    //set_field(vec2,str,vec2)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec2>()?;

        this.as_custom().with_data_mut(|data:&mut Vec2|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec2>().str().custom_ref::<Vec2>().end();

    //set_field(vec3,str,vec2)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec2>()?;

        this.as_custom().with_data_mut(|data:&mut Vec3|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec3>().str().custom_ref::<Vec2>().end();

    //set_field(vec3,str,vec3)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec3>()?;

        this.as_custom().with_data_mut(|data:&mut Vec3|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec3>().str().custom_ref::<Vec3>().end();

    //set_field(vec4,str,vec2)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec2>()?;

        this.as_custom().with_data_mut(|data:&mut Vec4|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec4>().str().custom_ref::<Vec2>().end();

    //set_field(vec4,str,vec3)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec3>()?;

        this.as_custom().with_data_mut(|data:&mut Vec4|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec4>().str().custom_ref::<Vec3>().end();

    //set_field(vec4,str,vec4)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec4>()?;

        this.as_custom().with_data_mut(|data:&mut Vec4|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec4>().str().custom_ref::<Vec4>().end();

    //set_field(ivec2,str,f)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_int()];

        this.as_custom().with_data_mut(|data:&mut IVec2|{
            set_field_util(fields.as_str(),&mut data.0,&to).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<IVec2>().str().int().end();

    //set_field(ivec3,str,f)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_int()];

        this.as_custom().with_data_mut(|data:&mut IVec3|{
            set_field_util(fields.as_str(),&mut data.0,&to).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<IVec3>().str().int().end();

    //set_field(ivec4,str,f)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_int()];

        this.as_custom().with_data_mut(|data:&mut IVec4|{
            set_field_util(fields.as_str(),&mut data.0,&to).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<IVec4>().str().int().end();

    //set_field(ivec2,str,ivec2)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec2>()?;

        this.as_custom().with_data_mut(|data:&mut IVec2|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    }).custom_ref::<IVec2>().str().custom_ref::<IVec2>().end();

    //set_field(ivec3,str,ivec2)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec2>()?;

        this.as_custom().with_data_mut(|data:&mut IVec3|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<IVec3>().str().custom_ref::<IVec2>().end();

    //set_field(ivec3,str,ivec3)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec3>()?;

        this.as_custom().with_data_mut(|data:&mut IVec3|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<IVec3>().str().custom_ref::<IVec3>().end();

    //set_field(ivec4,str,ivec2)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec2>()?;

        this.as_custom().with_data_mut(|data:&mut IVec4|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<IVec4>().str().custom_ref::<IVec2>().end();

    //set_field(ivec4,str,ivec3)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec3>()?;

        this.as_custom().with_data_mut(|data:&mut IVec4|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    }).custom_ref::<IVec4>().str().custom_ref::<IVec3>().end();

    //set_field(ivec4,str,ivec4)
    lib_scope.method("set_field",|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec4>()?;

        this.as_custom().with_data_mut(|data:&mut IVec4|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<IVec4>().str().custom_ref::<IVec4>().end();

    //neg(vec2)
    lib_scope.method("-",|context|{
        let mut v = context.param(0).as_custom().data_clone::<Vec2>()?;
        for i in 0..v.0.len() { v.0[i]=-v.0[i]; }
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<Vec2>().end();

    //neg(vec3)
    lib_scope.method("-",|context|{
        let mut v = context.param(0).as_custom().data_clone::<Vec3>()?;
        for i in 0..v.0.len() { v.0[i]=-v.0[i]; }
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<Vec3>().end();

    //neg(vec4)
    lib_scope.method("-",|context|{
        let mut v = context.param(0).as_custom().data_clone::<Vec4>()?;
        for i in 0..v.0.len() { v.0[i]=-v.0[i]; }
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<Vec4>().end();

    //neg(ivec2)
    lib_scope.method("-",|context|{
        let mut v = context.param(0).as_custom().data_clone::<IVec2>()?;
        for i in 0..v.0.len() { v.0[i]=-v.0[i]; }
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<IVec2>().end();

    //neg(ivec3)
    lib_scope.method("-",|context|{
        let mut v = context.param(0).as_custom().data_clone::<IVec3>()?;
        for i in 0..v.0.len() { v.0[i]=-v.0[i]; }
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<IVec3>().end();

    //neg(ivec4)
    lib_scope.method("-",|context|{
        let mut v = context.param(0).as_custom().data_clone::<IVec4>()?;
        for i in 0..v.0.len() { v.0[i]=-v.0[i]; }
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<IVec4>().end();

    //add(vec2,vec2)
    lib_scope.method("+",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() { a.0[i]+=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //add(vec3,vec3)
    lib_scope.method("+",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() { a.0[i]+=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //add(vec4,vec4)
    lib_scope.method("+",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() { a.0[i]+=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //add(ivec2,ivec2)
    lib_scope.method("+",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        for i in 0..a.0.len() { a.0[i]+=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //add(ivec3,ivec3)
    lib_scope.method("+",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        for i in 0..a.0.len() { a.0[i]+=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //add(ivec4,ivec4)
    lib_scope.method("+",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        for i in 0..a.0.len() { a.0[i]+=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //sub(vec2,vec2)
    lib_scope.method("-",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() { a.0[i]-=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //sub(vec3,vec3)
    lib_scope.method("-",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() { a.0[i]-=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //sub(vec4,vec4)
    lib_scope.method("-",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() { a.0[i]-=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //sub(ivec2,ivec2)
    lib_scope.method("-",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        for i in 0..a.0.len() { a.0[i]-=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //sub(ivec3,ivec3)
    lib_scope.method("-",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        for i in 0..a.0.len() { a.0[i]-=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //sub(ivec4,ivec4)
    lib_scope.method("-",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        for i in 0..a.0.len() { a.0[i]-=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //mul(vec2,f)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().end();

    //mul(f,vec2)
    lib_scope.method("*",|context|{
        let mut a = context.param(1).as_custom().data_clone::<Vec2>()?;
        let b=context.param(0).as_float();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).float().custom_ref::<Vec2>().end();

    //mul(vec3,f)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().end();

    //mul(f,vec3)
    lib_scope.method("*",|context|{
        let mut a = context.param(1).as_custom().data_clone::<Vec3>()?;
        let b=context.param(0).as_float();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).float().custom_ref::<Vec3>().end();

    //mul(vec4,f)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().float().end();

    //mul(f,vec4)
    lib_scope.method("*",|context|{
        let mut a = context.param(1).as_custom().data_clone::<Vec4>()?;
        let b=context.param(0).as_float();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).float().custom_ref::<Vec4>().end();

    //mul(ivec2,f)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().int().end();

    //mul(f,ivec2)
    lib_scope.method("*",|context|{
        let mut a = context.param(1).as_custom().data_clone::<IVec2>()?;
        let b=context.param(0).as_int();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).int().custom_ref::<IVec2>().end();

    //mul(ivec3,f)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().int().end();

    //mul(f,ivec3)
    lib_scope.method("*",|context|{
        let mut a = context.param(1).as_custom().data_clone::<IVec3>()?;
        let b=context.param(0).as_int();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).int().custom_ref::<IVec3>().end();

    //mul(ivec4,f)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().int().end();

    //mul(f,ivec4)
    lib_scope.method("*",|context|{
        let mut a = context.param(1).as_custom().data_clone::<IVec4>()?;
        let b=context.param(0).as_int();
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).int().custom_ref::<IVec4>().end();

    //mul(vec2,vec2)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() { a.0[i]*=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //mul(vec3,vec3)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() { a.0[i]*=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //mul(vec4,vec4)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() { a.0[i]*=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //mul(ivec2,ivec2)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        for i in 0..a.0.len() { a.0[i]*=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //mul(ivec3,ivec3)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        for i in 0..a.0.len() { a.0[i]*=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //mul(ivec4,ivec4)
    lib_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        for i in 0..a.0.len() { a.0[i]*=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //div(vec2,f)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() { a.0[i]/=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().end();

    //div(vec3,f)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() { a.0[i]/=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().end();

    //div(vec4,f)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() { a.0[i]/=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().float().end();

    //div(ivec2,f)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() { a.0[i]/=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().int().end();

    //div(ivec3,f)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() { a.0[i]/=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().int().end();

    //div(ivec4,f)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() { a.0[i]/=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().int().end();

    //div(vec2,vec2)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() { a.0[i]/=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //div(vec3,vec3)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() { a.0[i]/=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //div(vec4,vec4)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() { a.0[i]/=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //div(ivec2,ivec2)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        for i in 0..a.0.len() { a.0[i]/=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //div(ivec3,ivec3)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        for i in 0..a.0.len() { a.0[i]/=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //div(ivec4,ivec4)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        for i in 0..a.0.len() { a.0[i]/=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //eq(vec2,vec2)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::Bool(a.0==b.0))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //eq(vec3,vec3)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::Bool(a.0==b.0))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //eq(vec4,vec4)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::Bool(a.0==b.0))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //eq(ivec2,ivec2)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::Bool(a.0==b.0))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //eq(ivec3,ivec3)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::Bool(a.0==b.0))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //eq(ivec4,ivec4)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::Bool(a.0==b.0))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //dot(vec2,vec2)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::Float((0 .. a.0.len()).fold(0.0,|acc,i|acc+a.0[i]*b.0[i])))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //dot(vec3,vec3)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::Float((0 .. a.0.len()).fold(0.0,|acc,i|acc+a.0[i]*b.0[i])))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //dot(vec4,vec4)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::Float((0 .. a.0.len()).fold(0.0,|acc,i|acc+a.0[i]*b.0[i])))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //dot(ivec2,ivec2)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::Int((0 .. a.0.len()).fold(0,|acc,i|acc+a.0[i]*b.0[i])))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //dot(ivec3,ivec3)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::Int((0 .. a.0.len()).fold(0,|acc,i|acc+a.0[i]*b.0[i])))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //dot(ivec4,ivec4)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::Int((0 .. a.0.len()).fold(0,|acc,i|acc+a.0[i]*b.0[i])))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //cross(vec3,vec3)
    lib_scope.method("cross",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?.0;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?.0;
        Ok(Value::custom_unmanaged(Vec3([a[1]*b[2]-b[1]*a[2], a[2]*b[0]-b[2]*a[0], a[0]*b[1]-b[0]*a[1]])))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //cross(ivec3,ivec3)
    lib_scope.method("cross",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?.0;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?.0;
        Ok(Value::custom_unmanaged(IVec3([a[1]*b[2]-b[1]*a[2], a[2]*b[0]-b[2]*a[0], a[0]*b[1]-b[0]*a[1]])))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //len(vec2)
    lib_scope.method("len",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        Ok(Value::Float(d))
    }).custom_ref::<Vec2>().end();

    //len(vec3)
    lib_scope.method("len",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        Ok(Value::Float(d))
    }).custom_ref::<Vec3>().end();

    //len(vec4)
    lib_scope.method("len",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        Ok(Value::Float(d))
    }).custom_ref::<Vec4>().end();

    //norm(vec2)
    lib_scope.method("norm",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        if d==0.0 {return Err(context.error("len is zero"));}
        Ok(Value::custom_unmanaged(Vec2(a.map(|x|x/d))))
    }).custom_ref::<Vec2>().end();

    //norm(vec3)
    lib_scope.method("norm",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        if d==0.0 {return Err(context.error("len is zero"));}
        Ok(Value::custom_unmanaged(Vec3(a.map(|x|x/d))))
    }).custom_ref::<Vec3>().end();

    //norm(vec4)
    lib_scope.method("norm",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        if d==0.0 {return Err(context.error("len is zero"));}
        Ok(Value::custom_unmanaged(Vec4(a.map(|x|x/d))))
    }).custom_ref::<Vec4>().end();

    //min(vec2,f)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().end();

    //min(vec3,f)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().end();

    //min(vec4,f)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().float().end();

    //min(ivec2,f)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().int().end();

    //min(ivec3,f)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().int().end();

    //min(ivec4,f)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().int().end();

    //min(vec2,vec2)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //min(vec3,vec3)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //min(vec4,vec4)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //min(ivec2,ivec2)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //min(ivec3,ivec3)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //min(ivec4,ivec4)
    lib_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //max(vec2,f)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().end();

    //max(vec3,f)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().end();

    //max(vec4,f)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().float().end();

    //max(ivec2,f)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().int().end();

    //max(ivec3,f)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().int().end();

    //max(ivec4,f)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_int();
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().int().end();

    //max(vec2,vec2)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //max(vec3,vec3)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //max(vec4,vec4)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //max(ivec2,ivec2)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //max(ivec3,ivec3)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //max(ivec4,ivec4)
    lib_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //clamp(vec2,f,f)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();
        if b > c { return Err(context.error("min>max".to_string()));}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b,c);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().float().end();

    //clamp(vec3,f,f)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();
        if b > c { return Err(context.error("min>max".to_string()));}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b,c);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().float().end();

    //clamp(vec4,f,f)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();
        if b > c { return Err(context.error("min>max".to_string()));}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b,c);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().float().float().end();

    //clamp(ivec2,f,f)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_int();
        let c=context.param(2).as_int();
        if b > c { return Err(context.error("min>max".to_string()));}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b,c);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().int().int().end();

    //clamp(ivec3,f,f)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_int();
        let c=context.param(2).as_int();
        if b > c { return Err(context.error("min>max".to_string()));}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b,c);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().int().int().end();

    //clamp(ivec4,f,f)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_int();
        let c=context.param(2).as_int();
        if b > c { return Err(context.error("min>max".to_string()));}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b,c);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().int().int().end();

    //clamp(vec2,vec2,vec2)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_custom().data_clone::<Vec2>()?;
        let c=context.param(2).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() {if b.0[i]>c.0[i] {return Err(context.error("min>max".to_string()));}}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b.0[i],c.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //clamp(vec3,vec3,vec3)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_custom().data_clone::<Vec3>()?;
        let c=context.param(2).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() {if b.0[i]>c.0[i] {return Err(context.error("min>max".to_string()));}}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b.0[i],c.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //clamp(vec4,vec4,vec4)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_custom().data_clone::<Vec4>()?;
        let c=context.param(2).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() {if b.0[i]>c.0[i] {return Err(context.error("min>max".to_string()));}}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b.0[i],c.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //clamp(ivec2,ivec2,ivec2)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_custom().data_clone::<IVec2>()?;
        let c=context.param(2).as_custom().data_clone::<IVec2>()?;
        for i in 0..a.0.len() {if b.0[i]>c.0[i] {return Err(context.error("min>max".to_string()));}}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b.0[i],c.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //clamp(ivec3,ivec3,ivec3)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_custom().data_clone::<IVec3>()?;
        let c=context.param(2).as_custom().data_clone::<IVec3>()?;
        for i in 0..a.0.len() {if b.0[i]>c.0[i] {return Err(context.error("min>max".to_string()));}}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b.0[i],c.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //clamp(ivec4,ivec4,ivec4)
    lib_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_custom().data_clone::<IVec4>()?;
        let c=context.param(2).as_custom().data_clone::<IVec4>()?;
        for i in 0..a.0.len() {if b.0[i]>c.0[i] {return Err(context.error("min>max".to_string()));}}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b.0[i],c.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().custom_ref::<IVec4>().end();
}