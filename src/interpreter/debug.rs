// use std::path::PathBuf;

use super::super::common::*;
use super::Closure;
// use super::machine::*;
use super::value::*;


#[derive(Debug)]
pub enum StackTrace {
    Main {
        build : BuildT,
        loc : Option<Loc>,
        instr_pos : usize,
        instr_start_pos : usize,
        instr_end_pos : usize,
    },
    BuildFunc {
        name : String,
        build : BuildT,
        func_ind : usize,

        params_start : usize,
        params_num : usize,
        
        params : Vec<String>,

        loc : Option<Loc>,

        instr_pos : usize,
        instr_start_pos : usize,
        instr_end_pos : usize,
    },
    BoundFunc{
        name : String,

        params_start : usize,
        params_num : usize,
        
        params : Vec<String>,
    },
    Include {
        build : BuildT,
        loc : Option<Loc>,

        instr_pos : usize,
        instr_start_pos : usize,
        instr_end_pos : usize,
    }
}


#[derive(Debug,Clone)]
pub struct ValOrigin {
    build : BuildT,
    instr_pos : usize,
    name : Option<StringT>,
}

pub struct Debugger {
    disabled:bool,
    debug_print_disabled:bool,

    stack_traces : Vec<StackTrace>,

    result_val_info : Option<ValOrigin>,
    stack_val_infos : Vec<Option<ValOrigin>>,
    
    func_names : Vec<String>,
    stack_trace_been_pushed:bool,
}

impl Debugger {
    pub fn last_method_call_info(&self) -> Option<(String,Vec<String>)> {
        if let Some(StackTrace::BoundFunc { name, params, .. })= self.stack_traces.last() {
            Some((name.clone(),params.clone()))
        } else {
            None
        }
    }

    pub fn set_print(&mut self,enabled:bool) {
        self.debug_print_disabled=!enabled;
    }
    pub fn set_enabled(&mut self,enabled:bool) {
        self.disabled=!enabled;
    }
    pub fn new() -> Self { //enabled:bool,debug_print_enabled:bool build : BuildType, 
        Self {
            stack_traces : Vec::new(),
            result_val_info : None,
            stack_val_infos : Vec::new(),
            // stack_traces : vec![StackTrace2::Main { 
            //     loc: None, 
            //     instr_pos: 0, 
            //     instr_start_pos: 0, 
            //     instr_end_pos: build.main_instruct_len,
            //     build, 
            // }],
            disabled: false, // !enabled,
            debug_print_disabled: true, // !debug_print_enabled,
            func_names : Vec::new(),
            stack_trace_been_pushed:false,
            
        }
    }

    pub fn clear(&mut self) {
        self.stack_traces.clear();
        self.clear_func_names();
        self.stack_val_infos.clear();
    }
    pub fn stack_traces(&self) -> &Vec<StackTrace> {
        &self.stack_traces
    }

    pub fn add_func_name(&mut self, func_name : &str) {
        if self.stack_trace_been_pushed {
            self.clear_func_names();
        }

        self.func_names.push(func_name.to_string());
    }

    pub fn clear_func_names(&mut self) {
        self.func_names.clear();
        self.stack_trace_been_pushed=false;
    }

    pub fn last_func_name(&self) -> String {
        self.func_names.last().cloned().unwrap_or("_unknown".to_string())
    }

    fn get_func_name(&mut self) -> String {
        if self.func_names.len()>0 {
            self.func_names.remove(0)
        } else {
            // String::new()
            "_unknown".to_string()
        }
    }
    pub fn push_frame_main(&mut self, build : BuildT) {
        if self.disabled {return;}
        self.stack_trace_been_pushed=true;

        //
        self.stack_traces.push(StackTrace::Main { 
                build:build.clone(), 
                loc: None, 
                instr_pos: 0, 
                instr_start_pos: 0, 
                instr_end_pos: build.main_instruct_len,
        });
    }

