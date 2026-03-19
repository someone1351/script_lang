
// use std::any::{Any,TypeId};
// use std::collections::BTreeMap;

// use crate::{Custom, GcTraversable};

// use crate::custom_data::*;

use crate::{BuildT, StringVal};

// use super::super::common::*;
// use super::custom::*;
// use super::error::*;
use super::value::*;
// use super::gc_scope::*;

pub struct Vararg;

#[derive(Clone)]
pub struct GlobalAccessRef{
    pub name:StringVal,
    pub var:Value,
}

impl GcTraversable for GlobalAccessRef {
    fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=GcValue>+'a> {
        Box::new([&self.var].into_iter().filter_map(|v|v.gc_value()))
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
    fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=GcValue>+'a> {
        Box::new(self.captures.iter().filter_map(|v|v.gc_value()))
    }
}

impl GcTraversable for Value {
    fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=GcValue>+'a> {
        Box::new([self].into_iter().filter_map(|v|v.gc_value()))
    }
}

