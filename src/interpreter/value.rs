/*
TODO:
* add BasicValue or PrimitiveValue that only contains bool,int,float,string,nil
* store it as an enum in value, remove dups
* can use it as a key for dict
* add non gc ver of array and dict for it
** would want to be able to have unmanaged customs for that as well

TODO
* allow different types of strings, like Custom has different types
** have an enum Str(&'static str), Rc(Arc<String>), RcMut(Arc<Mutex<String>>)


TODO
* should implement math for all int and float types?
** so user can use val.into() for any type, and still have math work
* should use vec<float>, vec<int> ?

TODO
* have float be enum with (f32/f64) and int be enum with (i32,i64,u32,u64)

* have types when used with each other, convert into the other
** eg u32+i32(pos)=>u32
** eg u32+i32(neg)=>u32
** eg u32+u64=>u64, i32+i64=>i64
** eg u32+i64=>i64
** eg i32+u64=>
** eg f32+f64=>f64


** for instr with ints/floats, have them be a special type with (i64/f64)
*** so when used with another type, they are converted into them
*
*/

use std::any::{Any,TypeId};
// use std::cmp::Ordering;

// // use crate::MachineError;

// use crate::interpreter::error::MachineErrorType;
// use crate::MachineError;

use crate::GcTraversable;

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
    Float(FloatVal),
    Int(IntVal),
    // Int(IntValue),
    String(StringT),
    Custom(Custom),
}

impl Into<Value> for StringT {
    fn into(self) -> Value {
        Value::String(self.clone())
    }
}
impl Into<Value> for String {
    fn into(self) -> Value {
        Value::string(self)
    }
}
impl Into<Value> for &str {
    fn into(self) -> Value {
        Value::string(self)
    }
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        Value::bool(self)
    }
}

// impl Into<Value> for f32 {
//     fn into(self) -> Value {
//         Value::float(self)
//     }
// }

// impl Into<Value> for f64 {
//     fn into(self) -> Value {
//         Value::float(self)
//     }
// }

// impl Into<Value> for i32 {
//     fn into(self) -> Value {
//         Value::int(self)
//     }
// }
// impl Into<Value> for i64 {
//     fn into(self) -> Value {
//         Value::int(self)
//     }
// }
// impl Into<Value> for u32 {
//     fn into(self) -> Value {
//         Value::int(self)
//     }
// }
// impl Into<Value> for u64 {
//     fn into(self) -> Value {
//         Value::int(self)
//     }
// }
// impl Into<Value> for usize {
//     fn into(self) -> Value {
//         Value::int(self)
//     }
// }

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Nil,Self::Nil) => true,

            // (Self::Custom(_), Self::Custom(_)) => false,
            // _ => core::mem::discriminant(self) == core::mem::discriminant(other),
            _ => false,
        }
    }
}

// impl std::hash::Hash for Value {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         match self {
//             // Value::Undefined => todo!(),
//             Value::Nil => {
//                 core::mem::discriminant(self).hash(state);
//             }
//             // Value::Void => todo!(),
//             Value::Bool(x) => {
//                 x.hash(state);
//             }
//             // Value::Float(x) => x.hash(state),
//             Value::Int(x) => {
//                 x.hash(state);
//             }
//             Value::String(x) => {
//                 x.hash(state);
//             }
//             // Value::Custom(custom) => todo!(),
//             _ => {

