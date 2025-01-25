pub mod array;
pub mod dict;
pub mod vararg;
pub mod float;
pub mod float_math;
pub mod int;
pub mod int_math;
pub mod vector;
pub mod matrix;
pub mod string;
pub mod utils;
pub mod misc;
pub mod vec;
pub mod vec_math;
pub mod ivec;

use super::lib_scope::*;

pub fn register_all<X>(lib_scope : &mut LibScope<X>) {
    
    misc::register(lib_scope);
    array::register(lib_scope);
    dict::register(lib_scope);
    vararg::register(lib_scope);

    float::register(lib_scope);
    float_math::register(lib_scope);
    int::register(lib_scope);
    int_math::register(lib_scope);

    string::register(lib_scope);
    vector::register(lib_scope);
    matrix::register(lib_scope);

    vec::register(lib_scope);
    vec_math::register(lib_scope);
    
}