/*

todo:
* remove cede/take,
* replace takeables with prev parsed

*/
// // #![allow(unused_variables)]
// #![allow(unused)]
mod compiler_error;
// mod cmds;
mod rules;
mod builder_error;

use crate::builder::{Builder, BuilderError};
use crate::clike::compiler::builder_error::BuilderErrorType;
use crate::clike::grammar::container::{WalkGroupContainer, WalkGroupIterContainer};
use crate::clike::grammar::walker::GrammarWalker;
use crate::clike::grammar::GrammarWalkError;
// use crate::ccexpr_compiler::grammar::grammar_run;
// use std::path::PathBuf;
// use super::parser::parse;
use crate::clike::tokenizer::{tokenize, TokenTypeContainer, TokenizerErrorType};
use crate::primitive_types::StringVal;

use crate::{build::*, compiler::builder};
// use super::ccexpr_tokenizer::*;

use std::path::Path;

use crate::compiler::ast;


pub use compiler_error::*;

// use super::super::builder::*;

// use cmds::*;

// pub type CExprBuilder<'a>=Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>;
// type CExprBuilderTaken<'a>=BuilderTaken<'a,PrimitiveIterContainer<'a>,BuilderErrorType>;

// pub type Cmd = for<'a> fn(&mut PrimitiveIterContainer<'a>, &mut CExprBuilder<'a>) -> Result<(),BuilderError<BuilderErrorType>>;




    // pub fn run<'a,'t,'g>(&self,
    //     // builder:&mut CExprBuilder<'a>,
    //     builder:&mut Builder<'a,WalkGroupContainer<'t,'g>,BuilderErrorType>,
    //     top_group:WalkGroupContainer<'t,'g>,
    //     next_anon_id:&mut usize,
    // ) -> Result<(),BuilderError<BuilderErrorType>> {

type ClikeBuilder<'a,'t,'g> = Builder<'a,WalkGroupContainer<'t,'g>,BuilderErrorType>;
type ClikeBuilderResult=Result<(), BuilderError<BuilderErrorType>>;

pub fn op_and<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=and, op_ind={op_ind}, splits={splits:?}");

    builder.eval_func(op_run(op_ind+1, splits[0]));
    Ok(())
}
pub fn op_or<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=or, op_ind={op_ind}, splits={splits:?}");
    builder.eval_func(op_run(op_ind+1, splits[0]));
    Ok(())
}


pub fn op_eq<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=eq, op_ind={op_ind}, splits={splits:?}");

    builder.eval_func(op_run(op_ind+1, splits[0]));
    Ok(())
}
pub fn op_ne<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=ne, op_ind={op_ind}, splits={splits:?}");

    builder.eval_func(op_run(op_ind+1, splits[0]));
    Ok(())
}


pub fn op_lt<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=lt, op_ind={op_ind}, splits={splits:?}");
    builder.eval_func(op_run(op_ind+1, splits[0]));
    Ok(())
}
pub fn op_gt<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=gt, op_ind={op_ind}, splits={splits:?}");
    builder.eval_func(op_run(op_ind+1, splits[0]));
    Ok(())
}

pub fn op_le<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=le, op_ind={op_ind}, splits={splits:?}");
    builder.eval_func(op_run(op_ind+1, splits[0]));
    Ok(())
}
pub fn op_ge<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=ge, op_ind={op_ind}, splits={splits:?}");
    builder.eval_func(op_run(op_ind+1, splits[0]));
    Ok(())
}

