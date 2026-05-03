

use super::*;


#[derive(Clone,Copy,Debug)]
pub struct ValueContainer<'a,T> {
    pub primitive : TokenContainer<'a>,
    pub value : T,
}
