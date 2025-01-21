pub mod array;
pub mod dict;
pub mod vararg;
pub mod float;
pub mod int;
pub mod boolean;
pub mod vector;
pub mod matrix;
pub mod string;
pub mod utils;
pub mod misc;

use super::lib_scope::*;

pub fn register_all<X>(lib_scope : &mut LibScope<X>) {
    
    misc::register(lib_scope);
    array::register(lib_scope);
    dict::register(lib_scope);
    vararg::register(lib_scope);

    float::register(lib_scope);
    int::register(lib_scope);
    boolean::register(lib_scope);
    string::register(lib_scope);
    vector::register(lib_scope);
    matrix::register(lib_scope);
    
}