    pub fn push_frame_build_func(&mut self, 
        // machine : &Machine, 
        // func_name : Option<String>, 
        build : BuildT, 
        func_ind : usize, 
        params_num : usize,
        stack : &Vec<Value>,
        result_val:&Value,
    ) {
        if self.disabled {return;}

        //
        self.stack_trace_been_pushed=true;
        let func_name = self.get_func_name();

        //
        self.debug_print_stack_result(stack, result_val);
        self.debug_print_call_build_func(Some(func_name.clone()), params_num, build.clone(), func_ind);

        //
        let params_start = stack.len()-params_num;
        let func=build.functions.get(func_ind).unwrap();
        
        self.stack_traces.push(StackTrace::BuildFunc { 
            name: func_name, 
            build : build.clone(), 
            func_ind, 
            params_start, 
            params_num, 
            params : (params_start..stack.len()).rev().map(|i|stack.get(i).unwrap().type_string()).collect::<_>(), 
            loc: None,
            instr_pos : func.instruct_start_pos,
            instr_start_pos : func.instruct_start_pos,
            instr_end_pos : func.instruct_start_pos+func.instruct_len,
        });
    }

    pub fn push_frame_bound_func(&mut self, 
        // machine : &Machine, 
        // func_name : Option<String>, 
        params_num : usize,
        stack : &Vec<Value>,
        result_val:&Value,

    ) 
    {
        if self.disabled {return;}

        // let stack =machine.stack();
        // let result_val=machine.result_val();

        //
        self.stack_trace_been_pushed=true;
        let func_name = self.get_func_name();

        //
        self.debug_print_stack_result(stack, result_val);
        // println!("depth = {}",self.stack_traces.len());
        self.debug_print_call_bound_func(Some(func_name.clone()),params_num);


        //
        let params_start = stack.len()-params_num;

        self.stack_traces.push(StackTrace::BoundFunc { 
            name: func_name, 
            params_start, 
            params_num,
            params : (params_start..stack.len()).rev().map(|i|stack.get(i).unwrap().type_string()).collect::<_>(), 
        });
    }

    pub fn push_frame_include(&mut self, build : BuildT) {
        if self.disabled {return;}
        self.stack_trace_been_pushed=true;

        //
        self.stack_traces.push(StackTrace::Include { 
            build:build.clone(), 
            loc: None, 
            instr_pos : 0,
            instr_start_pos : 0,
            instr_end_pos : build.main_instruct_len,
        });
    }

    pub fn pop_frame(&mut self) {
        if self.disabled {return;}

        // if self.stack_traces.len() <=1 {
        //     panic!("scriptlang, debugger, pop, can't pop main");
        // }
        //
        self.stack_traces.pop().unwrap();
        self.clear_func_names();
    }


    pub fn set_result_val(&mut self) {
        if self.disabled {return;}

        let Some(last_stack_trace) = self.stack_traces.last() else {
            return;
        };

        match last_stack_trace {
            StackTrace::BuildFunc { build, instr_pos, .. }
            | StackTrace::Main { build, instr_pos, .. }
            | StackTrace::Include { build, instr_pos, .. }
            => {
                self.result_val_info=Some(ValOrigin{
                    build:build.clone(),
                    instr_pos:*instr_pos,
                    name : None, //self.get_result_val_name(),
                });
            }
            _ => {}
        }
    }
    pub fn push_stack_val(&mut self) {
        if self.disabled {return;}
        self.stack_val_infos.push(None);
    }
    
    pub fn set_stack_val_none(&mut self,stack_ind:usize) {
        if self.disabled {return;}

        //stack value only should not have any instr info when set by boundfunc(aka a method)
        if let Some(StackTrace::BoundFunc { .. })=self.stack_traces.last() {
            *self.stack_val_infos.get_mut(stack_ind).unwrap()=None;
        }
    }
    
