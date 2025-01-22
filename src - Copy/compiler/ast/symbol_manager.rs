use std::collections::HashMap;

use super::super::super::common::*;

pub struct SymbolManager {
    inds : HashMap::<String,usize>,
}

impl SymbolManager {
    pub fn new() -> Self {
        Self {
            inds:HashMap::new(),
        }
    }
    pub fn get(&mut self, symbol : &str) -> usize {
        let num = self.inds.len();
        *self.inds.entry(symbol.to_string()).or_insert(num)
    }
    pub fn to_vec(&self) -> Vec<StringT> {
        let mut symbols = Vec::<String>::new();
        symbols.resize(self.inds.len(), String::new());

        for (s,&i) in self.inds.iter() {
            *symbols.get_mut(i).unwrap()=s.clone();
        }

        symbols.iter().map(|x|StringT::new(x.clone())).collect::<_>()
    }
}