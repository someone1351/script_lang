use super::super::super::common::*;

use super::super::func_context::*;
use super::super::value::*;
use super::super::error::*;
use super::super::lib_scope::*;

use super::super::data::*;

#[derive(Clone)]
struct IVec2([IntT;2]);
#[derive(Clone)]
struct IVec3([IntT;3]);
#[derive(Clone)]
struct IVec4([IntT;4]);

pub fn register<X>(func_scope : &mut LibScope<X>) {

}