
// use std::any::{Any,TypeId};
// use std::collections::BTreeMap;

use super::super::common::*;
// use super::custom::*;
// use super::error::*;
use super::value::*;
use super::gc_scope::*;

pub struct Vararg;

#[derive(Clone)] 
pub struct GlobalAccessRef{
    pub name:StringT,
    pub var:Value,
}

impl GcTraversable for GlobalAccessRef {
    fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Value>+'a> {
        Box::new([&self.var].into_iter())
    }
}
//??? todo make fields read only, because sometimes used as unmanaged, and dont want things being added to it??

// #[derive(Clone)]
#[derive(Clone)]
pub struct Closure {
    pub captures : Vec<Value>,
    pub build:BuildT,
    pub func_ind:usize,
}

impl GcTraversable for Closure {
    fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Value>+'a> {
        Box::new(self.captures.iter())
    }
}

impl GcTraversable for Value {
    fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Value>+'a> {
        Box::new([self].into_iter())
    }
}

