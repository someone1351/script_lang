use super::super::super::common::*;

use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;

use super::super::data::*;


fn vec_eq<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?.0;
    let b = context.param(1).as_custom().data_clone::<MathVec<N>>()?.0;
    let c = (0..N).map(|i|a[i]==b[i]).fold(false,|acc,x|acc&&x);
    Ok(Value::Bool(c))
}

fn vec_min<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?.0;
    let b = context.param(1).as_custom().data_clone::<MathVec<N>>()?.0;
    Ok(Value::custom_unmanaged_mut(MathVec::<N>::new((0..N).map(|i|a[i].min(b[i])))))
}

fn vec_max<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?.0;
    let b = context.param(1).as_custom().data_clone::<MathVec<N>>()?.0;
    Ok(Value::custom_unmanaged_mut(MathVec::<N>::new((0..N).map(|i|a[i].max(b[i])))))
}

fn vec_clamp_scalar<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?.0;
    let b=context.param(1).as_float();
    let c=context.param(2).as_float();

    if b > c {
        return Err(context.error("min>max".to_string()));
    }

    Ok(Value::custom_unmanaged_mut(MathVec::<N>::new((0..N).map(|i|a[i].clamp(b,c)))))
}
fn vec_clamp<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?.0;
    let b = context.param(1).as_custom().data_clone::<MathVec<N>>()?.0;
    let c = context.param(2).as_custom().data_clone::<MathVec<N>>()?.0;

    for i in 0..N {

        if b[i] > c[i] {
            return Err(context.error("min>max".to_string()));
        }
    }

    Ok(Value::custom_unmanaged_mut(MathVec::<N>::new((0..N).map(|i|a[i].clamp(b[i],c[i])))))
}

fn vec_dot<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?.0;
    let b = context.param(1).as_custom().data_clone::<MathVec<N>>()?.0;
    Ok(Value::Float((0 .. N).fold(0.0,|acc,i|acc+a[i]*b[i])))
}

fn vec_len<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?.0;
    let d=(0 .. N).fold(0.0,|acc,i|acc+a[i]*a[i]);
    let d=d.sqrt();
    Ok(Value::Float(d))
}

fn vec_dist<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?.0;
    let b = context.param(1).as_custom().data_clone::<MathVec<N>>()?.0;

    let d=(0 .. N).fold(0.0,|acc,i|acc+(a[i]-b[i]).powf(2.0));
    let d=d.sqrt();
    Ok(Value::Float(d))
}

fn vec_norm<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?.0;
    let d=(0 .. N).fold(0.0,|acc,i|acc+a[i]*a[i]);
    let d=d.sqrt();
    Ok(Value::custom_unmanaged_mut(MathVec::<N>::new((0 .. N).map(|i|a[i]/d))))
}

fn vec_neg<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    let mut v = MathVec::<N>::default();

    for i in 0 .. N {
        v.0[i]=-a.0[i];
    }

    Ok(Value::custom_unmanaged_mut(v))
}

fn vec_add<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    let b = context.param(1).as_custom().data_clone::<MathVec<N>>()?;
    
    let mut v = MathVec::<N>::default();

    for i in 0 .. N {
        v.0[i]=a.0[i]+b.0[i];
    }

    Ok(Value::custom_unmanaged_mut(v))
}

fn vec_sub<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    let b = context.param(1).as_custom().data_clone::<MathVec<N>>()?;
    
    let mut v = MathVec::<N>::default();

    for i in 0 .. N {
        v.0[i]=a.0[i]+b.0[i];
    }

    Ok(Value::custom_unmanaged_mut(v))
}

fn vec_mul<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    let b = context.param(1).as_custom().data_clone::<MathVec<N>>()?;
    
    let mut v = MathVec::<N>::default();

    for i in 0 .. N {
        v.0[i]=a.0[i]*b.0[i];
    }

    Ok(Value::custom_unmanaged_mut(v))
}

