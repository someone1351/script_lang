
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use super::super::common::*;
use super::custom::*;
use super::gc_scope::*;
use super::value::*;
use super::data::*;
use super::error::*;
use super::lib_scope::*;
use super::var_scope::*;
use super::func_context::*;
use super::debug::*;

#[derive(Debug,Clone)]
pub struct StackFrame { 
    pub ret_build : Option<BuildT>,
    pub ret_instr_ind : usize,
    pub ret_instr_end : usize,

    pub finish : bool,
    pub stack_params_start : usize,

    pub stack_params_num : usize,
    pub func_params_num : usize,

    
    // pub ret_stack_size : usize,
    
}

// pub type IncludeResolver = dyn for <'a> FnMut(&'a Path) -> Option<BuildType>;
// pub type Includer<'a> = dyn FnMut(&Path) -> Option<BuildT> + 'a;
// pub type GlobalGet<'a> = dyn FnMut(&str) -> Option<&'a mut Value> + 'a;
// pub type GlobalSet<'a> = dyn FnMut(&str) + 'a;


pub struct Machine<'a,'c,X> { //,'b
    cur_build : Option<BuildT>,
    instr_pos : usize,
    instr_end_pos : usize,
    result_val : Value,
    stack : Vec<Value>,
    stack_frames : Vec<StackFrame>,
    error_state:bool,
    debugger : Debugger,
    gc_scope : &'a mut GcScope,
    var_scope : &'a mut VarScope, 
    lib_scope : &'a LibScope<'c,X>, //<'c> //<'b>
    // temp_scope  : &'a TempScope<'c>, //<'b>
    includer : Option<Box<dyn FnMut(&Path) -> Option<BuildT> + 'a>>, //can use lifetime a for some reason?
    // const_scope : Option<Box<dyn Fn(&str)->Option<Value> + 'a>>,
    const_scope:HashMap<&'a str,Value>,
    // phantom_data:PhantomData<&'b ()>,
    // global_get : Box<GlobalGet<'a>>,
    // global_set : Box<GlobalSet<'a>>,

    //
    // gc_check_remove_recursive : bool,
    core_val :   X, //&'a mut 
}


pub trait MachineTrait<'a,'c> {

    fn stack(&self) -> &Vec<Value>;
    
    fn stack_frames(&self) -> &Vec<StackFrame>;
    fn get_stack_val(&self,stack_ind:usize) -> Result<Value,MachineError>;
    
    fn result_val(&self) -> &Value;
    fn try_call_method(&mut self,name:&str,params : &Vec<Value>) -> Result<Option<Value>,MachineError>;

    fn call_value(&mut self,v:&Value,params : &Vec<Value>) -> Result<Value,MachineError>;
    
    fn call_method(&mut self,name:&str,params : &Vec<Value>) -> Result<Value,MachineError>;

    fn gc_scope(&mut self)->&mut GcScope;
    
    fn global_decl(&mut self,name:&str,to_value:Option<Value>) -> Result<(),MachineError>;    
    fn global_set(&mut self,name:&str,fields:&Vec<Value>,to_value:Value) -> Result<(),MachineError>;
    fn global_get(&mut self,name:&str,fields:&Vec<Value>) -> Result<Value,MachineError>;
    
    fn value_set(&mut self,value:Value,fields:&Vec<Value>,to_value:Value) -> Result<(),MachineError>;
    fn value_get(&mut self,value:Value,fields:&Vec<Value>) -> Result<Value,MachineError>;
    
    fn constant_get(&self,n:&str) -> Option<Value>;
    fn error(&self, msg:&str) -> MachineError ;

}

impl<'a,'c,X> MachineTrait<'a,'c> for Machine<'a,'c,X> {

    fn stack(&self) -> &Vec<Value> {
        &self.stack
    }
   
    
    fn stack_frames(&self) -> &Vec<StackFrame> {
        &self.stack_frames
    }
    fn get_stack_val(&self,stack_ind:usize) -> Result<Value,MachineError> {
        let stack_len = self.stack.len();

        if stack_ind >= stack_len {
            return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(stack_ind-stack_len) ));
        }

        Ok(self.stack.get(stack_ind).unwrap().clone())
    }
    
    fn result_val(&self) -> &Value {
        &self.result_val
    }
    fn try_call_method(&mut self,name:&str,params : &Vec<Value>) -> Result<Option<Value>,MachineError> {
        self.try_call_method(name, params)
    }
    fn call_value(&mut self,v:&Value,params : &Vec<Value>) -> Result<Value,MachineError> {
        // Machine::call_value(&mut self, v, params)
        self.call_value(v, params)
        // Ok(Value::Nil)
    }

    
    fn call_method(&mut self,name:&str,params : &Vec<Value>) -> Result<Value,MachineError> {
        self.call_method(name, params)
    }
    fn gc_scope(&mut self)->&mut GcScope {
        self.gc_scope
    }
    
    fn global_decl(&mut self,name:&str,to_value:Option<Value>) -> Result<(),MachineError> {
        self.global_decl(name, to_value)
    }
    fn global_set(&mut self,name:&str,fields:&Vec<Value>,to_value:Value) -> Result<(),MachineError> {
        self.global_set(name, fields, to_value)
    }
 
    fn global_get(&mut self,name:&str,fields:&Vec<Value>) -> Result<Value,MachineError> {
        self.global_get(name, fields)
    }
    fn value_set(&mut self,value:Value,fields:&Vec<Value>,to_value:Value) -> Result<(),MachineError> {
        self.value_set(value, fields, to_value)
    }
    fn value_get(&mut self,value:Value,fields:&Vec<Value>) -> Result<Value,MachineError> {
        self.value_get(value, fields)
    }
    
    fn constant_get(&self,n:&str) -> Option<Value> {
        self.constant_get(n)
    }
    fn error(&self, msg:&str) -> MachineError {
        let msg=msg.into();
        // if let Some((method_name,method_params))=self.machine.debugger().last_method_call_info() {

        // }

        MachineError::from_machine(self, MachineErrorType::MethodRunError(msg))
    }
}

impl<'a,'c,X:Copy> Machine<'a,'c,X> {
    pub fn get_core(&mut self) -> X {
        self.core_val
    }
}
// impl<'a,'c,X> Machine<'a,'c,&'a mut X> {
//     // pub fn get_core_mut(&'a mut self) -> &'a mut X {
//     //     self.core_val
//     // }    
//     pub fn get_core_ref(&'a self) -> &'a X {
//         self.core_val
//     }
// }
impl<'a,'c,X> Machine<'a,'c,& mut X> {
    pub fn get_core_mut(&mut self) -> &mut X {
        self.core_val
    }    
    pub fn get_core_ref(& self) -> &X {
        self.core_val
    }
}

impl<'a,'c,X> Machine<'a,'c,&X> {
    pub fn get_core_ref(&self) -> &X {
        self.core_val
    }
}