    pub fn set_stack_from_last(&mut self) {
        if self.disabled {return;}

        // println!("{:?}",self.stack_traces.last());
        let Some(last_stack_trace) = self.stack_traces.last() else {
            return;
        };

        match last_stack_trace {
            StackTrace::BuildFunc { build, instr_pos, .. }
            | StackTrace::Main { build, instr_pos, .. }
            | StackTrace::Include { build, instr_pos, .. }
            => {
                // println!("== {:?}",self.stack_val_infos.iter().rev().map(|x|x.as_ref().map(|x|x.instr_pos)).collect::<Vec<_>>());
                *self.stack_val_infos.last_mut().unwrap()=Some(ValOrigin{
                    build:build.clone(),
                    instr_pos:*instr_pos,                
                    name : None, //self.get_stack_val_name(),
                });
                // println!("=== {:?}",self.stack_val_infos.iter().rev().map(|x|x.as_ref().map(|x|x.instr_pos)).collect::<Vec<_>>());
            }
            _ => {}
        }
        // if let Some(StackTrace::BuildFunc { build, instr_pos, .. })=self.stack_traces.last() {
        //     *self.stack_val_infos.last_mut().unwrap()=Some(StackValInfo{build:build.clone(),instr_pos:*instr_pos});
        // }
    }
    // pub fn set_stack_val_some(&mut self,stack_ind:usize, build:BuildT,instr_pos:usize) {
    //     if self.disabled {return;}
    //     *self.stack_val_infos.get_mut(stack_ind).unwrap()=Some(ValCreator{build,instr_pos});
    // }

    // pub fn set_stack_val_offset_some(&mut self,stack_offset_ind:usize, build:BuildT,instr_pos:usize) {
    //     if self.disabled {return;}
        
    //     let stack_len = self.stack_val_infos.len();
    //     let stack_ind=stack_len - stack_offset_ind - 1;

    //     *self.stack_val_infos.get_mut(stack_ind).unwrap()=Some(ValOrigin{build,instr_pos});
    // }

    
    pub fn set_stack_val_offset_from_last(&mut self,stack_offset_ind:usize) {
        if self.disabled {return;}
        
        let Some(last_stack_trace) = self.stack_traces.last() else {
            return;
        };

        let stack_len = self.stack_val_infos.len();
        let stack_ind=stack_len - stack_offset_ind - 1;

        
        match last_stack_trace {
            StackTrace::BuildFunc { build, instr_pos, .. }
            | StackTrace::Main { build, instr_pos, .. }
            | StackTrace::Include { build, instr_pos, .. }
            => {
                *self.stack_val_infos.get_mut(stack_ind).unwrap()=Some(ValOrigin{
                    build:build.clone(),
                    instr_pos:*instr_pos,
                
                    name : None, //self.get_stack_val_name(),
                });
            }
            _ => {}
        }
    }

    pub fn stack_swap(&mut self) {
        if self.disabled {return;}

        let stack_len = self.stack_val_infos.len();
        self.stack_val_infos.swap(stack_len-1, stack_len-2);
    }
    
    pub fn stack_rot(&mut self) {
        if self.disabled {return;}
        
        let stack_len = self.stack_val_infos.len();
        self.stack_val_infos[stack_len-3 ..].rotate_left(1);
    }
    pub fn stack_insert_none(&mut self,stack_ind:usize, amount:usize) {
        if self.disabled {return;}
                
        self.stack_val_infos.splice(stack_ind .. stack_ind,std::iter::repeat(None).take(amount));

    }
    pub fn stack_extend_none(&mut self, amount:usize) {
        if self.disabled {return;}
                
        self.stack_val_infos.extend(std::iter::repeat(None).take(amount));

    }
    pub fn pop_stack_val_amount(&mut self, amount:usize) {
        if self.disabled {return;}

        let stack_len=self.stack_val_infos.len();
        self.stack_val_infos.truncate(stack_len-amount);
    }



