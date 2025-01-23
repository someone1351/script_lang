use super::super::super::common::*;

use super::super::data::*;
use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;
use super::utils::*;

fn custom_array_new(mut context:FuncContext) -> Result<Value,MachineError> {
    let data=(0..context.params_num()).map(|i|context.param(i)).collect::<Vec<_>>();
    Ok(Value::custom_managed_mut(Array(data), context.gc_scope()))
}

fn custom_array_len(context:FuncContext) -> Result<Value,MachineError> {
    context.param(0).as_custom().with_data_mut(|x:&mut Array|Ok(Value::int(x.0.len())))
}

fn custom_array_is_empty(context:FuncContext) -> Result<Value,MachineError> {
    context.param(0).as_custom().with_data_ref(|x:&Array|Ok(Value::Bool(x.0.is_empty())))
}

fn custom_array_push(context:FuncContext) -> Result<Value,MachineError> {
    let v=context.param(1).clone();
    
    context.param(0).as_custom().with_data_mut(|x:&mut Array|{
        x.0.push(v.clone());
        Ok(())
    })?;
    
    Ok(v)
}

fn custom_array_pop(context:FuncContext) -> Result<Value,MachineError> {
    context.param(0).as_custom().with_data_mut(|x:&mut Array|{
        Ok(x.0.pop().map(|x|x.clone()).unwrap_or(Value::Nil))
    })    
}

fn custom_array_extend(context:FuncContext) -> Result<Value,MachineError> {
    let from_array=context.param(1).as_custom().with_data_mut(|x:&mut Array|{
        Ok(x.0.clone())
    })?;

    context.param(0).as_custom().with_data_mut(move |x:&mut Array|{
        x.0.extend(from_array);
        Ok(())
    })?;
    
    Ok(Value::Void)
}

fn custom_array_get_field(context:FuncContext) -> Result<Value,MachineError> {
    //0 array, 1 index

    let field=context.param(1).as_int();
    
    context.param(0).as_custom().with_data_mut(|x:&mut Array|{
        Ok(calc_ind(field,x.0.len()).and_then(|i|x.0.get(i)).cloned().unwrap_or(Value::Nil))
    })
}

fn custom_array_set_field(context:FuncContext) -> Result<Value,MachineError> {
    //0 array, 1 index, 2 value
    let ind=context.param(1).as_int();
    let val=context.param(2);

    context.param(0).as_custom().with_data_mut(|x:&mut Array|{
        if x.0.len()==0 {
            return Err(context.error(format!("Array len is 0.")));
        }

        let Some(i)=calc_ind(ind,x.0.len()) else {
            return Err(context.error(format!("Invalid index to array: {:?}",ind)));
        };
        
        *x.0.get_mut(i).unwrap()=val.clone();

        Ok(())
    })?;

    Ok(Value::Void)
}

fn custom_array_to_string(mut context:FuncContext) -> Result<Value,MachineError> {
    let res=context.param(0).as_custom().with_data_mut(|data:&mut Array|{
        Ok(data.0.iter().map(|x|context.value_to_string(x).unwrap_or("_".to_string())).collect::<Vec<_>>().join(","))
    });

    match res {
        Ok(x)=>Ok(Value::string(format!("Array({x})",))),
        Err(MachineError{error_type:MachineErrorType::CustomDataBorrowMutError,..}) => Ok(Value::String(StringT::new("Array(_)"))),
        Err(x)=>Err(x),
    }
}

fn custom_array_clone(mut context:FuncContext) -> Result<Value,MachineError> {    
    let data: Array= context.param(0).as_custom().data_clone()?;
    Ok(Value::custom_managed_mut(data, context.gc_scope()))
}

fn custom_array_clear(context:FuncContext) -> Result<Value,MachineError> {
    let array=context.param(0).as_custom();

    array.with_data_mut(|data:&mut Array|{
        data.0.clear();
        Ok(())
    })?;
    
    Ok(Value::Void)
}

fn custom_array_remove(context:FuncContext) -> Result<Value,MachineError> {
    let array=context.param(0).as_custom();
    let from = context.param(1).as_int();
    let to = context.param(2).get_int();
    

    array.with_data_mut(|data:&mut Array|{
        if let Some(from)=calc_ind(from, data.0.len()) {
            let to = to.and_then(|to|calc_ind(to, data.0.len())).unwrap_or(from+1);
            let to = to.min(data.0.len());

            for _ in from .. to {
                data.0.remove(from);
            }
        }

        Ok(())
    })?;
    
    Ok(Value::Void)
}

