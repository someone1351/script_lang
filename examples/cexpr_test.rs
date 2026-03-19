// #![feature(mapped_lock_guards)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #[allow(unused_parens)]

use std::path::Path;


fn main() {

    let path=Path::new("scripts/cexpr/test16.script");
    let src = std::fs::read_to_string(path).unwrap();
    let compiler=script_lang::langs::cexpr_compiler::Compiler::new();
    let res=compiler.compile(src.as_str(), 0, Some(path), true);
    if let Err(e)=res {
        println!("{e}");
    }


}
