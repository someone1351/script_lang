

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::Hash;

// use crate::Custom;
// use crate::GcTraversable;
pub use crate::custom_data::*;

use super::super::super::common::*;

// use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;
// use super::super::data::*;
// use super::super::gc_scope::*;
// use super::array::Array;

#[derive(Debug,Clone,PartialEq, Eq,Hash,PartialOrd, Ord)]
// #[derive(Debug,)]
pub enum ValueKey {
    // Undefined,
    Nil,
    // Void,
    Bool(bool),
    // Float(FloatT),
    Int(IntVal),
    String(StringT),
    // Custom(Custom),
}

impl ValueKey {
    pub fn new(v:&Value) -> Option<Self> {
        match v {
            // Value::Undefined => todo!(),
            Value::Nil => Some(ValueKey::Nil),
            // Value::Void => todo!(),
            Value::Bool(x) => Some(ValueKey::Bool(*x)),
            // Value::Float(_) => todo!(),
            Value::Int(x) => Some(ValueKey::Int(*x)),
            Value::String(x) => Some(ValueKey::String(x.clone())),
            // Value::Custom(custom) => todo!(),
            _ => None,
        }
    }
    pub fn to_value(&self) -> Value {
        match self.clone() {
            ValueKey::Nil => Value::Nil,
            ValueKey::Bool(x) => Value::Bool(x),
            ValueKey::Int(x) => Value::Int(x),
            ValueKey::String(x) => Value::String(x),
        }
    }
}
#[derive(Clone,Default)]
pub struct Dict(pub BTreeMap<ValueKey,Value>);

impl GcTraversable for Dict {
    fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=Custom>+'a> {
        Box::new(self.0.values().filter_map(|v|v.get_custom()))
    }
}


