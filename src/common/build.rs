use std::{path::{PathBuf, Path}, collections::HashMap};

// use crate::script_lang::common::build;

use super::{Instruction, Loc, StringT};


#[derive(Clone,Debug,Default)]
pub struct BuildFunc {
    // pub instruct_start_ind : usize,
    // pub instruct_end_ind : usize,
    // pub variadic : bool,
    // pub captures_num : usize,

    pub instruct_start_pos : usize,
    pub instruct_len : usize,
    pub params_num : usize,
}


// #[derive(Clone,Debug,Default)]
// pub struct Include {
//     pub path : PathBuf,
//     pub first_loc : Option<Loc>,
// }

#[derive(Clone,Default)]
pub struct Build {
    pub includes : Vec<PathBuf>,
    pub symbols : Vec<StringT>, //rc?
    pub instructions : Vec<Instruction>,
    pub functions : Vec<BuildFunc>,
    pub main_instruct_len : usize,

    pub path:Option<PathBuf>,
    pub version:usize,
    pub src:Option<StringT>,

    pub instr_locs : HashMap<usize,Loc>,
    pub include_first_locs : HashMap<usize,Loc>,
    
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
    pub fn print(&self) {
        println!("Build");

        //
        println!("    includes");

        for (i,include) in self.includes.iter().enumerate() {
            println!("        {i}:{include:?}");
        }

        //
        println!("    symbols");

        for (i,symbol) in self.symbols.iter().enumerate() {
            println!("        {i}:{symbol:?}");
        }

        //
        println!("    functions");
        // for i in 1 .. self.functions.len() {

        // }

        for (i,func) in self.functions.iter().enumerate() {
            println!("        {i}: {}",func.params_num); //,func.variadic,func.instruct_start_pos
            let instruct_ind2 = self.functions.get(i+1).and_then(|x|Some(x.instruct_start_pos)).unwrap_or(self.instructions.len());
            // let instruct_ind2 = if i==0{0}else{self.functions.get(i-1).unwrap().instruct_end_ind};
            // println!("-- {} {}",func.instruct_ind,instruct_ind2);
            for (j, instruct) in self.instructions[func.instruct_start_pos .. instruct_ind2].iter().enumerate() {
                println!("            {j} ({}): {instruct:?}",func.instruct_start_pos+j);
            }
        }

        //
        println!("    main instructions");
        // let main_instr_start_ind = self.functions.last().and_then(|x|Some(x.instruct_end_ind)).unwrap_or(0);
        let instruct_ind2 = self.functions.get(0).and_then(|x|Some(x.instruct_start_pos)).unwrap_or(self.instructions.len());

        for (i,instruction) in self.instructions[0 .. instruct_ind2].iter().enumerate() {
            println!("        {i}: {instruction:?}");
        }

        // println!("{:?}",self.instructions);

        //
        // println!("==");
        // for (i,instruction) in self.instructions.iter().enumerate() {
        //     println!("        {i}: {instruction:?}");

        // }
        // println!("==");
        // for (i,func) in self.functions.iter().enumerate() {
        //     println!("        {i}: {:?}",func.instruct_ind);
        // }

    }
}

