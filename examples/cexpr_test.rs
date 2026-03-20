// #![feature(mapped_lock_guards)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #[allow(unused_parens)]

use std::path::Path;


pub fn test_script<P:AsRef<Path>>(path:P,debug_compile:bool,debug:bool) {

    let mut gc_scope= script_lang::GcScope::new();

    let path = path.as_ref();
    let src = std::fs::read_to_string(path).unwrap();

    let compiler=script_lang::langs::texpr_compiler::Compiler::new();

    let build = compiler.compile(src.as_str(), 0, Some(path), true,
        // false
    );


    // build.clone().unwrap().print();

    let lib_scope=script_lang::LibScope::<()>::new_full();

    if debug_compile && build.is_ok() {
        build.as_ref().unwrap().print();
    }

    if let Err(e)=&build {
        eprintln!("In {path:?}, {}",e.msg());
    } else {
        let mut var_scope=script_lang::VarScope::new();

        let mut core_val =();
        let mut machine = script_lang::Machine::new(&mut gc_scope,&lib_scope, &mut var_scope,  &mut core_val);

        if debug {
            machine.set_debug_print(true);
            machine.set_debug_print_simple(true);
        }

        // build.clone().unwrap().print();

        let res=machine.run_build(&build.unwrap());
        println!("the result is {res:?}");

        if let Err(e)=&res {
            e.eprint(None);
            machine.debug_print_stack_trace(true);
            machine.debug_print_stack();
            // machine.print_state();
        }

        // machine.debug_print_stack();
    }

    gc_scope.mark_and_sweep().unwrap();
    gc_scope.test();


}

pub fn test_compile<P:AsRef<Path>>(path:P) {
    let path=path.as_ref();

    let src = std::fs::read_to_string(path).unwrap();
    let compiler=script_lang::langs::cexpr_compiler::Compiler::new();
    let res=compiler.compile(src.as_str(), 0, Some(path), true);

    if let Err(e)=res {
        println!("{e}");
    }
}

fn main() {
    // test_script("scripts/cexpr/test16.script",false,false);
    test_compile("scripts/cexpr/test16.script");
}
