

use super::*;


#[derive(Clone,Copy)]
pub struct ValueContainer<'a,T> {
    pub primitive : PrimitiveContainer<'a>,
    pub value : T,
}
