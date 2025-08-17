/*
TODO
* for mark_and_sweep amount
- need to make sure variables that need to be cleaned up don't increase faster than they are cleaned up
- when clearing marked, make it so it can do up to a limit, and continue on from next time after reaching limit

TODO 2
* make failed locks etc not panic and crash the program, return an error instead

*/

#![allow(dead_code)]

use std::{any::Any, sync::{Arc, Mutex, Weak} };

// use parking_lot::Mutex;

use super::{custom::WeakValueInner, value::*};
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
    fn new(index:usize) -> Self { Self(Arc::new(Mutex::new(index))) }
    fn set(&mut self,index:usize) -> Result<(),()> {
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

    fn to_weak(&self) -> GcWeakIndex {
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
    fn new() -> Self {
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
    fn get(&self)->Result<usize,()> {
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
struct GcManaged {
    // data : Arc<Mutex<Box<dyn Any + Send>>>,
    // traverser:Box<dyn TraverserTrait+Send>,

    data : GcManagedInner, //Arc<Mutex<dyn GcTraversable>>,
    // type_name : &'static str,
    // type_info:TypeInfo, //unused?
    type_info:TypeInfo,


    managed_index:GcIndex,
    root_count:GcRootCount,


    marked : bool,
}

impl GcManaged {
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
#[derive(Default)]

pub struct GcScope {
    manageds : Vec<GcManaged>,
    dropper : GcDropper, //drop managed inds
    // stk : Vec<usize>,
}

// impl Default for GcScope {
//     fn default() -> Self {
//         Self {

//         }
//     }
// }
impl GcScope {
    pub fn new() -> Self {
        Self {
            manageds : Vec::new(),
            // dropper : Arc::new(Mutex::new(Vec::new())),
            dropper : GcDropper::default(),
            // stk : Vec::new(),
        }
    }

    pub fn new_other(&mut self,
        data:GcManagedInner, //Arc<Mutex<dyn GcTraversable>>,
        type_info:TypeInfo, //unused?
        // type_name:&'static str
    ) -> (GcWeakIndex,GcRootCount) {
        let val_index=GcIndex::new(self.manageds.len());
        let val_weak_index= val_index.to_weak();
        let root_count=GcRootCount::new();
        // root_count.incr();

        self.manageds.push(GcManaged {
            // inner : GcManagedInner::Other { data, },
            data,
            type_info, //unused?

            managed_index : val_index,
            root_count:root_count.clone(),
            // data,
            // traverser,
            marked:false,
        });

        (val_weak_index,root_count)
    }

    pub fn get_dropper(&self) -> GcDropper {
        // Arc::downgrade(&self.droppeds)
        self.dropper.clone()
    }


    pub fn test(&self) {
        // println!("gc_scope_test:");
        // for (i,managed) in self.manageds.iter().enumerate() {
        //     println!("\t{i}: {}", managed.type_info.short_name() );

        //     // println!("{i}: {:?}", managed.data.lock().);


        //     // if let Some(x)=&c.traverser
        //     // if let GcManagedInner::Custom { data, traverser }=&c.inner
        //     {

        //         let data=managed.data.lock();
        //         // let children=traverser.call(data.as_ref());
        //         let children=data.traverser();
        //         for a in children {
        //             println!("\t\t {:?}",a);
        //         }
        //     }
        //     // else {
        //     //     println!("\t Nothing");
        //     // }
        // }

        #[derive(Debug)]
        struct Test {
            managed_index:usize,
            gc_index:usize,
            root_count:usize,
            name:String,
            children:Vec<Child>,

        }

        #[derive(Debug)]
        struct Child {
            gc_index:Result<Option<usize>, ()>,
            name:String,
        }
        let mut test=Vec::new();
        for (managed_index,managed) in self.manageds.iter().enumerate() {

            let gc_index=managed.managed_index.get().unwrap();
            let root_count=managed.root_count.get().unwrap();
            let name=managed.type_info.short_name();
            let mut children=Vec::new();

            managed.with_data(|data|{
                for x in data.traverser() {
                    children.push(Child{gc_index:x.gc_index(),name:x.type_string()});
                }
                Ok(())
            }).unwrap();


            test.push(Test{ managed_index, gc_index, root_count, name, children });

        }
        println!("test {test:?}");
    }


    pub fn mark_and_sweep(&mut self, ) ->Result<(),()> { //amount:usize
        // for managed in self.manageds.iter_mut() {
        //     managed.marked=true;
        // }


        // let mut cur_val_ind=0;
        // let mut stk=self.manageds.iter().enumerate()
        //     .filter_map(|(managed_ind,managed)|{
        //         if managed.root_count.get()>0 //||Arc::weak_count(&val.data)>0
        //         {
        //             Some(managed_ind)
        //         } else {
        //             None
        //         }
        //         // if val.root_count.get()==0||Arc::weak_count(&val.data)>0 {
        //         //     None
        //         // }else{
        //         //     Some(val_ind)
        //         // }

        //     })
        //     .collect::<Vec<_>>();

        //
        //could loop through all manageds, check for weak_count==0, and remove, and keep looping over whole thing again til no weak_count==0
        //  or use drop trait, to update list to remove
        // loop {
        //     let Ok(droppeds)=self.dropper.drain() else {
        //         break;
        //     };


        // }


        //mark
        for managed in self.manageds.iter_mut() {
            managed.marked=true;
        }

        // println!("marking {:?}",(0..self.manageds.len()).collect::<Vec<usize>>());

        //
        let mut stk = Vec::new();

        //push roots on stk
        for (gc_index,managed) in self.manageds.iter().enumerate() {
            if managed.root_count.get()? > 0 {
                stk.push(gc_index);
            }
        }

        // println!("stk init {stk:?}");

        //sweep
        while let Some(managed_ind)=stk.pop() {
            let managed=self.manageds.get_mut(managed_ind).unwrap();

            if !managed.marked { //already done
                // println!("skipping {managed_ind}");
                continue;
            }

            managed.marked=false;

            // println!("unmarking {managed_ind}");

            // let managed=&*managed;
            let managed=self.manageds.get(managed_ind).unwrap();

            managed.with_data(|data|{
                for child_val in data.traverser() {
                    if let Some(gc_index)=child_val.gc_index()? {
                        if self.manageds.get(gc_index).unwrap().marked { //not done yet
                            stk.push(gc_index);

                            // println!("stk push {gc_index}");
                        }
                    }
                }
                Ok(())
            })?;
        }

        //remove
        for gc_index in (0..self.manageds.len()).rev() {
            let managed=self.manageds.get(gc_index).unwrap();
            if managed.marked {
                // println!("removing {gc_index} {:?}",managed.type_info.short_name());
                self.manageds.swap_remove(gc_index);

                if let Some(managed)=self.manageds.get_mut(gc_index) {
                    managed.managed_index.set(gc_index)?;
                }
            }
        }


        // //

        // if self.stk.is_empty() {

        //     //remove marked
        //     for gc_index in (0..self.manageds.len()).rev() {
        //         if self.manageds.get(gc_index).unwrap().marked {

        //             self.manageds.swap_remove(gc_index);
        //             // println!("removing {val_ind}");

        //             if let Some(val)=self.manageds.get_mut(gc_index) {
        //                 val.managed_index.set(gc_index)?;
        //             }
        //         }
        //     }

        //     for managed in self.manageds.iter_mut() {
        //         managed.marked=true;
        //     }

        //     for (managed_ind,managed) in self.manageds.iter().enumerate() {
        //         if managed.root_count.get()?>0 {
        //             self.stk.push(managed_ind);
        //         }
        //     }
        //     // self.stk.extend(self.manageds.iter().enumerate().filter_map(|(managed_ind,managed)|{
        //     //     if managed.root_count.get()>0 //||Arc::weak_count(&val.data)>0
        //     //     {
        //     //         Some(managed_ind)
        //     //     } else {
        //     //         None
        //     //     }

        //     // }));
        // }

        // // stk.reserve(10000);
        // // println!("!! {stk:?}");

        // // println!("{:?}",self.manageds.iter().map(|x|(
        // //     x.managed_index.get(),
        // //     x.marked,
        // //     x.root_count.get(),
        // //     x.root_count.strong_count(),
        // // )).collect::<Vec<_>>());

        // let mut c=0;
        // // let mut d=0;

        // while let Some(cur_val_ind)=self.stk.pop() {
        //     let cur_val=self.manageds.get_mut(cur_val_ind).unwrap();
        //     // d+=1;
        //     if !cur_val.marked {
        //         continue;
        //     }

        //     c+=1;

        //     cur_val.marked=false;

        //     match &cur_val.data {
        //         GcManagedInner::Mut(x) => {
        //             if let Ok(x)=x.try_lock() {
        //                 for val in x.traverser() {
        //                     if let Some(gc_index)=val.gc_index()? {
        //                         self.stk.push(gc_index);
        //                     }
        //                 }
        //                 // self.stk.extend(x.traverser().filter_map(|val|val.gc_index()));
        //             } else {
        //                 return  Err(());
        //             }
        //         }
        //         GcManagedInner::NonMut(x) => {
        //             for val in x.traverser() {
        //                 if let Some(gc_index)=val.gc_index()? {
        //                     self.stk.push(gc_index);
        //                 }
        //             }

        //             // self.stk.extend(x.traverser().filter_map(|val|val.gc_index()));
        //         }
        //         // GcManagedInner::MutExt(x) => {
        //         //     self.stk.extend(x.lock().traverser().filter_map(|val|val.gc_index()));
        //         // }
        //         // GcManagedInner::NonMutExt(x) => {
        //         //     self.stk.extend(x.traverser().filter_map(|val|val.gc_index()));
        //         // }
        //     }
        //     // stk.extend(cur_val.data.lock().traverser()
        //     //     .filter_map(|val|val.gc_index()));
        //     // match &cur_val.inner {
        //     //     GcManagedInner::Custom { data, traverser }=>{
        //     //         stk.extend(traverser.call(data.lock().as_ref())
        //     //             .filter_map(|val|{if let Value::Custom(c)=val{c.val_index()}else{None}}));
        //     //     }
        //     //     GcManagedInner::Other { data }=>{}
        //     // }
        //     if c!=0 && c>amount {
        //         break;
        //     }

        // }

        // // println!("{c} {d}");

        // // //remove marked
        // // for gc_index in (0..self.manageds.len()).rev() {
        // //     if self.manageds.get(gc_index).unwrap().marked {

        // //         self.manageds.swap_remove(gc_index);
        // //         // println!("removing {val_ind}");

        // //         if let Some(val)=self.manageds.get_mut(gc_index) {
        // //             val.managed_index.set(gc_index);
        // //         }
        // //     }
        // // }


        // // println!(":{:?}",self.manageds.iter().map(|x|(x.managed_index.get(),x.marked,x.root_count.get())).collect::<Vec<_>>());

        // // self.dropper.clear();
        let _=self.dropper.clear();

        Ok(())
    }


    //should probably run in at the end of a code block, storing vals to check in a vec?
    pub fn remove_norefs(&mut self,
        // //val:GcValue
        // gc_index:usize,
        // recursive:bool,
    ) {
        // // return;

        // //renable below (can't use with mark_and_sweep anymore, since that has been changed to run partially, and this will disrupt that)
        // loop {
        //     let Ok(mut droppeds)=self.dropper.drain() else {
        //         break;
        //     };

        //     if droppeds.is_empty() {
        //         break;
        //     }

        //     // println!("manageds {:?}",self.manageds.iter().map(|x|{
        //     //     (x.managed_index.get().unwrap(),x.type_info.short_name())
        //     // }).collect::<Vec<_>>());



        //     while let Some(weak_gc_index)=droppeds.pop() {
        //         let Some(gc_index)=weak_gc_index.to_strong() else {
        //             // panic!("a");
        //             continue;
        //         };
        //         let Ok(managed_ind) = gc_index.get() else {
        //             // break;
        //             panic!("b");
        //         };

        //         // self.manageds.get(managed_ind).unwrap().data.

        //         self.manageds.swap_remove(managed_ind);

        //         if self.manageds.is_empty() {
        //             continue;
        //         }
        //         //update managed_ind for the moved managed
        //         let Some(val)=self.manageds.get_mut(managed_ind) else {
        //             panic!("c {} of {}",managed_ind,self.manageds.len());
        //         };

        //         val.managed_index.set(managed_ind).unwrap();
        //             // if val.managed_index.set(managed_ind).is_err() { }

        //     }
        // }

    }
}


#[derive(Default,Clone)]
pub struct GcDropper {
    pub droppeds : Arc<Mutex<Vec<GcWeakIndex>>>,
}
impl GcDropper {
    // pub fn new() -> Self {
    //     Self {
    //         droppeds : Arc::new(Mutex::new(Vec::new())),
    //     }
    // }
    pub fn add(&mut self,gc_index:GcWeakIndex) -> Result<(),()> {
        let Ok(mut droppeds)=self.droppeds.try_lock() else {return Err(());};
        droppeds.push(gc_index);
        Ok(())
    }
    pub fn clear(&mut self) -> Result<(),()> {
        let Ok(mut droppeds)=self.droppeds.lock() else {return Err(());};
        droppeds.clear();
        Ok(())
    }
    pub fn drain(&mut self) -> Result<Vec<GcWeakIndex>,()> {
        let Ok(mut droppeds)=self.droppeds.lock() else {return Err(());};
        Ok(droppeds.drain(0 ..).collect())
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


