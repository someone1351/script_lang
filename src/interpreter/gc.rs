
use std::{any::Any, sync::{Arc, Mutex, Weak} };

// use parking_lot::Mutex;

use super::value::*;
// use super::error::*;


#[derive(Copy,Clone)]
pub struct TypeInfo {
    id : std::any::TypeId,
    name : &'static str,
}

impl TypeInfo {
    pub fn new<T:'static>() -> TypeInfo {
        Self {
            id : std::any::TypeId::of::<T>(),
            name : std::any::type_name::<T>(),
            // name:"[std::def::Thing<dyn (a::b::Def,c::d::Ghi)>,&i32]"
        }
    }

    pub fn id(&self) -> std::any::TypeId {
        self.id
    }

    pub fn short_name(&self) -> String {
        let mut s=String::new();
        let mut r=false;

        for c in self.name.chars().rev() {
            match c {
                ':' if !r => {
                    r=true;
                }
                '<'|'>'|'('|')'|'['|']'|' '|','|'&'|';' if r => {
                    r=false;
                    s.push(c);
                }
                _ if !r => {
                    s.push(c);
                }
                _ => {}
            }
        }

        s.chars().rev().collect::<String>()
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

pub type Traverser<'a> = Box<dyn Iterator<Item=&'a Value>+'a>;
pub trait GcTraversable : Any + Send {
    fn traverser<'a>(&'a self) -> Traverser<'a>;
}
// pub trait GcTraversableExt : GcTraversable + ToString {}

#[derive(Clone,Debug)]
pub struct GcIndex(Arc<Mutex<usize>>);

impl GcIndex {
    pub fn new(index:usize) -> Self { Self(Arc::new(Mutex::new(index))) }
    pub fn set(&mut self,index:usize) -> Result<(),()> {
        let Ok(mut data)=self.0.try_lock() else {return Err(());};
        *data=index;
        Ok(())
    }

    pub fn get(&self)->Result<usize,()> {
        let Ok(data)=self.0.try_lock() else {return Err(());};
        Ok(*data)
    }

    pub fn strong_count(&self)->usize {
        Arc::strong_count(&self.0)
    }

    pub fn weak_count(&self)->usize {
        Arc::weak_count(&self.0)
    }

    pub fn to_weak(&self) -> GcWeakIndex {
        GcWeakIndex(Arc::downgrade(&self.0))
    }
}

#[derive(Clone,Debug)]
pub struct GcWeakIndex(Weak<Mutex<usize>>);

impl GcWeakIndex {
    pub fn new() -> Self {
        Self(Weak::new())
    }
    pub fn to_strong(&self)->Option<GcIndex> {
        self.0.upgrade().and_then(|x|Some(GcIndex(x)))
    }

    pub fn strong_count(&self)->usize {
        Weak::strong_count(&self.0)
    }

    pub fn weak_count(&self)->usize {
        Weak::weak_count(&self.0)
    }
}

#[derive(Clone,Debug)]
pub struct GcRootCount(Arc<Mutex<usize>>);

impl GcRootCount {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(0)))
    }

    pub fn incr(&self) -> Result<(),()> {
        let Ok(mut data)=self.0.lock() else {return Err(());};
        *data+=1;
        // println!("incr!");
        Ok(())
    }
    pub fn decr(&self) -> Result<(),()> {
        let Ok(mut data)=self.0.lock() else {return Err(());};
        // println!("root({}) decr",self.get());
        *data-=1;
        // println!("decr!");
        Ok(())
    }
    pub fn get(&self)->Result<usize,()> {
        let Ok(data)=self.0.lock() else {return Err(());};
        Ok(*data)
    }
    pub fn strong_count(&self)->usize { Arc::strong_count(&self.0) }
}

#[derive(Clone)]
pub enum GcManagedInner {
    Mut(Arc<Mutex<dyn GcTraversable>>),
    NonMut(Arc<dyn GcTraversable + Sync>),
    // MutExt(Arc<Mutex<dyn GcTraversableExt>>),
    // NonMutExt(Arc<dyn GcTraversableExt + Sync>),
}

// #[derive(Debug)]
pub struct GcManaged {
    // data : Arc<Mutex<Box<dyn Any + Send>>>,
    // traverser:Box<dyn TraverserTrait+Send>,

    pub data : GcManagedInner, //Arc<Mutex<dyn GcTraversable>>,
    // type_name : &'static str,
    // type_info:TypeInfo, //unused?
    pub type_info:TypeInfo,


    pub managed_index:GcIndex,
    pub root_count:GcRootCount,


    pub marked : bool,
}

impl GcManaged {
    // pub fn new()
    pub fn with_data(&self, func:impl FnOnce(&dyn GcTraversable) ->Result<(),()>)->Result<(),()> {
        match &self.data {
            GcManagedInner::Mut(x) => {
                if let Ok(x)=x.try_lock() {
                    func(&*x)
                } else {
                    Err(())
                }
            }
            GcManagedInner::NonMut(x) => {
                func(x.as_ref())
            }
        }
    }
    pub fn _get(&self) -> Result<&dyn GcTraversable,()> {
        match &self.data {
            GcManagedInner::Mut(x) => {
                // let x=x.as_ref().lock().unwrap();

                if let Ok(_x)=x.try_lock() {





                    // let x=&x;
                    // x.
                    // let Ok(x)=std::sync::MutexGuard::try_map(x, |x|{
                    //     &x
                    // }) else {
                    //     return  Err(());
                    // };
                    // Ok(x.traverser())
                    // Err(())
                    // Ok(x.into())
                    Err(())
                } else {
                    return  Err(());
                }
            }
            GcManagedInner::NonMut(x) => {
                Ok(x.as_ref())

            }
        }
    }
}
