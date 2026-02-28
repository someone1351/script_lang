
use std::{any::Any, sync::{Arc, Mutex, Weak} };

// use parking_lot::Mutex;

use crate::{interpreter::{custom::WeakValueInner, gc_scope::GcDropper}, Custom, GcScope};

// use super::value::*;
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

pub type Traverser<'a> = Box<dyn Iterator<Item=Custom>+'a>;
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


pub struct GcValue {
    pub data:WeakValueInner, //Weak<Mutex<dyn Any+Send>>,
    pub gc_index:GcWeakIndex,
    pub root_count:GcRootCount,
    pub root:bool,
    // pub dropper : Option<GcDropper>, //always set unless an empty ... are empties necessary?
    pub dropper : GcDropper,
}

impl Drop for GcValue {
    fn drop(&mut self) {
        if self.root {
            self.root_count.decr().unwrap();
        }

        if self.gc_index.weak_count() == 1 {
            let _=self.dropper.add(self.gc_index.clone());
        }
        // // self.root=false;

        //renable below
        // if self.gc_index.weak_count() == 1 {
        //     // if let Some(dropper)=&mut self.dropper {
        //     //     dropper.add(self.gc_index.clone());
        //     // }

        //     self.dropper.as_mut().unwrap().add(self.gc_index.clone());
        // }

    }
}

impl Clone for GcValue {
    fn clone(&self) -> Self {
        // self.clone_to(None)


        // Self {
        //     data:self.data.clone(),
        //     gc_index:self.gc_index.clone(),
        //     root_count:self.root_count.clone(),
        //     root:false,
        //     dropper : self.dropper.clone(),
        // }

        // self.clone_leaf()
        self.clone_as_is()
    }
}

impl GcValue {
    pub fn is_alive(&self) -> bool {
        self.root_count.strong_count()!=0
    }
    pub fn new<T:GcTraversable>(data : T, gc_scope: &mut GcScope) -> Self {

        let data=Arc::new(Mutex::new(data));

        // let type_name=std::any::type_name::<T>();

        let (val_index, root_count)=gc_scope.new_other(
            GcManagedInner::Mut(Arc::clone(&data) as _),
            // type_name
            TypeInfo::new::<T>()
        );
        // root_count.incr();

        Self {
            data: WeakValueInner::Mut(Arc::downgrade(&data) as _), //Arc::downgrade(&data) as _,
            root: false,//true,
            gc_index: val_index,
            root_count,
            // dropper : Some(gc_scope.get_dropper()),
            dropper : gc_scope.get_dropper(),
        }
    }


    pub fn new_non_mut<T:GcTraversable+Sync>(data : T, gc_scope: &mut GcScope) -> Self {
        let data=Arc::new(data);

        let (val_index, root_count)=gc_scope.new_other(
            GcManagedInner::NonMut(Arc::clone(&data) as _),
            TypeInfo::new::<T>()
        );

        Self {
            data: WeakValueInner::NonMut(Arc::downgrade(&data) as _), //Arc::downgrade(&data) as _,
            root: false,//true,
            gc_index: val_index,
            root_count,
            // dropper : Some(gc_scope.get_dropper()),
            dropper : gc_scope.get_dropper(),
        }
    }

    //


    // pub fn new_ext<T:GcTraversableExt>(data : T, gc_scope: &mut GcScope) -> Self {

    //     let data=Arc::new(Mutex::new(data));

    //     // let type_name=std::any::type_name::<T>();

    //     let (val_index, root_count)=gc_scope.new_other(
    //         GcManagedInner::MutExt(Arc::clone(&data) as _),
    //         // type_name
    //         TypeInfo::new::<T>()
    //     );
    //     // root_count.incr();

    //     Self {
    //         data: WeakValueInner::MutExt(Arc::downgrade(&data) as _), //Arc::downgrade(&data) as _,
    //         root: false,//true,
    //         gc_index: val_index,
    //         root_count,
    //         dropper : Some(gc_scope.get_dropper()),
    //     }
    // }


    // pub fn new_non_mut_ext<T:GcTraversableExt+Sync>(data : T, gc_scope: &mut GcScope) -> Self {
    //     let data=Arc::new(data);

    //     let (val_index, root_count)=gc_scope.new_other(
    //         GcManagedInner::NonMutExt(Arc::clone(&data) as _),
    //         TypeInfo::new::<T>()
    //     );

    //     Self {
    //         data: WeakValueInner::NonMutExt(Arc::downgrade(&data) as _), //Arc::downgrade(&data) as _,
    //         root: false,//true,
    //         gc_index: val_index,
    //         root_count,
    //         dropper : Some(gc_scope.get_dropper()),
    //     }
    // }


    // pub fn _empty<T:Any+Send>() -> Self { //GcTraversable+
    //     let data=Weak::<Mutex<T>>::new();

    //     Self {
    //         data:WeakValueInner::Mut(data), //data,
    //         root: false,
    //         gc_index: GcWeakIndex::new(),
    //         root_count:GcRootCount::new(),
    //         // droppeds:Weak::new(),
    //         dropper : None,
    //     }
    // }
    // pub fn clone_to(&self,to_root:Option<bool>) -> Self {
    //     let root = match to_root {
    //         Some(true)=> {
    //             self.root_count.incr();
    //             true
    //         }
    //         Some(false)=> {
    //             false
    //         }
    //         None => {
    //             if self.root {
    //                 self.root_count.incr();
    //             }

    //             self.root
    //         }
    //     };

    //     Self {
    //         data:self.data.clone(),
    //         gc_index:self.gc_index.clone(),
    //         root_count:self.root_count.clone(),
    //         root,
    //     }
    // }

    pub fn clone_root(&self) -> Self {
        self.root_count.incr().unwrap();

        Self {
            data:self.data.clone(),
            gc_index:self.gc_index.clone(),
            root_count:self.root_count.clone(),
            root:true,
            dropper:self.dropper.clone(),
        }
    }

    pub fn clone_as_is(&self) -> Self {
        if self.root {
            self.root_count.incr().unwrap();
        }

        Self {
            data:self.data.clone(),
            gc_index:self.gc_index.clone(),
            root_count:self.root_count.clone(),
            root:self.root,
            dropper:self.dropper.clone(),
        }
    }
    pub fn clone_leaf(&self) -> Self {
        Self {
            data:self.data.clone(),
            gc_index:self.gc_index.clone(),
            root_count:self.root_count.clone(),
            root:false,
            dropper:self.dropper.clone(),
        }
    }
    // pub fn data(&self) -> Option<GcValueTyping2> //Option<Arc<Mutex<dyn Any+Send >>>
    // {
    //     match &self.data {
    //         GcValueTyping::Mut(x) => {
    //             x.upgrade().map(|x|GcValueTyping2::Mut(x))
    //         }
    //         GcValueTyping::NonMut(x) => {
    //             x.upgrade().map(|x|GcValueTyping2::NonMut(x))
    //         }
    //     }
    //     // self.data.upgrade()
    // }

    pub fn gc_index(&self) -> Result<Option<usize>,()> {
        if let Some(x)=self.gc_index.to_strong() {
            Ok(Some(x.get()?))
        } else {
            Ok(None)
        }

        // self.gc_index.to_strong().and_then(|x|Some(x.get()))
    }

    // pub fn ref_count(&self) -> usize {
    //     self.gc_index.weak_count()+self.gc_index.strong_count()
    // }

    pub fn _strong_count(&self) -> usize {
        self.gc_index.strong_count()
    }
    pub fn _weak_count(&self) -> usize {
        self.gc_index.weak_count()
    }
}