fn vec_div<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    let b = context.param(1).as_custom().data_clone::<MathVec<N>>()?;
    
    let mut v = MathVec::<N>::default();

    for i in 0 .. N {
        v.0[i]=a.0[i]/b.0[i];
    }

    Ok(Value::custom_unmanaged_mut(v))
}

fn vec_add_scalar<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    let b = context.param(1).as_float();
    
    let mut v = MathVec::<N>::default();

    for i in 0 .. N {
        v.0[i]=a.0[i]+b;
    }

    Ok(Value::custom_unmanaged_mut(v))
}

fn vec_sub_scalar<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    let b = context.param(1).as_float();
    
    let mut v = MathVec::<N>::default();

    for i in 0 .. N {
        v.0[i]=a.0[i]-b;
    }

    Ok(Value::custom_unmanaged_mut(v))
}

fn vec_mul_scalar<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    let b = context.param(1).as_float();
    
    let mut v = MathVec::<N>::default();

    for i in 0 .. N {
        v.0[i]=a.0[i]*b;
    }

    Ok(Value::custom_unmanaged_mut(v))
}

fn vec_div_scalar<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let a = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    let b = context.param(1).as_float();
    
    let mut v = MathVec::<N>::default();

    for i in 0 .. N {
        v.0[i]=a.0[i]/b;
    }

    Ok(Value::custom_unmanaged_mut(v))
}

fn vec_field_ind(field:char) -> usize {
    match field {
        'r'|'x'|'0' => 0,
        'g'|'y'|'1' => 1,
        'b'|'z'|'2' => 2,
        'a'|'w'|'3' => 3,
        _ => 4,
    }
}

fn vec_get_field_util<X,const N: usize>(context:&FuncContext<X>,this:&[FloatT],field:&String) -> Result<Value,MachineError> {
    let mut v=MathVec::<N>::default();

    for i in 0 .. N {
        let ind=vec_field_ind(field.chars().nth(i).unwrap());

        if let Some(&x)=this.get(ind) {
            v.0[i] = x as FloatT;
        } else {
            return Err(context.error("Invalid field"));
        }
    }

    return Ok(Value::custom_unmanaged_mut(v));
}

fn vec_get_field<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let this = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    let field = context.param(1).as_string();

    match field.len() {
        1 => {
            let ind=vec_field_ind(field.chars().nth(0).unwrap());
            
            if let Some(&x)=this.0.get(ind) {
                return Ok(Value::Float(x as FloatT))
            } else {
                return Err(context.error("Invalid field"));
            }
        }
        2 => {
            return vec_get_field_util::<X,2>(&context,&this.0,&field);
        }
        3 => {
            return vec_get_field_util::<X,3>(&context,&this.0,&field);
        }
        4 => {
            return vec_get_field_util::<X,4>(&context,&this.0,&field);
        }
        _ => {
            return Err(context.error("Invalid field"));
        }
    }

}


