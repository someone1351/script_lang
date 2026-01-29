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

use std::{collections::BTreeMap, path::Path};

use script_lang::{cexpr_parser::{BlockContainer, FieldContainer, ParamContainer, PrimitiveContainer, PrimitiveTypeContainer, RecordContainer}, error_msg, langs, Dict, Value
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
                            traverse_type: TraverseType::Primitive(p.as_primitive()),
                        });
                    }
                    TraverseType::Field(f)=>{
                        println!("{indent}Field");
                        stk.push(Traverse {
                            depth: traverse.depth+1,
                            traverse_type: TraverseType::Primitive(f.as_primitive()),
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

        let mut e= Dict(BTreeMap::new());
        let mut e0= Dict(BTreeMap::new());
        e0.0.insert(script_lang::libs::dict::ValueKey::String("width".into()), Value::Nil);
        e.0.insert(script_lang::libs::dict::ValueKey::Int(77), script_lang::Value::custom_unmanaged_mut(e0));
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


// pub fn test_script6<P:AsRef<Path>>(path:P,debug_compile:bool,debug:bool) {

//     println!("===");
//     let mut gc_scope= script_lang::GcScope::new();

//     let path = path.as_ref();
//     let src = std::fs::read_to_string(path).unwrap();

//     let compiler=script_lang::langs::cexpr_compiler::Compiler::new();

//     let build = compiler.compile(src.as_str(), 0, Some(path), true,
//         // false
//     );


//     // build.clone().unwrap().print();


//     let mut my_num:i32=123;

//     let mut lib_scope=script_lang::LibScope::<
//         // (&mut i32,)
//         i32
//     >::new_full();

//     struct Node(i32);

//     pub struct StuffResultEnv {
//         pub by_ind : Vec<Value>, //[local_node_ind]=node
//     }

//     pub struct StuffResult {
//         pub nodes : HashMap<usize,Value>, //[node_element_ind]=entity_val
//         pub envs : HashMap<usize,Value>, //[element_ind]=env
//     }
//     struct Stuff;


//     //
//  //call(stuff,ind,entity)
//     lib_scope.method("call",|mut context|{
//         let stuff=context.param(0).as_custom();
//         let top_entity:Entity = context.param(2).as_custom().data_clone()?;
//         let stub_ind=context.param(1).as_int().abs() as usize;

//         let world=context.core_mut();

//         stuff.with_data_ref(|stuff:&Stuff|{
//             let mut element_entity_map = HashMap::<usize,Entity>::from_iter([(0,top_entity)]);
//             let Some(node_range)=stuff.all_stubs.get(&stub_ind).cloned() else {return Ok(Value::Nil);};

//             for node_ind in node_range {
//                 let stuff_node = stuff.all_nodes.get(node_ind).unwrap();
//                 let names=stuff.all_names.get(stuff_node.names.clone()).unwrap();

//                 let mut e=world.spawn((UiLayoutComputed::default(),));

//                 //
//                 let &parent_entity=element_entity_map.get(&stuff_node.parent_element_ind).unwrap();
//                 // e.set_parent(parent_entity);
//                 e.insert(ChildOf(parent_entity));

//                 //
//                 let entity=e.id();
//                 element_entity_map.insert(stuff_node.element_ind, entity);

//                 //
//                 if !names.is_empty() {
//                     e.insert((UixName{ names:HashSet::from_iter(names.iter().map(|x|x.clone())) },));
//                 }

//                 //
//                 for attrib_ind in stuff_node.attribs.clone() {
//                     let attrib=stuff.all_attribs.get(attrib_ind).unwrap().0.clone();
//                     attrib(entity,world);
//                 }

//                 //
//                 let parent_entity_val=self_entity_from_world(world, parent_entity);

//                 // let mut pe=world.entity_mut(parent_entity);
//                 // let mut env=pe.entry::<UixEnv>().or_default();

//                 // for n in names.iter() {
//                 //     env.get_mut().env.entry(n.clone()).or_default().push(parent_entity_val.clone());
//                 // }
//             }

//             //
//             let element_entity_map2: HashMap<usize, Value>=element_entity_map.iter().map(|(&k,&v)|{
//                 let vv=self_entity_from_world(world, v);
//                 (k,vv)
//             }).collect();

//             //
//             let mut envs=HashMap::new();
//             //
//             if let Some(stub_envs)=stuff.all_envs.get(&stub_ind) {
//                 for (&env_element_ind,stuff_env) in stub_envs {
//                     let v=StuffResultEnv{
//                         by_ind: stuff_env.by_ind.iter().map(|&element_ind|element_entity_map2.get(&element_ind).unwrap().clone()).collect(),
//                         by_name: stuff_env.by_name.iter().map(|(name,named_env)|{
//                             (name.clone(),named_env.iter().map(|&element_ind|element_entity_map2.get(&element_ind).unwrap().clone()).collect())
//                         }).collect(),
//                     };
//                     let v=Value::custom_unmanaged(v);
//                     envs.insert(env_element_ind, v);
//                 }
//             }


//             //
//             Ok(Value::custom_unmanaged(StuffResult{ nodes: element_entity_map2, envs }))
//             // Ok(Value::custom_unmanaged(StuffResult(element_entity_map)))
//         })
//     }).custom_ref::<Stuff>().int().custom_ref::<Entity>().end();

//     //


//     if debug_compile && build.is_ok() {
//         build.as_ref().unwrap().print();
//     }
//     if let Err(e)=&build {
//         eprintln!("In {path:?}, {}",e.msg());
//     } else {

//         let mut var_scope=script_lang::VarScope::new();
//         let node123=Value::custom_unmanaged(Node(123));
//         let env123=Value::custom_unmanaged(StuffResultEnv{ by_ind: vec![node123.clone()] });

//         var_scope.decl("root", Some(node123.clone())).unwrap();
//         var_scope.decl("_stubs", Some(Value::custom_unmanaged(StuffResult{
//             nodes: HashMap::from([(123,node123.clone())]),
//             envs: HashMap::from([(123,env123.clone())]),
//         }))).unwrap();


//         let mut machine = script_lang::Machine::new(&mut gc_scope,&lib_scope, &mut var_scope,  &mut
//             // core
//             my_num
//         );

//         if debug {
//             machine.set_debug_print(true);
//             machine.set_debug_print_simple(true);
//         }

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

//     gc_scope.mark_and_sweep().unwrap();
//     gc_scope.test();


// }


fn main() {
    println!("Hello, world!");

    // // // test_script("examples/test.script");
    // // // test_script3("examples/test5.script");
    // // // test_script4("examples/test8.script");

    // // // test_script2("examples/test6.script");

    // // test_script3("examples/test7.script");
    // //  test_script3("examples/test13.script",true,true);
    // //  test_script6("examples/test14.script",true,true);
    // // test_script3("examples/test12.script",true,false);

    // // // test_script3("examples/test9.script");

    // // test_script5("examples/test11.script");

    test_script3("examples/test8.script",false,false);
    // test_script3("examples/test15.script",true,false);

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