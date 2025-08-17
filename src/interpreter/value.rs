
use std::any::{Any,TypeId};
use std::str::FromStr;

use super::super::common::*;
use super::custom::*;
// use super::error::*;
use super::gc_scope::*;
// use super::FuncContext;

#[derive(Debug,Clone)]
// #[derive(Debug,)]
pub enum Value {
    Undefined,
    Nil,
    Void,
    Bool(bool),
    Float(FloatT),
    Int(IntT),
    String(StringT),
    Custom(Custom),
}
impl Default for Value {
    fn default() -> Self { Value::Nil }
}
// impl std::fmt::Debug for Value {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let x=format!("{:?}",x);
//         let x = x.rfind(":").map(|i|x[i+1..].to_string()).unwrap_or(x.clone());
//         write!(f, "Hi: {}", self.id)
//     }
// }

// impl Clone for Value {
//     fn clone(&self) -> Self {
//         self.clone_as_is()
//     }
// }
impl Value {
    pub fn float<T: TryInto<FloatT>>(x:T) -> Self {
        Self::Float(x.try_into().ok().unwrap_or(0.0))
    }
    pub fn int<T: TryInto<IntT>+Sized>(x:T) -> Self {
        Self::Int(x.try_into().ok().unwrap_or(0))
    }
    pub fn bool<T: TryInto<bool>+Sized>(x:T) -> Self {
        Self::Bool(x.try_into().ok().unwrap_or(false))
    }
    pub fn string<S: Into<String>>(x:S) -> Self {
        Self::String(StringT::new(x.into()))
    }


    pub fn type_string(&self) -> String {
        match self {
            Value::Float(_)=>"float".to_string(),
            Value::Int(_)=>"int".to_string(),
            Value::Bool(_)=>"bool".to_string(),
            Value::String(_)=> "string".to_string(),

            Value::Custom(c)=> c.type_info().short_name().to_string(), //c.short_type_name().to_string(),

            Value::Nil=>"nil".to_string(),
            Value::Void => "void".to_string(),
            Value::Undefined=>"undefined".to_string(),
        }
    }

    pub fn is_bool(&self) -> bool {
        if let Value::Bool(_)=self{true}else{false}
    }

    pub fn is_float(&self) -> bool {
        if let Value::Float(_)=self{true}else{false}
    }

    pub fn is_int(&self) -> bool {
        if let Value::Int(_)=self{true}else{false}
    }

    pub fn is_string(&self) -> bool {
        if let Value::String(_)=self{true}else{false}
    }

    pub fn is_custom_any(&self) -> bool {
        if let Value::Custom(_)=self{true}else{false}
    }

    pub fn is_custom<T:Any>(&self) -> bool {
        if let Value::Custom(x)=self {
            x.type_info().id()==TypeId::of::<T>()
        } else {
            false
        }
    }

    pub fn is_nil(&self) -> bool {
        if let Value::Nil=self{true}else{false}
    }

    pub fn is_void(&self) -> bool {
        if let Value::Void = self {true} else {false}
    }

