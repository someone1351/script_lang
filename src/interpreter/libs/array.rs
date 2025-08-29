use super::super::super::common::*;

// use super::super::data::*;
// use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;
// use super::utils::*;
use super::super::gc_scope::*;

#[derive(Clone)]
pub struct Array(pub Vec<Value>);

impl GcTraversable for Array {
    fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Value>+'a> {
        Box::new(self.0.iter())
    }
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
// fn custom_array_remove(context:FuncContext) -> Result<Value,MachineError> {
//     let array=context.param(0).as_custom();
//     let from = context.param(1).as_int();
//     let to = context.param(2).get_int();


//     array.with_data_mut(|data:&mut Array|{
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
    //array(?any ...)
    lib_scope.method("array",|mut context|{
        let data=(0..context.params_num()).map(|i|context.param(i)).collect::<Vec<_>>();
        Ok(Value::custom_managed_mut(Array(data), context.gc_scope()))
    }).optional().any().variadic_end();

    //push(array,any)
    lib_scope.method("push",|context|{
        let v=context.param(1).clone();

        context.param(0).as_custom().with_data_mut(|x:&mut Array|{
            x.0.push(v.clone());
            Ok(v)
        })
    }).custom_ref::<Array>().any().end();

    //pop(array)
    lib_scope.method("pop",|context|{
        context.param(0).as_custom().with_data_mut(|x:&mut Array|{
            Ok(x.0.pop().map(|x|x.clone()).unwrap_or(Value::Nil))
        })
    }).custom_ref::<Array>().end();

    //extend(array,array)
    lib_scope.method("extend",|context|{
        let other=context.param(1).as_custom().with_data_ref(|y:&Array|{ Ok(y.clone()) })?;
        context.param(0).as_custom().with_data_mut(move |x:&mut Array|{
            x.0.extend(other.0.iter().map(|z|z.clone()));
            Ok(Value::Void)
        })
    }).custom_ref::<Array>().custom_ref::<Array>().end();

    //len(array)
    lib_scope.method("len",|context|{
        context.param(0).as_custom().with_data_ref(|x:&Array|Ok(Value::int(x.0.len())))
    }).custom_ref::<Array>().end();

    //is_empty(array)
    lib_scope.method("is_empty",|context|{
        context.param(0).as_custom().with_data_ref(|x:&Array|Ok(Value::Bool(x.0.is_empty())))
    }).custom_ref::<Array>().end();

    //get_field(array,int)
    lib_scope.field(false,|context|{

        context.param(0).as_custom().with_data_ref(|x:&Array|{
            let index=context.param(1).as_index(x.0.len());
            let v=index.map(|i|x.0.get(i).unwrap()).cloned().unwrap_or(Value::Nil);
            Ok(v)

            // Ok(calc_ind(context.param(1).as_int(),x.0.len()).and_then(|i|x.0.get(i)).cloned().unwrap_or(Value::Nil))
        })
    }).custom_ref::<Array>().int().end();

    //set_field(array,int,any)
    lib_scope.field(false,|context|{
        let ind=context.param(1).as_int();
        let val=context.param(2);
        let array=context.param(0);

        array.as_custom().with_data_mut(|x:&mut Array|{
            if x.0.len()==0 {
                return Err(context.error(format!("Array len is 0.")));
            }


            let Some(i)=
                context.param(1).as_index(x.0.len())
                // calc_ind(ind,x.0.len())
            else {
                return Err(context.error(format!("Invalid index to array: {:?}",ind)));
            };

            *x.0.get_mut(i).unwrap()=val.clone();

            Ok(Value::Void)
        })
    }).custom_ref::<Array>().int().any().end();

    //string(array)
    lib_scope.method("string",|mut context|{
        let res=context.param(0).as_custom().with_data_mut(|data:&mut Array|{
            Ok(data.0.iter().map(|x|context.value_to_string(x).unwrap_or("_".to_string())).collect::<Vec<_>>().join(","))
        });

        match res {
            Ok(x)=>Ok(Value::string(format!("Array({x})",))),
            Err(MachineError{error_type:MachineErrorType::CustomDataBorrowMutError,..}) => Ok(Value::String(StringT::new("Array(_)"))),
            Err(x)=>Err(x),
        }
    }).custom_ref::<Array>().end();

    //clone(array)
    lib_scope.method("clone",|mut context|{
        let data: Array= context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_managed_mut(data, context.gc_scope()))
    }).custom_ref::<Array>().end();

    //clear(array)
    lib_scope.method("clear",|context|{
        context.param(0).as_custom().with_data_mut(|data:&mut Array|{
            data.0.clear();
            Ok(Value::Void)
        })
    }).custom_ref::<Array>().end();

    // lib_scope.method("remove",|mut context|{})
    //     .custom_ref::<Array>()
    //     .int()
    //     .optional()
    //     .int()
    //     .end();

    // lib_scope.method("is_array",|mut context|{})
    //     .custom_ref::<Array>()
    //     .end();

    // lib_scope.method("is_array",|mut context|{})
    //     .any()
    //     .end();

    // lib_scope.method("each",custom_array_each)
    //     .custom_ref::<Array>()
    //     .func()
    //     .end();

    // lib_scope.method("map",custom_array_map)
    //     .custom_ref::<Array>()
    //     .func()
    //     .end();

}