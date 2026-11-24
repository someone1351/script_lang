// #![feature(mapped_lock_guards)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #[allow(unused_parens)]

use langs::clike::compiler::*;

use std::{collections::HashMap, path::{Path, PathBuf}};

use script_lang::*;

pub fn test_script<P:AsRef<Path>>(path:P,debug_compile:bool,debug:bool) {

    let mut gc_scope= GcScope::new();

    let path = path.as_ref();
    let src = std::fs::read_to_string(path).unwrap();

    let compiler=Compiler::new();

    let build = compiler.compile(src.as_str(), 0, Some(path), true,
        // false
    );

    // build.clone().unwrap().print();

    let lib_scope=LibScope::<()>::new_full();

    if debug_compile && build.is_ok() {
        build.as_ref().unwrap().print();
    }

    if let Err(e)=&build {
        eprintln!("In {path:?}, {}",e.msg());
        return;
    }

    //
    {
        let mut var_scope=VarScope::new();
        // var_scope.decl("self", Some(Value::int(5))).unwrap();

        let mut core_val=();
        let mut machine = Machine::new(&mut gc_scope,&lib_scope, &mut var_scope,  &mut core_val);


        let res=machine.run_build(&build.unwrap());
        println!("the result is {res:?}");

        if let Err(e)=&res {
            e.eprint(None);
            machine.print_stack_trace();
            machine.print_stack();
        }

        // machine.debug_print_stack();
    }

    gc_scope.mark_and_sweep().unwrap();
    gc_scope.test();
}

// pub fn test_compile<P:AsRef<Path>>(path:P) {
//     let path=path.as_ref();

//     let src = std::fs::read_to_string(path).unwrap();
//     let compiler=Compiler::new();
//     let res=compiler.compile(src.as_str(), 0, Some(path), true);

//     if let Err(e)=res {
//         println!("{e}");
//     }
// }


fn test_script2_run<'a>(build:BuildT,gc_scope:&mut GcScope,lib_scope:& LibScope<i32>, core_val:&'a mut i32) {
    let mut var_scope=VarScope::new();
    let mut machine = Machine::new(gc_scope,lib_scope, &mut var_scope,  core_val);

    let res=machine.run_build(&build);
    println!("the result is {res:?}");

    if let Err(e)=&res {
        e.eprint(None);
        machine.print_stack_trace();
        machine.print_stack();
    }
}
fn test_script2() {

    let mut gc_scope= GcScope::new();

    let path = None;
    let src = "go()";

    let compiler=Compiler::new();

    let build = compiler.compile(src, 0, path, true, );



    let mut lib_scope=LibScope::<i32>::new_full();

    lib_scope.method("go", |mut context|{
        *context.core_mut()+=5;
        Ok(Value::Void)
    }).end();


    if let Err(e)=&build {
        eprintln!("In {path:?}, {}",e.msg());
        return;
    }

    //

    {
        let mut core_val=4;

        test_script2_run(build.unwrap(),&mut gc_scope,&lib_scope, &mut core_val);
        println!("core is {core_val}");
    }

}

pub fn test_script3<P: AsRef<Path>>(path: P, debug_compile: bool, debug: bool) {
    let mut gc_scope = GcScope::new();

    let path = path.as_ref();
    let src = std::fs::read_to_string(path).unwrap();

    let compiler = Compiler::new();

    let build = compiler.compile(src.as_str(), 0, Some(path), true);

    let lib_scope = LibScope::<()>::new_full();

    if debug_compile && build.is_ok() {
        build.as_ref().unwrap().print();
    }

    if let Err(e) = &build {
        eprintln!("In {path:?}, {}", e.msg());
        return;
    }

    {
        let mut var_scope = VarScope::new();
        let mut core_val = ();
        let mut machine = Machine::new(&mut gc_scope, &lib_scope, &mut var_scope, &mut core_val);

        let mut include_cache: HashMap<PathBuf, BuildT> = HashMap::new();
        let include_compiler = Compiler::new();

        machine.set_includer(move |include_path: &Path| {
            let key = std::fs::canonicalize(include_path).unwrap_or_else(|_| include_path.to_path_buf());
            if let Some(cached) = include_cache.get(&key) {
                return Some(cached.clone());
            }

            let inc_src = std::fs::read_to_string(include_path).ok()?;
            match include_compiler.compile(inc_src.as_str(), 0, Some(include_path), true) {
                Ok(inc_build) => {
                    include_cache.insert(key, inc_build.clone());
                    Some(inc_build)
                }
                Err(e) => {
                    eprintln!("In {include_path:?}, {}", e.msg());
                    None
                }
            }
        });

        let res = machine.run_build(&build.unwrap());
        println!("the result is {res:?}");

        if let Err(e) = &res {
            e.eprint(None);
            machine.print_stack_trace();
            machine.print_stack();
        }
    }

    gc_scope.mark_and_sweep().unwrap();
    gc_scope.test();
}

fn main() {

    let start_time = std::time::Instant::now();
    test_script("scripts/clike/test18.script",false,false);
    let time_elapsed=start_time.elapsed().as_secs_f64();
    println!("Time elapsed3: {time_elapsed:?}" ,);

    // test_script2();
    // test_script3("scripts/clike/test19.script",false,false);
}