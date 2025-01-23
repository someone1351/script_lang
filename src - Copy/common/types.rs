use std::{ops::Deref, path::Path, sync::Arc};

use super::{Build, Instruction};

pub type FloatT = f64;
pub type IntT = i64;

#[derive(Clone,Debug,Hash,PartialEq, Eq)]
pub struct StringT(Arc<String>);

impl StringT {
    pub fn new<S: Into<String>>(s:S) -> Self { //AsRef<str>
        Self(Arc::new(s.into()))
    }
}

impl Deref for StringT {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}


#[derive(Clone,Debug)]
pub struct BuildT(Arc<Build>);

impl BuildT {
    pub fn new(b:Build) -> Self {
        Self(Arc::new(b))
    }

    pub fn from_paths(paths : Vec<&Path>) -> Self {
        let mut b = Build::default();

        for (i,p) in paths.iter().enumerate() {
            b.instructions.push(Instruction::Include(i));
            b.includes.push(p.to_path_buf());
        }
        
        b.main_instruct_len=paths.len();

        Self::new(b)
    }
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self::from_paths(vec![path.as_ref()])
    }

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