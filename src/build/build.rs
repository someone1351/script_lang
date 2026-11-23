use std::{collections::HashMap, path::{Path, PathBuf}, sync::Arc};

// use crate::script_lang::common::build;

use super::{Instruction, Loc, };

use super::super::primitive_types::*;

#[derive(Clone,Debug,Default)]
pub struct BuildFunc {
    // pub instruct_start_ind : usize,
    // pub instruct_end_ind : usize,
    // pub variadic : bool,
    // pub captures_num : usize,

    pub instruct_start_pos : usize,
    pub instruct_len : usize,
    pub params_num : usize,

    pub name: Option<StringVal>,
}


// #[derive(Clone,Debug,Default)]
// pub struct Include {
//     pub path : PathBuf,
//     pub first_loc : Option<Loc>,
// }

#[derive(Default)]
pub struct Build {
    pub includes : HashMap<Arc<PathBuf>,Loc>,
    // pub symbols : Vec<StringT>, //rc?
    pub instructions : Vec<Instruction>,
    pub functions : Vec<BuildFunc>,
    pub main_instruct_len : usize,

    pub path:Option<PathBuf>,
    pub version:usize,
    pub src:Option<StringVal>,

    pub instr_locs : HashMap<usize,Loc>,
    // pub include_first_locs : HashMap<usize,Loc>,

    pub instr_locs_alt : Vec<(usize,Option<Loc>)>, //[(start_instr_pos,)]

    pub instr_stack_var_names : HashMap<usize,usize>, //instr_ind, symbol_ind

    // pub func_names : HashMap<usize,usize>, //func_ind, symbol_ind
    // pub func_locs : HashMap<usize,Loc>, //func_ind, //can get from instr_locs[functions[i].instruct_start_pos]
}
impl std::fmt::Debug for Build {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "Build({:?})", self.path.clone().unwrap_or(PathBuf::from("_")))
        write!(f, "Build({})", self.path.as_ref().and_then(|x|Some(x.to_string_lossy().to_string())).unwrap_or("_".to_string()))
    }
}
impl Build {
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref().map(|p|p.as_path())
    }

    //google says so
    pub fn print(&self) {
        println!(
            "Build ({:?})",
            self.path.as_deref().unwrap_or(Path::new("_"))
        );

        let main_end = self
            .functions
            .first()
            .map(|f| f.instruct_start_pos)
            .unwrap_or(self.instructions.len());

        println!("    main instructions ({main_end}):");
        for (i, instruction) in self.instructions[0..main_end].iter().enumerate() {
            let loc_str = self
                .instr_locs
                .get(&i)
                .map(|l| format!(" @ {l}"))
                .unwrap_or_default();
            println!("        {i}: {instruction:?}{loc_str}");
        }

        println!("    functions ({}):", self.functions.len());
        for (i, func) in self.functions.iter().enumerate() {
            let name = func
                .name
                .as_ref()
                .map(|n| n.as_str())
                .unwrap_or("<anonymous>");
            let func_end = func.instruct_start_pos + func.instruct_len;
            println!(
                "        {i} ({name}): params={}, range={}..{}",
                func.params_num, func.instruct_start_pos, func_end
            );

            for (j, instruct) in self.instructions[func.instruct_start_pos..func_end]
                .iter()
                .enumerate()
            {
                let abs_pos = func.instruct_start_pos + j;
                let loc_str = self
                    .instr_locs
                    .get(&abs_pos)
                    .map(|l| format!(" @ {l}"))
                    .unwrap_or_default();
                println!("            {j} ({abs_pos}): {instruct:?}{loc_str}");
            }
        }
    }

}