    pub fn is_undefined(&self) -> bool {
        if let Value::Undefined = self {true} else {false}
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(x)=>Some(*x),
            _=>None
        }
    }
    pub fn get_int(&self) -> Option<IntT> {
        match self {
            Value::Int(x)=>Some(*x),
            _=>None,
        }
    }
    pub fn get_float(&self) -> Option<FloatT> {
        match self {
            Value::Float(x)=>Some(*x),
            _=>None,
        }
    }

    pub fn get_string(&self) -> Option<StringT> {
        match self {
            Value::String(x)=>Some(x.clone()),
            _=>None,
        }
    }

    pub fn get_parse<T:FromStr>(&self) -> Option<T> {
        self.get_string().and_then(|x|T::from_str(x.as_str()).ok())
    }

    pub fn get_custom(&self) -> Option<Custom> {
        match self {
            Value::Custom(c)=>Some(c.clone()),
            _=>None,
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Nil=>false,
            // Value::Float(f) if *f==0.0 =>false,
            // Value::Int(0)=>false,
            Value::Bool(x)=>*x,
            // Value::String(s) if s.is_empty()=>false,
            // Value::Void => false,
            _=>true
        }
    }

    pub fn as_int(&self) -> IntT {
        match self {
            Value::Int(x)=>*x,
            Value::Float(x)=>*x as IntT,
            Value::Bool(true) => 1,
            Value::String(x)=>x.parse::<IntT>().unwrap_or(0),
            _=>0,
        }
    }

    pub fn as_float(&self) -> FloatT {
        match self {
            Value::Int(x)=>*x as FloatT,
            Value::Float(x)=>*x,
            Value::Bool(true) => 1.0,
            Value::String(x)=>x.parse::<FloatT>().unwrap_or(0.0),
            _=>0.0,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::Int(x)=>x.to_string(),
            Value::Float(x)=>x.to_string(),
            Value::Bool(x) => x.to_string(),
            Value::String(x)=>x.to_string(),
            Value::Custom(c)=>
                // format!(
                //     "{}",
                //     //"Custom({})",
                //     c.type_info().short_name(),
                //     // c.type_name(),
                // )
                c.to_string(),
            Value::Nil => "nil".to_string(),
            Value::Undefined => "undefined".to_string(),
            Value::Void => "void".to_string(),
            // _=>String::new(),
        }
    }

    pub fn as_parse<T:FromStr+Default>(&self) -> T {
        self.get_parse::<T>().unwrap_or_default()
    }

    // pub fn as_string(&self,context : &mut FuncContext) -> Result<String,MachineError> {
    //     context.value_to_string(self)
    // }

    pub fn as_custom(&self) -> Custom {
        match self {
            Value::Custom(c)=>c.clone(), //c.clone_root(),
            _=>Custom::new_empty(), //Custom::new_unmanaged((),), //None
        }
    }


    // pub fn custom_managed<T:GcTraversable+Send+Sync>(data : T,is_mut:bool,gc_scope : &mut GcScope) -> Self {
    //     if is_mut {
    //         Self::Custom(Custom::new_managed_mut(data, gc_scope))
    //     } else {
    //         Self::Custom(Custom::new_managed_non_mut(data, gc_scope))
    //     }
    // }

    // pub fn custom_unmanaged<T:Any+Send+Sync>(data : T,is_mut:bool) -> Self {
    //     if is_mut {
    //         Self::Custom(Custom::new_unmanaged_mut(data))
    //     } else {
    //         Self::Custom(Custom::new_unmanaged_non_mut(data))
    //     }
    // }

    //

    pub fn custom_managed_mut<T:GcTraversable+Send>(data : T,gc_scope : &mut GcScope) -> Self {
        Self::Custom(Custom::new_managed_mut(data, gc_scope))
    }

    pub fn custom_managed<T:GcTraversable+Send+Sync>(data : T,gc_scope : &mut GcScope) -> Self {
        Self::Custom(Custom::new_managed(data, gc_scope))
    }

    pub fn custom_unmanaged_mut<T:Any+Send>(data : T) -> Self {
        Self::Custom(Custom::new_unmanaged_mut(data,))
    }

    pub fn custom_unmanaged<T:Any+Send+Sync>(data : T) -> Self {
        Self::Custom(Custom::new_unmanaged(data,))
    }

    pub fn custom_rc_mut<T:Any+Send>(data : T) -> Self {
        Self::Custom(Custom::new_rc_mut(data,))
    }

    pub fn custom_rc<T:Any+Send+Sync>(data : T) -> Self {
        Self::Custom(Custom::new_rc(data,))
    }
    //
    // pub fn custom_callable_managed_mut<T:GcTraversable+Send>(data : T, caller:Caller,gc_scope : &mut GcScope) -> Self {
    //     Self::Custom(Custom::new_managed_mut(data,Some(caller), gc_scope))
    // }

    // pub fn custom_callable_managed<T:GcTraversable+Send+Sync>(data : T, caller:Caller,gc_scope : &mut GcScope) -> Self {
    //     Self::Custom(Custom::new_managed(data,Some(caller), gc_scope))
    // }

    // pub fn custom_callable_unmanaged_mut<T:Any+Send>(data : T, caller:Caller) -> Self {
    //     Self::Custom(Custom::new_unmanaged_mut(data,Some(caller)))
    // }

    // pub fn custom_callable_unmanaged<T:Any+Send+Sync>(data : T, caller:Caller) -> Self {
    //     Self::Custom(Custom::new_unmanaged(data,Some(caller)))
    // }

    // // pub fn custom_callable_managed_mut<T:GcTraversable+Send>(data : T, caller:Option<Caller>,gc_scope : &mut GcScope) -> Self {
    // //     Self::Custom(Custom::new_managed_mut(data,caller, gc_scope))
    // // }

    // // pub fn custom_callable_managed<T:GcTraversable+Send+Sync>(data : T, caller:Option<Caller>,gc_scope : &mut GcScope) -> Self {
    // //     Self::Custom(Custom::new_managed(data,caller, gc_scope))
    // // }

    // // pub fn custom_callable_unmanaged_mut<T:Any+Send>(data : T, caller:Option<Caller>) -> Self {
    // //     Self::Custom(Custom::new_unmanaged_mut(data,caller))
    // // }

    // // pub fn custom_callable_unmanaged<T:Any+Send+Sync>(data : T, caller:Option<Caller>) -> Self {
    // //     Self::Custom(Custom::new_unmanaged(data,caller))
    // // }
    // //

    // // pub fn custom_managed_mut_ext<T:GcTraversableExt+Send>(data : T,gc_scope : &mut GcScope) -> Self {
    // //     Self::Custom(Custom::new_managed_mut_ext(data, gc_scope))
    // // }

    // // pub fn custom_unmanaged_mut_ext<T:Any+ToString+Send>(data : T) -> Self {
    // //     Self::Custom(Custom::new_unmanaged_mut_ext(data))
    // // }

    // // pub fn custom_managed_non_mut_ext<T:GcTraversableExt+Send+Sync>(data : T,gc_scope : &mut GcScope) -> Self {
    // //     Self::Custom(Custom::new_managed_non_mut_ext(data, gc_scope))
    // // }

    // // pub fn custom_unmanaged_non_mut_ext<T:Any+ToString+Send+Sync>(data : T) -> Self {
    // //     Self::Custom(Custom::new_unmanaged_non_mut_ext(data))
    // // }

    pub fn clone_root(&self) -> Self {
        if let Value::Custom(x)=self {
            Value::Custom(x.clone_root())
        } else {
            self.clone()
        }
    }

    pub fn clone_as_is(&self) -> Self {
        if let Value::Custom(x)=self {
            Value::Custom(x.clone_as_is())
        } else {
            self.clone()
        }
    }
    pub fn clone_leaf(&self) -> Self {
        if let Value::Custom(x)=self {
            Value::Custom(x.clone_leaf())
        } else {
            self.clone()
        }
    }
    pub fn gc_index(&self) -> Result<Option<usize>,()> {
        if let Value::Custom(x)=self {
            x.gc_index()
        } else {
            Ok(None)
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int(x)=>x.to_string(),
            Value::Float(x)=>x.to_string(),
            Value::Bool(x) => x.to_string(),
            Value::String(x)=>x.to_string(),
            Value::Custom(c)=>c.type_info().short_name().to_string(),
            Value::Nil => "nil".to_string(),
            Value::Undefined => "undefined".to_string(),
            Value::Void => "void".to_string(),
        }
    }
}