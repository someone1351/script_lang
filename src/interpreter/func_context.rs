use std::any::Any;
// use std::collections::HashMap;
// use std::fmt::Debug;
 
// use super::super::common::*;

// use super::debug::*;
use super::gc_scope::*;
use super::value::*;
use super::error::*;
use super::machine::*;

use super::custom::*;

//todo error funcs for specific params, so can report errors returned from methods at correct place
// what about get/set fields? done in cmds in sexpr_compiler?

/*
todo:
* need to be able to pause execution, but can't if rust method has called a script func
* * disallow calling script funcs from rust methods?
* * * only need to call script functions directly from the machine?
* * * what about calling other methods? necessary? what if the method called then calls a script func?
* * * needed for array's for_each and map methods

* * have method_ext(method), that takes dif func signature ()->Continuable
* * * Continuable {Return(Value),Call(Value)}
* * * need way to store information from last call, either store in return or use context eg context.store_continue<T>(name,val)
* * * * 

* * could allow inserting instructions or manipulating (or adding to) the stack?
*/

pub struct FuncContext<'q,'a,'c,X> { //,'b
    machine:&'q mut Machine<'a,'c, X>, //,'b
    params_start : usize,
    params_num : usize,
    
}

impl<'q,'a,'c,X:Copy> FuncContext<'q,'a,'c,X> { 
    pub fn get_core(&mut self) -> X {
        self.machine.get_core()
    }
}
// impl<'q:'a,'a,'c:'a,X> FuncContext<'q,'a,'c,&'a mut X> { 
//     // pub fn get_core_mut(&'a mut self) -> &'a mut X {
//     //     self.machine.get_core_mut()
//     // }
//     pub fn get_core_ref(&'a self) -> &'a X {
//         self.machine.get_core_ref()
//     }
// }

impl<'q,'a,'c,X> FuncContext<'q,'a,'c,&mut X> { 
    pub fn get_core_mut(&mut self) -> &mut X {
        self.machine.get_core_mut()
    }
    pub fn get_core_ref(&self) -> &X {
        self.machine.get_core_ref()
    }
}

impl<'q,'a,'c,X> FuncContext<'q,'a,'c,&X> { 
    pub fn get_core_ref(&self) -> &X {
        self.machine.get_core_ref()
    }
}

impl<'q,'a,'c,X> FuncContext<'q,'a,'c,X> { //,'b //,'b
    pub fn new(machine:&'q mut Machine<'a,'c,X>,params_num : usize,) -> Self { //,'b
        let stack_len = machine.stack().len();

        Self {
            machine,
            params_num,
            params_start : stack_len-params_num,
        }
    }
    
    pub fn last_stack_frame(&self) -> Option<StackFrame> {
        self.machine.stack_frames().last().cloned()
    }
    
    // pub fn stack(&self) -> &Vec<Value> {
    //     self.machine.stack()
    // }
    
    pub fn get_stack_val(&self,stack_ind:usize) -> Result<Value,MachineError> {
        self.machine.get_stack_val(stack_ind)
    }

    pub fn set_stack_val(&mut self,stack_ind:usize,v:Value) -> Result<(),MachineError> {
        self.machine.set_stack_val(stack_ind,v)
    }
    // pub fn stack_mut(&mut self) -> &mut Vec<Value> {
    //     self.machine.stack_mut()
    // }
    pub fn result_val(&self) -> &Value {
        self.machine.result_val()
    }
    pub fn params_len(&self) -> usize {
        self.params_num
    }
    // pub fn get_param(&self,ind:usize) -> Option<&Value> {
    //     if ind < self.params_num {
    //         self.machine.stack().get(self.params_start+self.params_num-1-ind)
    //     } else {
    //         None
    //     }
    // }

    pub fn param(&self,ind:usize) -> Value {
        if ind < self.params_num {
            let stack_ind=self.params_start+self.params_num-1-ind;
            self.machine.get_stack_val(stack_ind).ok().clone().unwrap_or(Value::Nil)
            // self.machine.stack(stack_ind).get().cloned().unwrap_or(Value::Nil)
        } else {
            Value::Nil
        }
    }

    // pub fn param_float(&self,ind:usize)->FloatType {
    //     self.param_value(ind).as_float()
    // }
    // pub fn param_int(&self,ind:usize)->IntType {
    //     self.param_value(ind).as_int()
    // }
    // pub fn param_bool(&self,ind:usize)->bool //Result<bool,MachineError> 
    // {
    //     self.param_value(ind).as_bool()

    //     // let val=self.get_param_value(ind);

    //     // Ok(self.machine
    //     //     .try_call_method("bool", vec![val.clone_to_root()?])?
    //     //     .and_then(|x|Some(x.as_bool()))
    //     //     .unwrap_or_else(||val.as_bool()))

