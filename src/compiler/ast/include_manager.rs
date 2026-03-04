use std::{collections::HashMap, path::PathBuf, sync::Arc};



use super::super::super::build::*;

pub struct IncludeManager {
    // inds : HashMap::<String,(usize,Option<Loc>)>,
    inds : HashMap::<String,(Arc<PathBuf>,Loc)>,
}

impl IncludeManager {
    pub fn new() -> Self {
        Self {
            inds:HashMap::new(),
        }
    }
    // pub fn get(&mut self, symbol : &str, loc:Option<Loc>) -> usize {
    //     let num = self.inds.len();
    //     let x=self.inds.entry(symbol.to_string()).or_insert((num,loc));
    //     x.0
    // }

    pub fn get(&mut self, symbol : &str, loc:Loc) -> Arc<PathBuf> {
        let x=self.inds.entry(symbol.to_string()).or_insert((Arc::new(PathBuf::from(symbol)),loc)).clone();
        x.0
    }

    pub fn to_map(&self) -> Vec<(Arc<PathBuf>,Loc)> {
        self.inds.iter().map(|(_k,v)|v.clone()).collect()
    }
    // pub fn to_paths(&self) -> Vec<PathBuf> {
    //     let mut symbols = Vec::<PathBuf>::new();
    //     symbols.resize(self.inds.len(), PathBuf::new());

    //     for (s,&(ind,_)) in self.inds.iter() {

    //         *symbols.get_mut(ind).unwrap()=PathBuf::from(s);

    //     }

    //     symbols
    // }


    // pub fn to_locs(&self) -> HashMap<usize,Loc> {
    //     let mut symbols = HashMap::new();

    //     for (_,&(ind,loc)) in self.inds.iter() {
    //         if let Some(loc)=loc {
    //             symbols.insert(ind, loc);
    //         }
    //     }

    //     symbols
    // }
}