impl Into<Dict> for Vec<(&str,Value)> {
    fn into(self) -> Dict {
        Dict(self.iter().map(|(k,v)|(ValueKey::String((*k).into()),v.clone())).collect())
    }
}
impl Into<Dict> for Vec<(String,Value)> {
    fn into(self) -> Dict {
        Dict(self.iter().map(|(k,v)|(ValueKey::String(k.clone().into()),v.clone())).collect())
    }
}

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    //dict(any? ...)
    lib_scope.method("dict",|mut context|{
        let mut data=BTreeMap::new();
        let mut i=0;

        while i <context.params_num() {
            // let k = context.value_to_string(&context.param(i))?;
            let k=context.param(i);
            let Some(k) = ValueKey::new(&k) else { return Err(MachineError::method(format!("invalid key '{}' at {i}",k.type_string()))); };
            let v=context.param(i+1).clone();
            data.insert(k, v);
            i+=2;
        }

        Ok(Value::custom_managed_mut(Dict(data), context.gc_scope()))
    }).optional().any().variadic_end();

    //insert(dict,any,any)
    lib_scope.method("insert", | context|{
        let dict=context.param(0).as_custom();
        // let key=context.value_to_string(&context.param(1))?;
        let key=context.param(1);
        let val=context.param(2).clone();

        // let Some(kk) = ValueKey::new(&k) else { return Err(MachineError::method(format!("invalid key '{}'",k.type_string()))); };
        let key=ValueKey::new(&key).unwrap();
        dict.with_data_mut_ext(|data:&mut Dict|{
            data.0.insert(key, val.clone());
            Ok(val)
        })
    }).custom_mut_ref::<Dict>()
        .int().or_bool().or_nil().or_str()
        .any().end();

    //remove(dict,any)
    lib_scope.method("remove", | context|{
        // let key=context.value_to_string(&context.param(1))?;
        let key=context.param(1);
        let key=ValueKey::new(&key).unwrap();

        context.param(0).as_custom().with_data_mut_ext(|data:&mut Dict|{
            Ok(data.0.remove(&key).unwrap_or(Value::Nil))
        })
    }).custom_mut_ref::<Dict>()
        .int().or_bool().or_nil().or_str()
        .end();

    //len(dict)
    lib_scope.method("len", |context|{
        context.param(0).as_custom().with_data_ref_ext(|data:&Dict|Ok(Value::int(data.0.len())))
    }).custom_ref::<Dict>().end();

    //get_field(dict,any)
    lib_scope.field( | context|{
        let dict=context.param(0).as_custom();
        // let key=context.value_to_string(&context.param(1))?;
        let key=context.param(1);
        let key=ValueKey::new(&key).unwrap();
        dict.with_data_ref_ext(|data:&Dict|{Ok(data.0.get(&key).and_then(|x|Some(x.clone())).unwrap_or(Value::Nil))})
    }).custom_ref::<Dict>()
        .int().or_bool().or_nil().or_str()
        .end();

    //set_field(dict,any,any)
    lib_scope.field( | context|{
        let dict=context.param(0);
        // let key=context.value_to_string(&context.param(1))?;

        let key=context.param(1);
        let key=ValueKey::new(&key).unwrap();
        let val=context.param(2).clone();

        dict.as_custom().with_data_mut_ext(|data:&mut Dict|{
            data.0.insert(key, val);
            Ok(Value::Void)
        })
    }).custom_mut_ref::<Dict>()
        .int().or_bool().or_nil().or_str()
        .any().end();

    //string(dict)
    lib_scope.method("string", |mut context|{
        let dict=context.param(0).as_custom();

        // match
        dict.with_data_ref_ext(|data:&Dict|{
            let mut element_strings=Vec::new();

            for (k,v) in data.0.iter() {
                let y = if v.is_custom_any() {
                    context.try_call_method("type", [v.clone()])?.map(|x|x.as_string()).unwrap_or(v.as_string())
                } else {
                    v.as_string()
                };
                // let y=context.try_call_method("type", [v.clone()])?.map(|x|"bbb".to_string()).unwrap_or("aaa".to_string());
                // let y=v.as_string();
                element_strings.push(format!("{}:{}",k.to_value().as_string(),y)); //context.value_to_string(v)?
            } //v.as_string()

            Ok(Value::string(format!("Dict({})",element_strings.join(","))))
        })
        //  {
        //     Err(x) if x.error_type==MachineErrorType::CustomDataBorrowMutError => Ok(Value::String(StringT::new("Dict(_)"))),
        //     x=>x
        // }
    }).custom_ref::<Dict>().end();

    //clone(dict)
    lib_scope.method("clone", |mut context|{
        // context.param(0).as_custom().with_data_ref(|data:&Dict|Ok(Value::custom_managed_mut(Dict(data.0.clone()), context.gc_scope())))

        let param=context.param(0);
        let data: Dict= param.as_custom().data_clone()?;

        if param.is_mut() {
            Ok(Value::custom_managed_mut(data, context.gc_scope()))
        } else {
            Ok(Value::custom_managed(data, context.gc_scope()))
        }
    }).custom_ref::<Dict>().end();

    lib_scope.method("clear", |context|{
        context.param(0).as_custom().with_data_mut_ext(|data:&mut Dict|{
            data.0.clear();
            Ok(Value::Void)
        })
    }).custom_ref::<Dict>().end();

    //extend(dict,dict)
    lib_scope.method("extend", |context|{
        let dict=context.param(0).as_custom();
        // let other=context.param(1).as_custom().with_data_ref_ext(|data2:&Dict|{ data2.clone() })?;
        let other:Dict=context.param(1).as_custom().data_clone()?;

        dict.with_data_mut(|data:&mut Dict|{
            data.0.extend(other.0.iter().map(|(k,v)|(k.clone(),v.clone())));
        })?;

        Ok(Value::Void)
    })
        .custom_mut_ref::<Dict>()
        .custom_ref::<Dict>()
        .end();

    //keys(dict)
    lib_scope.method("keys", |mut context|{
        let dict=context.param(0).as_custom();

        dict.with_data_ref_ext(|data:&Dict|{
            let keys=data.0.keys().map(|k|k.to_value()).collect::<Vec<_>>();
            Ok(Value::custom_managed_mut(keys, context.gc_scope()))
        })
    })
        .custom_ref::<Dict>()
        .end();

    //contains(dict,any)
    lib_scope.method("contains", |context|{
        let dict=context.param(0).as_custom();
        // let key=context.param(1).as_string();

        let key=context.param(1);
        let key=ValueKey::new(&key).unwrap();
        dict.with_data_ref_ext(|data:&Dict|Ok(Value::Bool(data.0.contains_key(&key))))
    })
        .custom_ref::<Dict>()
        .int().or_bool().or_nil().or_str()
        .end();

    //
    // //get_field(dict,any)
    // lib_scope.field( | context|{
    //     let dict=context.param(0).as_custom();
    //     let key=context.param(1).as_string();
    //     dict.with_data_ref_ext(|data:&HashMap<&str,Value>|{Ok(data.get(&key.as_str()).and_then(|x|Some(x.clone())).unwrap_or(Value::Nil))})
    // }).custom_ref::<HashMap<&str,Value>>().str().end();

    // //get_field(dict,any)
    // lib_scope.field( | context|{
    //     let dict=context.param(0).as_custom();
    //     let key=context.param(1).as_string();
    //     dict.with_data_ref_ext(|data:&HashMap<String,Value>|{Ok(data.get(&key).and_then(|x|Some(x.clone())).unwrap_or(Value::Nil))})
    // }).custom_ref::<HashMap<String,Value>>().str().end();

    //get_field(dict,any)
    lib_scope.field( | context|{
        let dict=context.param(0).as_custom();
        let key=context.param(1).as_string();
        dict.with_data_ref_ext(|data:&HashMap<StringT,Value>|{Ok(data.get(&key).and_then(|x|Some(x.clone())).unwrap_or(Value::Nil))})
    }).custom_ref::<HashMap<StringT,Value>>().str().end();

}