pub fn op_add<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=add, op_ind={op_ind}, splits={splits:?}");

    // let splits: Vec<WalkGroupIterContainer<'t,'g>>=groups.split(|x|x.name()==OPS[i].0);

    //a+b+c+d = ((a+b)+c)+d

    builder.eval_func(op_run(op_ind+1, splits[0]));
    // println!("hmma splits[i*2+1]={:?}",splits[0]);
    // let infix_num=splits.len()/2;
    // let val_num=splits.len()-infix_num;


    //
    for i in 0 .. (splits.len()-1)/2 {
        let loc=splits[i*2+1].first().unwrap().tokens().first().unwrap().start_loc();

        // println!("hmmb{i} splits[{}]={:?}",(i+1)*2,splits[(i+1)*2]);

        builder
            .param_push() //push last result
            // .eval(sexpr.get(i).unwrap())
            .eval_func(op_run(op_ind+1, splits[(i+1)*2]))

            .param_push()
            .swap()
            .loc(loc)
            .call_method("add",2); //,loc
    }

    Ok(())
}
pub fn op_sub<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=sub, op_ind={op_ind}, splits={splits:?}");

    builder.eval_func(op_run(op_ind+1, splits[0]));

    //
    for i in 0 .. (splits.len()-1)/2 {
        let loc=splits[i*2+1].first().unwrap().tokens().first().unwrap().start_loc();

        builder
            .param_push() //push last result
            .eval_func(op_run(op_ind+1, splits[(i+1)*2]))
            .param_push()
            .swap()
            .loc(loc)
            .call_method("sub",2); //,loc
    }

    Ok(())
}
pub fn op_mul<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=mul, op_ind={op_ind}, splits={splits:?}");

    builder.eval_func(op_run(op_ind+1, splits[0]));

    //
    for i in (0 .. (splits.len()-1)/2).rev() {
        let loc=splits[i*2+1].first().unwrap().tokens().first().unwrap().start_loc();

        builder
            .param_push() //push last result
            .eval_func(op_run(op_ind+1, splits[(i+1)*2]))
            .param_push()
            .swap()
            .loc(loc)
            .call_method("mul",2); //,loc
    }

    Ok(())
}
pub fn op_div<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=div, op_ind={op_ind}, splits={splits:?}");

    builder.eval_func(op_run(op_ind+1, splits[0]));

    //
    for i in (0 .. (splits.len()-1)/2).rev() {
        let loc=splits[i*2+1].first().unwrap().tokens().first().unwrap().start_loc();

        builder
            .param_push() //push last result
            .eval_func(op_run(op_ind+1, splits[(i+1)*2]))
            .param_push()
            .swap()
            .loc(loc)
            .call_method("div",2); //,loc
    }

    Ok(())
}


pub fn op_pow<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=pow, op_ind={op_ind}, splits={splits:?}");
    builder.eval_func(op_run(op_ind+1, splits[0]));
    Ok(())
}
pub fn op_mod<'a,'t,'g>(op_ind:usize, splits:Vec<WalkGroupIterContainer<'t,'g>>,builder:&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult{
    println!("=mod, op_ind={op_ind}, splits={splits:?}");
    builder.eval_func(op_run(op_ind+1, splits[0]));
    Ok(())
}

//type OpsFunc<'a,'t,'g>= fn(usize,Vec<WalkGroupIterContainer<'t,'g>>, &mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult;

