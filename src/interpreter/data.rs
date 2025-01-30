
// use std::any::{Any,TypeId};
use std::collections::BTreeMap;

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

#[derive(Clone)] 
pub struct Array(pub Vec<Value>);

impl GcTraversable for Array {
    fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Value>+'a> {
        Box::new(self.0.iter())
    }
}

#[derive(Clone,Default)] 
pub struct Dict(pub BTreeMap<String,Value>);

impl GcTraversable for Dict {
    fn traverser<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Value>+'a> {
        Box::new(self.0.values())
    }
}


// #[derive (Clone)]
// pub struct MathVec<const N: usize>(pub [FloatT;N]);

// impl<const N: usize> Default for MathVec<N> {
//     fn default() -> Self { 
//         Self([0.0 as FloatT;N])
//     }
// }

// pub type Vec2 = MathVec<2>;
// pub type Vec3 = MathVec<3>;
// pub type Vec4 = MathVec<4>;

// impl<const N: usize> MathVec<N> {
//     pub fn new<I: IntoIterator<Item=FloatT>>(a:I) -> Self {
//         let mut b=[0.0 as FloatT;N];

//         for (i,x) in a.into_iter().enumerate() {
//             b[i]=x;
//         }

//         Self(b)
//     }

//     pub fn from_f32<I: IntoIterator<Item=f32>>(a:I) -> Self {
//         Self::new(a.into_iter().map(|x|x as FloatT))
//     }

//     pub fn from_f64<I: IntoIterator<Item=f64>>(a:I) -> Self {
//         Self::new(a.into_iter().map(|x|x as FloatT))
//     }
    
//     pub fn as_f32(&self) -> [f32;N] {
//         let mut a = [0.0 as f32;N];
        
//         for (i,&x) in self.0.iter().enumerate() {
//             a[i]=x as f32;
//         }

//         a
//     }
//     pub fn as_f64(&self) -> [f64;N] {
//         let mut a = [0.0 as f64;N];
        
//         for (i,&x) in self.0.iter().enumerate() {
//             a[i]=x as f64;
//         }

//         a
//     }
// }

// pub struct Mat<const R: usize,const C: usize>(pub [[FloatT;C];R]);
