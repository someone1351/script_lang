

use std::collections::BTreeMap;

use super::super::super::common::*;

use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;
use super::super::data::*;

// #[derive(Hash,PartialEq, Eq)]
// enum DictKey {
//     String(StringT),
//     Int(IntT),
//     Bool(bool),
//     Nil,
// }

fn custom_dict_new(mut context:FuncContext) -> Result<Value,MachineError> {
    // let dict=context.new_custom_managed(Dict::new(),);
    // let dict_data=dict.data();
    // let mut dict_data=dict_data.borrow_mut::<Dict>()?;
    
    let mut data=BTreeMap::new();
    let mut i=0;

    while i <context.params_num() {
        // let k = context.param_to_string(i)?;
        let k = context.value_to_string(&context.param(i))?;
        let v=context.param(i+1).clone();
        data.insert(k, v);
        i+=2;
    }

    // Ok(Value::Custom(context.new_custom_managed(Dict(data)))) //.clone_to_root()
    
    // Ok(context.custom_managed_mut(Dict(data)))
    Ok(Value::custom_managed_mut(Dict(data), context.gc_scope()))
}

fn custom_dict_insert(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    let dict_data=dict.data();
    let mut dict_data=dict_data.get_mut::<Dict>()?;

    // let key=context.param_to_string(1)?;
    let key=context.value_to_string(&context.param(1))?;
    let val=context.param(2).clone();

    dict_data.0.insert(key, val.clone());

    Ok(val) //returns new val
}

fn custom_dict_remove(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();

    dict.with_data_mut(|data:&mut Dict|{
        // let key=context.param_to_string(1)?;
        let key=context.value_to_string(&context.param(1))?;
        let v=data.0.remove(&key).unwrap_or(Value::Nil);
        Ok(v)
    })
}

fn custom_dict_len(context:FuncContext) -> Result<Value,MachineError> {
    // let dict=context.param(0).as_custom();
    let dict_data=context.param(0).as_custom().data();
    let dict_data=dict_data.get_mut::<Dict>()?;

    Ok(Value::Int(dict_data.0.len() as IntT))
}

fn custom_dict_get_field(mut context:FuncContext) -> Result<Value,MachineError> {
    //0 dict, 1 key

    // let dict=context.param(0).as_custom();
    let dict_data=context.param(0).as_custom().data();
    let dict_data=dict_data.get_mut::<Dict>()?;

    // let key = context.param_to_string(1)?;
    let key=context.value_to_string(&context.param(1))?;
    let val=dict_data.0.get(&key).and_then(|x|Some(x.clone())).unwrap_or(Value::Nil);

    Ok(val)
}

fn custom_dict_set_field(mut context:FuncContext) -> Result<Value,MachineError> {
    //0 dict, 1 key, 2 val

    let dict=context.param(0).as_custom();
    let dict_data=dict.data();
    let mut dict_data=dict_data.get_mut::<Dict>()?;

    // let key = context.param_to_string(1)?;
    let key=context.value_to_string(&context.param(1))?;
    let val=context.param(2).clone();

    // let Some(element)=dict_data.0.get_mut(&key) else {
    //     return Err(context.error(format!("Invalid key to dict: {:?}",context.param(1))));
    // };

    // *element=val.clone();

    // dict_data.0.entry(key).
    // if let Some(element)=dict_data.0.get_mut(&key) {
    //     *element=val.clone();
    // } else {
        dict_data.0.insert(key, val);
    // }
    
    Ok(Value::Void)
}

// fn custom_dict_last(mut context:FuncContext2) -> Result<Value,MachineError> {
//     //0 dict
//     context.param(0).as_custom().with_data(|dict:&mut Dict|{
//         Ok(dict.0.last_key_value()
//             .map(|last|last.1.clone())
//             .unwrap_or(Value::Nil)
//         )
//     })
// }

