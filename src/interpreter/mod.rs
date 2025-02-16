
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
mod mem_scope;

pub mod libs;

pub use mem_scope::*;

pub use error::MachineError;

pub use machine::Machine;

pub use value::Value;
pub use custom::Custom;

pub use var_scope::VarScope;
pub use lib_scope::LibScope;
pub use gc_scope::{GcScope,GcTraversable,Traverser};

pub use func_context::FuncContext;

pub use data::*;

pub use libs::array::Array;

pub use libs::dict::Dict;

pub use libs::math::*;

// pub use libs::fields::{default_get_field,default_set_field};
// pub use libs as interpreter_libs;