impl<'a,'c,X> Machine<'a,'c,X> 
{ //,'b //,'b

    // pub fn core_val(&mut self) -> &mut X {
    //     &mut self.core_val
    // }


    pub fn new (
        gc_scope : &'a mut GcScope,
        var_scope : &'a mut VarScope, 
        lib_scope : &'a LibScope<'c,X>,
        core_val :  X,//&'a mut X,
    ) -> Self
    {
        let debugger = Debugger::new(); //debugger_enabled,debugger_print
        
        // debugger.set_enabled(false);

        Self {
            // phantom_data:PhantomData::default(),
            instr_pos : 0,
            instr_end_pos : 0, //build.main_instruct_len,
            cur_build: None, //Some(build.clone()),
            result_val : Value::Nil,
            stack : Vec::new(),
            stack_frames : Vec::new(),
            error_state:false,

            debugger,

            gc_scope,
            var_scope,
            lib_scope,
            // temp_scope,
            // global_get : Box::new(global_get),
            // global_set : Box::new(global_set),
            includer : None, //Box::new(include_resolver),
            const_scope:HashMap::new(),
            // gc_check_remove_recursive:true,
            core_val,
        }
    }

    pub fn set_debug_print(&mut self,enabled:bool) {
        self.debugger.set_print(enabled);
    }
    pub fn set_debug(&mut self,enabled:bool) {
        self.debugger.set_enabled(enabled);
    }

    pub fn debugger(&self) -> &Debugger {
        &self.debugger
    }

    // pub fn set_gc_check_remove_recursive(&mut self,b:bool) {
    //     self.gc_check_remove_recursive=b;
    // }

    // pub fn set_const_scope<F:Fn(&str)->Option<Value> + 'a>(&mut self,f:F) {
    //     self.const_scope=Some(Box::new(f));
    // }

    pub fn set_const(&mut self,n:&'a str,v:Value) {
        if v.is_undefined() {
            self.const_scope.remove(n);
        } else {
            self.const_scope.insert(n,v.clone_root());
        }
    }

