
// use std::ops::Range;

/*
TODO
* make new record out of existing one by specifying param range eg param_start,param_end (make sure can accumulate eg record=>new_record1=>new_record2)
* take single record from existing block, and return in its own block
*/
mod block;
mod record;
mod primitive;
mod primitive_type;
mod param;
mod field;
mod record_iter;
mod param_iter;
mod field_iter;

pub use block::*;
pub use record::*;
pub use primitive::*;
pub use primitive_type::*;
pub use param::*;
pub use field::*;
pub use record_iter::*;
pub use param_iter::*;
pub use field_iter::*;