    pub fn stack_trace_path(&self) -> String {
        if self.disabled {return String::new();}

        //
        // let mut ss=String::new();
        let mut ss = String::new();
        
        for stack_trace in self.stack_traces.iter() {
            let s = match stack_trace {
                StackTrace::Main { .. }=>{
                    format!("main")
                }
                StackTrace::BuildFunc { name, build, func_ind, .. }=>{
                    // let name=name.as_ref().and_then(|x|Some(x.to_string()+",")).unwrap_or(String::new());
                    let name=name.clone()+",";
                    let path=build.path.as_ref().and_then(|x|Some(x.to_string_lossy().to_string())).unwrap_or("".to_string());
                    format!("build_func({name}{path},{func_ind})",)
                }
                StackTrace::BoundFunc { name, .. }=>{
                    // let name=name.as_ref().and_then(|x|Some(x.to_string()+",")).unwrap_or(String::new());
                    let name=name.clone()+",";

                    format!("bound_func({name})")
                }
                StackTrace::Include { build, .. } =>{
                    let path=build.path.as_ref().and_then(|x|Some(x.to_string_lossy().to_string())).unwrap_or("".to_string());
                    format!("include({path:?})")
                }
                // StackFrame::BuildFunc {func_ind,func_name,..}=>format!("build_func({},{})",func_ind,func_name.clone().unwrap_or(String::new())),
                // StackFrame::BoundFunc { func_name,.. }=>format!("bound_func({})",func_name),
                // StackFrame::Include { path,.. }=>format!("include({:?})",path.to_string_lossy()),                        
            };
            
            // stack_frame_str.push_str(format!("{:?}",stack_frame).as_str());
            if ss.len()>0 {
                ss.push_str("/");

            }
            ss.extend(s.chars());

        }
        ss
    }

    pub fn move_instr_pos(&mut self, new_instr_pos:usize) {
        if self.disabled {return;}

        //
        match self.stack_traces.last_mut() {
            Some(StackTrace::Main { 
                // instr_start_pos,
                // instr_end_pos,
                instr_pos, 
                build,
                loc,
                ..
            })=>{
                *instr_pos=new_instr_pos;

                if let Some(&loc2)=build.instr_locs.get(&new_instr_pos) {
                    *loc = Some(loc2);
                }
            }
            Some(StackTrace::BuildFunc { 
                // instr_start_pos,
                // instr_end_pos,
                instr_pos, 
                // name,
                build,
                // func_ind,
                // params_start,
                // params_num,
                // params,
                loc,
                ..
            })=>{
                *instr_pos=new_instr_pos;

                if let Some(&loc2)=build.instr_locs.get(&new_instr_pos) {
                    *loc = Some(loc2);
                }
            }
            Some(StackTrace::Include { 
                // instr_start_pos,
                // instr_end_pos,
                instr_pos, build,loc, 
                ..
            }) =>{
                *instr_pos=new_instr_pos;

                if let Some(&loc2)=build.instr_locs.get(&new_instr_pos) {
                    *loc = Some(loc2);
                }
            }
            Some(StackTrace::BoundFunc { 
                // name,  params,params_num,params_start,
                ..
            })=>{
               
                // panic!("");
            }
            None=>{
                // panic!("");
            }
        }
    }

    
    pub fn debug_print_stack_result(&self, stack : &Vec<Value>,result_val:&Value) {
        if self.disabled||self.debug_print_disabled {return;}
        
        let val=if result_val.is_custom::<Closure>() {
            let (func_build,func_ind)=result_val.as_custom().with_data_mut(|x:&mut Closure|Ok((x.build.clone(),x.func_ind))).unwrap();
            let p=func_build.path.as_ref().and_then(|p|p.to_str()).map(|p|p.to_string()+", ").unwrap_or("".to_string());
            format!("{p}func[{func_ind}]")
        }else{
            // result_val.as_string()
            String::new()
        };
        let val = if !val.is_empty(){" =".to_string()+val.as_str()}else{val};

        if let Some(result_val_info)=&self.result_val_info {
            let n=result_val_info.name.as_ref().map(|x|" :".to_string()+x.as_str()).unwrap_or_default();
            
            

            println!("$    r:({}{}): {result_val:?}{val}{n}",
                result_val_info.build.path.as_ref().and_then(|p|p.to_str()).map(|p|p.to_string()+", ").unwrap_or("".to_string()),
                result_val_info.instr_pos,
                
            );
        } else {
            println!("$    r: {result_val:?}{val}");
        }

        for (i,x) in stack.iter().enumerate().rev() {
            let j=stack.len()-i-1;

            // let x=format!("{:?}",x);
            // let x = x.rfind(":").map(|i|x[i+1..].to_string()).unwrap_or(x.clone());
            
            let val=if x.is_custom::<Closure>() {
                let (func_build,func_ind)=x.as_custom().with_data_mut(|x:&mut Closure|Ok((x.build.clone(),x.func_ind))).unwrap();
                let p=func_build.path.as_ref().and_then(|p|p.to_str()).map(|p|p.to_string()+", ").unwrap_or("".to_string());
                format!("{p}func[{func_ind}]")
            } else {
                // x.as_string()
                String::new()
            };

            let val = if !val.is_empty(){" =".to_string()+val.as_str()}else{val};

            if let Some(stack_info)=&self.stack_val_infos[i] {
                let n=stack_info.name.as_ref().map(|x|" :".to_string()+x.as_str()).unwrap_or_default();
                
                println!("*    {j}:({}{}): {x:?}{val}{n}",

                    stack_info.build.path.as_ref().and_then(|p|p.to_str()).map(|p|p.to_string()+", ").unwrap_or("".to_string()),
                    stack_info.instr_pos,
                
                );

            } else {
                println!("*    {j}: {x:?}{val}");
            }
            
        }

    }