fn vec_set_field<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let this = context.param(0).as_custom().data();
    let mut this = this.get_mut::<MathVec<N>>()?;

    let field = context.param(1).as_string();
    let to = context.param(2);

    //
    //vec4 v; v.xy = vec2(1.0);
    //vec2 v; v.yx = vec2(1.0);
    //vec2 v; v.xx = vec2(1.0);
    
    //v.x=f
    //v.xx=vec2
    //v.xxx=vec3
    //v.xxxx=vec4

    //check field_inds are all < N
    //then check field_len==to_vec.size

    match field.len() {
        1 => if to.is_float() {
            let field_ind = vec_field_ind(field.chars().nth(0).unwrap());
            // this.0[field_ind]=to.as_float();

            let Some(x)=this.0.get_mut(field_ind) else {
                return Err(context.error("Invalid field2"));
            };

            *x=to.as_float();

        }
        2 => {
            let to=to.as_custom().data_clone::<MathVec::<2>>()?;

            for i in 0 .. 2 {
                let field_ind = vec_field_ind(field.chars().nth(i).unwrap());
                // this.0[field_ind]=to.0[i];
                    
                let Some(x)=this.0.get_mut(field_ind) else {
                    return Err(context.error("Invalid field2"));
                };

                *x=to.0[i];
            }
        }
        3 if to.is_custom::<MathVec::<3>>() => {
            let to=to.as_custom().data_clone::<MathVec::<3>>()?;

            for i in 0 .. 3 {
                let field_ind = vec_field_ind(field.chars().nth(i).unwrap());
                // this.0[field_ind]=to.0[i];
                
                let Some(x)=this.0.get_mut(field_ind) else {
                    return Err(context.error("Invalid field2"));
                };

                *x=to.0[i];
            }
        }
        4 if to.is_custom::<MathVec::<4>>() => {
            let to=to.as_custom().data_clone::<MathVec::<4>>()?;

            for i in 0 .. 4 {
                let field_ind = vec_field_ind(field.chars().nth(i).unwrap());
                // this.0[field_ind]=to.0[i];
                
                let Some(x)=this.0.get_mut(field_ind) else {
                    return Err(context.error("Invalid field2"));
                };

                *x=to.0[i];
            }
        }
        _=> {
            return Err(context.error("Invalid field3"));
        }
    }

    Ok(Value::Void)

}

fn vec_to_string<X,const N: usize>(context:FuncContext<X>) -> Result<Value,MachineError> {
    let this = context.param(0).as_custom().data_clone::<MathVec<N>>()?;
    Ok(Value::string(format!("vec{N}({})",this.0.map(|x|format!("{x:?}")).join(","))))
}


