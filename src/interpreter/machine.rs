/*
TODO
* allow execution to be paused
* * for methods, if calling build func, how to pause?
* * * could return pause related data in context.call() return,
* * * * have a way to store that, and addtional custom user data eg for loop index
* * * * then when resuming, recall the method, providing the pause and user data

* for not enough params provided to build func call, it pushes missing nil param onto stack
* for method call, with optional params missing ... ?

* have a runtime type resolving for var decls
* * eg var a {vec3 0.0}; set a {some_method}; #where some_method()->vec3
* * methods can optionally specify a ret type, err check if ret matches specified type?
* * not viaable? eg method1(method2()) #cant specify ret for method2, have to be any

* instead of pushing missing params on stk for func calls,
** add instr eg Param(usize), that will return the correct val from the stk or nil if param missing

TODO
* want an error for setting a var to void
** put checks on set global var, local var, local var deref
** because for loop macro, stores void, and might be replaced with last val used, added allow_void to set stk instrs
*** can't rely on result_val because it is ued to calc for loop index
*** could push last val on stack, do ind calc, and then pop last val back onto result val? messy
*/

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use super::super::common::*;
// use super::custom::*;
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

    pub finish : bool, //for when calling machine.call(), ends after that call, otherwise if called from within machine, then don't want it to finish ...
    pub stack_params_start : usize,

    pub stack_params_num : usize,
    pub func_params_num : usize,

    // pub ret_stack_size : usize,
}

pub struct Machine<'a,X> { //,'b
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
    lib_scope : &'a LibScope<X>, //<'c> //<'b>
    includer : Option<Box<dyn FnMut(&Path) -> Option<BuildT> + 'a>>, //can use lifetime a for some reason?
    const_scope:HashMap<&'a str,Value>,
    core_val : &'a mut X, //&'a mut
    // core_empty : (), //&'a mut

    stack_limit:usize,
}


// impl<'a,X> Machine<'a,&mut X> {
//     pub fn core_mut(&mut self) -> &mut X {
//         self.core_val
//     }
//     pub fn core(&self) -> &X {
//         self.core_val
//     }
// }

// impl<'a,X> Machine<'a,&X> {
//     pub fn core(&self) -> &X {
//         self.core_val
//     }
// }


// impl<'a> Machine<'a,()> {
//     pub fn new2 (
//         gc_scope : &'a mut GcScope,
//         var_scope : &'a mut VarScope,
//         lib_scope : &'a LibScope<()>,
//     ) -> Self
//     {
//         // let mut core=();

//         Self::new(gc_scope,var_scope,lib_scope,&mut self core)
//     }
// }

impl<'a,X> Machine<'a,X> {
    // pub fn get_core_mut(&mut self) -> &mut X {
    //     // &mut
    //     self.core_val
    // }
    // pub fn get_core(& self) -> &X {
    //     // &
    //     self.core_val
    // }
    pub fn core_mut(&mut self) -> &mut X {
        self.core_val
    }
    pub fn core(& self) -> &X {
        self.core_val
    }

    pub fn new (
        gc_scope : &'a mut GcScope,
        lib_scope : &'a LibScope<X>,
        var_scope : &'a mut VarScope,
        core_val :  &'a mut X,//&'a mut X,
    ) -> Self
    {
        let debugger = Debugger::new(); //debugger_enabled,debugger_print

        // debugger.set_enabled(false);

        Self {
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
            includer : None, //Box::new(include_resolver),
            const_scope:HashMap::new(),
            core_val,
            // core_empty:(),
            stack_limit:1024,
        }
    }

    pub fn set_debug_print(&mut self,enabled:bool) {
        self.debugger.set_print(enabled);
    }
    pub fn set_debug_print_simple(&mut self,enabled:bool) {
        self.debugger.set_print_simple(enabled);
    }
    pub fn set_debug(&mut self,enabled:bool) {
        self.debugger.set_enabled(enabled);
    }
    pub fn debugger(&self) -> &Debugger {
        &self.debugger
    }
    pub fn debug_print_stack_trace(&self, skip_first:bool) {
        self.debugger.print_stack_trace(skip_first);
    }
    pub fn debug_print_stack(&self) {
        self.debugger.print_stack(&self.stack);
    }
    pub fn debug_print_state(&self) {
        println!("machine state:");
        println!("\tcur_build = {:?}",self.cur_build);
        println!("\tinstr_pos = {:?}",self.instr_pos);
        println!("\tinstr_end_pos = {:?}",self.instr_end_pos);
        println!("\tresult_val = {:?}",self.result_val());
        println!("\tstack = {:?}",self.stack);
        println!("\tstack_frames = {:?}",self.stack_frames);
        println!("\terror_state = {:?}",self.error_state);
    }

