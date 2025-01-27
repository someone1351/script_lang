
mod error;
mod debug;
mod machine;

mod gc_scope;
mod var_scope;
mod lib_scope;

mod value;
mod custom;
mod data;

mod func_context;
pub mod libs;


pub use error::MachineError;

pub use machine::Machine;

pub use value::Value;
pub use custom::Custom;

pub use var_scope::VarScope;
pub use lib_scope::LibScope;
pub use gc_scope::{GcScope,GcTraversable};

pub use func_context::FuncContextExt;

pub use data::*;

// pub use libs::fields::{default_get_field,default_set_field};
// pub use libs as interpreter_libs;