    pub fn set_includer<F:FnMut(&Path) -> Option<BuildT> + 'a>(&mut self,f:F) {
        self.includer=Some(Box::new(f));
    }
    pub fn gc_scope(&mut self)->&mut GcScope {
        self.gc_scope
    }

    // fn propagate_err<'q>(&self) -> impl Fn(MachineError) -> MachineError+'q 
    // where 'a:'q
    // {
    //     move|e|MachineError::machine_new(& self, e.error_type)
    // }

    pub fn clear(&mut self) {
        self.cur_build=None;
        self.instr_pos=0;
        self.instr_end_pos=0;
        // self.result_val=Value::Nil;
        self.set_result_val(Value::Nil);

        self.stack.clear();
        self.stack_frames.clear();

        self.error_state=false;

        self.debugger.clear();
    }

    pub fn print_state(&self) {
        println!("machine state:");
        println!("\tcur_build = {:?}",self.cur_build);
        println!("\tinstr_pos = {:?}",self.instr_pos);
        println!("\tinstr_end_pos = {:?}",self.instr_end_pos);
        println!("\tresult_val = {:?}",self.result_val);
        println!("\tstack = {:?}",self.stack);
        println!("\tstack_frames = {:?}",self.stack_frames);
        println!("\terror_state = {:?}",self.error_state);
        
     
    }

    pub fn debug_print_stack_trace(&self, skip_first:bool) {
        self.debugger.print_stack_trace(skip_first);
    }

    pub fn debug_print_stack(&self) {
        // println!("stack:");
        // for (i,x) in self.stack.iter().rev().enumerate() {
        //     println!("\t{i}: {x:?}");
        // }
        self.debugger.print_stack(&self.stack);
    }

    pub fn cur_build(&self) -> Option<BuildT> {
        self.cur_build.clone()
    }

    pub fn var_scope(&self) -> &VarScope {
        self.var_scope
    }

    pub fn var_scope_mut(&mut self) -> &mut VarScope {
        self.var_scope
    }
    pub fn instr_pos(&self) -> usize {
        self.instr_pos
    }

    pub fn instr_end_pos(&self) -> usize {
        self.instr_end_pos
    }

    pub fn stack(&self) -> &Vec<Value> {
        &self.stack
    }

    // pub fn stack_mut(&mut self) -> &mut Vec<Value> {
    //     &mut self.stack
    // }

   

    pub fn result_val(&self) -> &Value {
        &self.result_val
    }

    pub fn stack_frames(&self) -> &Vec<StackFrame> {
        &self.stack_frames
    }

    fn run(&mut self) -> Result<(),MachineError> {
        // if self.debugger.stack_traces().len()==0 {
        //     self.debugger.push_main(self.cur_build.clone());
        // }

        // self.stack_frames.push(Vec::new());
        loop {

            //
            if self.instr_pos>self.instr_end_pos {
                self.print_state();
                self.debugger.print_stack_trace(true);
                panic!("scriptlang, machine, instr ind ({}) incremented past end ({})",self.instr_pos,self.instr_end_pos);
            }

            //debug
            self.debugger.step(&self.stack,&self.result_val);
           
            //
            if self.instr_pos == self.instr_end_pos { 
                if self.stack_frame_pop()?  { //at end of program
                    break;
                }
            } else { //if self.instr_ind <= self.instr_end
                self.step()?;
            }
        }

        Ok(())
    }

    fn step(&mut self) -> Result<(),MachineError> {
        let cur_build=self.cur_build.clone().unwrap();
        let instr=cur_build.instructions.get(self.instr_pos).unwrap();

        match instr {
            Instruction::JmpUp{cond,instr_offset} => { //more like relative instr up ind
                if cond.and_then(|x|Some(x==self.result_val.as_bool())).unwrap_or(true) {
                    if self.instr_pos > *instr_offset {
                        self.instr_pos-=*instr_offset+1;
                        
                        self.debugger.move_instr_pos(self.instr_pos);

                        return Ok(()); //continue;
                    } else {
                        return Err(MachineError::from_machine(self, MachineErrorType::JmpUpErr(*instr_offset)));
                    }
                }
            }
            Instruction::JmpDown{cond,instr_offset} => {
                if cond.and_then(|x|Some(x==self.result_val.as_bool())).unwrap_or(true) {
                    if self.instr_pos + *instr_offset + 1 <= self.instr_end_pos //build.instructions.len() 
                    {
                        self.instr_pos+=*instr_offset+1;
                        self.debugger.move_instr_pos(self.instr_pos);
                        return Ok(()); //continue;
                    } else {
                        return Err(MachineError::from_machine(self, MachineErrorType::JmpDownErr(*instr_offset) ));
                    }
                }
            }
            
            Instruction::ResultBool(v)  => {
                self.result_val = Value::Bool(*v);
            }
            Instruction::ResultInt(v)  => {
                self.result_val = Value::Int(*v);
            }
            Instruction::ResultFloat(v)  => {
                self.result_val = Value::Float(*v);
            }
            Instruction::ResultVoid => {
                self.result_val = Value::Void;
            }
            Instruction::ResultNil => {
                self.result_val = Value::Nil;
            }
            Instruction::ResultSymbol(symbol_ind)  => {
                let symbol=self.get_symbol( *symbol_ind)?;
                self.result_val = Value::String(symbol.clone());
            }
            Instruction::ResultVararg => {
                self.result_val=Value::Custom(Custom::new_unmanaged_mut(Vararg));
                // self.result_val=Value::Vararg;
                // self.call_method("vararg", vec![])?;
            }
            &Instruction::ResultFunc(func_ind, captures_num)  => { //todo
                // let func=Value::BuildFunc(self.cur_build.clone().unwrap(),*func_ind);

                // if *captures_num ==0 {
                //     self.result_val = func;
                // } else 
                {
                    // let mut params=vec![func];
                    // params.extend(self.stack_params_iter(*captures_num).map(|capture|capture.clone_root()));

                    // self.stack_pop_amount(*captures_num)?;
                    // let v=self.call_method("closure", params)?;
                    // self.result_val = v;

                    let captures=self.stack_params_iter(captures_num)
                        .rev()                        
                        .map(|capture|capture.clone())
                        .collect::<Vec<_>>();
                    
                    //println!("@@@@@ popping captures");
                    self.stack_pop_amount(captures_num)?;
                    
                    // self.result_val=Value::Closure(GcValueType::<Closure>::new(Closure { 
                    //     captures, 
                    //     build: self.cur_build.clone().unwrap(), 
                    //     func_ind: *func_ind 
                    // }, self.gc_scope));

                    let closure=Closure{ 
                        captures, 
                        build: self.cur_build.clone().unwrap(), 
                        func_ind, 
                    };

                    if captures_num==0 {
                        // self.result_val=Value::custom_unmanaged(closure);
                        self.set_result_val(Value::custom_unmanaged(closure));
                    } else {
                        // self.result_val=Value::custom_managed(closure, self.gc_scope);
                        let v=Value::custom_managed(closure, self.gc_scope);
                        self.set_result_val(v);
                        
                    }

                }
            }

            Instruction::StackPush => {
                // println!("aa=== {} {}",self.stack.len(),self.debugger.stack_val_infos.len());
                let v=self.result_val.clone();
                
                // if v.is_void() {
                //     return Err(MachineError::from_machine(self, MachineErrorType::VoidNotExpr));
                // }

                self.push_stack_val(v);
                // self.stack.push(self.result_val.clone_root());
                
                self.debugger.set_stack_from_last();
                // self.debugger.set_stack_val_some(self.stack.len()-1,self.cur_build.clone().unwrap(),self.instr_pos);
            }
            // Instruction::StackDup(stack_offset_ind) => {
            //     let v = self.get_stack_offset_value(*stack_offset_ind)?;             
            //     self.push_stack_val(v.clone());
            //     self.debugger.set_stack_from_last(); //what does this do?
            // }
            Instruction::StackDup => {
                let v = self.get_stack_offset_value(0)?;             
                self.push_stack_val(v.clone());
                self.debugger.set_stack_from_last(); //what does this do?
            }
            Instruction::StackLocals(amount) => {
                // let v=self.copy_val(self.result_val.clone())?;
                // // self.stack.push(v);                
                // self.push_stack_val(v);

                // self.push_stack_val(Value::Nil);

                for _ in 0..(*amount) {
                    self.push_stack_val(Value::Undefined); //have Value::Undefined ?
                    //
                    self.debugger.set_stack_from_last();
                }
            }
            Instruction::StackPop(amount) => {
                //println!("@@@@@ instr pop");
                self.stack_pop_amount(*amount)?;
            } 
            Instruction::StackSwap => {
                let stack_len = self.stack.len();

                if stack_len>=2 {
                    self.stack.swap(stack_len-1, stack_len-2);
                    self.debugger.stack_swap();
                } else {
                    return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(2) ));
                }
            }
            Instruction::StackRot => {
                let stack_len = self.stack.len();

                if stack_len>=3 {
                    self.stack[stack_len-3 ..].rotate_left(1);
                    self.debugger.stack_rot();
                } else {
                    return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(3) ));
                }
            }
            Instruction::SetStackVar(stack_offset_ind) => {
                let v=self.copy_val(self.result_val.clone())?;

                // if v.is_void() {
                //     return Err(MachineError::from_machine(self, MachineErrorType::VoidNotExpr));
                // }

                // *self.stack_value_mut(*stack_offset_ind)?=v;
                self.set_stack_offset_val(*stack_offset_ind, v)?;
                
                // self.debugger.set_stack_val_offset_some(*stack_offset_ind,self.cur_build.clone().unwrap(),self.instr_pos);
                self.debugger.set_stack_val_offset_from_last(*stack_offset_ind);
            }
            Instruction::GetStackVar(stack_offset_ind) => {
                let v = self.get_stack_offset_value(*stack_offset_ind)?;
                // self.result_val = v.clone_root();
                self.set_result_val(v.clone());
            }
            Instruction::MakeStackVarRef(stack_offset_ind) => {
                let v = self.copy_val(self.get_stack_offset_value(*stack_offset_ind)?.clone())?;
                let v=Value::Custom(Custom::new_managed_mut(v.clone(), self.gc_scope));
                
                // *self.stack_value_mut(*stack_offset_ind)?=v;
                self.set_stack_offset_val(*stack_offset_ind, v)?;
                
                // self.debugger.set_stack_val_offset_some(*stack_offset_ind,self.cur_build.clone().unwrap(),self.instr_pos);
                self.debugger.set_stack_val_offset_from_last(*stack_offset_ind);
            }
            Instruction::SetStackVarDeref(stack_offset_ind, init) => {
                let to_val=self.copy_val(self.result_val.clone())?;

                
                // if to_val.is_void() {
                //     return Err(MachineError::from_machine(self, MachineErrorType::VoidNotExpr));
                // }
                // let data=self.get_stack_offset_value(*stack_offset_ind)?.as_custom().data();
                // *data.borrow_mut::<Value>()?=val;

                let custom=self.get_stack_offset_value(*stack_offset_ind)?.as_custom();

                if custom.is_type::<GlobalAccessRef>() {
                    let data=custom.data_clone::<GlobalAccessRef>()
                        .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
                    data.var.as_custom().with_data_mut(|v:&mut Value|{
                        //don't care about init, since this is supposed behave like a normal global_set, not a deref_set
                        if v.is_undefined() {
                            return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(data.name.to_string()) ));
                        }

                        //
                        *v=to_val.clone();
                        Ok(())
                    }).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
                    
                    // if !self.var_scope.set(data.name.as_str(),val)? {}
                } else {
                    let data=custom.data();
                    let mut data=data.get_mut::<Value>()?;
    
                    // if data.is_type(ValueType::custom::<UninitVar>()) {
                    //     // let symbol=self.get_symbol( *symbol_ind)?;
                    //     let symbol="todo";
                    //     return Err(MachineError::machine_new(self, MachineErrorType::GlobalNotFound(symbol.to_string()) ));
                    // }
    
                    if !init && data.is_undefined() {
                        return Err(MachineError::from_machine(self, MachineErrorType::SetUndefinedVar ));
                    }
                    
                    *data=to_val;
    
                }

            }
            Instruction::GetStackVarDeref(stack_offset_ind) => { //todo
                // let stack_var = self.get_stack_offset_value(*stack_offset_ind)?;
                // let data=stack_var.as_custom().data();
                // let v=self.copy_val(data.borrow_mut::<Value>()?.clone())?;

                let custom=self.get_stack_offset_value(*stack_offset_ind)?.as_custom();
                
                if custom.is_type::<GlobalAccessRef>() {
                    let data=custom.data_clone::<GlobalAccessRef>()
                        .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
                    let var_data=data.var.as_custom().data_clone::<Value>()
                        .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;

                    if !var_data.is_undefined() {
                        self.set_result_val(var_data);
                    // if let Some(v)=self.var_scope.get(&data.name)? {
                    //     self.set_result_val(v);
                    } else if let Some(v)=self.constant_get(&data.name) //self.lib_scope.get_constant(&data.name) 
                    {
                        self.set_result_val(v);
                    } else {
                        return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(data.name.to_string()) ));
                    }
                } else {
                    //                
                    // let data=custom.data();
                    // let data=data.borrow_mut::<Value>()?;

                    // // if data.is_type(ValueType::custom::<UninitVar>()) {
                    // //     // let symbol=self.get_symbol( *symbol_ind)?;
                    // //     let symbol="_";
                    // //     return Err(MachineError::machine_new(self, MachineErrorType::GlobalNotFound(symbol.to_string()) ));
                    // // }

                    // if data.is_undefined() {
                    //     return Err(MachineError::from_machine(self, MachineErrorType::GetUndefinedVar ));
                    // }
                    
                    // //
                    // let v=self.copy_val(data.clone())?;

                    // //
                    // self.set_result_val(v);

                    //
                            
                    custom.with_data_mut(|data:&mut Value|{
                        if data.is_undefined() {
                            return Err(MachineError::from_machine(self, MachineErrorType::GetUndefinedVar ));
                        }
                        
                        let v=self.copy_val(data.clone())?;
    
                        self.set_result_val(v);
                        Ok(())
                    })?;
                }

            }

            
            Instruction::CallStackVarDeref(stack_offset_ind,params_num) => {
                let params_num=*params_num;

                // let stack_var = self.get_stack_offset_value(*stack_offset_ind)?;
                let custom=self.get_stack_offset_value(*stack_offset_ind)?.as_custom();
                
                if custom.is_type::<GlobalAccessRef>() {
                    let data=custom.data_clone::<GlobalAccessRef>()
                        .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
                    let var_data=data.var.as_custom().data_clone::<Value>()
                        .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
                    
                    if !var_data.is_undefined() {

                    // if let Some(v)=self.var_scope.get(&data.name)? {
                    //     //call v

                        self.debugger.add_func_name(data.name.as_str());
                        self.inner_call_value(params_num,&var_data,false)?;
                        return Ok(()); //continue;
                    } else if let Some(v)=self.constant_get(&data.name) //self.lib_scope.get_constant(&data.name) 
                    {
                        //call v

                        self.debugger.add_func_name(data.name.as_str());
                        self.inner_call_value(params_num,&v,false)?;
                        return Ok(()); //continue;
                    } else if let Some(x)=self.get_method(data.name.as_str(), params_num) {
                        self.debugger.add_func_name(data.name.as_str());
                        self.inner_call_bound_func(params_num, x)?; //,symbol.clone()
                        //return Ok(()); //continue;
                    } else {
                        let param_types=self.get_stack_param_types(params_num);
                        return Err(MachineError::from_machine(self, MachineErrorType::GlobalFuncOrMethodNotFound(data.name.to_string(),param_types) ));
                    }
                } else {
                    let data=custom.data_clone::<Value>()
                        .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
                    //                
                    // let data=custom.data();
                    // let data=data.borrow_mut::<Value>()?;

                    if data.is_undefined() {
                        return Err(MachineError::from_machine(self, MachineErrorType::GetUndefinedVar ));
                    }
                    
                    //
                    //let v=self.copy_val(data.clone())?;
                    let v=data;
                    
                    self.inner_call_value(params_num,&v,false)?;
                    return Ok(()); //continue;
                    //
                }

            }
            Instruction::DeclGlobalVar(symbol_ind) => {
                let symbol=self.get_symbol( *symbol_ind)?;
                // let v=self.copy_val(self.result_val.clone())?;
                // self.var_scope.decl(symbol.as_str(),v);
                self.var_scope.decl(symbol.as_str(),None)
                    .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
            }

            Instruction::SetGlobalVar(symbol_ind) => {
                let symbol=self.get_symbol( *symbol_ind)?;
                // let v=self.result_val.clone_root();
                let v=self.copy_val(self.result_val.clone())?;

                // if v.is_void() {
                //     return Err(MachineError::from_machine(self, MachineErrorType::VoidNotExpr));
                // }

                if !self.var_scope.set(&symbol,v)
                    .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))? 
                {
                    return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(symbol.to_string()) ));
                }

                // let Some(g)=self.var_scope.get_mut(&symbol) else {
                //     return Err(MachineError::machine_new(self, MachineErrorType::GlobalNotFound(symbol.to_string()) ));
                // };

                // *g=v;
            }
            Instruction::GetGlobalVarOrConst(symbol_ind, get_global) => {
                let symbol=self.get_symbol( *symbol_ind)?;
                
                if let Some(v)=self.inner_global_get(symbol.as_str(),*get_global)? {

                // if let Some(v)=self.var_scope.get(&symbol)
                //     .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?   {
             
                    // self.result_val=v.clone_root();
                    self.set_result_val(v);
                } else if let Some(v)=self.constant_get(&symbol) //self.lib_scope.get_constant(&symbol) 
                {
                    // self.result_val=v.clone_root();
                    self.set_result_val(v);
                } else {
                    return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(symbol.to_string()) ));
                }
            }

            &Instruction::GetGlobalVarRef(symbol_ind,  ) => {
                let symbol=self.get_symbol(symbol_ind)?;
                
                let refvar=self.var_scope.get_ref(&symbol,self.gc_scope);
                self.set_result_val(refvar);

                // if let Some(refvar)=self.var_scope.get_ref(&symbol,self.gc_scope) {
                //     self.set_result_val(refvar);
                // }
                // // else if !write {
                // //     if let Some(v)=self.lib_scope.get_constant(&symbol) {
                // //         let refvar=Value::new_custom_managed(v, self.gc_scope);
                // //         self.set_result_val(refvar);
                // //     } else if !read && call { //method
                // //         let m=Value::new_custom_unmanaged(MethodCall(symbol));
                // //         // let refvar=Value::new_custom_managed(m, self.gc_scope);
                // //         let refvar=Value::new_custom_unmanaged(m);
                // //         self.set_result_val(refvar);
                // //     } else {
                // //         // println!("x {symbol:?}, read {read}, write {write}, call {call}");
                // //         return Err(MachineError::machine_new(self, MachineErrorType::GlobalNotFound(symbol.to_string()) ));
                // //     }
                // // } 
                // else {
                //     return Err(MachineError::machine_new(self, MachineErrorType::GlobalNotFound(symbol.to_string()) ));
                // }

            }

            &Instruction::GetGlobalAccessRef(symbol_ind,  ) => {
                let symbol=self.get_symbol(symbol_ind)?;

                // if self.var_scope.contains(&symbol) {
                //     let refvar=self.var_scope.get_ref(&symbol,self.gc_scope);
                //     println!("!111 {symbol:?}");
                //     self.set_result_val(refvar);
                // } else {
                    let refvar=self.var_scope.get_ref(&symbol,self.gc_scope);
                    // println!("!222 {symbol:?}");
                    let val=Value::custom_managed_mut(GlobalAccessRef{
                        name:symbol,
                        var: refvar,
                    },self.gc_scope);

                    self.set_result_val(val);
                // }
            }
            Instruction::Include(include_ind) => {
                //todo check circular includes

                let include_path = self.cur_build.as_ref().unwrap().includes.get(*include_ind).unwrap();

                let mut path=self.cur_build.as_ref().unwrap().path.clone().unwrap_or(PathBuf::new());
                path.pop();
                path.push(include_path);

                let Some(include_build) = self.includer.as_mut().and_then(|x|x(path.as_path())) //(*self.includer)(path.as_path()) 
                else {
                    return Err(MachineError::from_machine(self, MachineErrorType::IncludeResolveError(include_path.clone()) ));
                };

                self.debugger.push_frame_include(include_build.clone());
                
                self.stack_frames.push(StackFrame {
                    ret_build : self.cur_build.clone(), 
                    ret_instr_ind: self.instr_pos+1, 
                    ret_instr_end : self.instr_end_pos,
                    stack_params_num : 0,
                    finish : false,
                    
                    stack_params_start:self.stack.len(),
                    func_params_num :0,
                });

                self.cur_build=Some(include_build.clone());
                self.instr_pos=0;
                self.instr_end_pos=include_build.main_instruct_len;//self.get_instr_end_ind();
                return Ok(());//continue;
            }
            
            //
            Instruction::GetGlobalOrConstOrCallMethod(symbol_ind,get_global) => {
                let symbol=self.get_symbol( *symbol_ind)?;
                // let g=self.var_scope.get(&symbol);
                // let g=g.or_else(|e|Err(MachineError::from_machine(&self, e.error_type)));

                // let g=if *get_global {
                //     self.var_scope.get(&symbol).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?
                // } else {
                //     None
                // };

                // if let Some(v)=g 
                if let Some(v)=self.inner_global_get(symbol.as_str(),*get_global)? {
                    self.set_result_val(v);
                } else if let Some(v)=self.constant_get(&symbol) { //self.lib_scope.get_constant(&symbol) 
                    self.set_result_val(v);
                } else if let Some(x)=self.get_method(symbol.as_str(), 0) {
                    self.debugger.add_func_name(&symbol.as_str());
                    self.inner_call_bound_func(0, x)?;
                } else {
                    return Err(MachineError::from_machine(self, MachineErrorType::MethodOrGlobalVarNotFound(symbol.to_string()) ));
                }

            }

            Instruction::CallMethod(symbol_ind, params_num) => {
                let params_num =*params_num;
                let symbol=self.get_symbol( *symbol_ind)?;

                if let Some(x)=self.get_method(symbol.as_str(), params_num) {
                    self.debugger.add_func_name(&symbol.as_str());
                    self.inner_call_bound_func(params_num, x)?;
                } else {
                    let param_types=self.get_stack_param_types(params_num);
                    return Err(MachineError::from_machine(self, MachineErrorType::MethodNotFound(symbol.to_string(),param_types) ));
                }
            }
            
            Instruction::TryCallMethod(symbol_ind, params_num) => {
                let params_num =*params_num;
                let symbol=self.get_symbol( *symbol_ind)?;

                if let Some(x)=self.get_method(symbol.as_str(), params_num) {
                    self.debugger.add_func_name(&symbol.as_str());
                    self.inner_call_bound_func(params_num, x)?;
                } else {
                    //println!("@@@@@ try call method");
                    self.stack_pop_amount(params_num)?;
                }
            }
            
            Instruction::CallGlobalOrMethod(symbol_ind, params_num)  => {
                let params_num =*params_num;
                let symbol=self.get_symbol(*symbol_ind)?;

                if let Some(v)=&self.var_scope.get(&symbol)
                    .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?
                {
                    self.debugger.add_func_name(symbol.as_str());
                    self.inner_call_value(params_num,v,false)?;
                    return Ok(()); //continue;
                } else if let Some(x)=self.get_method(symbol.as_str(), params_num) {
                    self.debugger.add_func_name(symbol.as_str());
                    self.inner_call_bound_func(params_num, x)?; //,symbol.clone()
                } else {
                    
                    let param_types=self.get_stack_param_types(params_num);
                    return Err(MachineError::from_machine(self, MachineErrorType::GlobalFuncOrMethodNotFound(symbol.to_string(),param_types) ));
                }
            }

            Instruction::CallResult(params_num)  => {
                let params_num=*params_num;
                let v = self.result_val.clone();
                // println!("call result");
                self.inner_call_value(params_num,&v,false)?;
                return Ok(()); //continue;
            }
            // Instruction::GetFields(params_num) => {
            //     //stk=[fields_n .. fields_0]

            // }
            // Instruction::SetFields(params_num) => {
            //     //stk=[tovalm,fields_n .. fields_0]
                
            // }
        }

        self.instr_pos+=1;
        self.debugger.move_instr_pos(self.instr_pos);

        Ok(())
    }

    fn stack_frame_pop(&mut self) -> Result<bool,MachineError> {
        if let Some(stack_frame)=self.stack_frames.pop() {
            self.debugger.pop_frame();

            self.cur_build=stack_frame.ret_build;

            self.instr_pos=stack_frame.ret_instr_ind;
            self.debugger.move_instr_pos(self.instr_pos);

            self.instr_end_pos=stack_frame.ret_instr_end;
            // println!("===stack_frame popping! {} {}",self.stack.len(),stack_frame.stack_params_num);
            //println!("@@@@@ stackframe pop");

            if stack_frame.stack_params_num>0 { //what is stack_params_num?
                self.stack_pop_amount(stack_frame.stack_params_num)?; //+captures_num
            }

            Ok(stack_frame.finish)
        } else {
            Ok(true)
        }
    }

    fn copy_val(&mut self, v:Value) -> Result<Value,MachineError> {
        if !v.is_custom_any() { //should only use copy on customs?
            return Ok(v);
        }

        Ok(self.try_call_method("copy", vec![v.clone()])?.unwrap_or(v))
    }

    fn try_copy_val(&mut self, v:Value) -> Result<Option<Value>,MachineError> {
        if !v.is_custom_any() { //should only use copy on customs?
            return Ok(None);
        }

        Ok(self.try_call_method("copy", vec![v.clone()])?)
    }
    fn get_symbol(&self,symbol_ind:usize) -> Result<StringT,MachineError> {
        //src,path
        if let Some(symbol) = self.cur_build.as_ref().unwrap().symbols.get(symbol_ind) {
            Ok(symbol.clone())
        } else {
             Err(MachineError::from_machine(self, MachineErrorType::MissingSymbol(symbol_ind) ))
        }
    }

    fn set_result_val(&mut self, v:Value) {
        // let old_gc_ind=v.gc_index();

        self.result_val=v.clone_root();
        self.debugger.set_result_val();
        
        // if let Some(gc_ind) = old_gc_ind {
        // self.gc_scope.check_remove(gc_ind,self.gc_check_remove_recursive);
        // }

        self.gc_scope.remove_norefs();

    }
    pub fn get_stack_val(&self,stack_ind:usize) -> Result<Value,MachineError> {
        let stack_len = self.stack.len();

        if stack_ind >= stack_len {
            return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(stack_ind-stack_len) ));
        }

        Ok(self.stack.get(stack_ind).unwrap().clone())
    }
    

    pub fn set_stack_val(&mut self,stack_ind:usize, v:Value) -> Result<(),MachineError> {
        let stack_len = self.stack.len();

        if stack_ind >= stack_len {
            return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(stack_ind-stack_len) ));
        }

        let s=self.stack.get_mut(stack_ind).unwrap();
        *s=v.clone_root();
            
            
        self.debugger.set_stack_val_none(stack_ind);

        Ok(())
    }
    
    fn set_stack_offset_val(&mut self,stack_offset_ind:usize, v:Value) -> Result<(),MachineError> {
        let stack_len = self.stack.len();

        if stack_offset_ind >= stack_len {
            return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(stack_offset_ind) ));
        }

        let stack_ind=stack_len - stack_offset_ind - 1;

        let s=self.stack.get_mut(stack_ind).unwrap();
        *s=v.clone_root();

        self.debugger.set_stack_val_none(stack_ind);
            
        Ok(())
    }
    
    fn push_stack_val(&mut self,v:Value) {
        self.stack.push(v.clone_root());
        self.debugger.push_stack_val();
    }

    // fn get_stack_value(&self,stack_ind:usize) -> Result<&Value,MachineError> {
    //     let stack_len = self.stack.len();

    //     if stack_ind < stack_len {
    //         Ok(self.stack.get(stack_ind).unwrap())
    //     } else {
    //         let stack_offset_ind=stack_ind-stack_len;
    //         Err(MachineError::machine_new(self, MachineErrorType::InvalidStackAccess(stack_offset_ind) ))
    //     }
    // }

    fn get_stack_offset_value(&self,stack_offset_ind:usize) -> Result<&Value,MachineError> {
        let stack_len = self.stack.len();

        if stack_offset_ind < stack_len {
            Ok(self.stack.get(stack_len - stack_offset_ind - 1).unwrap())
        } else {
            Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(stack_offset_ind) ))
        }
    }
    
    // fn stack_value_mut(&mut self,stack_offset_ind:usize) -> Result<&mut Value,MachineError> {
    //     let stack_len = self.stack.len();

    //     if stack_offset_ind < stack_len {
    //         Ok(self.stack.get_mut(stack_len - stack_offset_ind - 1).unwrap())
    //     } else {
    //         Err(MachineError::machine_new(self,  MachineErrorType::InvalidStackAccess(stack_offset_ind) ))
    //     }
    // }

    fn stack_pop_amount(&mut self,amount:usize) -> Result<(),MachineError> {
        let stack_len = self.stack.len();

        if amount > stack_len {
            Err(MachineError::from_machine(self,  MachineErrorType::InvalidStackAccess(amount )))
        } else if amount==0 {
            Ok(())
        } else {
            let n=stack_len-amount;

            // println!("======= stack {:?}",self.stack);

            // let gc_inds=self.stack.drain(n..).filter_map(|x|x.gc_index()).collect::<Vec<_>>();

            // for gc_ind in gc_inds {
            //     self.gc_scope.check_remove(gc_ind,self.gc_check_remove_recursive);
            // }

            // for v in self.stack.drain(n..).rev() {
            //     if let Some(v.gc_index()
            // }

            self.stack.truncate(n);
            
            self.gc_scope.remove_norefs();
            self.debugger.pop_stack_val_amount(amount);
            Ok(())
        }
    }

    fn inner_call_bound_func(&mut self, params_num : usize, bound_func : Method<'c,X>) -> Result<(),MachineError> {
        // println!("m {} {:?}",self.debugger.last_func_name(),bound_func.args_path);
        
        self.debugger.push_frame_bound_func(params_num, &self.stack, &self.result_val);
        
        //
        for (param_ind,arg) in bound_func.args_path.iter().enumerate() {
            match arg {
                Arg::CustomRef(_)|Arg::CustomAnyRef => {}
                _=> {
                    if let Some(v)=self.try_copy_val(self.stack_param_get(params_num, param_ind))? {
                        self.stack_param_set(params_num, param_ind, v);
                    }
                }
            }
        }

        //
        let v=match bound_func.method_type {
            MethodType::Static(x)=>{
                x(FuncContext2::new(self,params_num))
            }
            MethodType::Temp(x) => {
                if let Some(mut x)=x.try_lock() {
                    x(FuncContext2::new(self,params_num))
                } else {
                    Err(MachineError::from_machine(self, MachineErrorType::FuncBorrowMutError))
                }
            }

            MethodType::StaticExt(x)=>{
                x(FuncContext::new(self,params_num))
            }
            MethodType::TempExt(x) => {
                if let Some(mut x)=x.try_lock() {
                    x(FuncContext::new(self,params_num))
                } else {
                    Err(MachineError::from_machine(self, MachineErrorType::FuncBorrowMutError))
                }
            }
        };

        match v {
            Ok(v)=>{

                self.set_result_val(v);
                //println!("@@@@@ bound func");
                
                self.stack_pop_amount(params_num)?;
            
                self.debugger.pop_frame();
        
                self.gc_scope.remove_norefs(); //hmm called already with set_result? and possibly on stack_pop_amount(params_num>0)

                Ok(())
            }
            Err(e) => {
                if e.build.is_none() {
                    Err(MachineError::from_machine(&self, e.error_type))
                } else {
                    Err(e)
                }
            }
        }


    }


    fn inner_call_build_func(&mut self, stack_params_num:usize, build:BuildT, func_ind:usize, finish:bool) -> Result<(),MachineError> {

        let stack_params_start=self.stack.len()-stack_params_num; //todo error check?
        self.debugger.push_frame_build_func(build.clone(), func_ind, stack_params_num, &self.stack, &self.result_val);

        let func = build.functions.get(func_ind).unwrap();

        //copy params
        for stack_ind in stack_params_start..stack_params_start+stack_params_num {
            let v=self.copy_val(self.stack.get(stack_ind).unwrap().clone())?;            
            // *self.stack_mut().get_mut(stack_ind).unwrap()=v;

            self.set_stack_val(stack_ind, v)?;
        }

        //fill missing params with nil
        if stack_params_num < func.params_num {
            let dif = func.params_num - stack_params_num;
            self.stack.splice(stack_params_start .. stack_params_start,std::iter::repeat(Value::Nil).take(dif));
            self.debugger.stack_insert_none(stack_params_start, dif);
        }

        self.stack_frames.push(StackFrame {
            ret_build:self.cur_build.clone(),
            ret_instr_ind: self.instr_pos+ (if finish {0}else{1}),
            ret_instr_end:self.instr_end_pos,
            stack_params_num:if stack_params_num<func.params_num{func.params_num}else{stack_params_num},
            finish,
            stack_params_start,
            func_params_num :func.params_num,
        });

        self.cur_build=Some(build.clone());
        self.instr_pos=func.instruct_start_pos;
        self.instr_end_pos= func.instruct_start_pos+func.instruct_len;//self.get_instr_end_ind();

        Ok(())
    }

    fn inner_call_value(&mut self, params_num:usize, v:&Value, finish:bool) -> Result<(),MachineError> {
        if v.is_custom::<Closure>() {
            
            let data=v.as_custom().data_clone::<Closure>()?;

            // let data=v.as_custom().data();
            // let data=data.borrow_mut::<Closure>()
            //     .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;

            let func_build=data.build.clone();
            let func_ind=data.func_ind;
            
            let params_num = params_num+data.captures.len();     

            for x in data.captures.iter() {
                self.push_stack_val(x.clone());
                self.debugger.set_stack_from_last();
            }

            self.inner_call_build_func(params_num, func_build, func_ind, finish)?;

            Ok(())
        } else {
            // println!("v is {v:?}");
            Err(MachineError::from_machine(self,  MachineErrorType::ValueNotAFunc(v.type_string()) ))
        }
    }

    fn get_stack_param_types(&self,params_num : usize) -> Vec<String> {
        self.stack[self.stack.len() - params_num .. self.stack.len()].iter().rev()
            .map(|x|x.type_string())
            .collect::<Vec<_>>()
    }

    // fn get_method_with_params<'x,I:IntoIterator<Item=&'x Value>>(&self, method_name : &str, params : I) -> Option<BoundFuncType<'c>> {
    //     self.lib_scope.get_method(method_name,params,self.var_scope)
    // }
    
    fn stack_push_params<I:AsRef<[Value]>>(&mut self,params : I) -> Result<usize,MachineError> {
        // let params=params.into_iter().collect::<Vec<_>>();
        let params=params.as_ref().to_vec();
        let params_num=params.len();
        self.stack.extend(params.into_iter().rev());
        self.debugger.stack_extend_none(params_num);
        Ok(params_num)
    }
    // fn stack_push_params(&mut self,params : &Vec<Value>) -> Result<usize,MachineError> {
    //     let mut params_num=0;

    //     for p in params.into_iter().rev() {
    //         self.stack.push(p.clone_to_root());
    //         params_num+=1;
    //     }

    //     Ok(params_num)
    // }

    fn stack_params_iter(&self, params_num : usize) -> impl DoubleEndedIterator<Item=&Value> {
        let params_start = self.stack.len()-params_num;
        let params = self.stack[params_start..].iter().rev();
        params
    }

    fn stack_param_get(&self, params_num : usize, param_ind : usize) -> Value {
        let params_start = self.stack.len()-params_num;
        let stack_ind = params_start + params_num - param_ind - 1;
        self.stack.get(stack_ind).unwrap().clone()
    }
    fn stack_param_set(&mut self, params_num : usize, param_ind : usize, value:Value) {
        let params_start = self.stack.len()-params_num;
        let stack_ind = params_start + params_num - param_ind - 1;

        self.set_stack_val(stack_ind, value).unwrap();
        // *self.stack.get_mut(stack_ind).unwrap()=value;
    }
    // fn stack_params_iter_mut(&mut self, params_num : usize) -> impl Iterator<Item=&mut Value> {
    //     let params_start = self.stack.len()-params_num;
    //     let params = self.stack[params_start..].iter_mut().rev();
    //     params
    // }
    fn get_method(&self, method_name : &str, params_num : usize) -> Option<Method<'c,X>> {
        self.lib_scope.get_method(method_name,self.stack_params_iter(params_num),
        // self.var_scope
        )
    }
    

    pub fn run_build(&mut self, build:&BuildT) -> Result<Value,MachineError> {
        //allow this to be run from func_context?

        if self.error_state {
            self.clear();
        }

        //
        self.stack_frames.push(StackFrame { 
            ret_build: self.cur_build.clone(), 
            ret_instr_ind: self.instr_pos, 
            ret_instr_end: self.instr_end_pos, 
            finish: true, 
            stack_params_start: self.stack.len(), 
            stack_params_num: 0, 
            func_params_num: 0, 
        });

        self.debugger.push_frame_main(build.clone());

        //
        self.cur_build=Some(build.clone());
        self.instr_pos=0;
        self.instr_end_pos = build.main_instruct_len;

        self.result_val = Value::Nil;
        // self.stack.clear();
        // self.stack_frames.clear();


        //
        self.run()?;
        // let result_val= self.result_val.clone_root();
        let result_val= self.copy_val(self.result_val.clone())?.clone_root();

        //
        // self.debugger.pop();
        self.stack_frame_pop()?;

        // self.cur_build=None;

        Ok(result_val)
    }
    
    pub fn call_method<I:AsRef<[Value]>>(&mut self,name:&str,params : I) -> Result<Value,MachineError> {
        if self.error_state {
            self.clear();
        }

        //
        let params_num=self.stack_push_params(params)?;     

        // println!("== call_method {name:?}", );
        if let Some(func)=self.get_method(name,params_num) {
            // self.debugger.add_func_name("call");
            self.debugger.add_func_name(name);
            self.inner_call_bound_func(params_num, func)?; 
            Ok(self.result_val.clone())
        } else {
            let param_types=self.get_stack_param_types(params_num);
            Err(MachineError::from_machine(self, MachineErrorType::MethodNotFound(name.to_string(),param_types)))
        }
    }

    pub fn try_call_method<I:AsRef<[Value]>>(&mut self,name:&str,params : I) -> Result<Option<Value>,MachineError> {
        if self.error_state {
            self.clear();
        }

        //
        let params_num=self.stack_push_params(params)?;

        // if let Some(x)=self.lib_scope.get_method(name,params.into_iter(),self.var_scope)
        if let Some(x)=self.get_method(name, params_num)
        {
            // let params_num=self.stack_push_params(params)?;
            let symbol = StringT::new(name);

            self.debugger.add_func_name(&symbol.as_str());
            self.inner_call_bound_func(params_num, x)?; //,symbol.clone()
            Ok(Some(self.result_val.clone()))
        } else {
            //println!("@@@@@ trycall method {name:?}");
            self.stack_pop_amount(params_num)?;
            Ok(None)
        }
    }
    pub fn call_value<I:AsRef<[Value]>>(&mut self,v:&Value,params : I) -> Result<Value,MachineError> {
        if self.error_state {
            self.clear();
        }

        // self.debugger.push_frame_main(build.clone());
        //
        let params_num=self.stack_push_params(params)?;

        self.inner_call_value(params_num, v, true)?;
        self.run()?;

        // self.debugger.pop_frame();

        Ok(self.result_val.clone())
    }
    pub fn try_call_global<I:AsRef<[Value]>>(&mut self,name:&str,params : I) -> Result<Option<Value>,MachineError> {
        if self.error_state {
            self.clear();
        }
        
        //
        let params_num=self.stack_push_params(params)?;

        if let Some(v)=&self.var_scope.get(name)
            .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?
        {
            if self.stack_frames.len()==0 {
                //self.stack_frames.push();

            }

            self.debugger.add_func_name(name);
            self.inner_call_value(params_num,v,true)?;
            self.run()?;

        } else if let Some(x)=self.get_method(name, params_num) {
            self.debugger.add_func_name(name);
            self.inner_call_bound_func(params_num, x)?; //,symbol.clone()
        } else {
            return Ok(None);
        }

        Ok(Some(self.result_val.clone()))
    }

    pub fn call_global<I:AsRef<[Value]>>(&mut self,name:&str,params : I) -> Result<Value,MachineError> {
        // let params=params.into_iter().collect::<Vec<_>>();
        let params=params.as_ref().to_vec();
        let params_num=params.len();

        if let Some(r)=self.try_call_global(name, params)? {
            Ok(r)
        } else {
            let param_types=self.get_stack_param_types(params_num); //params pushed on stack by try_call_global?
            Err(MachineError::from_machine(self, MachineErrorType::GlobalFuncOrMethodNotFound(name.to_string(),param_types) ))
        }
    }

    pub fn global_decl(&mut self,name:&str,to_value:Option<Value>) -> Result<(),MachineError> {
        if self.error_state { //why?
            self.clear();
        }

        //
        self.var_scope.decl(name, to_value)
            .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
        Ok(())
    }
    
    pub fn global_set<T:AsRef<[Value]>>(&mut self,name:&str,fields:T,to_value:Value) -> Result<(),MachineError> {
        if self.error_state { //why?
            self.clear();
        }

        //
        let fields = fields.as_ref().to_vec();

        if fields.len()==0 { //set global to the value  
            if !self.var_scope.set(name,to_value)
                .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?
            {
                return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(name.to_string()) ));
            }
            // let Some(global_val) = self.var_scope.get_mut(name) else {
            //     return Err(MachineError::machine_new(self, MachineErrorType::GlobalNotFound(name.to_string()) ));
            // };

            // *global_val=to_value.clone_root();            
        } else {
            let Some(global_val) = self.var_scope.get(name)
                .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?
            else {
                return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(name.to_string()) ));
            };

            self.value_set(global_val, fields, to_value)?;            
        }
        
        return Ok(());
    }
    
    pub fn global_get<T:AsRef<[Value]>>(&mut self,name:&str,fields:T) -> Result<Value,MachineError> {
        if self.error_state { //why?
            self.clear();
        }

        //
        let Some(global_val) = self.var_scope.get(name)
            .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?
        else {
            return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(name.to_string()) ));
        };
        
        self.value_get(global_val, fields)
    }

    pub fn value_set<T:AsRef<[Value]>>(&mut self,value:Value,fields:T,to_value:Value) -> Result<(),MachineError> {
        if self.error_state { //why?
            self.clear();
        }

        //
        let fields = fields.as_ref().to_vec();

        if fields.len() == 0 {
            return Ok(()); //no fields provided, do nothing
        }

        // let Some(last_field)=fields.pop() else {
        //     return Ok(()); //no fields provided, do nothing
        // };

        // // let value2=self.value_get(value, fields)?;
        // let value2=self.inner_value_get(value,fields,false)?;

        // self.call_method("set_field", [value2,last_field,to_value])?;

        //
        let mut rets = vec![value.clone()];

        //gets
        for i in 0..fields.len()-1 {
            let ret=self.call_method("get_field", [rets.last().unwrap().clone(),fields.get(i).unwrap().clone()])?;
            rets.push(ret);
        }

        //
        rets.push(to_value);

        //sets
        for i in (0..rets.len()-1).rev() {
            self.call_method("set_field", [
                rets.get(i).unwrap().clone(),
                fields.get(i).unwrap().clone(),
                rets.get(i+1).unwrap().clone(),
            ])?;
        }

        //
        Ok(())
    }

    pub fn value_get<T:AsRef<[Value]>>(&mut self,value:Value,fields:T) -> Result<Value,MachineError> {
        if self.error_state { //why?
            self.clear();
        }
        
        //
        self.inner_value_get::<T>(value,fields) //,true
    }

    fn inner_value_get<T:AsRef<[Value]>>(&mut self,value:Value,fields:T
        // ,b:bool
    ) -> Result<Value,MachineError> {
        let mut fields = fields.as_ref().to_vec();
        fields.reverse();
        
        let mut cur_value=value;

        while let Some(field)=fields.pop() {
            // let is_end= fields.len()==0;
            // let m=if b&&is_end{"get_field"}else{"ret_field"};
            cur_value = self.call_method("get_field", [cur_value.clone(),field])?;
        }

        Ok(cur_value)
    }

    pub fn constant_get(&self,n:&str) -> Option<Value> {
        // self.const_scope.as_ref()
        //     .and_then(|c|c(n))
        //     .or_else(||self.lib_scope.get_constant(n))

        // self.lib_scope.get_constant(n)

        self.const_scope.get(n).cloned().or_else(||self.lib_scope.get_constant(n))
    }
    pub fn inner_global_get(&self,n:&str,enabled:bool) -> Result<Option<Value>,MachineError> {
        if !enabled {
            return Ok(None);
        }


        self.var_scope.get(&n).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))
    }
    // pub fn custom_managed_value<T:GcTraversable>(&mut self,data : T) -> Value {
    //     Value::new_custom_managed(data, self.gc_scope)
    // }
}


