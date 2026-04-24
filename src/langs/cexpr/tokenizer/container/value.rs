

use super::*;


#[derive(Clone,Copy)]
pub struct ValueContainer<'a,T> {
    pub primitive : TokenContainer<'a>,
    pub value : T,
}
