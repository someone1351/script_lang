// use std::sync::{Arc, Mutex};

// pub struct MemScope {
//     obj_count:Arc<Mutex<usize>>,
//     str_count:Arc<Mutex<usize>>,
// }

// impl MemScope {
//     pub fn new() -> Self {
//         Self {
//             obj_count:Arc::new(Mutex::new(0)),
//             str_count:Arc::new(Mutex::new(0)),
//         }
//     }
//     pub fn obj_add(&self,amount:usize) -> Result<(),()> {
//         Ok(())
//     }
//     pub fn obj_remove(&self,amount:usize) -> Result<(),()> {
//         Ok(())
//     }
//     pub fn str_add(&self,amount:usize) -> Result<(),()> {
//         Ok(())
//     }
//     pub fn str_remove(&self,amount:usize) -> Result<(),()> {
//         Ok(())
//     }
// }