    pub fn clear(&mut self) {
        self.cur_build=None;
        self.instr_pos=0;
        self.instr_end_pos=0;

        self.set_result_val(Value::Nil);

        self.stack.clear();
        self.stack_frames.clear();

        self.error_state=false;

        self.debugger.clear();
    }

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
    pub fn stack_frames(&self) -> &Vec<StackFrame> {
        &self.stack_frames
    }
    pub fn result_val(&self) -> Value {
        self.result_val.clone_root()
    }

    fn stack_frame_pop(&mut self) -> Result<bool,MachineError> {
        if let Some(stack_frame)=self.stack_frames.pop() {
            self.debugger.pop_frame();
            self.cur_build=stack_frame.ret_build;
            self.instr_pos=stack_frame.ret_instr_ind;
            self.debugger.move_instr_pos(self.instr_pos);
            self.instr_end_pos=stack_frame.ret_instr_end;

            if stack_frame.stack_params_num>0 { //what is stack_params_num?
                self.stack_pop_amount(stack_frame.stack_params_num)?; //+captures_num
            }

            Ok(stack_frame.finish)
        } else {
            Ok(true)
        }
    }

    fn stack_pop_amount(&mut self,amount:usize) -> Result<(),MachineError> {
        let stack_len = self.stack.len();

        if amount > stack_len {
            Err(MachineError::from_machine(self,  MachineErrorType::InvalidStackAccess(amount )))
        } else if amount==0 {
            Ok(())
        } else {
            self.stack.truncate(stack_len-amount);
            // println!("hmm {:?}",self.result_val);
            // self.gc_scope.remove_norefs();

            self.debugger.pop_stack_val_amount(amount);
            Ok(())
        }
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
        self.result_val=v.clone_root();
        self.debugger.set_result_val();
        // self.gc_scope.remove_norefs();
    }

