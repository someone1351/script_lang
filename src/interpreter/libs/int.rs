
// use super::super::super::common::*;
// use super::super::func_context::*;
use super::super::value::*;
// use super::super::error::*;
use super::super::lib_scope::*;



pub fn register<X>(lib_scope : &mut LibScope<X>) {
    
	//
    lib_scope.method("is_int", |_|Ok(Value::Bool(true)))
        .int().end();

    lib_scope.method("is_int", |_|Ok(Value::Bool(false)))
        .any().end();



}