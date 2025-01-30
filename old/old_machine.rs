
    JmpUp{cond:Option<bool>,instr_offset:usize},
    JmpDown{cond:Option<bool>,instr_offset:usize}, 

//
	let Instruction::JmpDown { instr_offset , ..} = instructions.get_mut(to_end_instr_ind).unwrap() else {
		panic!("scriptlang,builder,ast, expected JmpDown instr");
	};

	//end_instr_offset
	*instr_offset=cur_instr_ind-to_end_instr_ind-1;

//

	instructions.push(Instruction::JmpDown { cond: not_cond, instr_offset: 2 });
	
//

	let cur_instr_ind = instructions.len();
	let jmp_instr_ind = block_start_instr_inds.get(&block_node_ind).unwrap();
	let jmp_instr_offset = cur_instr_ind-*jmp_instr_ind-1+1;
	instructions.push(Instruction::JmpUp{cond,instr_offset:jmp_instr_offset}); //(cond,jmp_instr_offset)

//
	instructions.push(Instruction::JmpDown{cond:not_cond,instr_offset:2}); //(not_cond,2)

//
	instructions.push(Instruction::JmpDown { cond, instr_offset: 0 }); //(cond,0)
	
//
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
//
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
    includer : Option<Box<dyn FnMut(&Path) -> Option<BuildT> + 'a>>, //can use lifetime a for some reason?
    const_scope:HashMap<&'a str,Value>,

    // gc_check_remove_recursive : bool,
    core_val :   X, //&'a mut 
}


// impl<'a,'c,X:Copy> Machine<'a,'c,X> {
//     pub fn get_core(&mut self) -> &X {
//         &self.core_val
//     }
// }

// impl<'a,'c,X> Machine<'a,'c,& mut X> {
//     pub fn get_core_mut(&mut self) -> &mut X {
//         self.core_val
//     }    
//     pub fn get_core_ref(& self) -> &X {
//         self.core_val
//     }
// }

// impl<'a,'c,X> Machine<'a,'c,&X> {
//     pub fn get_core_ref(&self) -> &X {
//         self.core_val
//     }
// }

impl<'a,'c,X> Machine<'a,'c,X> 
{ //,'b //,'b

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


    // pub fn set_gc_check_remove_recursive(&mut self,b:bool) {
    //     self.gc_check_remove_recursive=b;
    // }


    // fn propagate_err<'q>(&self) -> impl Fn(MachineError) -> MachineError+'q 
    // where 'a:'q
    // {
    //     move|e|MachineError::machine_new(& self, e.error_type)
    // }

    fn run(&mut self) -> Result<(),MachineError> {
        // if self.debugger.stack_traces().len()==0 {
        //     self.debugger.push_main(self.cur_build.clone());
        // }

        // self.stack_frames.push(Vec::new());
	}