    // }
    pub fn value_to_string(&mut self,value: &Value) -> Result<String,MachineError> {
        let m=self.machine.try_call_method("string", vec![value.clone()])?;
        let s=m.and_then(|x|Some(x.as_string())).unwrap_or_else(||value.as_string());
        Ok(s)
    }
    pub fn param_to_string(&mut self,ind:usize)->Result<String,MachineError> {
        let val=self.param(ind);
        self.value_to_string(&val)
    }
    
    // pub fn param_string(&self,ind:usize)->String {
    //     self.param_value(ind).as_string()
    // }
    // pub fn get_param_custom(&self,ind:usize)->Custom {
    //     self.param(ind).as_custom()
    // }

    pub fn param_is_nil(&self,ind:usize)->bool {
        self.param(ind).is_nil()
        // if let Value::Nil=self.get_param(ind).unwrap_or(&Value::Nil){true}else{false}
    }

    // pub fn param_call(&mut self,ind:usize,params : Vec<Value>)->Result<Value,MachineError> {
    //     // self.machine.call_func(self.params.get(ind).unwrap_or(&Value::Nil), params).unwrap();
    //     // true
    //     let x = self.get_param(ind).unwrap_or(&Value::Nil).clone();
    //     self.machine.call_val(&x, params)
    // }

    // //todo replace calls with cont calls
    
    // pub fn set_func_call<I:IntoIterator<Item=Value>>(&mut self,value:&Value,params : I) {
    //     // self.machine.call_value(value, params)
    // }

    // pub fn set_method_call<I:IntoIterator<Item=Value>>(&mut self,name:&str,params : I) {
    //     // println!("===========x");
    //     // self.machine.call_method(name, params)
    // }

    // //
    // pub fn get_call_ret() -> Option<(Value,Value)> //Result<Value,MachineError> 
    // {
    //     //first val is the return from the original method that made the cont call
    //     // and the second is the result from the actual call
    //     None

    // }


    // //remove calls below

    pub fn call_value<I:IntoIterator<Item=Value>>(&mut self,value:&Value,params : I) -> Result<Value,MachineError> {
        self.machine.call_value(value, params)
    }

    // // pub fn call_global(&mut self,name:&str,params : Vec<Value>)->Result<Value,MachineError> {
    // //     self.machine.call_global(name, params)
    // // }

    pub fn call_method<I:IntoIterator<Item=Value>>(&mut self,name:&str,params : I)->Result<Value,MachineError> {
        // println!("===========x");
        self.machine.call_method(name, params)
    }

    pub fn try_call_method<I:IntoIterator<Item=Value>>(&mut self,name:&str,params : I)->Result<Option<Value>,MachineError> {
        self.machine.try_call_method(name, params)
    }

    //what is this for?
    // pub fn run_build(&mut self, build:&BuildT) -> Result<Value,MachineError> {
    //     self.machine.run_build(build)
    // }
    

    //traverser:  Option<for<'z> fn(&'z T)->Box<dyn Iterator<Item=& Value>+'z>>
    // pub fn new_custom_managed<T:GcTraversable>(&mut self, data:T, ) -> Custom {
    //     Custom::new_managed(data,self.machine.gc_scope())
    // }
    
    // pub fn new_custom_unmanaged<T:Any+Send>(&mut self, data:T, ) -> Custom {
    //     Custom::new_unmanaged(data)
    // }

    // pub fn custom_managed<T:GcTraversable+Send+Sync>(&mut self, data : T,is_mut:bool) -> Value {
    //     Value::custom_managed(data, is_mut, self.machine.gc_scope())
    // }

    // pub fn custom_unmanaged<T:Any+Send+Sync>(&self, data : T,is_mut:bool) -> Value {
    //     Value::custom_unmanaged(data, is_mut)
    // }
    
    //
    
    pub fn custom_managed_mut<T:GcTraversable+Send>(&mut self, data:T, ) -> Value {
        Value::Custom(Custom::new_managed_mut(data,self.machine.gc_scope()))
    }
    
    pub fn custom_unmanaged_mut<T:Any+Send>(&self, data:T, ) -> Value {
        Value::Custom(Custom::new_unmanaged_mut(data))
    }


    pub fn custom_managed<T:GcTraversable+Send+Sync>(&mut self, data:T, ) -> Value {
        Value::Custom(Custom::new_managed(data,self.machine.gc_scope()))
    }
    
    pub fn custom_unmanaged<T:Any+Send+Sync>(&self, data:T, ) -> Value {
        Value::Custom(Custom::new_unmanaged(data))
    }

    // pub fn new_custom_managed<T:GcTraversable>(&mut self, data:T, ) -> Custom {
    //     Custom::new_managed(data,self.machine.gc_scope())
    // }