    fn debug_print_call_build_func(&self, name : Option<String>,params_num : usize, build:BuildT, func_ind:usize) { //
        if self.disabled||self.debug_print_disabled {return;}

        let depth=self.stack_traces.len();//-1;
        let stack_trace_str=self.stack_trace_path();
        let name = name.unwrap_or("_".to_string());


        println!("{} {stack_trace_str:?} : CallBuildFunc({name:?},{params_num}) [{build:?}:{func_ind}]",
            "%".repeat(1+depth),
        );
    }
    
    fn debug_print_call_bound_func(&self, name : Option<String>,params_num : usize) { //
        if self.disabled||self.debug_print_disabled {return;}
        
        let depth=self.stack_traces.len();//-1;
        let stack_trace_str=self.stack_trace_path();
        let name = name.unwrap_or("_".to_string());
        println!("{} {stack_trace_str:?} : CallBoundFunc({name:?},{params_num})","%".repeat(1+depth));
    }

    fn instruction_to_string(&self,cur_build:BuildT,cur_instr_pos:usize, 
        // stack_len:usize
    ) -> String {
                  
        let cur_instr = cur_build.instructions.get(cur_instr_pos).cloned().unwrap();

        match &cur_instr {
            Instruction::CallGlobalOrMethod(symbol_ind,params_num)=>{
                format!("CallGlobal({:?},{params_num})",cur_build.symbols.get(*symbol_ind).unwrap().to_string())
            }
            Instruction::CallMethod(symbol_ind,params_num)=>{
                format!("CallMethod({:?},{params_num})",cur_build.symbols.get(*symbol_ind).unwrap().to_string())
            }
            Instruction::TryCallMethod(symbol_ind,params_num)=>{
                format!("TryCallMethod({:?},{params_num})",cur_build.symbols.get(*symbol_ind).unwrap().to_string())
            }
            Instruction::DeclGlobalVar(symbol_ind)=>{
                format!("DeclGlobalVar({:?})",cur_build.symbols.get(*symbol_ind).unwrap().to_string())
            }
            Instruction::SetGlobalVar(symbol_ind)=>{
                format!("SetGlobalVar({:?})",cur_build.symbols.get(*symbol_ind).unwrap().to_string())
            }
            Instruction::GetGlobalVarOrConst(symbol_ind,..)=>{
                format!("GetGlobalVar({:?})",cur_build.symbols.get(*symbol_ind).unwrap().to_string())
            }
            Instruction::GetGlobalVarRef(symbol_ind)=>{
                format!("GetGlobalVarRef({:?})",cur_build.symbols.get(*symbol_ind).unwrap().to_string())
            }
            Instruction::GetGlobalAccessRef(symbol_ind)=>{
                format!("GetGlobalAccessRef({:?})",cur_build.symbols.get(*symbol_ind).unwrap().to_string())
            }
            
            Instruction::Include(path_ind)=>{
                format!("Include({:?})",cur_build.includes.get(*path_ind).unwrap())
            }
            Instruction::ResultSymbol(symbol_ind)=>{
                format!("ResultSymbol({:?})",cur_build.symbols.get(*symbol_ind).unwrap().to_string())
            }
            // Instruction::GetStackVar(offset)=>{
            //     // format!("GetStackVar({offset}):@[{}]",stack_len-1-offset)
            //     format!("GetStackVar({offset})")

            // }
            // Instruction::SetStackVar(offset)=>{
            //     // format!("SetStackVar({offset}):@[{}]",stack_len-1-offset)
            //     format!("SetStackVar({offset})")
            // }
            x=>{format!("{x:?}")}
        }
    }

