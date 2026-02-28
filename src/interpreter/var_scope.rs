
// use std::any::Any;
use std::collections::HashMap;
// use std::collections::HashSet;

// use super::custom::*;
use super::error::*;
// use super::gc_scope::*;
use super::value::*;

#[derive(Debug)]
pub struct Var {
    value:Value,
    is_refvar : bool,
    init:bool,
}

impl Clone for Var {
    fn clone(&self) -> Self {
        Self {
            value:self.value.clone_root(),
            is_refvar:self.is_refvar,
            init:self.init,
        }
    }
}

#[derive(Debug,Clone,Default)]
pub struct VarScope {
    // vars : HashMap<String,Value>,
    // refvars : HashSet<String>,
    // gc : StrongGc,
    vars : HashMap<String,Var>,
}

// impl GcTraversable for VarScope {
//     fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=&Value>+'a> {
//         Box::new(self.vars.values())
//     }
// }


impl VarScope {
    pub fn new() -> Self {
        Self {
            vars: Default::default(),
            // gc : StrongGc::new(),
            // refvars: Default::default(),
        }
    }

    pub fn get(&self,n : &str) -> Result<Option<Value>,MachineError> {
        let Some(var)=self.vars.get(&n.to_string()) else {
            return Ok(None);
        };

        if !var.init {
            return Ok(None);
        }

        if var.is_refvar {
            return Ok(Some(var.value.as_custom().data_clone::<Value>()?));
        }

        Ok(Some(var.value.clone_root()))
    }

    pub fn get_ref(&mut self,n : &str,gc_scope:&mut GcScope) -> Value { //Option<Value>
        // let Some(var)=self.vars.get_mut(&n.to_string()) else {
        //     return None;
        // };

        let var=self.vars.entry(n.to_string()).or_insert_with(||Var {
            // value : Value::new_custom_unmanaged(UninitVar),
            value : Value::Undefined,
            is_refvar: false,
            init:false,
        });

        if !var.is_refvar {
            let refvar=Value::custom_managed_mut(var.value.clone_leaf(), gc_scope);
            var.value=refvar.clone_root(); //as stored as global
            var.is_refvar=true;
        }

        var.value.clone_root()
    }

    pub fn contains(&mut self,n : &str) -> bool {
        self.vars.contains_key(n)
    }

    // pub fn decl(&mut self,n : &str,value:Value) -> Result<(),MachineError> {
    //     if !self.set(n, value.clone())? {
    //         self.vars.insert(n.to_string(), Var {
    //             value:value.clone_root(),
    //             is_refvar: false,
    //         });
    //     }

    //     Ok(())
    // }
    pub fn decl(&mut self,n : &str,value:Option<Value>) -> Result<(),MachineError> { //,value:Value
        // //overwrites prev, if prev captured for closure, then it will no longer point to this global
        // self.vars.insert(n.to_string(), Var {
        //     value:value.clone_root(),
        //     is_refvar: false,
        // });

        //doesn't overwrite
        let var=self.vars.entry(n.to_string()).or_insert_with(||Var {
            //value:Value::Float(0.5),//what?? was a test?
			value:Value::Nil,
            is_refvar: false,
            init:true,
        });

        if value.is_none() && var.is_refvar && !var.init {
            var.init=true;
            self.set(n, value.unwrap_or(Value::Nil))?;
        } else if let Some(value)=value {
            self.set(n, value)?;

        }
        // println!("==={n} {:?}",var.value);



        // if let Some(value)=value {
        //     self.set(n, value)?;
        // //     g.value=value;
        // //     g.init=true;
        // } else

        Ok(())
    }

    pub fn set(&mut self,n : &str,value:Value) -> Result<bool,MachineError> {
        let Some(var)=self.vars.get_mut(&n.to_string()) else {
            return Ok(false);
        };

        if !var.init { //what is this for? undefined? should be an error then? ie trying to set undefined var, todo!, or is handled elsewhere?
            return Ok(false);
        }

        if value.is_void() {
            return Err(MachineError::new(MachineErrorType::VoidNotExpr));
        }

        if var.is_refvar {
            // let data=var.value.as_custom().data();
            // let mut data=data.borrow_mut::<Value>()?;
            // *data=value.clone();

            var.value.as_custom().with_data_mut(|x:&mut Value|{
                *x=value.clone_leaf();
            })?;
        } else {
            var.value=value.clone_root();
        }

        Ok(true)
    }

    pub fn test(&self) {
        // println!("{:?}",self.gc.borrow());
        // self.gc.borrow().test();

        println!("vars {:?}",self.vars);

    }

}