
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

use script_lang::{cexpr_parser::{BlockContainer, FieldContainer, ParamContainer, PrimitiveContainer, PrimitiveTypeContainer, RecordContainer}, error_msg, langs};

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
    
        let  lib_scope=script_lang::LibScope::new_full();
        let mut machine = script_lang::Machine::new(&mut gc_scope,&mut var_scope, &lib_scope,  ());
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

pub fn test_script3<P:AsRef<Path>>(path:P) {

    println!("===");
    let mut gc_scope= script_lang::GcScope::new();
    
    let path = path.as_ref();
    let src = std::fs::read_to_string(path).unwrap();

    let compiler=script_lang::langs::cexpr_compiler::Compiler::new();
    
    let build = compiler.compile(src.as_str(), 0, None, true,
        // false
    );

    let mut mynum=123;

    if let Err(e)=&build {
        eprintln!("In {path:?}, {}",e.msg());
    } else {

        let mut var_scope=script_lang::VarScope::new();
        var_scope.decl("self", Some(script_lang::Value::int(4))).unwrap();
    
        let lib_scope=script_lang::LibScope::new_full();
        let mut machine = script_lang::Machine::new(&mut gc_scope,&mut var_scope, &lib_scope,  &mut mynum);
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
        // machine.debug_print_stack();
    }

    //gc_scope.mark_and_sweep();
    gc_scope.test();
    

}



fn main() {
    println!("Hello, world!");

    // test_script("examples/test.script");
    // test_script3("examples/test5.script");
    // test_script4("examples/test8.script");
    
    // test_script2("examples/test6.script");
    test_script3("examples/test7.script");
}
