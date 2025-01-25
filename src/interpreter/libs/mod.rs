pub mod array;
pub mod dict;
pub mod vararg;
pub mod string;
pub mod utils;
pub mod misc;
pub mod math;

use super::lib_scope::*;

pub fn register_all<X>(lib_scope : &mut LibScope<X>) {
    
    misc::register(lib_scope);
    array::register(lib_scope);
    dict::register(lib_scope);
    vararg::register(lib_scope);


    string::register(lib_scope);

    math::register(lib_scope);
    
}