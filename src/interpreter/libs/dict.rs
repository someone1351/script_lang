

use std::collections::BTreeMap;

use super::super::super::common::*;

// use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;
use super::super::data::*;

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    //dict(any? ...)
    lib_scope.method("dict",|mut context|{
        let mut data=BTreeMap::new();
        let mut i=0;
    
        while i <context.params_num() {
            let k = context.value_to_string(&context.param(i))?;
            let v=context.param(i+1).clone();
            data.insert(k, v);
            i+=2;
        }
    
        Ok(Value::custom_managed_mut(Dict(data), context.gc_scope()))
    }).optional().any().variadic_end();
    
    //insert(dict,any,any)
    lib_scope.method("insert", |mut context|{
        let dict=context.param(0).as_custom();
        let key=context.value_to_string(&context.param(1))?;
        let val=context.param(2).clone();
    
        dict.with_data_mut(|data:&mut Dict|{
            data.0.insert(key, val.clone());
            Ok(val)
        })
    }).custom_ref::<Dict>().any().any().end();

    //remove(dict,any)
    lib_scope.method("remove", |mut context|{
        let key=context.value_to_string(&context.param(1))?;
    
        context.param(0).as_custom().with_data_mut(|data:&mut Dict|{
            Ok(data.0.remove(&key).unwrap_or(Value::Nil))
        })
    }).custom_ref::<Dict>().any().end();

    //len(dict)
    lib_scope.method("len", |context|{
        context.param(0).as_custom().with_data_ref(|data:&Dict|Ok(Value::int(data.0.len())))
    }).custom_ref::<Dict>().end();

    //get_field(dict,any)
    lib_scope.method("get_field", |mut context|{
        let dict=context.param(0).as_custom();
        let key=context.value_to_string(&context.param(1))?;
        dict.with_data_ref(|data:&Dict|{Ok(data.0.get(&key).and_then(|x|Some(x.clone())).unwrap_or(Value::Nil))})
    }).custom_ref::<Dict>().any().end();

    //set_field(dict,any,any)
    lib_scope.method("set_field", |mut context|{
        let dict=context.param(0).as_custom();
        let key=context.value_to_string(&context.param(1))?;
        let val=context.param(2).clone();
        
        dict.with_data_mut(|data:&mut Dict|{
            data.0.insert(key, val);
            Ok(Value::Void)
        })
    }).custom_ref::<Dict>().any().any().end();

    //string(dict)
    lib_scope.method("string", |mut context|{
        let dict=context.param(0).as_custom();

        match dict.with_data_ref(|data:&Dict|{
            let mut element_strings=Vec::new();
        
            for (k,v) in data.0.iter() {
                element_strings.push(format!("{}:{}",k,context.value_to_string(v)?));
            }
    
            Ok(Value::string(format!("Dict({})",element_strings.join(","))))
        }) {
            Err(x) if x.error_type==MachineErrorType::CustomDataBorrowMutError => Ok(Value::String(StringT::new("Dict(_)"))),
            x=>x
        }
    }).custom_ref::<Dict>().end();

    //clone(dict)
    lib_scope.method("clone", |mut context|{
        context.param(0).as_custom().with_data_ref(|data:&Dict|Ok(Value::custom_managed_mut(Dict(data.0.clone()), context.gc_scope())))
    }).custom_ref::<Dict>().end();

    lib_scope.method("clear", |context|{
        context.param(0).as_custom().with_data_mut(|data:&mut Dict|{
            data.0.clear();
            Ok(Value::Void)
        })
    }).custom_ref::<Dict>().end();
    
    //extend(dict,dict)
    lib_scope.method("extend", |context|{
        let dict=context.param(0).as_custom();
        let dict2=context.param(1).as_custom();
    
        dict.with_data_mut(|data:&mut Dict|{
            dict2.with_data_ref(|data2:&Dict|{
                data.0.extend(data2.0.iter().map(|(k,v)|(k.clone(),v.clone())));
                Ok(Value::Void)
            })
        })
    })
        .custom_ref::<Dict>()
        .custom_ref::<Dict>()
        .end();

    //keys(dict)
    lib_scope.method("keys", |mut context|{
        let dict=context.param(0).as_custom();

        dict.with_data_ref(|data:&Dict|{
            let keys=data.0.keys().map(|k|Value::string(k)).collect::<Vec<_>>();
            Ok(Value::custom_managed_mut(Array(keys), context.gc_scope()))
        })
    })
        .custom_ref::<Dict>()
        .end();

    //contains(dict,any)
    lib_scope.method("contains", |context|{
        let dict=context.param(0).as_custom();
        let key=context.param(1).as_string();
        dict.with_data_ref(|data:&Dict|Ok(Value::Bool(data.0.contains_key(&key))))
    })
        .custom_ref::<Dict>()
        .str()
        .end();
}