pub fn register<X>(func_scope : &mut LibScope<X>) {
    //contructor()
    func_scope.method("vec2",|_|{
        Ok(Value::custom_unmanaged_mut(MathVec::<2>::default()))
    }).end();

    func_scope.method("vec3",|_|{
        Ok(Value::custom_unmanaged_mut(MathVec::<3>::default()))
    }).end();

    func_scope.method("vec4",|_|{
        Ok(Value::custom_unmanaged_mut(MathVec::<4>::default()))
    }).end();

    //constructor(float)
    func_scope.method("vec2",|context|{
        let a=context.param(0).as_float();
        Ok(Value::custom_unmanaged_mut(MathVec::<2>::new([a;2])))
    }).float().end();

    func_scope.method("vec3",|context|{
        let a=context.param(0).as_float();
        Ok(Value::custom_unmanaged_mut(MathVec::<3>::new([a;3])))
    }).float().end();

    func_scope.method("vec4",|context|{
        let a=context.param(0).as_float();
        Ok(Value::custom_unmanaged_mut(MathVec::<4>::new([a;4])))
    }).float().end();

    //constructor(float ...)
    func_scope.method("vec2",|context|{
        Ok(Value::custom_unmanaged_mut(MathVec::<2>::new((0 .. 2).map(|i|context.param(i).as_float()))))
    }).float().float().end();
    
    func_scope.method("vec3",|context|{
        Ok(Value::custom_unmanaged_mut(MathVec::<3>::new((0 .. 3).map(|i|context.param(i).as_float()))))
    }).float().float().float().end();
    
    func_scope.method("vec4",|context|{
        Ok(Value::custom_unmanaged_mut(MathVec::<4>::new((0 .. 4).map(|i|context.param(i).as_float()))))
    }).float().float().float().float().end();

    //vec3(vec2,float)
    func_scope.method("vec3",|context|{
        let a=context.param(0).as_custom().data_clone::<MathVec<2>>()?;
        let b=context.param(1).as_float();
        Ok(Value::custom_unmanaged_mut(MathVec::<3>::new([a.0[0],a.0[1],b])))
    }).custom::<MathVec<2>>().float().end();
    
    //vec3(float,vec2)
    func_scope.method("vec3",|context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_custom().data_clone::<MathVec<2>>()?;
        Ok(Value::custom_unmanaged_mut(MathVec::<3>::new([a,b.0[0],b.0[1]])))
    }).float().custom::<MathVec<2>>().end();

    //vec4(vec2,vec2)
    func_scope.method("vec4",|context|{
        let a=context.param(0).as_custom().data_clone::<MathVec<2>>()?;
        let b=context.param(1).as_custom().data_clone::<MathVec<2>>()?;
        Ok(Value::custom_unmanaged_mut(MathVec::<4>::new([a.0[0],a.0[1],b.0[0],b.0[1]])))
    }).custom::<MathVec<2>>().custom::<MathVec<2>>().end();

    //vec4(vec2,float,float)
    func_scope.method("vec4",|context|{
        let a=context.param(0).as_custom().data_clone::<MathVec<2>>()?;
        let b=context.param(1).as_float();
        let c=context.param(2).as_float();
        Ok(Value::custom_unmanaged_mut(MathVec::<4>::new([a.0[0],a.0[1],b,c])))
    }).custom::<MathVec<2>>().float().float().end();

    //vec4(float,float,vec2)
    func_scope.method("vec4",|context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_float();
        let c=context.param(2).as_custom().data_clone::<MathVec<2>>()?;
        Ok(Value::custom_unmanaged_mut(MathVec::<4>::new([a,b,c.0[0],c.0[1]])))
    }).float().float().custom::<MathVec<2>>().end();

    //vec4(float,vec2,float)
    func_scope.method("vec4",|context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_custom().data_clone::<MathVec<2>>()?;
        let c=context.param(1).as_float();
        Ok(Value::custom_unmanaged_mut(MathVec::<4>::new([a,b.0[0],b.0[1],c])))
    }).float().custom::<MathVec<2>>().float().end();

    //vec4(vec3,float)
    func_scope.method("vec4",|context|{
        let a=context.param(0).as_custom().data_clone::<MathVec<3>>()?;
        let b=context.param(1).as_float();
        Ok(Value::custom_unmanaged_mut(MathVec::<4>::new([a.0[0],a.0[1],a.0[2],b])))
    }).custom::<MathVec<3>>().float().end();

    //vec4(float,vec3)
    func_scope.method("vec4",|context|{
        let a=context.param(0).as_float();
        let b=context.param(1).as_custom().data_clone::<MathVec<3>>()?;
        Ok(Value::custom_unmanaged_mut(MathVec::<4>::new([a,b.0[0],b.0[1],b.0[2]])))
    }).float().custom::<MathVec<3>>().end();

    //string
    func_scope.method("string",vec_to_string::<X,2>).custom::<MathVec<2>>().end();
    func_scope.method("string",vec_to_string::<X,3>).custom::<MathVec<3>>().end();
    func_scope.method("string",vec_to_string::<X,4>).custom::<MathVec<4>>().end();
    
    //get_field
    func_scope.method("get_field",vec_get_field::<X,2>)
        .custom::<MathVec<2>>()
        .str().or_int()
        .end();

    func_scope.method("get_field",vec_get_field::<X,3>)
        .custom::<MathVec<3>>()
        .str().or_int()
        .end();

    func_scope.method("get_field",vec_get_field::<X,4>)
        .custom::<MathVec<4>>()
        .str().or_int()
        .end();

    //set_field
    func_scope.method("set_field",vec_set_field::<X,2>)
        .custom::<MathVec<2>>()
        .str().or_int()
        .float().or_custom::<MathVec<2>>().or_custom::<MathVec<3>>().or_custom::<MathVec<4>>()
        .end();

    func_scope.method("set_field",vec_set_field::<X,3>)
        .custom::<MathVec<3>>()
        .str().or_int()
        .float().or_custom::<MathVec<2>>().or_custom::<MathVec<3>>().or_custom::<MathVec<4>>()
        .end();

    func_scope.method("set_field",vec_set_field::<X,4>)
        .custom::<MathVec<4>>()
        .str().or_int()
        .float().or_custom::<MathVec<2>>().or_custom::<MathVec<3>>().or_custom::<MathVec<4>>()
        .end();

    //dot
    func_scope.method("dot",vec_dot::<X,2>)
        .custom::<MathVec<2>>()
        .custom::<MathVec<2>>()
        .end();

    func_scope.method("dot",vec_dot::<X,3>)
        .custom::<MathVec<3>>()
        .custom::<MathVec<3>>()
        .end();

    func_scope.method("dot",vec_dot::<X,4>)
        .custom::<MathVec<4>>()
        .custom::<MathVec<4>>()
        .end();

    //cross
    func_scope.method("cross",|context|{
        let a = context.param(0).as_custom().data_clone::<MathVec<3>>()?.0;
        let b = context.param(1).as_custom().data_clone::<MathVec<3>>()?.0;
        Ok(Value::custom_unmanaged_mut(MathVec::<3>::new([a[1]*b[2]-b[1]*a[2], a[2]*b[0]-b[2]*a[0], a[0]*b[1]-b[0]*a[1]])))
    })
        .custom::<MathVec<3>>()
        .custom::<MathVec<3>>()
        .end();
    
    //len
    func_scope.method("len",vec_len::<X,2>)
        .custom::<MathVec<2>>()
        .end();

    func_scope.method("len",vec_len::<X,3>)
        .custom::<MathVec<3>>()
        .end();

    func_scope.method("len",vec_len::<X,4>)
        .custom::<MathVec<4>>()
        .end();

    //dist
    func_scope.method("dist",vec_dist::<X,2>)
        .custom::<MathVec<2>>()
        .custom::<MathVec<2>>()
        .end();

    func_scope.method("dist",vec_dist::<X,3>)
        .custom::<MathVec<3>>()
        .custom::<MathVec<3>>()
        .end();
    
    func_scope.method("dist",vec_dist::<X,4>)
        .custom::<MathVec<4>>()
        .custom::<MathVec<4>>()
        .end();

    //norm
    func_scope.method("norm",vec_norm::<X,2>)
        .custom::<MathVec<2>>()
        .end();
    
    func_scope.method("norm",vec_norm::<X,3>)
        .custom::<MathVec<3>>()
        .end();
    
    func_scope.method("norm",vec_norm::<X,4>)
        .custom::<MathVec<4>>()
        .end();

    //eq
    func_scope.method("=",vec_eq::<X,2>)
        .custom::<MathVec<2>>()
        .custom::<MathVec<2>>()
        .end();

    func_scope.method("=",vec_eq::<X,3>)
        .custom::<MathVec<3>>()
        .custom::<MathVec<3>>()
        .end();

    func_scope.method("=",vec_eq::<X,4>)
        .custom::<MathVec<4>>()
        .custom::<MathVec<4>>()
        .end();

    //min    
    func_scope.method("min",vec_min::<X,2>)
        .custom::<MathVec<2>>()
        .custom::<MathVec<2>>()
        .end();

    func_scope.method("min",vec_min::<X,3>)
        .custom::<MathVec<3>>()
        .custom::<MathVec<3>>()
        .end();

    func_scope.method("min",vec_min::<X,4>)
        .custom::<MathVec<4>>()
        .custom::<MathVec<4>>()
        .end();


    //max    
    func_scope.method("max",vec_max::<X,2>)
        .custom::<MathVec<2>>()
        .custom::<MathVec<2>>()
        .end();

    func_scope.method("max",vec_max::<X,3>)
        .custom::<MathVec<3>>()
        .custom::<MathVec<3>>()
        .end();

    func_scope.method("max",vec_max::<X,4>)
        .custom::<MathVec<4>>()
        .custom::<MathVec<4>>()
        .end();

    //clamp
    func_scope.method("clamp",vec_clamp_scalar::<X,2>)
        .custom::<MathVec<2>>()
        .float()
        .float()
        .end();

    func_scope.method("clamp",vec_clamp_scalar::<X,3>)
        .custom::<MathVec<3>>()
        .float()
        .float()
        .end();

    func_scope.method("clamp",vec_clamp_scalar::<X,4>)
        .custom::<MathVec<4>>()
        .float()
        .float()
        .end();

    func_scope.method("clamp",vec_clamp::<X,2>)
        .custom::<MathVec<2>>()
        .custom::<MathVec<2>>()
        .end();

    func_scope.method("clamp",vec_clamp::<X,3>)
        .custom::<MathVec<3>>()
        .custom::<MathVec<3>>()
        .end();

    func_scope.method("clamp",vec_clamp::<X,4>)
        .custom::<MathVec<4>>()
        .custom::<MathVec<4>>()
        .end();

    //neg
    func_scope.method("-",vec_neg::<X,2>)
        .custom::<MathVec<2>>()
        .end();

    func_scope.method("-",vec_neg::<X,3>)
        .custom::<MathVec<3>>()
        .end();

    func_scope.method("-",vec_neg::<X,4>)
        .custom::<MathVec<4>>()
        .end();

    //
    func_scope.method("+",vec_add::<X,2>).custom::<MathVec<2>>().custom::<MathVec<2>>().end();
    func_scope.method("+",vec_add::<X,3>).custom::<MathVec<3>>().custom::<MathVec<3>>().end();
    func_scope.method("+",vec_add::<X,4>).custom::<MathVec<4>>().custom::<MathVec<4>>().end();
    
    func_scope.method("-",vec_sub::<X,2>).custom::<MathVec<2>>().custom::<MathVec<2>>().end();
    func_scope.method("-",vec_sub::<X,3>).custom::<MathVec<3>>().custom::<MathVec<3>>().end();
    func_scope.method("-",vec_sub::<X,4>).custom::<MathVec<4>>().custom::<MathVec<4>>().end();
        
    func_scope.method("*",vec_mul::<X,2>).custom::<MathVec<2>>().custom::<MathVec<2>>().end();
    func_scope.method("*",vec_mul::<X,3>).custom::<MathVec<3>>().custom::<MathVec<3>>().end();
    func_scope.method("*",vec_mul::<X,4>).custom::<MathVec<4>>().custom::<MathVec<4>>().end();
    
    func_scope.method("/",vec_div::<X,2>).custom::<MathVec<2>>().custom::<MathVec<2>>().end();
    func_scope.method("/",vec_div::<X,3>).custom::<MathVec<3>>().custom::<MathVec<3>>().end();
    func_scope.method("/",vec_div::<X,4>).custom::<MathVec<4>>().custom::<MathVec<4>>().end();

    //
    func_scope.method("+",vec_add_scalar::<X,2>).custom::<MathVec<2>>().float().end();
    func_scope.method("+",vec_add_scalar::<X,3>).custom::<MathVec<3>>().float().end();
    func_scope.method("+",vec_add_scalar::<X,4>).custom::<MathVec<4>>().float().end();

    func_scope.method("-",vec_sub_scalar::<X,2>).custom::<MathVec<2>>().float().end();
    func_scope.method("-",vec_sub_scalar::<X,3>).custom::<MathVec<3>>().float().end();
    func_scope.method("-",vec_sub_scalar::<X,4>).custom::<MathVec<4>>().float().end();

    func_scope.method("*",vec_mul_scalar::<X,2>).custom::<MathVec<2>>().float().end();
    func_scope.method("*",vec_mul_scalar::<X,3>).custom::<MathVec<3>>().float().end();
    func_scope.method("*",vec_mul_scalar::<X,4>).custom::<MathVec<4>>().float().end();
    
    func_scope.method("/",vec_div_scalar::<X,2>).custom::<MathVec<2>>().float().end();
    func_scope.method("/",vec_div_scalar::<X,3>).custom::<MathVec<3>>().float().end();
    func_scope.method("/",vec_div_scalar::<X,4>).custom::<MathVec<4>>().float().end();

    //

}