    pub fn get_stack_val(&self,stack_ind:usize) -> Result<Value,MachineError> {
        let stack_len = self.stack.len();

        if stack_ind >= stack_len {
            return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(stack_ind-stack_len) ));
        }

        Ok(self.stack.get(stack_ind).unwrap().clone_root())
    }

    fn set_stack_val(&mut self,stack_ind:usize, v:Value) -> Result<(),MachineError> {
        let stack_len = self.stack.len();

        if stack_ind >= stack_len {
            return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(stack_ind-stack_len) ));
        }

        *self.stack.get_mut(stack_ind).unwrap()=v.clone_root();
        self.debugger.set_stack_val_none(stack_ind);
        // self.gc_scope.remove_norefs();
        Ok(())
    }

    fn set_stack_offset_val(&mut self,stack_offset_ind:usize, v:Value) -> Result<(),MachineError> {
        let stack_len = self.stack.len();

        if stack_offset_ind >= stack_len {
            return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(stack_offset_ind) ));
        }

        let stack_ind=stack_len - stack_offset_ind - 1;
        *self.stack.get_mut(stack_ind).unwrap()=v.clone_root();
        self.debugger.set_stack_val_none(stack_ind);

        // self.gc_scope.remove_norefs();
        Ok(())
    }

    fn push_stack_val(&mut self,v:Value) -> Result<(),MachineError> {

        // if v.is_void() { //not necessary?
        //     // println!("v1");
        //     return Err(MachineError::from_machine(self, MachineErrorType::VoidNotExpr));
        // }

        if self.stack.len()+1>self.stack_limit{
            return Err(MachineError::from_machine(self, MachineErrorType::StackLimitReached(self.stack.len()+1)));
        }

        self.stack.push(v.clone_root());
        self.debugger.push_stack_val();
        Ok(())
    }

    fn get_stack_offset_value(&self,stack_offset_ind:usize) -> Result<Value,MachineError> {
        let stack_len = self.stack.len();



        if stack_offset_ind < stack_len {
            let stack_ind=stack_len - stack_offset_ind - 1;

            Ok(self.stack.get(stack_ind).unwrap().clone_root())
        } else {
            Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(stack_offset_ind) ))
        }
    }

    fn stack_params_iter(&self, params_num : usize) -> impl DoubleEndedIterator<Item=&Value> {
        // println!("=== {} {}",self.stack.len(),params_num);
        let params_start = self.stack.len()-params_num;
        self.stack[params_start..].iter().rev()
    }

    pub fn constant_get(&self,n:&str) -> Option<Value> {
        self.const_scope.get(n).map(|x|x.clone_root()).or_else(||self.lib_scope.get_constant(n))
    }

    fn get_method(&self, method_name : &str, params_num : usize) -> Option<Method<X>> {
        self.lib_scope.get_method(method_name,self.stack_params_iter(params_num),)
    }

    fn get_stack_param_types(&self,params_num : usize) -> Vec<String> {
        self.stack[self.stack.len() - params_num .. self.stack.len()].iter().rev().map(|x|x.type_string()).collect()
    }

    fn inner_global_get(&self,n:&str,enabled:bool) -> Result<Option<Value>,MachineError> {
        if !enabled {
            return Ok(None);
        }

        self.var_scope.get(&n).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))
    }

    fn stack_param_get(&self, params_num : usize, param_ind : usize) -> Value { //used by bound funcs (rust funcs)
        let params_start = self.stack.len()-params_num;
        let stack_ind = params_start + params_num - param_ind - 1;
        self.stack.get(stack_ind).unwrap().clone_root()
    }
    fn stack_param_set(&mut self, params_num : usize, param_ind : usize, value:Value) { //used by bound funcs (rust funcs)
        let params_start = self.stack.len()-params_num;
        let stack_ind = params_start + params_num - param_ind - 1;
        self.set_stack_val(stack_ind, value).unwrap();
    }

    fn copy_val(&mut self, v:Value) -> Result<Value,MachineError> {
        if !v.is_custom_any() { //should only use copy on customs?
            return Ok(v);
        }

        Ok(self.inner_try_call_method_wparams("copy", vec![v.clone_root()])?.unwrap_or(v))
    }

    fn inner_try_call_method_wparams<I:AsRef<[Value]>>(&mut self,name:&str,params : I) -> Result<Option<Value>,MachineError> {
        //
        let params_num=self.stack_push_params(params)?;

        let r=self.inner_try_call_method(name,params_num)?;

        if r.is_none() {
            self.stack_pop_amount(params_num)?;
        }

        Ok(r)
    }

    fn inner_try_call_method(&mut self,name:&str,params_num : usize) -> Result<Option<Value>,MachineError> {

        // if let Some(x)=self.lib_scope.get_method(name,params.into_iter(),self.var_scope)
        if let Some(x)=self.get_method(name, params_num) {
            let symbol = StringT::new(name);
            self.debugger.add_func_name(&symbol.as_str());
            self.inner_call_bound_func(params_num, x)?; //,symbol.clone()
            Ok(Some(self.result_val()))
        } else {
            Ok(None)
        }
    }
    fn stack_push_params<I:AsRef<[Value]>>(&mut self,params : I) -> Result<usize,MachineError> {
        let params=params.as_ref().to_vec();
        let params_num=params.len();


        if self.stack.len()+params_num>self.stack_limit{
            return Err(MachineError::from_machine(self, MachineErrorType::StackLimitReached(self.stack.len()+params_num)));
        }

        self.stack.extend(params.into_iter().rev());
        self.debugger.stack_extend_none(params_num);
        Ok(params_num)
    }

    fn run(&mut self) -> Result<(),MachineError> {
        loop {
            //
            if self.instr_pos>self.instr_end_pos {
                self.debug_print_state();
                self.debugger.print_stack_trace(true);
                panic!("scriptlang, machine, instr ind ({}) incremented past end ({})",self.instr_pos,self.instr_end_pos);
            }

            //debug
            self.debugger.step(&self.stack,&self.result_val());

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
            &Instruction::GetField { is_field_symbol } => {

                // println!("hmm {self_val:?} {field_val:?}");

                let mut done=false;
                let field_val=self.get_stack_offset_value(1)?;
                let self_val=self.get_stack_offset_value(0)?;

                if is_field_symbol {
                    // let field_name=self.get_stack_offset_value(1)?.get_string().unwrap();

                    if let Some(field_name)=field_val.get_string() {
                        if let Some(x)=self.lib_scope.get_method(field_name.as_str(),[&self_val])
                            .or_else(||self.lib_scope.get_method_field_named(field_name.as_str(),[&self_val]))
                        {
                            // println!("--");
                            self.stack_swap()?; // self=>field
                            self.stack_pop_amount(1)?; //field
                            // println!("==");

                            self.debugger.add_func_name(field_name.as_str());
                            self.inner_call_bound_func(1, x)?;

                            done=true;
                        }
                    }
                }

                if !done {
                    let params_num =2;
                    // let symbol="get_field";

                    let x= if is_field_symbol {
                        self.lib_scope.get_method_field(false, [&self_val,&field_val])
                    } else {
                        self.lib_scope.get_method_field(false, [&self_val,&field_val])
                            .or_else(||self.lib_scope.get_method_field(true, [&self_val,&field_val]))
                    };

                    if let Some(x)=x
                        // self.get_method(symbol, params_num)

                    {
                        self.debugger.add_func_name("field"); //should have enum eg method(name), field,
                        self.inner_call_bound_func(params_num, x)?;
                    } else {
                        let param_types=self.get_stack_param_types(params_num);
                        return Err(MachineError::from_machine(self, MachineErrorType::FieldNotFound(
                            // symbol.to_string(),
                            param_types) ));
                    }
                }
            }
            &Instruction::SetField{is_field_symbol,is_last} => {
                let params_num =3;
                // let symbol="set_field";

                let to_val=self.get_stack_offset_value(2)?;
                let field_val=self.get_stack_offset_value(1)?;
                let self_val=self.get_stack_offset_value(0)?;

                //
                if to_val.is_void() {
                    // println!("v2");
                    return Err(MachineError::from_machine(self, MachineErrorType::VoidNotExpr));
                }

                //
                let mut done=false;

                if is_field_symbol {
                    if let Some(field_name)=field_val.get_string() {
                        if let Some(x)=self.lib_scope.get_method_field_named(field_name.as_str(),[&self_val,&to_val])
                        {
                            // println!("--");
                            self.stack_swap()?; // self=>field
                            self.stack_pop_amount(1)?; //field
                            // println!("==");

                            self.debugger.add_func_name(field_name.as_str());
                            self.inner_call_bound_func(2, x)?;

                            done=true;
                        }
                    }
                }

                if !done {
                    let x= if is_field_symbol {
                        self.lib_scope.get_method_field(false, [&self_val,&field_val,&to_val])
                    } else {
                        self.lib_scope.get_method_field(false, [&self_val,&field_val,&to_val])
                            .or_else(||self.lib_scope.get_method_field(true, [&self_val,&field_val,&to_val]))
                    };
                    // println!("hmm {:?}",self.get_stack_offset_value(1));
                    // if let Some(x)=self.get_method(symbol, 2) {
                    // }
                    // else
                    if let Some(x)=x //self.get_method(symbol, params_num)
                    {
                        self.debugger.add_func_name("field");
                        self.inner_call_bound_func(params_num, x)?;
                    } else if is_last {
                        let param_types=self.get_stack_param_types(params_num);
                        return Err(MachineError::from_machine(self, MachineErrorType::FieldNotFound(param_types) ));
                    }
                }
            }
            Instruction::Jmp{cond,instr_pos:new_instr_pos, debug} => {
                if *new_instr_pos > cur_build.instructions.len() {
                    return Err(MachineError::from_machine(self, MachineErrorType::JmpErr(*new_instr_pos)));
                }

                let b= match cond {
                    JmpCond::None => true,
                    JmpCond::True => self.result_val().as_bool(),
                    JmpCond::False => !self.result_val().as_bool(),
                    JmpCond::Undefined => self.result_val().is_undefined(),
                    JmpCond::NotUndefined => !self.result_val().is_undefined(),
                };

                if b //cond.and_then(|x|Some(x==self.result_val().as_bool())).unwrap_or(true)

                {
                    if (*new_instr_pos as i64) != (self.instr_pos as i64)+debug.1 {
                        let id=debug.0;
                        let offset=debug.1;
                        let cur=self.instr_pos;
                        println!("id={id} cur:{cur} offset:{offset}, new:{new_instr_pos}",);
                        panic!("");
                    }

                    self.instr_pos=*new_instr_pos;
                    self.debugger.move_instr_pos(self.instr_pos);

                    return Ok(());
                }
            }
            Instruction::ResultBool(v)  => {
                self.set_result_val(Value::Bool(*v));
            }
            Instruction::ResultInt(v)  => {
                self.set_result_val(Value::Int(*v));
            }
            Instruction::ResultFloat(v)  => {
                self.set_result_val(Value::Float(*v));
            }
            Instruction::ResultVoid => {
                self.set_result_val(Value::Void);
            }
            Instruction::ResultNil => {
                self.set_result_val(Value::Nil);
            }
            Instruction::ResultSymbol(symbol_ind)  => {
                self.set_result_val(Value::String(self.get_symbol( *symbol_ind)?));
            }
            Instruction::ResultVararg => {
                self.set_result_val(Value::custom_unmanaged(Vararg));
            }
            &Instruction::ResultFunc(func_ind, captures_num)  => { //todo
                let captures=self.stack_params_iter(captures_num).rev().map(|capture|capture.clone_leaf()).collect::<Vec<_>>();
                self.stack_pop_amount(captures_num)?;
                let closure=Closure{ captures, build: self.cur_build.clone().unwrap(), func_ind, };

                if captures_num==0 {
                    self.set_result_val(Value::custom_unmanaged(closure));
                } else {
                    let v=Value::custom_managed(closure, self.gc_scope);
                    self.set_result_val(v);
                }
            }

            Instruction::StackPush => {
                let v=self.result_val();
                self.push_stack_val(v)?;

                self.debugger.set_stack_from_last();
            }
            Instruction::StackDup => {
                let v = self.get_stack_offset_value(0)?;
                self.push_stack_val(v.clone_root())?;
                self.debugger.set_stack_from_last(); //what does this do?
            }
            Instruction::StackLocals(amount) => {
                for _ in 0..(*amount) {
                    self.push_stack_val(Value::Undefined)?; //have Value::Undefined ?
                    //
                    self.debugger.set_stack_from_last();
                }
            }
            Instruction::StackPop(amount) => {
                self.stack_pop_amount(*amount)?;
                self.gc_scope.remove_norefs();
            }
            Instruction::StackSwap => {
                self.stack_swap()?;
            }
            Instruction::StackRotRight => {
                self.stack_rot_right()?;
            }
            Instruction::StackRotLeft => {
                self.stack_rot_left()?;
            }
            Instruction::SetStackVar(stack_offset_ind,allow_void) => {
                let result_val=self.result_val();

                if !*allow_void && result_val.is_void() {
                    // let v=self.copy_val(result_val)?.clone_root();
                    // println!("v4 {v:?}");
                    return Err(MachineError::from_machine(self, MachineErrorType::VoidNotExpr));
                }

                let v=self.copy_val(result_val)?.clone_root(); //should be root, since on stack

                // if v.is_void() { return Err(MachineError::from_machine(self, MachineErrorType::VoidNotExpr)); }

                self.set_stack_offset_val(*stack_offset_ind, v)?;

                self.debugger.set_stack_val_offset_from_last(*stack_offset_ind);
            }
            Instruction::GetStackVar(stack_offset_ind) => {
                let v = self.get_stack_offset_value(*stack_offset_ind)?;
                self.set_result_val(v.clone_root());
            }
            Instruction::MakeStackVarRef(stack_offset_ind) => {
                let v = self.copy_val(self.get_stack_offset_value(*stack_offset_ind)?)?.clone_leaf();
                let v2=Value::custom_managed_mut(v, self.gc_scope);
                self.set_stack_offset_val(*stack_offset_ind, v2)?;
                self.debugger.set_stack_val_offset_from_last(*stack_offset_ind);
            }
            Instruction::SetStackVarDeref(stack_offset_ind, init,allow_void) => {
                let result_val=self.result_val();

                if !*allow_void && result_val.is_void() {
                    // println!("v0");
                    return Err(MachineError::from_machine(self, MachineErrorType::VoidNotExpr));
                }
                let to_val=self.copy_val(result_val.clone_leaf())?;
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
                        *v=to_val.clone_leaf();
                        Ok(())
                    }).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;

                    // if !self.var_scope.set(data.name.as_str(),val)? {}
                } else {
                    custom.with_data_mut(|data:&mut Value|{
                        if !init && data.is_undefined() {
                            Err(MachineError::from_machine(self, MachineErrorType::SetUndefinedVar ))
                        } else {
                            *data=to_val;
                            Ok(())
                        }
                    })?;


                }

            }
            Instruction::GetStackVarDeref(stack_offset_ind) => { //todo
                let custom=self.get_stack_offset_value(*stack_offset_ind)?.as_custom();

                if custom.is_type::<GlobalAccessRef>() {
                    let data=custom.data_clone::<GlobalAccessRef>()
                        .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
                    let var_data=data.var.as_custom().data_clone::<Value>()
                        .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;

                    if !var_data.is_undefined() {
                        self.set_result_val(var_data);
                    } else if let Some(v)=self.constant_get(&data.name) {
                        self.set_result_val(v);
                    } else {
                        return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(data.name.to_string()) ));
                    }
                } else {
                    custom.with_data_ref(|data:& Value|{
                        if data.is_undefined() {
                            return Err(MachineError::from_machine(self, MachineErrorType::GetUndefinedVar ));
                        }

                        let v=self.copy_val(data.clone_leaf())?; //leaf, to skip incr root count

                        self.set_result_val(v);
                        Ok(())
                    })?;
                }

            }


            Instruction::CallStackVarDeref(stack_offset_ind,params_num) => {
                let params_num=*params_num;

                let custom=self.get_stack_offset_value(*stack_offset_ind)?.as_custom();

                if custom.is_type::<GlobalAccessRef>() {
                    let data=custom.data_clone::<GlobalAccessRef>()
                        .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
                    let var_data=data.var.as_custom().data_clone::<Value>()
                        .or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;

                    if !var_data.is_undefined() {

                        self.debugger.add_func_name(data.name.as_str());

                        if self.inner_call_value(params_num,var_data,false)? {
                            return Ok(()); //continue;
                        }
                    } else if let Some(v)=self.constant_get(&data.name)  //not needed for deref? there was the option  for globals (and constants) being captured

                    {
                        //call v

                        self.debugger.add_func_name(data.name.as_str());

                        if self.inner_call_value(params_num,v,false)? {
                            return Ok(()); //continue;
                        }
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

                    if data.is_undefined() {
                        return Err(MachineError::from_machine(self, MachineErrorType::GetUndefinedVar ));
                    }

                    //
                    //let v=self.copy_val(data.clone())?;
                    let v=data;

                    if self.inner_call_value(params_num,v,false)? {
                        return Ok(()); //continue;
                    }
                }

            }
            Instruction::DeclGlobalVar(symbol_ind) => {
                let symbol=self.get_symbol( *symbol_ind)?;
                self.var_scope.decl(symbol.as_str(),None).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?;
            }

            Instruction::SetGlobalVar(symbol_ind) => {
                let symbol=self.get_symbol( *symbol_ind)?;
                // let v=self.result_val.clone_root();
                let v=self.copy_val(self.result_val())?;

                if !self.var_scope.set(&symbol,v).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))? {
                    return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(symbol.to_string()) ));
                }
            }
            Instruction::GetGlobalVarOrConst(symbol_ind, get_global) => {
                let symbol=self.get_symbol( *symbol_ind)?;

                if let Some(v)=self.inner_global_get(symbol.as_str(),*get_global)? {
                    self.set_result_val(v);
                } else if let Some(v)=self.constant_get(&symbol) {
                    self.set_result_val(v);
                } else {
                    return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(symbol.to_string()) ));
                }
            }

            &Instruction::GetGlobalVarRef(symbol_ind,  ) => {
                let symbol=self.get_symbol(symbol_ind)?;

                let refvar=self.var_scope.get_ref(&symbol,self.gc_scope);
                self.set_result_val(refvar);
            }

            &Instruction::GetGlobalAccessRef(symbol_ind,  ) => {
                let symbol=self.get_symbol(symbol_ind)?;

                let refvar=self.var_scope.get_ref(&symbol,self.gc_scope);

                let val=Value::custom_managed_mut(GlobalAccessRef{
                    name:symbol,
                    var: refvar,
                },self.gc_scope);

                self.set_result_val(val);
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

                // self.var_scope.get(&n).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))


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
                    self.stack_pop_amount(params_num)?;
                }
            }

            Instruction::CallGlobalOrMethod(symbol_ind, params_num)  => {
                let params_num =*params_num;
                let symbol=self.get_symbol(*symbol_ind)?;

                if let Some(v)=self.var_scope.get(&symbol).or_else(|e: MachineError|Err(MachineError::from_machine(&self, e.error_type)))?
                {
                    self.debugger.add_func_name(symbol.as_str());

                    if self.inner_call_value(params_num,v,false)? {
                        return Ok(()); //continue;
                    }
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
                let v = self.result_val();
                if self.inner_call_value(params_num,v,false)? {
                    return Ok(()); //continue;
                }
            }
        }

        self.instr_pos+=1;
        self.debugger.move_instr_pos(self.instr_pos);

        Ok(())
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

        self.set_result_val(Value::Nil);
        // self.stack.clear();
        // self.stack_frames.clear();


        //
        self.run()?;
        // let result_val= self.result_val.clone_root();
        let result_val= self.copy_val(self.result_val())?;

        //
        // self.debugger.pop();
        self.stack_frame_pop()?;

        // self.cur_build=None;

        Ok(result_val)
    }


    fn inner_call_bound_func(&mut self, params_num : usize, bound_func : Method<X>) -> Result<(),MachineError> {

        self.debugger.push_frame_bound_func(params_num, &self.stack, &self.result_val());

        //
        for (param_ind,arg) in bound_func.args_path.iter().enumerate() {
            if arg.is_custom_not_ref() {
                let v=self.copy_val(self.stack_param_get(params_num, param_ind))?;
                self.stack_param_set(params_num, param_ind, v);
            }

            // match arg {
            //     Arg::Custom(_)|Arg::CustomAny| Arg::CustomMut(_)|Arg::CustomAnyMut => {

            //     }
            //     Arg::CustomRef(_)|Arg::CustomAnyRef |Arg::CustomMutRef(_)|Arg::CustomAnyMutRef => {
            //     }
            //     _=> {
            //     }
            // }
        }

        //
        let v=match bound_func.method_type {
            MethodType::NonMut(x)=>{
                x(FuncContext::new(self,params_num))
            }
            MethodType::Mut(x) => {
                if let Ok(mut x)=
                    // x.lock()
                    x.try_lock()
                {
                    x(FuncContext::new(self,params_num))
                } else {
                    Err(MachineError::from_machine(self, MachineErrorType::FuncBorrowMutError))
                }
            }
        };

        match v {
            Ok(v)=>{

                self.set_result_val(v);

                self.stack_pop_amount(params_num)?;

                self.debugger.pop_frame();

                // self.gc_scope.remove_norefs(); //hmm called already with set_result? and possibly on stack_pop_amount(params_num>0)

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
        self.debugger.push_frame_build_func(build.clone(), func_ind, stack_params_num, &self.stack, &self.result_val());

        let func = build.functions.get(func_ind).unwrap();

        //copy params
        for stack_ind in stack_params_start..stack_params_start+stack_params_num {

            let v=self.copy_val(self.get_stack_val(stack_ind)?)?;
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

    fn inner_call_value(&mut self, params_num:usize, v:Value, finish:bool) -> Result<bool,MachineError> {
        // println!("~== {params_num} : {}",self.stack.len());
        if v.is_custom::<Closure>() {

            let data=v.as_custom().data_clone::<Closure>()?;

            let func_build=data.build.clone();
            let func_ind=data.func_ind;

            let params_num = params_num+data.captures.len();

            for x in data.captures.iter() {
                self.push_stack_val(x.clone_root())?;
                self.debugger.set_stack_from_last();
            }

            self.inner_call_build_func(params_num, func_build, func_ind, finish)?;

            Ok(true)
        } else {
            // self.stack_push_params([v.clone_root()])?;
            self.push_stack_val(v.clone_root())?;
            self.debugger.set_stack_from_last();

            // println!("~=== {params_num} : {}",self.stack.len());
            if let Some(x)=self.get_method("call", params_num+1) {
                self.debugger.add_func_name("call");
                self.inner_call_bound_func(params_num+1, x)?;

                //
                self.instr_pos+=1;
                self.debugger.move_instr_pos(self.instr_pos);

                //
                Ok(true)
            } else {
            // }

            // if let Some(_)=self.inner_try_call_method("call",params_num+1)? {
            //     Ok(true)
            // } else {
                self.stack_pop_amount(1)?;
                // println!("noo {params_num}");
                Err(MachineError::from_machine(self,  MachineErrorType::ValueNotAFunc(v.type_string()) ))
            }
        }
    }


    ////////


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
            Ok(self.result_val())
        } else {
            let param_types=self.get_stack_param_types(params_num);
            Err(MachineError::from_machine(self, MachineErrorType::MethodNotFound(name.to_string(),param_types)))
        }
    }




    pub fn try_call_method<I:AsRef<[Value]>>(&mut self,name:&str,params : I) -> Result<Option<Value>,MachineError> {
        if self.error_state {
            self.clear();
        }

        Ok(self.inner_try_call_method_wparams(name,params)?.map(|x|x.clone_leaf()))
    }

    pub fn call_value<I:AsRef<[Value]>>(&mut self,v:Value,params : I) -> Result<Value,MachineError> {
        if self.error_state {
            self.clear();
        }

        // self.debugger.push_frame_main(build.clone());
        //
        let params_num=self.stack_push_params(params)?;
        if self.inner_call_value(params_num, v, true)? {
            self.run()?;
        }

        // self.debugger.pop_frame();

        Ok(self.result_val())
    }

    pub fn try_call_global<I:AsRef<[Value]>>(&mut self,name:&str,params : I) -> Result<Option<Value>,MachineError> {
        if self.error_state {
            self.clear();
        }

        //
        let params_num=self.stack_push_params(params)?;

        if let Some(v)=self.var_scope.get(name).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))? {
            if self.stack_frames.len()==0 { //?
                //self.stack_frames.push();
            }

            self.debugger.add_func_name(name);
            if self.inner_call_value(params_num,v,true)? {
                self.run()?;
            }
        } else if let Some(x)=self.get_method(name, params_num) {
            self.debugger.add_func_name(name);
            self.inner_call_bound_func(params_num, x)?; //,symbol.clone()
        } else {
            return Ok(None);
        }

        Ok(Some(self.result_val()))
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
            if !self.var_scope.set(name,to_value).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))? {
                return Err(MachineError::from_machine(self, MachineErrorType::GlobalOrConstNotFound(name.to_string()) ));
            }

        } else {
            let Some(global_val) = self.var_scope.get(name).or_else(|e|Err(MachineError::from_machine(&self, e.error_type)))?
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

        //
        let mut rets = vec![value.clone_leaf()];

        //gets
        for i in 0..fields.len()-1 {
            let ret=self.call_method("get_field", [rets.last().unwrap().clone_root(),fields.get(i).unwrap().clone_root()])?;
            rets.push(ret);
        }

        //
        rets.push(to_value);

        //sets
        for i in (0..rets.len()-1).rev() {
            self.call_method("set_field", [
                rets.get(i).unwrap().clone_root(),
                fields.get(i).unwrap().clone_root(),
                rets.get(i+1).unwrap().clone_root(),
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

    fn inner_value_get<T:AsRef<[Value]>>(&mut self,value:Value,fields:T) -> Result<Value,MachineError> {
        let mut fields = fields.as_ref().to_vec();
        fields.reverse();

        let mut cur_value=value;

        while let Some(field)=fields.pop() {
            cur_value = self.call_method("get_field", [cur_value,field])?;
        }

        Ok(cur_value)
    }

    fn stack_swap(&mut self) -> Result<(),MachineError> {
        let stack_len = self.stack.len();

        if stack_len>=2 {
            self.stack.swap(stack_len-1, stack_len-2);
            self.debugger.stack_swap();
        } else {
            return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(2) ));
        }

        Ok(())
    }

    fn stack_rot_right(&mut self) -> Result<(),MachineError> {
        let stack_len = self.stack.len();

        if stack_len>=3 {
            self.stack[stack_len-3 ..].rotate_left(1);
            self.debugger.stack_rot_right();
        } else {
            return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(3) ));
        }

        Ok(())
    }
    fn stack_rot_left(&mut self) -> Result<(),MachineError> {
        let stack_len = self.stack.len();

        if stack_len>=3 {
            self.stack[stack_len-3 ..].rotate_right(1);
            self.debugger.stack_rot_left();
        } else {
            return Err(MachineError::from_machine(self, MachineErrorType::InvalidStackAccess(3) ));
        }

        Ok(())
    }
}


