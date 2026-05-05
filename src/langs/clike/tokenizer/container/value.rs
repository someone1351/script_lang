

use super::*;


#[derive(Clone,Copy,)]
pub struct ValueContainer<'a,T> {
    pub primitive : TokenContainer<'a>,
    pub value : T,
}
impl<'a,T> std::fmt::Debug for ValueContainer<'a,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self.primitive)
        // f.debug_struct("ValueContainer").field("primitive", &self.primitive).field("value", &self.value).finish()
    }
}