fn custom_dict_to_string(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    let dict_data=dict.data();
    // let dict_data=dict_data.borrow_mut::<CustomDict>()?;

    let dict_data=match dict_data.get_mut::<Dict>() {
        Ok(x)=>Ok(x),
        Err(x) if x.error_type==MachineErrorType::CustomDataBorrowMutError => {
            return Ok(Value::String(StringT::new("Dict(_)")))
        },
        Err(x)=>Err(x),
    }?;

    //
    let mut element_strings=Vec::new();
    
    for (k,v) in dict_data.0.iter() {
        element_strings.push(format!("{}:{}",k,context.value_to_string(v)?));
    }

    // let element_strings=dict_data.iter()
    //     .map(|(k,v)|{
    //         format!("{}:{}",k,context.value_to_string(v).unwrap_or("_".to_string()))
    //     }) 
    //     .collect::<Vec<_>>();
        
    Ok(Value::string(format!("Dict({})",element_strings.join(","))))
    // Ok(Value::new_string(format!("Dict({:?})",dict_data)))
}

fn custom_dict_clone(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    
    dict.with_data_mut(|data:&mut Dict|{
        // Ok(context.custom_managed_mut(Dict(data.0.clone())))
        Ok(Value::custom_managed_mut(Dict(data.0.clone()), context.gc_scope()))
    })
}

fn custom_dict_clear(context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();

    dict.with_data_mut(|data:&mut Dict|{
        data.0.clear();
        Ok(())
    })?;
    
    Ok(Value::Void)
}

fn custom_dict_extend(context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    let dict2:Dict=context.param(1).as_custom().data_clone()?;

    dict.with_data_mut(|data:&mut Dict|{
        data.0.extend(dict2.0);
        Ok(())
    })?;
    
    Ok(Value::Void)
}

fn custom_dict_keys(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();

    dict.with_data_mut(|data:&mut Dict|{
        let keys=data.0.keys().map(|k|Value::string(k)).collect::<Vec<_>>();
        // Ok(context.custom_managed_mut(Array(keys)))
        Ok(Value::custom_managed_mut(Array(keys), context.gc_scope()))
    })
}

fn custom_dict_pairs(mut context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();

    dict.with_data_mut(|data:&mut Dict|{
        let pairs=data.0.iter().map(|(k,v)|{
            // context.custom_managed_mut(Array(vec![Value::string(k),v.clone()])) 
            Value::custom_managed_mut(Array(vec![Value::string(k),v.clone()]), context.gc_scope())
        }).collect::<Vec<_>>();
        
        // Ok(context.custom_managed_mut(Array(pairs)))
        Ok(Value::custom_managed_mut(Array(pairs), context.gc_scope()))
    })
}

fn custom_dict_contains(context:FuncContext) -> Result<Value,MachineError> {
    let dict=context.param(0).as_custom();
    let key=context.param(1).as_string();

    dict.with_data_mut(|data:&mut Dict|{
        Ok(Value::Bool(data.0.contains_key(&key)))
    })
}

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    lib_scope.method("dict",custom_dict_new)
        .optional()
        .any()
        .variadic_end()
        // .end()
        ;
        
    lib_scope.method("insert", custom_dict_insert)
        .custom::<Dict>()
        .any()
        .any()
        .end();

    lib_scope.method("remove", custom_dict_remove)
        .custom::<Dict>()
        .any()
        .end();

    lib_scope.method("len", custom_dict_len)
        .custom::<Dict>()
        .end();

    lib_scope.method("get_field", custom_dict_get_field)
        .custom::<Dict>()
        .any()
        .end();

    lib_scope.method("set_field", custom_dict_set_field)
        .custom::<Dict>()
        .any()
        .any()
        .end();

    lib_scope.method("string", custom_dict_to_string)
        .custom::<Dict>()
        .end();

    lib_scope.method("clone", custom_dict_clone)
        .custom::<Dict>()
        .end();
    
    // lib_scope.method("last", custom_dict_last)
    //     .custom::<Dict>()
    //     .end();

    lib_scope.method("clear", custom_dict_clear)
        .custom::<Dict>()
        .end();
    
    lib_scope.method("extend", custom_dict_extend)
        .custom::<Dict>()
        .custom::<Dict>()
        .end();

    lib_scope.method("keys", custom_dict_keys)
        .custom::<Dict>()
        .end();

    lib_scope.method("pairs", custom_dict_pairs)
        .custom::<Dict>()
        .end();
    
    lib_scope.method("contains", custom_dict_contains)
        .custom::<Dict>()
        .str()
        .end();
}

