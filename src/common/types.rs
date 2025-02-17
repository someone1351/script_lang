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
impl std::borrow::Borrow<std::string::String> for StringT {
    fn borrow(&self) -> &std::string::String {
        &self.0
    }
}
impl std::borrow::Borrow<str> for StringT {
    fn borrow(&self) -> &str {
        &self.0.as_str()
    }
}
// impl Equivalent<StringT> for String {
    
// }

// impl std::hash::Hash for StringT {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//     }
// }

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