fn custom_is_array_true(_:FuncContext) -> Result<Value,MachineError> {
    Ok(Value::Bool(true))
}

fn custom_is_array_false(_:FuncContext) -> Result<Value,MachineError> {
    Ok(Value::Bool(false))
}


// fn custom_array_each(mut context:FuncContext2) -> Result<Value,MachineError> {
//     //0 array, 1 func

//     let array=context.param(0).as_custom();
//     let func = context.param(1);
//     let len = array.with_data_mut(|x:&mut Array|Ok(x.0.len()))?;
    
//     for i in 0 .. len {
//         let val=context.param(0).as_custom().with_data_mut(|x:&mut Array|{
//             Ok(x.0.get(i).unwrap().clone())
//         })?;

//         context.call_value(&func, [val,Value::int(i)])?;
//     }

//     Ok(Value::Void)
// }

// fn custom_array_map(mut context:FuncContext2) -> Result<Value,MachineError> {
//     //0 array, 1 func

//     let array=context.param(0).as_custom();
//     let func = context.param(1);
//     let len = array.with_data_mut(|x:&mut Array|Ok(x.0.len()))?;

//     let mut outputs = Vec::new();
    
//     for i in 0 .. len {
//         let val=context.param(0).as_custom().with_data_mut(|x:&mut Array|{
//             Ok(x.0.get(i).unwrap().clone())
//         })?;

//         let output = context.call_value(&func, [val,Value::int(i)])?;

//         outputs.push(if output.is_undefined()||output.is_void() {Value::Nil}else{output});
//     }

//     // Ok(context.custom_managed_mut(Array(outputs)))
//     Ok(Value::custom_managed_mut(Array(outputs), context.gc_scope()))
// }

// fn custom_array_position(mut context:FuncContext2) -> Result<Value,MachineError> {
//     let array=context.param(0).as_custom();
//     let val = context.param(1);    

//     array.with_data(|data:&mut Array|{
//         data.0.
//         if let Some(from)=calc_ind(from, data.0.len()) {
//             let to = to.and_then(|to|calc_ind(to, data.0.len())).unwrap_or(from+1);
//             let to = to.min(data.0.len());

//             for _ in from .. to {
//                 data.0.remove(from);
//             }
//         }

//         Ok(())
//     })?;
    
//     Ok(Value::Void)
// }

pub fn register<X>(lib_scope : &mut LibScope<X>) {
    lib_scope.method("array",custom_array_new)
        .optional()
        .any()
        .variadic_end();

    lib_scope.method("push",custom_array_push)
        .custom::<Array>()
        .any()
        .end();
    
    lib_scope.method("pop",custom_array_pop)
        .custom::<Array>()
        .end();

    lib_scope.method("extend",custom_array_extend)
        .custom::<Array>()
        .custom::<Array>()
        .end();

    lib_scope.method("len",custom_array_len)
        .custom::<Array>()
        .end();

    lib_scope.method("is_empty",custom_array_is_empty)
        .custom::<Array>()
        .end();
    
    lib_scope.method("get_field",custom_array_get_field)
        .custom::<Array>()
        .int()
        .end();
    
    lib_scope.method("set_field",custom_array_set_field)
        .custom::<Array>()
        .int()
        .any()
        .end();

    lib_scope.method("string",custom_array_to_string)
        .custom::<Array>()
        .end();

    lib_scope.method("clone",custom_array_clone)
        .custom::<Array>()
        .end();

    lib_scope.method("clear",custom_array_clear)
        .custom::<Array>()
        .end();

    lib_scope.method("remove",custom_array_remove)
        .custom::<Array>()
        .int()
        .optional()
        .int()
        .end();

    lib_scope.method("is_array",custom_is_array_true)
        .custom::<Array>()
        .end();
    
    lib_scope.method("is_array",custom_is_array_false)
        .any()
        .end();

    // lib_scope.method("each",custom_array_each)
    //     .custom::<Array>()
    //     .func()
    //     .end();

    // lib_scope.method("map",custom_array_map)
    //     .custom::<Array>()
    //     .func()
    //     .end();

}