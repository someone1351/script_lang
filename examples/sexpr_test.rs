
use std::path::Path;
use script_lang::langs::sexpr_compiler::Compiler;

pub fn test_script<P:AsRef<Path>>(path:P) {

    let mut gc_scope= script_lang::GcScope::new();

    let path = path.as_ref();
    let src = std::fs::read_to_string(path).unwrap();

    let compiler = Compiler::new_core();
    let build = compiler.compile(src.as_str(), 0, None, true,
        false
    );

    if let Err(e)=&build {
        eprintln!("In {path:?}, {}",e.msg());
    } else {

        let mut var_scope=script_lang::VarScope::new();
        var_scope.decl("self", Some(script_lang::Value::int(4))).unwrap();
        // let x=5;
        let  lib_scope=script_lang::LibScope::new_full();

        // let mut core=&x;
        let mut core=();
        let mut machine = script_lang::Machine::new(&mut gc_scope,&lib_scope,&mut var_scope,   &mut core);
        // machine.set_debug_print(true);

        // build.clone().unwrap().print();

        let res=machine.run_build(&build.unwrap());
        println!("the result is {res:?}");

        if let Err(e)=&res {
            e.eprint(None);
            machine.debug_print_stack_trace(true);
            machine.debug_print_stack();
            // machine.print_state();
        }
        machine.debug_print_stack();
    }

    //gc_scope.mark_and_sweep();
    gc_scope.test();


}

fn main() {
    test_script("scripts/sexpr/test.script");
}