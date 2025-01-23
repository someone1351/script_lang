

use std::collections::BTreeMap;

use super::super::super::common::*;

use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;
use super::super::data::*;

fn dict_new(mut context:FuncContext) -> Result<Value,MachineError> {
    let mut data=BTreeMap::new();
    let mut i=0;

    while i <context.params_num() {
        let k = context.value_to_string(&context.param(i))?;
        let v=context.param(i+1).clone();
        data.insert(k, v);
        i+=2;
    }

    Ok(Value::custom_managed_mut(Dict(data), context.gc_scope()))
}

fn dict_insert(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    let key=context.value_to_string(&context.param(1))?;
    let val=context.param(2).clone();

    dict.with_data_mut(|data:&mut Dict|{
        data.0.insert(key, val.clone());
        Ok(())
    })?;

    Ok(val)
}

fn dict_remove(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    let key=context.value_to_string(&context.param(1))?;

    dict.with_data_mut(|data:&mut Dict|{
        Ok(data.0.remove(&key).unwrap_or(Value::Nil))
    })
}

fn dict_len(context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();    
    dict.with_data_mut(|data:&mut Dict|Ok(Value::int(data.0.len())))
}

fn dict_get_field(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    let key=context.value_to_string(&context.param(1))?;

    dict.with_data_ref(|data:&Dict|{
        Ok(data.0.get(&key).and_then(|x|Some(x.clone())).unwrap_or(Value::Nil))
    })
}

fn dict_set_field(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    let key=context.value_to_string(&context.param(1))?;
    let val=context.param(2).clone();
    
    dict.with_data_mut(|data:&mut Dict|{
        data.0.insert(key, val);
        Ok(Value::Void)
    })
}

fn dict_to_string(mut context:FuncContext) -> Result<Value,MachineError> {
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
}

fn dict_clone(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    dict.with_data_ref(|data:&Dict|Ok(Value::custom_managed_mut(Dict(data.0.clone()), context.gc_scope())))
}

fn dict_clear(context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();

    dict.with_data_mut(|data:&mut Dict|{
        data.0.clear();
        Ok(Value::Void)
    })
}

fn dict_extend(context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    let dict2=context.param(1).as_custom();

    dict.with_data_mut(|data:&mut Dict|{
        dict2.with_data_ref(|data2:&Dict|{
            data.0.extend(data2.0.iter().map(|(k,v)|(k.clone(),v.clone())));
            Ok(Value::Void)
        })
    })
}

fn dict_keys(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();

    dict.with_data_ref(|data:&Dict|{
        let keys=data.0.keys().map(|k|Value::string(k)).collect::<Vec<_>>();
        Ok(Value::custom_managed_mut(Array(keys), context.gc_scope()))
    })
}

fn dict_contains(context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    let key=context.param(1).as_string();
    dict.with_data_ref(|data:&Dict|Ok(Value::Bool(data.0.contains_key(&key))))
}

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    lib_scope.method("dict",dict_new)
        .optional()
        .any()
        .variadic_end();
        
    lib_scope.method("insert", dict_insert)
        .custom::<Dict>()
        .any()
        .any()
        .end();

    lib_scope.method("remove", dict_remove)
        .custom::<Dict>()
        .any()
        .end();

    lib_scope.method("len", dict_len)
        .custom::<Dict>()
        .end();

    lib_scope.method("get_field", dict_get_field)
        .custom::<Dict>()
        .any()
        .end();

    lib_scope.method("set_field", dict_set_field)
        .custom::<Dict>()
        .any()
        .any()
        .end();

    lib_scope.method("string", dict_to_string)
        .custom::<Dict>()
        .end();

    lib_scope.method("clone", dict_clone)
        .custom::<Dict>()
        .end();

    lib_scope.method("clear", dict_clear)
        .custom::<Dict>()
        .end();
    
    lib_scope.method("extend", dict_extend)
        .custom::<Dict>()
        .custom::<Dict>()
        .end();

    lib_scope.method("keys", dict_keys)
        .custom::<Dict>()
        .end();

    lib_scope.method("contains", dict_contains)
        .custom::<Dict>()
        .str()
        .end();
}

