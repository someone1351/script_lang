
use script_lang::langs::texpr_compiler::Compiler;

use std::{collections::BTreeMap, path::Path};

use script_lang::{Dict, Value};


pub fn test_script<P:AsRef<Path>>(path:P,debug_compile:bool,debug:bool) {

    let mut gc_scope= script_lang::GcScope::new();

    let path = path.as_ref();
    let src = std::fs::read_to_string(path).unwrap();

    let compiler=Compiler::new();

    let build = compiler.compile(src.as_str(), 0, Some(path), true,
        // false
    );


    // build.clone().unwrap().print();


    let mut my_num:i32=123;

    let mut lib_scope=script_lang::LibScope::<
        // (&mut i32,)
        i32
    >::new_full();

    lib_scope.method("get_test", |context|{
        // Ok(script_lang::Value::Int(*context.core().0 as script_lang::IntT))
        Ok((*context.core()).into())
    }).end();
    lib_scope.method("set_test", |mut context|{
        // *context.core_mut().0=context.param(0).as_int() as i32;
        *context.core_mut()=context.param(0).as_int().try_into()?;
        Ok(script_lang::Value::Void)
    }).int().end();

    // let mut test_val=0;

    // lib_scope.method_mut("do_test2", move|_context|{
    //     test_val+=1;
    //     Ok(script_lang::Value::int(test_val))
    // }).end();



    // let mut core=(&mut my_num,);

    if debug_compile && build.is_ok() {
        build.as_ref().unwrap().print();
    }
    if let Err(e)=&build {
        eprintln!("In {path:?}, {}",e.msg());
    } else {

        let mut var_scope=script_lang::VarScope::new();
        var_scope.decl("self", Some(script_lang::Value::int(4))).unwrap();

        let mut e= Dict(BTreeMap::new());
        let mut e0= Dict(BTreeMap::new());
        e0.0.insert(script_lang::libs::dict::ValueKey::String("width".into()), Value::Nil);
        e.0.insert(script_lang::libs::dict::ValueKey::Int(77.into()), script_lang::Value::custom_unmanaged_mut(e0));
        var_scope.decl("e", Some(script_lang::Value::custom_unmanaged_mut(e))).unwrap();


        // var_scope.decl("goa", Some(script_lang::Value::custom_callable_unmanaged(45 as i32,|context|{
        //     let mut x:i32=context.param(0).as_custom().data_copy()?;
        //     for i in 1 .. context.params_num() {
        //         x+= context.param(i).as_int() as i32;
        //     }
        //     Ok(Value::int(x))
        // }))).unwrap();

        let mut machine = script_lang::Machine::new(&mut gc_scope,&lib_scope, &mut var_scope,  &mut
            // core
            my_num
        );

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



fn main() {
    // // test_script3("examples/texpr/test15.script",true,false);
    test_script("scripts/texpr/test8.script",false,false);


}