//             }
//         }
//         // core::mem::discriminant(self).hash(state);
//     }
// }

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
    pub fn is_mut(&self) -> bool {
        self.get_custom().map(|c|c.is_mut()).unwrap_or_default()
    }

    pub fn is_managed(&self) -> bool {
        self.get_custom().map(|c|c.is_managed()).unwrap_or_default()
    }

    pub fn is_unmanaged(&self) -> bool {
        self.get_custom().map(|c|c.is_unmanaged()).unwrap_or_default()
    }

    // pub fn try_float<T: TryInto<FloatT>>(x:T) -> Result<Self,MachineError> {
    //     x.try_into().map(|x|Self::Float(x)).map_err(MachineError::method("msg"))
    // }
    // pub fn try_int<T: TryInto<IntT>+Sized>(x:T) -> Self {
    //     Self::Int(x.try_into().ok().unwrap_or(0))
    // }

    // pub fn float<T: TryInto<FloatT>>(x:T) -> Self {
    //     Self::Float(x.try_into().ok().unwrap_or(0.0))
    // }
    // pub fn int<T: TryInto<IntT>+Sized>(x:T) -> Self {
    //     Self::Int(x.try_into().ok().unwrap_or(0))
    // }
    // pub fn bool<T: TryInto<bool>+Sized>(x:T) -> Self {
    //     Self::Bool(x.try_into().ok().unwrap_or(false))
    // }


    pub fn float<T: Into<FloatVal>>(x:T) -> Self {
        x.into().into()
    }
    pub fn int<T: Into<IntVal>+Sized>(x:T) -> Self {
        x.into().into()
    }
    pub fn bool<T: Into<bool>+Sized>(x:T) -> Self {
        Self::Bool(x.into())
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
    pub fn get_int(&self) -> Option<IntVal> {
        match self {
            Value::Int(x)=>Some(*x),
            _=>None,
        }
    }
    pub fn get_float(&self) -> Option<FloatVal> {
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

    // pub fn get_parse<T:FromStr>(&self) -> Option<T> {
    //     self.get_string().and_then(|x|T::from_str(x.as_str()).ok())
    // }

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
            &Value::Bool(x)=>x,
            // Value::String(s) if s.is_empty()=>false,
            // Value::Void => false,
            &Value::Int(x) => !x.is_zero(),
            &Value::Float(x) => !x.is_zero(),
            _=>true
        }
    }

    pub fn as_int(&self) -> IntVal {
        match self.clone() {
            Value::Int(x)=>x,
            Value::Float(x)=>x.into(),
            Value::Bool(true) => 1.into(),
            Value::String(x)=>x.parse::<IntVal>().unwrap_or(0.into()),
            _=>0.into(),
        }
    }

    pub fn as_index(&self,len:usize) -> Option<usize> {
        let Some(len ) = len.try_into().ok() else {return None;};

        let Some(ind):Option<isize>=self.as_int().try_into().ok() else {
            return None;
        };

        if ind >= len || (ind < 0 && ind.abs() > len) {
            None
        } else {
            let ind = if ind<0 {len+ind} else {ind};
            // Some(ind.try_into().unwrap_or_default())
            Some(ind as usize)
        }
    }

    pub fn as_float(&self) -> FloatVal {
        match self.clone() {
            Value::Int(x)=>x.try_into().unwrap_or_default(),
            Value::Float(x)=>x,
            Value::Bool(true) => 1.0.into(),
            Value::String(x)=>x.parse::<FloatVal>().unwrap_or_default(),
            _=>0.0.into(),
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

    // pub fn as_parse<T:FromStr+Default>(&self) -> T {
    //     self.get_parse::<T>().unwrap_or_default()
    // }

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

    // pub fn custom_rc_mut<T:Any+Send>(data : T) -> Self {
    //     Self::Custom(Custom::new_rc_mut(data,))
    // }

    // pub fn custom_rc<T:Any+Send+Sync>(data : T) -> Self {
    //     Self::Custom(Custom::new_rc(data,))
    // }
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


    pub fn to_strong(&self) -> Option<Self> {
        self.get_custom().and_then(|c|c.to_strong()).map(|c|Value::Custom(c))
    }

    pub fn to_weak(&self) -> Option<Self> {
        self.get_custom().and_then(|c|c.to_weak()).map(|c|Value::Custom(c))
    }

    // pub fn unmanaged_copy<T:Clone+Send+Sync+'static>(&self,) -> Result<Option<Value>,super::MachineError> {
    //     if let Some(custom)=self.get_custom() {
    //         Ok(custom.unmanaged_copy::<T>()?.map(|custom|Self::Custom(custom)))
    //     } else {
    //         Ok(None)
    //     }
    // }
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

// impl std::fmt::Display for Value {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.as_string())
//     }
// }





impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Self::Float(v.into())
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::Float(v.into())
    }
}


impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Self::Int(v.into())
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Self::Int(v.into())
    }
}


impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::Int(v.into())
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Int(v.into())
    }
}

impl From<i128> for Value {
    fn from(v: i128) -> Self {
        Self::Int(v.into())
    }
}

impl From<isize> for Value {
    fn from(v: isize) -> Self {
        Self::Int(v.into())
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Self::Int(v.into())
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Self::Int(v.into())
    }
}


impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Self::Int(v.into())
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Self::Int(v.into())
    }
}

impl From<u128> for Value {
    fn from(v: u128) -> Self {
        Self::Int(v.into())
    }
}

impl From<usize> for Value {
    fn from(v: usize) -> Self {
        Self::Int(v.into())
    }
}


impl From<FloatVal> for Value {
    fn from(v: FloatVal) -> Self {
        Self::Float(v)
    }
}
impl From<IntVal> for Value {
    fn from(v: IntVal) -> Self {
        Self::Int(v)
    }
}