    fn step(&mut self) -> Result<(),MachineError> {
        let cur_build=self.cur_build.clone().unwrap();
        let instr=cur_build.instructions.get(self.instr_pos).unwrap();

        match instr {
            Instruction::ResultVararg => {
                // self.result_val=Value::Custom(Custom::new_unmanaged_mut(Vararg,None));
                self.result_val=Value::custom_unmanaged(Vararg);
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
                    custom.with_data_mut(|data:&mut Value|{
                        if !init && data.is_undefined() {
                            Err(MachineError::from_machine(self, MachineErrorType::SetUndefinedVar ))
                        } else {
                            *data=to_val;
                            Ok(())
                        }
                    })?;

                    
                    // let data=custom.data();
                    // let mut data=data.get_mut::<Value>()?;
    
                    // // if data.is_type(ValueType::custom::<UninitVar>()) {
                    // //     // let symbol=self.get_symbol( *symbol_ind)?;
                    // //     let symbol="todo";
                    // //     return Err(MachineError::machine_new(self, MachineErrorType::GlobalNotFound(symbol.to_string()) ));
                    // // }
    
                    // if !init && data.is_undefined() {
                    //     return Err(MachineError::from_machine(self, MachineErrorType::SetUndefinedVar ));
                    // }
                    
                    // *data=to_val;
    
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

                        if self.inner_call_value(params_num,&var_data,false)? {
                            return Ok(()); //continue;
                        }
                    } else if let Some(v)=self.constant_get(&data.name)  //not needed for deref? there was the option  for globals (and constants) being captured
                    //self.lib_scope.get_constant(&data.name) 
                    {
                        //call v

                        self.debugger.add_func_name(data.name.as_str());

                        if self.inner_call_value(params_num,&v,false)? {
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
                    //                
                    // let data=custom.data();
                    // let data=data.borrow_mut::<Value>()?;

                    if data.is_undefined() {
                        return Err(MachineError::from_machine(self, MachineErrorType::GetUndefinedVar ));
                    }
                    
                    //
                    //let v=self.copy_val(data.clone())?;
                    let v=data;
                    
                    if self.inner_call_value(params_num,&v,false)? {
                        return Ok(()); //continue;
                    }
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


    fn set_result_val(&mut self, v:Value) {
        // let old_gc_ind=v.gc_index();

        self.result_val=v.clone_root();
        self.debugger.set_result_val();
        
        // if let Some(gc_ind) = old_gc_ind {
        // self.gc_scope.check_remove(gc_ind,self.gc_check_remove_recursive);
        // }

        self.gc_scope.remove_norefs();

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



    fn inner_call_value(&mut self, params_num:usize, v:&Value, finish:bool) -> Result<bool,MachineError> {
        // println!("=====- {} {}",v.type_string(),v.as_float());
        // println!("stk is {:?}", self.stack());
        // if let Some(caller)=v.get_custom().and_then(|custom|custom.get_caller()) {
        //     // println!("yea0");

        //     self.debugger.push_frame_bound_func(params_num, &self.stack, &self.result_val); //before params_num+1 ...
        
        //     let params_num = params_num+1;
            
        //     //
        //     {
        //         self.push_stack_val(v.clone());
        //         self.debugger.set_stack_from_last();
        //     }
          
        //     //
        //     let vv= caller(FuncContext::new(self,params_num));
    
        //     match vv {
        //         Ok(vv)=>{
    
        //             self.set_result_val(vv);
        //             //println!("@@@@@ bound func");
                    
        //             self.stack_pop_amount(params_num)?;
                
        //             self.debugger.pop_frame();
            
        //             self.gc_scope.remove_norefs(); //hmm called already with set_result? and possibly on stack_pop_amount(params_num>0)
    
        //             Ok(false)
        //         }
        //         Err(e) => {
        //             if e.build.is_none() {
        //                 Err(MachineError::from_machine(&self, e.error_type))
        //             } else {
        //                 Err(e)
        //             }
        //         }
        //     }
        // } else 
        if v.is_custom::<Closure>() {
            // println!("yea1");

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

            Ok(true)
        } else {
            // println!("yea2");

            // println!("v is {v:?}");
            Err(MachineError::from_machine(self,  MachineErrorType::ValueNotAFunc(v.type_string()) ))
        }
    }

    // fn stack_push_params(&mut self,params : &Vec<Value>) -> Result<usize,MachineError> {
    //     let mut params_num=0;

    //     for p in params.into_iter().rev() {
    //         self.stack.push(p.clone_to_root());
    //         params_num+=1;
    //     }

    //     Ok(params_num)
    // }

}


  fn try_copy_val(&mut self, v:Value) -> Result<Option<Value>,MachineError> {
        if !v.is_custom_any() { //should only use copy on customs?
            return Ok(None);
        }

        Ok(self.try_call_method("copy", vec![v.clone_root()])?)
    }

    


    

    // fn stack_params_iter_mut(&mut self, params_num : usize) -> impl Iterator<Item=&mut Value> {
    //     let params_start = self.stack.len()-params_num;
    //     let params = self.stack[params_start..].iter_mut().rev();
    //     params
    // }