pub fn op_run<'a,'t:'a,'g:'a>(op_ind:usize, groups:WalkGroupIterContainer<'t,'g>) ->
// ClikeBuilderResult
impl Fn(&mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult
{
    // static  OPS: &[&str]=&["div", "mul", "sub", "add", "ge", "le", "gt", "lt", "ne", "eq", "and", "or"];
    static OPS: &[(&'static str, for<'a,'t,'g,> fn(usize,Vec<WalkGroupIterContainer<'t,'g>>, &mut ClikeBuilder<'a,'t,'g>) -> ClikeBuilderResult)]
    =&[
        // ("or",op_or),("and",op_and),
        // ("eq",op_eq),("ne",op_ne),
        // ("lt",op_lt),("gt",op_gt),
        // ("le",op_le),("ge",op_ge),
        ("add",op_add),
        ("sub",op_sub),
        ("mul",op_mul),
        ("div",op_div),
        // ("mod",op_mod),("pow",op_pow),
    ];

    println!("=run, op_ind={op_ind}, groups={groups:?}");



    move|builder|{
        // println!("groups are {groups:?}");


        if groups.len()==1 {
            println!("groups ==1, {groups:?}");
            builder.eval(groups.first().unwrap());
            return Ok(());
        } else if op_ind>=OPS.len() {
            panic!("");
        } else if groups.len()%2 ==0 {
            panic!("");
        }

        let op_name=OPS[op_ind].0;


        let op_func=OPS[op_ind].1;
        // println!("-gooo {groups:?}");
        let splits: Vec<WalkGroupIterContainer<'t,'g>>=groups.split_between(|x|{
            // println!("\t\tx.name()={:?}, op_name={op_name:?}",x.name());
            x.name()==op_name
        });
        println!("\tsplits={splits:?}");


        op_func(op_ind,splits,builder)

    }

}
// pub struct Ops<'a,'t,'g> {

// }

// impl<'a,'t,'g> Ops<'a,'t,'g> {
//     pub fn run(i:usize,builder:&mut Builder<'a,WalkGroupContainer<'t,'g>,BuilderErrorType>,) -> Result<(),BuilderError<BuilderErrorType>>{
//                         // static  OPS: &[&str]=&["div", "mul", "sub", "add", "ge", "le", "gt", "lt", "ne", "eq", "and", "or"];
//                 // static OPS: &[&str]=&["or","and","eq","ne","lt","gt","le","ge","add","sub","mul","div"];
//                 // static OP_FUNCS:&[Box<dyn Fn(&mut Builder<'a,WalkGroupContainer<'t,'g>,BuilderErrorType>)->Result<(),E>>]=&[Box::new(|builder|{
//                 //     // builder.
//                 //     Ok(())
//                 // })];

//         Ok(())
//     }
//     fn run_and(builder:&mut Builder<'a,WalkGroupContainer<'t,'g>,BuilderErrorType>,) {

//     }
// }
pub struct Compiler {
    // cmds : HashMap<&'static str,Vec<Cmd>>,

}

impl Compiler {

    pub fn new() -> Self {
        // let mut cmds: HashMap<&'static str,Vec<Cmd>> = HashMap::new();

        // cmds.insert("break", vec![break_cmd]);
        // cmds.insert("continue", vec![continue_cmd]);
        // cmds.insert("for", vec![for_cmd]);
        // cmds.insert("format", vec![format_cmd,]);
        // cmds.insert("fn", vec![func_cmd, lambda_cmd]);
        // cmds.insert("if", vec![if_cmd]);
        // cmds.insert("include", vec![include_cmd]);
        // cmds.insert("print", vec![print_cmd]);
        // cmds.insert("println", vec![println_cmd]);
        // cmds.insert("return", vec![return_cmd]);
        // cmds.insert("var", vec![var_cmd]);
        // cmds.insert("while", vec![while_cmd]);

        Self {
            // cmds
        }
    }







    pub fn compile(&self,src : &str, version:usize, path : Option<&Path>, keep_src : bool, ) -> Result<BuildT,CompileError> {
        let mut next_anon_id=1;

        let src= StringVal::new(src);
        let pathbuf=path.map(|x|x.to_path_buf());


        //
        let tokenized=tokenize(src.as_str(), rules::is_keyword );

        //
        let Ok(tokenized)=tokenized else {
            let e=tokenized.err().unwrap();

            match e.error_type {
                TokenizerErrorType::Unexpected => {
                    panic!("TokenizerErrorType::Unexpected");
                }
                _ => {
                    // return Err(ParserError { loc: e.loc, error_type: ParserErrorType::Tokenizer(e.error_type) });
                    return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::Tokenizer(e.error_type)});
                }
            }
        };

        //
        let mut walker=GrammarWalker::new(tokenized.tokens(), rules::get_non_term,);
        // walker.set_debug(true);

        //
        if let Err(e)=walker.run("start") {
            match e {
                GrammarWalkError::FailedParse => {
                    return Err(CompileError{path:pathbuf,src,loc:walker.last_loc(),error_type:CompileErrorType::ParserExpected(walker.expecteds_string())});
                }
                // GrammarWalkError::Unfinished => todo!(),
                // GrammarWalkError::RecursiveNonTerm(_) => todo!(),
                // GrammarWalkError::MissingNonTerm(_) => todo!(),
                _ => {
                    // println!("{:?} {:?}",walker.expecteds_string(),walker.last_loc());
                    panic!("{e:?}");
                }
            }
        }

        //
        println!("-----------------");
        let walk=walker.get_walk();

        println!("{}",walk.root());


        return Ok(BuildT::new(Build::default()));

        //
        let mut builder = builder::Builder::new();
        // // // builder.eval(parsed.root_block_primitive().get_block().unwrap().primitives());
        for g in walk.root().children() {
            println!("={:?}",g.name());

            builder.eval(g);
        }


        //builder needs to be passed a primitive_iter instead of primitive?

        //
        let mut ast = ast::Ast::new(false,true);

        if let Err(e)=builder.generate_ast(&mut ast,|builder,group|{
            self.run(builder, group,&mut next_anon_id)
        }) {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::Builder(e.error_type)});
        }

        if let Err(e)=ast.calc_vars(false) {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::AstVar(e.error_type)});
        }

        if let Err(e)=ast.calc_labels_gotos() {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::AstVar(e.error_type)});
        }

        // if print_ast { ast.print(); }
        ast.print();

        //
        let kept_src=if keep_src {Some(src.clone())} else {None};
        let build = ast.compile(version, path, kept_src,true,true);
        Ok(BuildT::new(build))

        //
        // Ok(BuildT::new(Build::default()))
    }


    pub fn run<'a,'t,'g>(&self,
        // builder:&mut CExprBuilder<'a>,
        builder:&mut Builder<'a,WalkGroupContainer<'t,'g>,BuilderErrorType>,
        top_group:WalkGroupContainer<'t,'g>,
        next_anon_id:&mut usize,
    ) -> Result<(),BuilderError<BuilderErrorType>> {
        println!("{:?}:",top_group.name());

        match top_group.name() {
            "primitive" => {
                if let Some(p)=top_group.tokens().first() {
                    builder.loc(p.start_loc());
                    println!("\t{:?}",p.token_type());

                    match p.token_type() {
                        TokenTypeContainer::Float(x) => {
                            builder.result_float(x);
                        },
                        TokenTypeContainer::Int(x) => {
                            builder.result_int(x);
                        },
                        TokenTypeContainer::String(x) => {
                            builder.result_string(x);
                        },
                        TokenTypeContainer::Identifier(x) => {
                            builder.get_var(x);
                        },
                        _ => {panic!("");}
                    }
                }
            }
            "nil" => {
                let p=top_group.tokens().first().unwrap();
                builder.loc(p.start_loc());
                builder.result_nil();
            }
            "void" => {
                let p=top_group.tokens().first().unwrap();
                builder.loc(p.start_loc());
                builder.result_void();
            }
            "true" => {
                let p=top_group.tokens().first().unwrap();
                builder.loc(p.start_loc());
                builder.result_bool(true);
            }
            "false" => {
                let p=top_group.tokens().first().unwrap();
                builder.loc(p.start_loc());
                builder.result_bool(false);
            }
            "expr" => {
                builder.eval_func(op_run(0, top_group.children()));

            }
            "pos" => {}
            "neg" => {
                builder.param_push();
                builder.call_method("neg", 1);
            }
            "not" => {
                builder.param_push();
                builder.call_method("not", 1);
            }
            "index" => {
                builder
                    .loc(top_group.tokens().first().unwrap().start_loc())
                    .param_push()
                    .eval(top_group.child(0).unwrap())
                    .param_push()
                    .swap()
                    .get_field(false)
                    ;
            }
            "field_index" => {
                let field=top_group.tokens().first().unwrap().get_int().unwrap();
                builder
                    .loc(field.token.start_loc())
                    .param_push()
                    .result_int(field.value)

                    .param_push()
                    .swap()
                    .get_field(false)
                    ;
            }
            "field_name" => {
                let field=top_group.tokens().first().unwrap().get_identifier().unwrap();
                builder
                    .loc(field.token.start_loc())
                    .param_push()
                    .result_string(field.value)

                    .param_push()
                    .swap()
                    .get_field(false)
                    ;
            }

            "params" => {
                let params_group=top_group.child(1).unwrap();

                for param in params_group.children().rev() {
                    builder
                        .eval(param)
                        .param_push();
                }
            }
            "call_field_index" => {
                let field_group=top_group.child(0).unwrap();
                let params_group=top_group.child(1).unwrap();
                let field_val=field_group.tokens().first().unwrap().get_int().unwrap();
                let params_num=params_group.children().len();

                //self.ind(..) => self[ind](self, ..)

                builder
                    .decl_anon_var("self", false)
                    .set_anon_var("self")
                    .eval(params_group)
                    .get_anon_var("self")
                    .param_push()
                    .eval(field_group)
                    .loc(field_val.token.start_loc())
                    .call_result(params_num)
                    ;
            }
            "call_field_name" => {
                let field_group=top_group.child(0).unwrap();
                let params_group=top_group.child(1).unwrap();
                let field_val=field_group.tokens().first().unwrap().get_identifier().unwrap();
                let params_num=params_group.children().len();

                //self.field(..) => self.field(self, ..)

                builder
                    .decl_anon_var("self", false)
                    .set_anon_var("self")
                    .eval(params_group)
                    .get_anon_var("self")
                    .param_push()
                    .eval(field_group)
                    .loc(field_val.token.start_loc())
                    .call_method_or_result(field_val.value, params_num)
                    ;
            }
            "call_idn" => {
                let idn_group=top_group.child(0).unwrap();
                let params_group=top_group.child(1).unwrap();
                let name=idn_group.tokens().first().unwrap().get_identifier().unwrap();

                builder
                    .eval(params_group)
                    .loc(name.token.start_loc())
                    .call(name.value, params_group.children().len());
            }
            "call_val" => {
                let params_group=top_group.child(0).unwrap();

                builder
                    .eval(params_group)
                    .call_result(params_group.children().len())
                    ;
            }

            "val" => {
                let mut groups=top_group.children();

                //prefixes
                let prefixes= if groups.first().unwrap().name()=="prefixes" {
                    groups.pop_front()
                } else {
                    None
                };

                //val
                let val=groups.pop_front().unwrap();
                // println!("val is {}",val.name());
                builder.eval(val);

                //field(s), index(s), call(s)
                for rest in groups.rev() {
                    builder.eval(rest);
                }

                //
                if let Some(prefixes)=prefixes {
                    for prefix in prefixes.children().rev() {
                        builder.eval(prefix);
                    }
                }
            }
            // "block" => {
            //     builder.eval(primitive)
            // }
            _ => {
                panic!("{}",top_group.name());
                // builder.eval(primitive)
                // top_group.children()

            }
        }


        Ok(())
    }

}


    //     let mut cur_exprs: Vec<ExprVal>= Vec::new();

    //     //
        // while let Ok(first_primitive)=top_primitive_iter.pop_front()
    //     loop
    //     {
    // //         match top_primitive_iter.first().map(|p|p.primitive_type()) {
    // //             Ok(PrimitiveTypeContainer::Eob) => {

    // //             }
    // //             Ok(PrimitiveTypeContainer::Eol) => {

    // //             }
    // //             Ok(PrimitiveTypeContainer::Symbol(";")) => {

    // //             }
    // //             Ok(PrimitiveTypeContainer::Identifier(idn)) => {
    // //                 match cur_exprs.last() {
    // //                     Some(ExprVal::Identifier(_)) => {

    // //                     }
    // //                     Some(ExprVal::Builder(_)) => {

    // //                     }
    // //                     _ => {

    // //                     }
    // //                 }
    // //                 if let Some(ExprVal::Identifier(_))=cur_exprs.last() {

    // //                 }

    // //             }
    // //             Err(_) => {break;}
    // //             _ => {}
    // //         }


    // //         let Ok(first_primitive)=top_primitive_iter.pop_front() else {break;};
    // //         // let Ok(first_primitive)=top_primitive_iter.first() else { return Ok(()) };
    // //         builder.loc(first_primitive.start_loc());
    // //         println!("hmm {first_primitive:?}",);

    // //         let mut done=false;


    // //         builder.mark();

    // //         match first_primitive.primitive_type() {
    // //             PrimitiveTypeContainer::CurlyBlock(b) => { //code block
    // //                 builder.eval(b);
    // //                 cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    // //             }
    // //             PrimitiveTypeContainer::SquareBlock(b) => { //array or dict
    // //                 // let is_dict=b.children().find(|p|p.get_symbol().map(|s|s.eq(":")).unwrap_or(false)).is_some();
    // //             }
    // //             PrimitiveTypeContainer::ParenthesesBlock(b) => {} //expr

    // //             PrimitiveTypeContainer::Float(x) => { //float
    // //                 builder.result_float(x);
    // //             }
    // //             PrimitiveTypeContainer::Int(x) => { //int
    // //                 builder.result_int(x);
    // //             }
    // //             PrimitiveTypeContainer::String(x) => { //string
    // //                 builder.result_string(x);
    // //             }
    // //             PrimitiveTypeContainer::Symbol(x) => { //
    // //                 match x {
    // //                     ";" => {
    // //                         // done=true;
    // //                     }
    // //                     _ => {
    // //                         cur_exprs.push(ExprVal::Symbol(first_primitive));
    // //                     }
    // //                 }
    // //             }
    // //             PrimitiveTypeContainer::Identifier(x) => { //cmd or idn
    // //                 // println("")
    // //                 if let Some(cmds)=self.cmds.get(x) {
    // //                     // let mut primitives=top_primitive_iter.clone();
    // //                     let mut errors=Vec::<BuilderError<BuilderErrorType>>::new();

    // //                     //
    // //                     builder.set_anon_scope(*next_anon_id);

    // //                     //
    // //                     for cmd in cmds {
    // //                         let mut primitives=top_primitive_iter.clone();

    // //                         if let Err(e)=cmd(&mut primitives,builder) {
    // //                             errors.push(e);
    // //                             builder.discard_from_mark();
    // //                         } else { //ok
    // //                             errors.clear();
    // //                             *next_anon_id+=1;
    // //                             top_primitive_iter=primitives;
    // //                             cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    // //                             break;
    // //                         }
    // //                     }

    // //                     //
    // //                     builder.set_anon_scope(0);

    // //                     //
    // //                     if errors.len()>0 {
    // //                         errors.sort_by(|a,b|a.loc.cmp(&b.loc));
    // //                         return Err(errors.last().unwrap().clone());
    // //                     }
    // //                 } else {
    // //                     match x {
    // //                         "true" => {
    // //                             builder.result_bool(true);
    // //                             cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    // //                         }
    // //                         "false" => {
    // //                             builder.result_bool(false);
    // //                             cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    // //                         }
    // //                         "nil" => {
    // //                             builder.result_nil();
    // //                             cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    // //                         }
    // //                         "void" => {
    // //                             builder.result_void();
    // //                             cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    // //                         }
    // //                         "var" => {
    // //                             //var decls ...
    // //                         }
    // //                         _ => {
    // //                             cur_exprs.push(ExprVal::Identifier(first_primitive));
    // //                             // builder.get_var(x);
    // //                         }
    // //                     }
    // //                 }
    // //             }
    // //             // PrimitiveTypeContainer::End => {} //eol or eof //ignore
    // //             PrimitiveTypeContainer::Eol|PrimitiveTypeContainer::Eob => {
    // //                 if let Some(ExprVal::Symbol(_))=cur_exprs.last() {
    // //                     //there aren't any postfix symbols, so don't need to handle
    // //                 } else {
    // //                     done=true;
    // //                 }
    // //             }
    // //         }

    //     }

    //     //handle exprs

    //     //