    fn cur_instr_pos_start_end(&self) -> (usize,usize,usize) {
        match self.stack_traces.last() {
            Some(StackTrace::Main { instr_pos,instr_start_pos,instr_end_pos, .. })=>{
                (*instr_pos,*instr_start_pos,*instr_end_pos)
            }
            Some(StackTrace::BuildFunc { instr_pos,instr_start_pos,instr_end_pos, .. })=>{
                (*instr_pos,*instr_start_pos,*instr_end_pos)
            }
            Some(StackTrace::BoundFunc { .. })=>{
                (0,0,0)
            }
            Some(StackTrace::Include { instr_pos,instr_start_pos,instr_end_pos, ..  }) =>{
                (*instr_pos,*instr_start_pos,*instr_end_pos)
            }
            None=>{
                (0,0,0)
            }
        }
    }

    fn cur_build(&self) -> Option<BuildT> {
        match self.stack_traces.last() {
            Some(StackTrace::Main { build, .. })=>{
                Some(build.clone())
            }
            Some(StackTrace::BuildFunc { build, .. })=>{
                Some(build.clone())
            }
            Some(StackTrace::BoundFunc { .. })=>{
                None
            }
            Some(StackTrace::Include { build, ..  }) =>{
                Some(build.clone())
            }
            None=>{
                None
            }
        }
    }
    
    pub fn step(&self, stack : &Vec<Value>,result_val:&Value) { //,stack_len:usize,machine:&Machine
        if self.disabled||self.debug_print_disabled {return;}

        //
        self.debug_print_stack_result(stack, result_val);
        // let stack_len=stack.len();

        //
        let cur_build=self.cur_build();        
        let (cur_instr_pos,cur_instr_start,cur_instr_end)=self.cur_instr_pos_start_end();
        let cur_instr_rel_ind=cur_instr_pos-cur_instr_start;
        let cur_instr_rel_len=cur_instr_end-cur_instr_start;

        let instr_ind_str = format!("[{cur_instr_rel_ind}/{cur_instr_rel_len}]:({cur_instr_pos})");
        let stack_trace_str=self.stack_trace_path();
        let depth=self.stack_traces.len();//-1;

        if cur_instr_pos < cur_instr_end {
            let instr_str=self.instruction_to_string(cur_build.clone().unwrap(),cur_instr_pos,); // stack_len       
            println!("{} {instr_ind_str} : {stack_trace_str} : {instr_str}","+".repeat(1+depth));
        } else {    
            println!("{} {instr_ind_str} : {stack_trace_str}","-".repeat(1+depth));
        }
    }


