// #![feature(mapped_lock_guards)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #[allow(unused_parens)]

/*
* want to add machine::new(.., &mut some_val)
- but methods context needs to know the type
- could add generic to all methods? and have the libscope also have the generic and fill it
- but will have to specify lifetimes + the generic for all context params?
*/

use std::path::Path;

use script_lang::{cexpr_parser::{BlockContainer, FieldContainer, ParamContainer, PrimitiveContainer, PrimitiveTypeContainer, RecordContainer}, error_msg, langs,  Value,
// MachineError, Value
};

// use script_lang::{cmd_lang::{self,parser::{Block, BlockContainer, Primitive, PrimitiveContainer, PrimitiveType, PrimitiveTypeContainer, Record, RecordContainer}}, error_msg};

// use conf_lang2::{def::grammar::Grammar, error_line_src, print_tree, walk::traverse, };

// use crate::conf_lang2::Grammar;

// mod script_lang;


pub fn test_script<P:AsRef<Path>>(path:P) {

    println!("===");
    let mut gc_scope= script_lang::GcScope::new();

    let path = path.as_ref();
    let src = std::fs::read_to_string(path).unwrap();

    let compiler = script_lang::langs::sexpr_compiler::Compiler::new_core();
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


pub fn test_script2<P:AsRef<Path>>(path:P) {

    println!("===");

    let path = path.as_ref();
    let src = std::fs::read_to_string(path).unwrap();

    let res=langs::cexpr_parser::parse(src.as_str());

    match res {
        Ok(parsed)=>{
            // println!("{}",parsed.root_block().len());
            enum TraverseType<'a> {
                Block(BlockContainer<'a>),
                Record(RecordContainer<'a>),
                Param(ParamContainer<'a>),
                Field(FieldContainer<'a>),
                Primitive(PrimitiveContainer<'a>),
            }

            struct Traverse<'a> {
                depth:usize,
                traverse_type:TraverseType<'a>,
            }

            let mut stk=vec![Traverse{depth:0,traverse_type:TraverseType::Block(parsed.root_block())}];

            while let Some(traverse)=stk.pop() {
                let indent="    ".repeat(traverse.depth);

                match traverse.traverse_type {
                    TraverseType::Block(b)=>{
                        // println!("{indent}Block({}) rs={:?}",b.block_ind,b.records_range());
                        println!("{indent}Block");

                        stk.extend(b.records().rev().map(|x|Traverse {
                            depth: traverse.depth+1,
                            traverse_type: TraverseType::Record(x),
                        }));

                    }
                    TraverseType::Record(r)=>{
                        // println!("{indent}Record({}) ps={:?}",r.record_ind,r.primitives_range());
                        println!("{indent}Record");

                        stk.extend(r.params().rev().map(|x|Traverse {
                            depth: traverse.depth+1,
                            traverse_type: TraverseType::Param(x),
                        }));
                    }
                    TraverseType::Param(p)=>{
                        println!("{indent}Param");

                        stk.extend(p.fields().rev().map(|x|Traverse {
                            depth: traverse.depth+1,
                            traverse_type: TraverseType::Field(x),
                        }));

                        stk.push(Traverse {
                            depth: traverse.depth+1,
                            traverse_type: TraverseType::Primitive(p.primitive()),
                        });
                    }
                    TraverseType::Field(f)=>{
                        println!("{indent}Field");
                        stk.push(Traverse {
                            depth: traverse.depth+1,
                            traverse_type: TraverseType::Primitive(f.primitive()),
                        });
                    }
                    TraverseType::Primitive(p)=>{
                        // println!("{indent}p={}",p.primitive_ind);
                        if let PrimitiveTypeContainer::Block(b)=p.primitive_type() {
                            stk.push(Traverse{
                                depth:traverse.depth,
                                traverse_type:TraverseType::Block(b)
                            });
                        } else {
                            println!("{indent}{:?}",p.primitive_type());
                            // println!("{indent}{:?}",p.primitive_ind);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("In {path:?}, {}",error_msg(e.error_type, e.loc, Some(src.as_str()), Some(path)));
        }
    }

    // let src="1";

    // cmd_lang::parse("12");
    // cmd_lang::parse("+13");
    // cmd_lang::parse("-14");
    // cmd_lang::parse(".5");
    // cmd_lang::parse("+.5");
    // cmd_lang::parse("01.5");
    // cmd_lang::parse("5.");
    // cmd_lang::parse(".500");
    // cmd_lang::parse("001");

    // cmd_lang::parse("\"hel\\\"lo\"");
    // println!("{:?}",cmd_lang::parse_string(input))
    // cmd_lang::parse("1");
}

// fn testttt(context: script_lang::FuncContextExt<_>) -> Result<Value,MachineError>{
//     Ok(script_lang::Value::int(4))
// }

pub fn test_script3<P:AsRef<Path>>(path:P,debug_compile:bool,debug:bool) {

    println!("===");
    let mut gc_scope= script_lang::GcScope::new();

    let path = path.as_ref();
    let src = std::fs::read_to_string(path).unwrap();

    let compiler=script_lang::langs::cexpr_compiler::Compiler::new();

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
        Ok(script_lang::Value::Int(*context.core() as script_lang::IntT))
    }).end();
    lib_scope.method("set_test", |mut context|{
        // *context.core_mut().0=context.param(0).as_int() as i32;
        *context.core_mut()=context.param(0).as_int() as i32;
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




// pub fn test_script4<P:AsRef<Path>>(path:P) {

//     println!("===");
//     let mut gc_scope= script_lang::GcScope::new();

//     let path = path.as_ref();
//     let src = std::fs::read_to_string(path).unwrap();

//     let compiler=script_lang::langs::cexpr_compiler::Compiler::new();

//     let build = compiler.compile(src.as_str(), 0, None, true,
//         // false
//     );

//     let mut my_num:i32=123;
//     let mut my_num2:i32=456;

//     if let Err(e)=&build {
//         eprintln!("In {path:?}, {}",e.msg());
//     } else {

//         let mut var_scope=script_lang::VarScope::new();
//         var_scope.decl("self", Some(script_lang::Value::int(41))).unwrap();


//         let  lib_scope=script_lang::LibScope::<(&mut i32,&mut i32)>::new_full();
//         // lib_scope.method_ext("get_test", |context|{
//         //     Ok(script_lang::Value::Int(*context.get_core_ref().0 as script_lang::IntT))
//         // }).end();
//         // lib_scope.method_ext("set_test", |mut context|{
//         //     context.g().0=context.param(0).as_int() as i32;
//         //     Ok(script_lang::Value::Void)
//         // }).int().end();
//         let mut machine = script_lang::Machine::new(&mut gc_scope,&mut var_scope, &lib_scope,  (&mut my_num,&mut my_num2));
//         // machine.set_debug_print(true);

//         // build.clone().unwrap().print();

//         let res=machine.run_build(&build.unwrap());
//         println!("the result is {res:?}");

//         if let Err(e)=&res {
//             e.eprint(None);
//             machine.debug_print_stack_trace(true);
//             machine.debug_print_stack();
//             // machine.print_state();
//         }
//         // machine.debug_print_stack();
//     }

//     //gc_scope.mark_and_sweep();
//     gc_scope.test();


// }


pub fn test_script5<P:AsRef<Path>>(path:P) {

    println!("===");
    let mut gc_scope= script_lang::GcScope::new();

    let path = path.as_ref();
    let src = std::fs::read_to_string(path).unwrap();

    let compiler=script_lang::langs::cexpr_compiler::Compiler::new();

    let build = compiler.compile(src.as_str(), 0, Some(path), true,
        // false
    );

    build.clone().unwrap().print();

    let mut lib_scope=script_lang::LibScope::<Vec<Value>>::new_full();

    lib_scope.method("add_event_listener",|mut context|{
        let listener=context.param(2);
        let listeners=context.core_mut();
        listeners.push(listener.clone_root());
        Ok(Value::Void)
    }).any().str().func().end();

    let mut myval: Vec<Value>=vec![];
    let mut var_scope=script_lang::VarScope::new();

    if let Err(e)=&build {
        eprintln!("In {path:?}, {}",e.msg());
    } else {

        // let mut core=();
        let mut machine = script_lang::Machine::new(&mut gc_scope,&lib_scope, &mut var_scope,  &mut myval);
        machine.set_debug_print(true);

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

    gc_scope.test();
    gc_scope.mark_and_sweep().unwrap();
    gc_scope.test();

    println!("abs is {:?}",myval);


}



fn main() {
    println!("Hello, world!");

    // // test_script("examples/test.script");
    // // test_script3("examples/test5.script");
    // // test_script4("examples/test8.script");

    // // test_script2("examples/test6.script");

    // test_script3("examples/test7.script");
    //  test_script3("examples/test8.script",true,false);
    test_script3("examples/test12.script",true,false);

    // // test_script3("examples/test9.script");

    // test_script5("examples/test11.script");

}
/*
TODO
* fix gc
* replace value clone in machine with value.clone_leaf()
** make values returned from func_context, var_scope, lib_scope, machine be leaves
** make all values used in working roots?
** if custom dropped, and weak count==0, can remove it without mark and sweep
*** iterate children, check their weak counts?

* allow method decl from script?
* make dict keys accept non strings
** make value's hashable? for customs based on type id
* add matrices to lib
* remove unnecessary debug code
* add c like syntax

* add limits for running, memory usage,
** limit size of strings? at both compile time and runtime
*** at add/remove/init?
** limit size of file willing to compile?
** limit size of arrays/dicts?
*** at push/pop/init ?
** either limit loops (ie to_block_start/end, includeding nested loops) or limit number of instructs run
** limit number of variables declared?
*** globals?
*** manageds?
*** rest?


* use call for function calls
** if used not on a func, then call method "call"
** disable $ prefix
*/