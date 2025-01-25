use std::collections::HashSet;

use super::super::super::common::*;

use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;

use super::super::data::*;

#[derive(Clone)]
pub struct Vec2([FloatT;2]);
#[derive(Clone)]
pub struct Vec3([FloatT;3]);
#[derive(Clone)]
pub struct Vec4([FloatT;4]);


#[derive(Clone)]
pub struct IVec2([IntT;2]);
#[derive(Clone)]
pub struct IVec3([IntT;3]);
#[derive(Clone)]
pub struct IVec4([IntT;4]);

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

pub fn register<X>(func_scope : &mut LibScope<X>) {

    //vec2(), vec2(f), vec2(f,f)    
    func_scope.method("vec2",|context|{
        let x=context.param(0).as_float();
        let y=context.get_param(1).map(|q|q.as_float()).unwrap_or(x);
        Ok(Value::custom_unmanaged_mut(Vec2([x,y])))
    })
        .end()
        .float().end()
        .float().float().end();
    
    //vec3(), vec3(f), vec3(f,f,f)
    func_scope.method("vec3",|context|{
        let x=context.param(0).as_float();
        let y=context.get_param(1).map(|q|q.as_float()).unwrap_or(x);
        let z=context.get_param(2).map(|q|q.as_float()).unwrap_or(x);
        Ok(Value::custom_unmanaged_mut(Vec3([x,y,z])))
    })
        .end()
        .float().end()
        .float().float().float().end();

    //vec4(), vec4(f), vec4(f,f,f,f)
    func_scope.method("vec4",|context|{
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
    func_scope.method("vec3",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        let f=context.param(1).as_float();
        Ok(Value::custom_unmanaged_mut(Vec3([v.0[0],v.0[1],f])))
    })
        .custom_ref::<Vec2>().float().end();

    //vec3(f,v2)
    func_scope.method("vec3",|context|{
        let f=context.param(0).as_float();
        let v=context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged_mut(Vec3([f,v.0[0],v.0[1]])))
    })
        .float().custom_ref::<Vec2>().end();

    //vec4(v3,f)
    func_scope.method("vec4",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        let f=context.param(1).as_float();
        Ok(Value::custom_unmanaged_mut(Vec4([v.0[0],v.0[1],v.0[2],f])))
    })
        .custom_ref::<Vec3>().float().end();

    //vec4(v3,f)
    func_scope.method("vec4",|context|{
        let f=context.param(0).as_float();
        let v=context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged_mut(Vec4([f,v.0[0],v.0[1],v.0[2]])))
    })
        .float().custom_ref::<Vec3>().end();

    //vec4(v2,v2)
    func_scope.method("vec4",|context|{
        let v0=context.param(0).as_custom().data_clone::<Vec2>()?;
        let v1=context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged_mut(Vec4([v0.0[0],v0.0[1],v1.0[0],v1.0[1]])))
    })
        .custom_ref::<Vec2>().custom_ref::<Vec2>().end();
    
    //vec4(v2,f,f)
    func_scope.method("vec4",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        let f0=context.param(1).as_float();
        let f1=context.param(2).as_float();
        Ok(Value::custom_unmanaged_mut(Vec4([v.0[0],v.0[1],f0,f1])))
    })
        .custom_ref::<Vec2>().float().float().end();

    //vec4(f,f,v2)
    func_scope.method("vec4",|context|{
        let f0=context.param(0).as_float();
        let f1=context.param(1).as_float();
        let v=context.param(2).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged_mut(Vec4([f0,f1,v.0[0],v.0[1],])))
    })
        .float().float().custom_ref::<Vec2>().end();

    //vec4(f,v2,f)
    func_scope.method("vec4",|context|{
        let f0=context.param(0).as_float();
        let v=context.param(1).as_custom().data_clone::<Vec2>()?;
        let f1=context.param(2).as_float();
        Ok(Value::custom_unmanaged_mut(Vec4([f0,v.0[0],v.0[1],f1])))
    })
        .float().custom_ref::<Vec2>().float().end();
    
    //string(vec2)
    func_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        Ok(Value::string(format!("Vec2({}, {})",v.0[0],v.0[1])))
    }).custom_ref::<Vec2>().end();

    //string(vec3)
    func_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        Ok(Value::string(format!("Vec3({}, {}, {})",v.0[0],v.0[1],v.0[2])))
    }).custom_ref::<Vec3>().end();

    //string(vec4)
    func_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec4>()?;
        Ok(Value::string(format!("Vec4({}, {}, {}, {})",v.0[0],v.0[1],v.0[2],v.0[3])))
    }).custom_ref::<Vec4>().end();

    //get_field(vec2,ind)
    func_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..2 => Value::float(v.0[ind as usize]), _ => Value::Nil,})
    })
        .custom_ref::<Vec2>().int().end();

    //get_field(vec3,ind)
    func_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..3 => Value::float(v.0[ind as usize]), _ => Value::Nil,})
    })
        .custom_ref::<Vec3>().int().end();

    //get_field(vec4,ind)
    func_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec4>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..4 => Value::float(v.0[ind as usize]), _ => Value::Nil,})
    })
        .custom_ref::<Vec4>().int().end();

    //get_field(vec2,str)
    func_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        let fields = context.param(1).as_string();
        get_field_util_float(fields.as_str(), & v.0).map_err(|e|context.error(e))
    })
        .custom_ref::<Vec2>().str().end();

    //get_field(vec3,str)
    func_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        let fields = context.param(1).as_string();
        get_field_util_float(fields.as_str(), & v.0).map_err(|e|context.error(e))
    })
        .custom_ref::<Vec3>().str().end();

    //get_field(vec4,str)
    func_scope.method("get_field",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec4>()?;
        let fields = context.param(1).as_string();
        get_field_util_float(fields.as_str(), & v.0).map_err(|e|context.error(e))
    })
        .custom_ref::<Vec4>().str().end();

    //set_field(vec2,str,f)
    func_scope.method("set_field",|context|{
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_float()];
        
        context.param(0).as_custom().with_data_mut(|data:&mut Vec2|{
            set_field_util(fields.as_str(),&mut data.0,&to).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec2>().str().float().end();

    //set_field(vec3,str,f)
    func_scope.method("set_field",|context|{
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_float()];
        
        context.param(0).as_custom().with_data_mut(|data:&mut Vec3|{
            set_field_util(fields.as_str(),&mut data.0,&to).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec3>().str().float().end();

    //set_field(vec4,str,f)
    func_scope.method("set_field",|context|{
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_float()];
        
        context.param(0).as_custom().with_data_mut(|data:&mut Vec4|{
            set_field_util(fields.as_str(),&mut data.0,&to).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec4>().str().float().end();

    //set_field(vec2,str,vec2)
    func_scope.method("set_field",|context|{
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec2>()?;
        
        context.param(0).as_custom().with_data_mut(|data:&mut Vec2|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec2>().str().float().end();

    //set_field(vec3,str,vec2)
    func_scope.method("set_field",|context|{
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec2>()?;
        
        context.param(0).as_custom().with_data_mut(|data:&mut Vec3|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec3>().str().custom_ref::<Vec2>().end();

    //set_field(vec3,str,vec3)
    func_scope.method("set_field",|context|{
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec3>()?;
        
        context.param(0).as_custom().with_data_mut(|data:&mut Vec3|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec3>().str().custom_ref::<Vec3>().end();

    //set_field(vec4,str,vec2)
    func_scope.method("set_field",|context|{
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec2>()?;
        
        context.param(0).as_custom().with_data_mut(|data:&mut Vec4|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec4>().str().custom_ref::<Vec2>().end();

    //set_field(vec4,str,vec3)
    func_scope.method("set_field",|context|{
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec3>()?;
        
        context.param(0).as_custom().with_data_mut(|data:&mut Vec4|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec4>().str().custom_ref::<Vec3>().end();

    //set_field(vec4,str,vec4)
    func_scope.method("set_field",|context|{
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec4>()?;
        
        context.param(0).as_custom().with_data_mut(|data:&mut Vec4|{
            set_field_util(fields.as_str(),&mut data.0,&to.0).map_err(|e|context.error(e))?;
            Ok(Value::Void)
        })
    })
        .custom_ref::<Vec4>().str().custom_ref::<Vec4>().end();

    //neg(vec2)
    func_scope.method("-",|context|{
        let mut v = context.param(0).as_custom().data_clone::<Vec2>()?;
        for i in 0..v.0.len() { v.0[i]=-v.0[i]; }
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<Vec2>().end();

    //neg(vec3)
    func_scope.method("-",|context|{
        let mut v = context.param(0).as_custom().data_clone::<Vec3>()?;
        for i in 0..v.0.len() { v.0[i]=-v.0[i]; }
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<Vec3>().end();

    //neg(vec4)
    func_scope.method("-",|context|{
        let mut v = context.param(0).as_custom().data_clone::<Vec4>()?;
        for i in 0..v.0.len() { v.0[i]=-v.0[i]; }
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<Vec4>().end();

    //add(vec2,vec2)
    func_scope.method("+",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() { a.0[i]+=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //add(vec3,vec3)
    func_scope.method("+",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() { a.0[i]+=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //add(vec4,vec4)
    func_scope.method("+",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() { a.0[i]+=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //sub(vec2,vec2)
    func_scope.method("-",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() { a.0[i]-=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //sub(vec3,vec3)
    func_scope.method("-",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() { a.0[i]-=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //sub(vec4,vec4)
    func_scope.method("-",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() { a.0[i]-=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //mul(vec2,f)
    func_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float();        
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().end();

    //mul(f,vec2)
    func_scope.method("*",|context|{
        let mut a = context.param(1).as_custom().data_clone::<Vec2>()?;
        let b=context.param(0).as_float();        
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).float().custom_ref::<Vec2>().end();

    //mul(vec3,f)
    func_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float();        
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().end();

    //mul(f,vec3)
    func_scope.method("*",|context|{
        let mut a = context.param(1).as_custom().data_clone::<Vec3>()?;
        let b=context.param(0).as_float();        
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).float().custom_ref::<Vec3>().end();

    //mul(vec4,f)
    func_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float();        
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().float().end();

    //mul(f,vec4)
    func_scope.method("*",|context|{
        let mut a = context.param(1).as_custom().data_clone::<Vec4>()?;
        let b=context.param(0).as_float();        
        for i in 0..a.0.len() { a.0[i]*=b; }
        Ok(Value::custom_unmanaged(a))
    }).float().custom_ref::<Vec4>().end();

    //mul(vec2,vec2)
    func_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() { a.0[i]*=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //mul(vec3,vec3)
    func_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() { a.0[i]*=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //mul(vec4,vec4)
    func_scope.method("*",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() { a.0[i]*=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //div(vec2,f)
    func_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float();        
        for i in 0..a.0.len() { a.0[i]/=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().end();

    //div(vec3,f)
    func_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float();        
        for i in 0..a.0.len() { a.0[i]/=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().end();

    //div(vec4,f)
    func_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float();        
        for i in 0..a.0.len() { a.0[i]/=b; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().float().end();

    //div(vec2,vec2)
    func_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() { a.0[i]/=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //div(vec3,vec3)
    func_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() { a.0[i]/=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //div(vec4,vec4)
    func_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() { a.0[i]/=b.0[i]; }
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //eq(vec2,vec2)
    func_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::Bool(a.0==b.0))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //eq(vec3,vec3)
    func_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::Bool(a.0==b.0))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //eq(vec4,vec4)
    func_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::Bool(a.0==b.0))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //dot(vec2,vec2)
    func_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::Float((0 .. a.0.len()).fold(0.0,|acc,i|acc+a.0[i]*b.0[i])))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //dot(vec3,vec3)
    func_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::Float((0 .. a.0.len()).fold(0.0,|acc,i|acc+a.0[i]*b.0[i])))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //dot(vec4,vec4)
    func_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::Float((0 .. a.0.len()).fold(0.0,|acc,i|acc+a.0[i]*b.0[i])))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //cross(vec3,vec3)
    func_scope.method("cross",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?.0;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?.0;
        Ok(Value::custom_unmanaged(Vec3([a[1]*b[2]-b[1]*a[2], a[2]*b[0]-b[2]*a[0], a[0]*b[1]-b[0]*a[1]])))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();
    
    //len(vec2)
    func_scope.method("len",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        Ok(Value::Float(d))
    }).custom_ref::<Vec2>().end();

    //len(vec3)
    func_scope.method("len",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        Ok(Value::Float(d))
    }).custom_ref::<Vec3>().end();

    //len(vec4)
    func_scope.method("len",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        Ok(Value::Float(d))
    }).custom_ref::<Vec4>().end();

    //norm(vec2)
    func_scope.method("norm",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        if d==0.0 {return Err(context.error("len is zero"));}
        Ok(Value::custom_unmanaged(Vec2(a.map(|x|x/d))))
    }).custom_ref::<Vec2>().end();

    //norm(vec3)
    func_scope.method("norm",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        if d==0.0 {return Err(context.error("len is zero"));}
        Ok(Value::custom_unmanaged(Vec3(a.map(|x|x/d))))
    }).custom_ref::<Vec3>().end();

    //norm(vec4)
    func_scope.method("norm",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?.0;
        let d=(0 .. a.len()).fold(0.0,|acc,i|acc+a[i]*a[i]).sqrt();
        if d==0.0 {return Err(context.error("len is zero"));}
        Ok(Value::custom_unmanaged(Vec4(a.map(|x|x/d))))
    }).custom_ref::<Vec4>().end();

    //min(vec2,f)
    func_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().end();

    //min(vec3,f)
    func_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().end();

    //min(vec4,f)
    func_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().float().end();

    //min(vec2,vec2)
    func_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //min(vec3,vec3)
    func_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();
    
    //min(vec4,vec4)
    func_scope.method("min",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].min(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //max(vec2,f)
    func_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().end();

    //max(vec3,f)
    func_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().end();

    //max(vec4,f)
    func_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float();
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().float().end();

    //max(vec2,vec2)
    func_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //max(vec3,vec3)
    func_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();
    
    //max(vec4,vec4)
    func_scope.method("max",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        for i in 0..a.0.len() {a.0[i]=a.0[i].max(b.0[i]);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //clamp(vec2,f,f)
    func_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();
        if b > c { return Err(context.error("min>max".to_string()));}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b,c);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().float().end();

    //clamp(vec3,f,f)
    func_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();
        if b > c { return Err(context.error("min>max".to_string()));}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b,c);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().float().end();

    //clamp(vec4,f,f)
    func_scope.method("clamp",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();
        if b > c { return Err(context.error("min>max".to_string()));}
        for i in 0..a.0.len() {a.0[i]=a.0[i].clamp(b,c);}
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().float().float().end();
}