    pub fn print_stack_trace(&self, skip_first:bool) {
        if self.disabled {return;}

        eprintln!("Stack Trace:");

        //

        for (i,stack_trace_frame) in self.stack_traces.iter().rev().enumerate() {
            if skip_first && i==0 {
                continue;
            }
            
            // if i+1 == stack_trace.len() {break;}
            // let j = stack_trace.len()-i-2;

            // let mut s=String::new();

            match stack_trace_frame {
                StackTrace::BoundFunc { name, 
                    // params_start, params_num, 
                    params ,
                    ..
                }=>{
                    // let params = params.iter().map(|x|{x.rfind(":").map(|i|x[i+1..].to_string()).unwrap_or(x.clone())}).collect::<Vec<_>>();
                    let params=params.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(", ");
                    // let name=name.clone().and_then(|x|Some(x.to_string())).unwrap_or("_".to_string());
                    // let name=if name.is_empty(){"_unknown".to_string()}else{name.clone()};
                    
                    eprintln!("\tBoundFunc => {name}({})",params);
                }
                StackTrace::BuildFunc { 
                    name, build, func_ind, 
                    // params_start, params_num,                     
                    params, 
                    loc, 
                    instr_pos, instr_start_pos, instr_end_pos ,
                    ..
                }=>{
                    let instr_rel_ind=instr_pos-instr_start_pos;
                    let instr_rel_len=instr_end_pos-instr_start_pos;

                    // let params = params.iter().map(|x|{x.rfind(":").map(|i|x[i+1..].to_string()).unwrap_or(x.clone())}).collect::<Vec<_>>();
                    let params=params.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(", ");
                    // let name=name.clone().and_then(|x|Some(x.to_string())).unwrap_or("_".to_string());
                    // let name=if name.is_empty(){"_unknown".to_string()}else{name.clone()};
                    let path=build.path.clone().and_then(|x|Some(x.to_string_lossy().to_string())).unwrap_or("_".to_string());
                    
                    eprint!("\tBuildFunc({path:?},{func_ind},{instr_rel_ind}/{instr_rel_len}) :: {name}({})",params);

                    if let Some(loc)=loc {
                        eprint!(", at {loc}");
                    }

                    eprintln!("");
                }
                StackTrace::Include { build, loc, instr_pos, instr_start_pos, instr_end_pos }=>{
                    // let path=build.path.clone();
                    let path=build.path.clone().and_then(|x|Some(x.to_string_lossy().to_string())).unwrap_or("_".to_string());

                    let instr_rel_ind=instr_pos-instr_start_pos;
                    let instr_rel_len=instr_end_pos-instr_start_pos;

                    eprint!("\tInclude({path:?},{instr_rel_ind}/{instr_rel_len})");

                    if let Some(loc)=loc {
                        eprint!(", at {loc}");
                    }

                    eprintln!("");
                }
                StackTrace::Main { build, loc, instr_pos, instr_start_pos, instr_end_pos }=>{
                    let path=build.path.clone().and_then(|x|Some(x.to_string_lossy().to_string()));
                    
                    let instr_rel_ind=instr_pos-instr_start_pos;
                    let instr_rel_len=instr_end_pos-instr_start_pos;

                    eprint!("\tMain({path:?},{instr_rel_ind}/{instr_rel_len})");

                    if let Some(loc)=loc {
                        eprint!(", at {loc}");
                    }

                    eprintln!("");
                }
                // StackTraceFrameType::BoundFunc(n,params)=>{
                //     //format!("{x:?}")
                //     s=format!(", in {n}({})",params.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(", "));
                // }
                // StackTraceFrameType::BuildFunc(n,params)=>{
                //     if let Some(n)=n {
                //         // s=format!(", in {n:?}");
                //         s=format!(", in {n}({})",params.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(", "));
                //     } else {
                //         // s=format!(", in anonymous");
                //         s=format!(", in anonymous({})",params.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(", "));
                //     }
                // }
                // _=>{}
            }
            // println!("\t{j}: {:?} at {}{s}",  // : {:?}
            //     stack_trace_frame.path.clone().unwrap_or_default(),
            //     stack_trace_frame.loc.unwrap_or_default(),
            //     // stack_trace_frame.frame_type,
            // );

            match stack_trace_frame {
                StackTrace::BuildFunc {build,loc,..}
                | StackTrace::Include { build, loc, ..}
                | StackTrace::Main { build, loc, ..}
                =>{

                    // let path = build.path.as_ref().map(|p|p.as_path());                    
                    let src=build.src.as_ref().map(|s|s.as_str());

                    if let (Some(src),Some(loc))=(src,loc) {
                        let msg=error_line_src(src, *loc);
                        eprintln!("{msg}");

                    }
                }
                _=>{}
            }
        }
    }

