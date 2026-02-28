mod instruction;
mod loc;
mod build;
mod error;

use std::{ops::Deref, sync::Arc};

// pub use instruction::Instruction;
pub use instruction::*;
pub use loc::*;
pub use error::*;
pub use build::*;


#[derive(Clone,Debug)]
pub struct BuildT(Arc<Build>);

impl BuildT {
    pub fn new(b:Build) -> Self {
        Self(Arc::new(b))
    }

    // pub fn from_paths(paths : Vec<&Path>) -> Self {
    //     let mut b = Build::default();

    //     for (i,p) in paths.iter().enumerate() {
    //         b.instructions.push(Instruction::Include(i));
    //         b.includes.push(p.to_path_buf());
    //     }

    //     b.main_instruct_len=paths.len();

    //     Self::new(b)
    // }
    // pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
    //     Self::from_paths(vec![path.as_ref()])
    // }

}

impl Deref for BuildT {
    type Target = Build;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}


impl Default for BuildT {
    fn default() -> Self {
        Self(Arc::new(Build::default()))
    }
}