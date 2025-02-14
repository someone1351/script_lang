// use std::any::Any;
// use std::collections::HashMap;
// use std::fmt::Debug;
 
// use super::super::common::*;

// use super::debug::*;
use super::gc_scope::*;
use super::value::*;
use super::error::*;
use super::machine::*;

// use super::custom::*;

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

pub struct FuncContext<'q,'a,X> { //,'b
    machine:&'q mut Machine<'a, X>, //,'b
    params_start : usize,
    params_num : usize,
    
}

// impl<'q,'a,X> FuncContext<'q,'a,&mut X> { 
//     pub fn core_mut(&mut self) -> &mut X {
//         self.machine.core_mut()
//     }
//     pub fn core(&self) -> &X {
//         self.machine.core()
//     }
// }

// impl<'q,'a,X> FuncContext<'q,'a,&X> { 
//     pub fn core(&self) -> &X {
//         self.machine.core()
//     }
// }

impl<'q,'a,'c,X> FuncContext<'q,'a,X> { //,'b //,'b

    // pub fn get_core_mut(&mut self) -> &mut X {
    //     self.machine.get_core_mut()
    // }
    // pub fn get_core(& self) -> &X {
    //     self.machine.get_core()
    // }
    
    pub fn core_mut(&mut self) -> &mut X {
        self.machine.core_mut()
    }
    pub fn core(&self) -> &X {
        self.machine.core()
    }
    pub fn new(machine:&'q mut Machine<'a,X>,params_num : usize,) -> Self { //,'b
        let stack_len = machine.stack().len();

        Self {
            machine,
            params_num,
            params_start : stack_len-params_num,
        }
    }
    

    pub fn params_num(&self) -> usize {
        self.params_num
    }
    pub fn param(&self,ind:usize) -> Value {
        if ind < self.params_num {
            let stack_ind=self.params_start+self.params_num-1-ind;
            self.machine.get_stack_val(stack_ind).ok().map(|x|x.clone_leaf()).unwrap_or(Value::Nil) //unwrap nil is for func calls with less params than there is on stack?
        } else {
            Value::Nil
        }
    }

    pub fn get_param(&self,ind:usize) -> Option<Value> {
        if ind < self.params_num {
            let stack_ind=self.params_start+self.params_num-1-ind;
            Some(self.machine.get_stack_val(stack_ind).ok().map(|x|x.clone_leaf()).unwrap_or(Value::Nil)) //unwrap nil is for func calls with less params than there is on stack?
        } else {
            None
        }
    }

    pub fn error<S: Into<String>>(&self, msg:S) -> MachineError {
        MachineError{
            build : self.machine.cur_build(),
            loc : self.machine.cur_build().and_then(|cur_build|cur_build.instr_locs.get(&self.machine.instr_pos()).cloned()),
            error_type:MachineErrorType::MethodRunError(msg.into()),
        }
    }

    pub fn stack_frame(&self) -> Option<StackFrame> {
        self.machine.stack_frames().last().cloned()
    }    
    pub fn stack_val(&self,stack_ind:usize) -> Result<Value,MachineError> {
        self.machine.get_stack_val(stack_ind)
    }
    pub fn result_val(&self) -> Value {
        self.machine.result_val()
    }
    pub fn gc_scope(&mut self) -> &mut GcScope {
        self.machine.gc_scope()
    }

    pub fn value_to_string(&mut self,value: &Value) -> Result<String,MachineError> {
        Ok(self.machine.try_call_method("string", &vec![value.clone_root()])?
            .and_then(|x|Some(x.as_string()))
            .unwrap_or_else(||value.as_string())
        )
    }

    // pub fn call_value<I:AsRef<[Value]>>(&mut self,value:&Value,params : I) -> Result<Value,MachineError> {
    //     self.machine.call_value(value, &params.as_ref().to_vec())
    // }
    pub fn call_method<I:AsRef<[Value]>>(&mut self,name:&str,params : I)->Result<Value,MachineError> {
        self.machine.call_method(name, &params.as_ref().to_vec())
    }
    pub fn try_call_method<I:AsRef<[Value]>>(&mut self,name:&str,params : I)->Result<Option<Value>,MachineError> {
        self.machine.try_call_method(name, &params.as_ref().to_vec())
    }
    
    pub fn global_decl(&mut self,name:&str,to_value:Option<Value>) -> Result<(),MachineError> {
        self.machine.global_decl(name, to_value)
    }
    pub fn global_set<T:AsRef<[Value]>>(&mut self,name:&str,fields:T,to_value:Value) -> Result<(),MachineError> {
        self.machine.global_set(name, &fields.as_ref().to_vec(), to_value)
    }
    pub fn global_get<T:AsRef<[Value]>>(&mut self,name:&str,fields:T) -> Result<Value,MachineError> {
        self.machine.global_get(name, &fields.as_ref().to_vec())
    }
    pub fn value_set<T:AsRef<[Value]>>(&mut self,value:Value,fields:T,to_value:Value) -> Result<(),MachineError> {        
        self.machine.value_set(value, &fields.as_ref().to_vec(), to_value)
    }
    pub fn value_get<T:AsRef<[Value]>>(&mut self,value:Value,fields:T) -> Result<Value,MachineError> {
        self.machine.value_get(value, &fields.as_ref().to_vec())
    }
    pub fn constant_get(&self,name:&str) -> Option<Value> {
        self.machine.constant_get(name)
    }

}
 
// fn ordinal(i:usize)->String {
//     match i.to_string().chars().last().unwrap() {
//         '1' => format!("{i}st"),
//         '2' => format!("{i}nd"),
//         '3' => format!("{i}rd"),
//         _ => format!("{i}th"),
//     }.to_string()
// }