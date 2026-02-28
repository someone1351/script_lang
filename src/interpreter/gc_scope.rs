/*
TODO
* for mark_and_sweep amount
- need to make sure variables that need to be cleaned up don't increase faster than they are cleaned up
- when clearing marked, make it so it can do up to a limit, and continue on from next time after reaching limit

TODO 2
* make failed locks etc not panic and crash the program, return an error instead

*/

#![allow(dead_code)]

// use std::sync::{Arc, Mutex};

// use parking_lot::Mutex;

use super::gc::*;
// use super::error::*;
#[derive(Default)]

pub struct GcScope {
    manageds : Vec<GcManaged>,
    // dropper : GcDropper, //drop managed inds
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
            // // dropper : Arc::new(Mutex::new(Vec::new())),
            // dropper : GcDropper::default(),
            // // stk : Vec::new(),
        }
    }

    pub fn new_other(&mut self,
        data:GcManagedInner, //Arc<Mutex<dyn GcTraversable>>,
        // type_info:TypeInfo, //unused?
        // // type_name:&'static str
    ) -> GcValueNew {
        let val_index=GcIndex::new(self.manageds.len());
        let val_weak_index= val_index.to_weak();
        let root_count=GcRootCount::new();
        // root_count.incr();

        self.manageds.push(GcManaged {
            // inner : GcManagedInner::Other { data, },
            data,
            // type_info, //unused?

            managed_index : val_index,
            root_count:root_count.clone(),
            // data,
            // traverser,
            marked:false,
        });

        GcValueNew { val_index:val_weak_index, root_count }
    }

    // pub fn get_dropper(&self) -> GcDropper {
    //     // Arc::downgrade(&self.droppeds)
    //     self.dropper.clone()
    // }


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
            // let name=managed.type_info.short_name();
            let name="".to_string();
            let mut children=Vec::new();

            managed.with_data(|data|{
                for x in data.traverser() {
                    children.push(Child{gc_index:x.gc_index(),name:x.type_info().short_name()});
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

        // // // self.dropper.clear();
        // let _=self.dropper.clear();

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


// #[derive(Default,Clone)]
// pub struct GcDropper {
//     pub droppeds : Arc<Mutex<Vec<GcWeakIndex>>>,
// }
// impl GcDropper {
//     // pub fn new() -> Self {
//     //     Self {
//     //         droppeds : Arc::new(Mutex::new(Vec::new())),
//     //     }
//     // }
//     pub fn add(&mut self,gc_index:GcWeakIndex) -> Result<(),()> {
//         let Ok(mut droppeds)=self.droppeds.try_lock() else {return Err(());};
//         droppeds.push(gc_index);
//         Ok(())
//     }
//     pub fn clear(&mut self) -> Result<(),()> {
//         let Ok(mut droppeds)=self.droppeds.lock() else {return Err(());};
//         droppeds.clear();
//         Ok(())
//     }
//     pub fn drain(&mut self) -> Result<Vec<GcWeakIndex>,()> {
//         let Ok(mut droppeds)=self.droppeds.lock() else {return Err(());};
//         Ok(droppeds.drain(0 ..).collect())
//     }
// }