    pub fn print_stack(&self, stack : &Vec<Value>) {
        
        println!("stack:");

        for (i,x) in stack.iter().enumerate().rev() {
            
            let j=stack.len()-i-1;

            if let Some(stack_info)=&self.stack_val_infos[i] {
                println!("\t{j}:({}{}): {x:?}",
                    stack_info.build.path.as_ref().and_then(|p|p.to_str()).map(|p|p.to_string()+", ").unwrap_or("".to_string()),
                    stack_info.instr_pos,
                );
            } else {
                println!("\t{j}: {x:?}");
            }


        }
    }



    // fn _get_result_val_name(&self) -> Option<StringT> {
        
    //     let Some(last_stack_trace) = self.stack_traces.last() else {
    //         return None;
    //     };
    //     match last_stack_trace {
    //         StackTrace::BuildFunc { build, instr_pos, .. }
    //         | StackTrace::Main { build, instr_pos, .. }
    //         | StackTrace::Include { build, instr_pos, .. } => 
    //         {
    //             if let Some(instr)=build.instructions.get(*instr_pos) {

    //                 match instr {
    //                     Instruction::GetGlobalVarOrConst(symbol_ind,..)
    //                     | Instruction::GetGlobalVarRef(symbol_ind)
    //                     | Instruction::GetGlobalAccessRef(symbol_ind)
    //                     =>{
    //                         let n=build.symbols.get(*symbol_ind).unwrap().clone();
    //                         return Some(n);
    //                     }
    //                     Instruction::GetStackVar(stack_offset_ind)|Instruction::GetStackVarDeref(stack_offset_ind) => {                            
    //                         let stack_len = self.stack_val_infos.len();
    //                         let stack_ind=stack_len - stack_offset_ind - 1;
    //                         return self.stack_val_infos.get(stack_ind).unwrap().as_ref().and_then(|x|x.name.clone());
    //                     }
    //                     _ => {    
    //                     }
    //                 }
    //             }
    //         }
    //         _ => {
                
    //         }
    //     }

    //     None
    // }
    
    // fn _get_stack_val_name(&self) -> Option<StringT> {

    //     let Some(last_stack_trace) = self.stack_traces.last() else {
    //         return None;
    //     };
    //     match last_stack_trace {
    //         StackTrace::BuildFunc { build, instr_pos, .. }
    //         | StackTrace::Main { build, instr_pos, .. }
    //         | StackTrace::Include { build, instr_pos, .. } => 
    //         {
    //             if let Some(instr)=build.instructions.get(*instr_pos) {

    //                 match instr {
    //                     Instruction::StackPush => {
    //                         return self.result_val_info.as_ref().and_then(|x|x.name.clone());
    //                     }
    //                     Instruction::GetStackVar(stack_offset_ind)|Instruction::GetStackVarDeref(stack_offset_ind) => {                            
    //                         let stack_len = self.stack_val_infos.len();
    //                         let stack_ind=stack_len - stack_offset_ind - 1;
    //                         return self.stack_val_infos.get(stack_ind).unwrap().as_ref().and_then(|x|x.name.clone());
    //                     }
    //                     _ => {    
    //                     }
    //                 }
    //             }
    //         }
    //         _ => {
                
    //         }
    //     }

    //     None
    // }
}