    pub fn global_decl(&mut self,name:&str,to_value:Option<Value>) -> Result<(),MachineError> {
        self.machine.global_decl(name, to_value)
    }
    
    pub fn global_set<T:AsRef<[Value]>>(&mut self,name:&str,fields:T,to_value:Value) -> Result<(),MachineError> {
        self.machine.global_set(name, fields, to_value)
    }
    
    pub fn global_get<T:AsRef<[Value]>>(&mut self,name:&str,fields:T) -> Result<Value,MachineError> {
        self.machine.global_get(name, fields)
    }

    pub fn value_set<T:AsRef<[Value]>>(&mut self,value:Value,fields:T,to_value:Value) -> Result<(),MachineError> {        
        self.machine.value_set(value, fields, to_value)
    }

    pub fn value_get<T:AsRef<[Value]>>(&mut self,value:Value,fields:T) -> Result<Value,MachineError> {
        self.machine.value_get(value, fields)
    }
    
    pub fn constant_get(&self,name:&str) -> Option<Value> {
        self.machine.constant_get(name)
    }
    pub fn error<S: Into<String>>(&self, msg:S) -> MachineError {
        let msg=msg.into();
        // if let Some((method_name,method_params))=self.machine.debugger().last_method_call_info() {

        // }

        MachineError::from_machine(&self.machine, MachineErrorType::MethodRunError(msg))
    }

    // pub fn param_expect<S: Into<String>>(&self, msg:S) -> Result<(),MachineError> {
    //     let msg=msg.into();
    //     // MachineError::machine_new(&self.machine, MachineErrorType::MethodRunError(msg))

    // }

    // pub fn set_error_loc(&mut self, param:Option<usize>) {

    // }
    
    // pub fn set_error_locs<T:AsRef<[usize]>>(&mut self, params:T) {
    //     //first is func, rest are params
    //     //by default none are set
    //     //any rust func calls while they are set, will use the locs as their own
    //     // if too little params, will just use last param specified or maybe func loc (first in array)
    //     // if no error locs specified, will just default to original func call's loc (probably same as doing nothing)

    // }
    // pub fn clear_error_locs(&mut self) {

    // }
    pub fn param_expect_int(&self,ind:usize) -> Result<Value,MachineError> {
        let v=self.param(ind);
        if !v.is_int() { return Err(self.error(format!("Expecting {} at {} param","int",ordinal(ind+1)))); }
        Ok(v)
    }

    pub fn param_expect_float(&self,ind:usize) -> Result<Value,MachineError> {
        let v=self.param(ind);
        if !v.is_float() { return Err(self.error(format!("Expecting {} at {} param","float",ordinal(ind+1)))); }
        Ok(v)
    }

    pub fn param_expect_bool(&self,ind:usize) -> Result<Value,MachineError> {
        let v=self.param(ind);
        if !v.is_bool() { return Err(self.error(format!("Expecting {} at {} param","bool",ordinal(ind+1)))); }
        Ok(v)
    }
    
    pub fn param_expect_string(&self,ind:usize) -> Result<Value,MachineError> {
        let v=self.param(ind);
        if !v.is_string() { return Err(self.error(format!("Expecting {} at {} param","string",ordinal(ind+1)))); }
        Ok(v)
    }
    pub fn param_expect_custom<T:'static>(&self,ind:usize) -> Result<Value,MachineError> {
        let v=self.param(ind);
        if !v.is_custom::<T>() { return Err(self.error(format!("Expecting {} at {} param",std::any::type_name::<T>(),ordinal(ind+1)))); }
        Ok(v)
    }

    // pub fn param_expects(&self) -> ParamExpecter {

    // }
    
}

// struct ParamExpecter {

// }

// impl ParamExpecter {
//     pub fn int(self) -> Self {
//         self
//     }
//     pub fn float(self) -> Self {
//         self
//     }
//     pub fn bool(self) -> Self {
//         self
//     }
//     pub fn str(self) -> Self {
//         self
//     }
//     pub fn nil(self) -> Self {
//         self
//     }
    
//     pub fn func(mut self) -> Self {
//         self.args.push(vec![Arg::custom::<Closure>()]);
//         self
//     }
//     pub fn custom<T:'static>(mut self) -> Self {
//         self.args.push(vec![Arg::custom::<T>()]);
//         self
//     }
//     pub fn end() -> Result<(),MachineError> {
//         Ok(())
//     }
// }
fn ordinal(i:usize)->String {
    match i.to_string().chars().last().unwrap() {
        '1' => format!("{i}st"),
        '2' => format!("{i}nd"),
        '3' => format!("{i}rd"),
        _ => format!("{i}th"),
    }.to_string()
}