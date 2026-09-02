/*

todo:
* remove cede/take,
** replace takeables with prev parsed
** when taking prev, need to check if the group it is in needs to be removed or not if it is empty after having prev removed
*** how to tell if group needs to be removed?
**** it's tokens start/end are the same?
***** what if [Always, X.opt()].and()
****** can't be taken from, so doesn't matter
***** what if X.opt().group()
*** has to do with Or?
**** only grammars in Or are saved eg Or(And(X,Y),X) ? No want the X from the And
** still need take, might aswell keep cede for it to be used in certain circumstances
** when doing prev, what if or(and(X,Y,Z),and(X,Y)), should X,Y from the first be stored?

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

        let start_time = std::time::Instant::now();
        let result=walker.run("start") ;


        let time_elapsed=start_time.elapsed().as_secs_f64();
        //
        if let Err(e)=result {


            println!("Time elapsed: {time_elapsed:?} {}" ,walker.step_count());

            match e {
                GrammarWalkError::FailedParse => {
                    let expecteds=walker.expects_string();

                    let error_type= if expecteds.is_empty() {
                        CompileErrorType::ParserUnexpected
                    } else {
                        CompileErrorType::ParserExpected(expecteds)
                    };
                    // let error_type=CompileErrorType::ParserExpected(expecteds);

                    return Err(CompileError{path:pathbuf,src,loc:walker.last_loc(),error_type});

                    // return Err(CompileError{path:pathbuf,src,loc:walker.last_loc(),error_type:CompileErrorType::ParserExpected(walker.expects_string())});
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

        println!("Time elapsed: {time_elapsed:?} {}" ,walker.step_count());

        // return Ok(BuildT::new(Build::default()));

        //
        let mut builder = builder::Builder::new();
        // // // builder.eval(parsed.root_block_primitive().get_block().unwrap().primitives());
        for g in walk.root().children() {
            println!("={:?}",g.name());

            builder.eval(g);
        }

        //
        builder.end_stack_check();

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

        *next_anon_id+=1;
        builder.set_anon_scope(*next_anon_id);


        match top_group.name() {
            "primitive" => {
                if let Ok(p)=top_group.tokens().trimmed().first() {
                    builder.loc(p.start_loc());
                    //println!("\t{:?}",p.token_type());

                    match p.token_type() {
                        TokenTypeContainer::Float(x) => { builder.result_float(x); },
                        TokenTypeContainer::Int(x) => { builder.result_int(x); },
                        TokenTypeContainer::String(x) => { builder.result_string(x); },
                        TokenTypeContainer::Identifier(x) => { builder.get_var(x); },
                        TokenTypeContainer::Keyword("true") => { builder.result_bool(true); },
                        TokenTypeContainer::Keyword("false") => { builder.result_bool(false); },
                        TokenTypeContainer::Keyword("nil") => { builder.result_nil(); },
                        TokenTypeContainer::Keyword("void") => { builder.result_void(); },
                        _ => {panic!("");}
                    }
                }
            }
            "expr" => {
                builder.eval(top_group.child(0).unwrap());
            }
            "or"|"and" => {
                let cond=if top_group.name()=="or" {JmpCond::True}else{JmpCond::False};

                builder.block_start(None);

                for c in top_group.children() {
                    builder
                        .eval(c)
                        .to_block_end(cond,0)
                        ;
                }

                builder.block_end();
            }
            "xor" => {
                let mut cs=top_group.children();

                builder.eval(cs.next().unwrap());

                while !cs.is_empty() {
                    builder
                        .param_push()
                        .eval(cs.next().unwrap())
                        .param_push()
                        .swap()
                        .call_method("xor", 2)
                        ;
                }
            }
            "lt"|"le"|"gt"|"ge"|"eq"|"ne" => {
                let func=match top_group.name() {
                    "lt" => "lt",
                    "gt" => "gt",
                    "le" => "le",
                    "ge" => "ge",
                    "eq" => "eq",
                    "ne" => "ne",
                    _ => panic!(""),
                };
                builder
                    .eval(top_group.child(1).unwrap())
                    .param_push()
                    .eval(top_group.child(0).unwrap())
                    .param_push()
                    .call_method(func,2)
                    ;
            }
            "factor"|"term" => {
                let mut cs=top_group.children();
                builder
                    .eval(cs.next().unwrap())
                    ;

                while !cs.is_empty() {
                    let op=cs.next().unwrap();
                    let func=match op.name() {
                        "add" => "add",
                        "sub" => "sub",
                        "mul" => "mul",
                        "div" => "div",
                        "mod" => "mod",
                        _ => {panic!("");}
                    };

                    builder
                        .param_push()
                        .eval(cs.next().unwrap())
                        .param_push()
                        .swap()
                        .loc(op.start_loc())
                        .call_method(func, 2)
                        ;
                }
            }

            "neg"|"not" => {
                let func=match top_group.name() {
                    "neg" => "neg",
                    "not" => "not",
                    _ => panic!(""),
                };

                builder.param_push();
                builder.call_method(func, 1);
            }

            "prefixes" => {
                let prefixes=top_group.child(0).unwrap().children();
                let val=top_group.child(1).unwrap();

                builder.eval(val);

                for p in prefixes.rev() {
                    let func=match p.name() {
                        "not" => "not",
                        "neg" => "neg",
                        _ => panic!(""),
                    };

                    builder
                        .param_push()
                        .call_method(func, 1)
                        ;
                }
            }
            "postfixes" => {
                let val= top_group.child(0).unwrap();
                let postfixes = top_group.child(1).unwrap();

                builder.eval(val);

                //field(s), index(s), call(s)
                for x in postfixes.children() {
                    builder.eval(x);
                }
            }
            "field"|"index" => {
                let field_inner= top_group.child(0).unwrap(); //field_name|field_index|expr
                let is_symbol = field_inner.name()=="field_name";

                builder
                    .param_push() //self
                    .eval(field_inner)
                    .param_push() //val
                    .swap() //todo remove
                    .loc(top_group.start_loc())
                    .get_field(is_symbol);
            }
            "field_name" => {
                builder.result_string(top_group.tokens().trimmed().first().unwrap().get_identifier().unwrap().value);
            }
            "field_index" => {
                builder.result_int(top_group.tokens().trimmed().first().unwrap().get_int().unwrap().value);
            }

            "set_var" => {
                let name= top_group.child(0).unwrap().tokens().trimmed().first().unwrap().get_identifier().unwrap();
                let val= top_group.child(2).unwrap();
                let op=top_group.child(1).unwrap();
                let func=match op.name() {
                    "add" => "add",
                    "sub" => "sub",
                    "mul" => "mul",
                    "div" => "div",
                    "mod" => "mod",
                    "and" => "and",
                    "or" => "or",
                    "xor" => "xor",
                    "eq" => "",
                    _ => {panic!("");}
                    };

                //
                builder.eval(val);

                //
                if !func.is_empty() {
                    builder
                        .param_push()
                        .loc(name.token.start_loc())
                        .get_var(name.value)
                        .param_push()
                        .loc(op.start_loc())
                        .call_method(func, 2)
                        ;
                }

                //
                builder
                    .loc(name.token.start_loc())
                    .set_var(name.value)
                    ;
            }

            "set_field"|"set_index" => {
                let postfixes = top_group.child(0).unwrap();
                let op = top_group.child(1).unwrap();
                let expr = top_group.child(2).unwrap();

                let func=match op.name() {
                    "add" => "add",
                    "sub" => "sub",
                    "mul" => "mul",
                    "div" => "div",
                    "mod" => "mod",
                    "and" => "and",
                    "or" => "or",
                    "xor" => "xor",
                    "eq" => "",
                    _ => {panic!("");}
                    };

                let var=postfixes.child(0).unwrap();
                let fields=postfixes.child(1).unwrap();


                //
                builder.eval(var); //self

                //
                let last_call_ind = (0..fields.children().len()-1).rev().find(|&i|{
                    let c = fields.child(i).unwrap();
                    c.name()=="call_val" || c.name()=="call_field"
                }).unwrap_or(0);

                //
                if fields.children().len() != 1 {
                    //get field(s), index(s), call(s)
                    for i in last_call_ind..fields.children().len()-1 {
                        let field=fields.child(i).unwrap();

                        builder
                            .param_push() //self, dup
                            .eval(field) //evals field|index
                            ;
                    }

                    //
                    builder.param_push(); //dup
                }

                //
                builder
                    .param_push() //self
                    ;

                //
                if !func.is_empty() {
                    builder
                        .eval(fields.last().unwrap()) //evals field|index
                        .param_push() //prev_val
                        ;
                }

                //
                builder

                    .eval(expr)
                    .param_push()  //to
                ;

                //
                if !func.is_empty() {
                    builder
                        .swap()
                        .loc(op.start_loc())
                        .call_method(func, 2)
                        .param_push() //prev_val+to
                        ;
                }

                //
                {
                    let field=fields.last().unwrap(); //field|index
                    let field_inner=field.child(0).unwrap(); //field_name|field_index|expr
                    let is_symbol=field_inner.name()=="field_name";

                    builder
                        .eval(field_inner) //evals field_name|field_index|expr
                        .param_push() //field

                        .rot_left() //todo remove

                        .loc(field.start_loc())
                        .set_field(is_symbol, true) //why need islast? non last are optional?

                        ;
                }

                //set remaing fields chain
                for i in (0..fields.children().len()-1).rev() {
                    let field=fields.child(i).unwrap();  //field|index
                    let field_inner=field.child(0).unwrap(); //field_name|field_index|expr
                    let is_symbol=field_inner.name()=="field_name";

                    if i!=0 {
                        builder
                            .swap()
                            .dup()
                            .rot_left()
                            ;
                    }

                    builder
                        .eval(field_inner) //evals field_name|field_index|expr
                        .param_push() //field

                        .rot_left() //todo remove

                        .loc(field_inner.start_loc())
                        .set_field(is_symbol, false)
                        ;
                }
            }
            "call_val" => {
                let params=top_group.child(0).unwrap();

                builder
                    .block_start(None) //todo remove

                        .decl_anon_var("self", false) //todo remove
                        .set_anon_var("self") //todo remove

                        //
                        .eval(params)

                        //
                        .get_anon_var("self") //todo remove, replace with push above it's decl?

                        //
                        .loc(top_group.start_loc())
                        .call_result(params.children().len(),)

                    .block_end() //todo remove
                    ;
            }
            "call_field" => {
                let field= top_group.child(0).unwrap();
                let params=top_group.child(1).unwrap();

                builder
                    .block_start(None) //todo remove

                        .decl_anon_var("self", false) //todo remove
                        .set_anon_var("self") //todo remove

                        //
                        .eval(params)

                        //
                        .eval(field)
                        .param_push() //val

                        //
                        .get_anon_var("self") //todo remove, replace with push above it's decl?
                        .param_push() //self

                        //
                        .loc(top_group.start_loc())
                        .call_field(params.children().len(),)

                        // .call_method(name, params_num)
                    .block_end() //todo remove
                    ;
            }


            "call_func" => {
                let name= top_group.child(0).unwrap().tokens().trimmed().first().unwrap().get_identifier().unwrap();
                let params= top_group.child(1).unwrap();

                builder
                    .eval(params)
                    .loc(name.token.start_loc())
                    .call(name.value, params.children().len());
            }
            "params" => {
                for p in top_group.children().rev() {
                    builder
                        .eval(p)
                        .param_push();
                }
            }
            "block" => {
                builder.block_start(None);

                for x in top_group.children() {
                    builder.eval(x);
                }

                builder.block_end();
            }
            "if" => {
                //
                builder.block_start(None);

                //
                for c in top_group.children() {
                    if c.name()=="cond" {
                        builder
                            .block_start(None)
                                .eval(c.child(0).unwrap())
                                .to_block_end(JmpCond::False,0)
                                .eval(c.child(1).unwrap())
                                .to_block_end(JmpCond::None,1)
                            .block_end();
                    } else if c.name()=="else" {
                        builder.eval(c.child(0).unwrap());
                    } else {
                        panic!("");
                    }
                }

                //
                builder.block_end();
            }
            "for" => {
                let var= top_group.child(0).unwrap().name();
                let from= top_group.child(1).unwrap();
                let func=match top_group.child(2).unwrap().name() {
                    "to" => "lt",
                    "to_eq" => "le",
                    _ => panic!(""),
                };
                let to= top_group.child(3).unwrap();
                let body= top_group.child(4).unwrap();

                builder
                    .block_start(None)
                        .decl_var_start(var, false)

                            //why do these inside idn decl?
                            .eval(to)
                            .decl_anon_var("n", false)
                            .set_anon_var("n")

                            .decl_anon_var("r", false)
                            .result_void() //why set to void?? oh "r" is the value of the body's return
                            .set_anon_var("r")

                        .decl_var_end()

                        .eval(from)
                        .set_var(var)

                        .block_start(Some("loop"))
                            .block_start(None)
                                .get_anon_var("n")
                                .param_push()
                                .get_var(var) //shouldn't this be anon var i?
                                .param_push()
                                .call_method(func, 2)
                                // .to_block_end_label(Some(false),"loop", None)
                                .to_block_end(JmpCond::False, 1)

                                .result_void()
                                .eval(body)
                                .set_anon_var("r") //
                            .block_end()

                            //incr index
                            .result_int(1)
                            .param_push()
                            .get_var(var)
                            .param_push()
                            .call_method("add", 2)

                            .set_var(var)

                            //
                            .to_block_start(JmpCond::None,0)
                        .block_end()
                        .get_anon_var("r")
                    .block_end()
                    ;
            }

            "while" => {
                let cond= top_group.child(0).unwrap();
                let body= top_group.child(1).unwrap();

                builder
                    // .loop_instr()
                    .block_start(Some("loop"))
                        .eval_with_flags(cond,[("in_loop_cond",1)]) //so breaks/continues in cond, don't break out of this loop, instead break out of an outer loop
                        .param_push()
                        .call_method("not", 1)
                        .to_block_end(JmpCond::True //False
                            ,0)
                        // .eval_sexprs(body_stmts)
                        .eval(body)
                        .to_block_start(JmpCond::None,0)
                    .block_end()
                // .value_instr(&Value::Void)
                ;
            }

            "continue" => {
                let e=BuilderError::new(top_group.start_loc(), BuilderErrorType::ContinueNotInLoop);
                let skip=builder.get_flag("in_loop_cond").is_some();
                let skip = if skip {1} else {0};
                builder.to_block_start_label(JmpCond::None,"loop",skip,Some(e));
            }
            "break" => {
                let e=BuilderError::new(top_group.start_loc(), BuilderErrorType::ContinueNotInLoop);
                let skip=builder.get_flag("in_loop_cond").is_some();
                let skip = if skip {1} else {0};
                builder.to_block_end_label(JmpCond::None,"loop",skip,Some(e));
            }
            "return" => {
                if top_group.children().len()==1 {
                    builder.eval(top_group.child(0).unwrap());
                } else {
                    builder.result_void();
                }

                let e = BuilderError::new(top_group.start_loc(), BuilderErrorType::ReturnNotInFunc);
                builder.to_block_end_label(JmpCond::None, "func",0,Some(e));
            }
            "include" => {
                let v = top_group.child(0).unwrap().tokens().trimmed().first().unwrap().get_string().unwrap();
                builder.include(v.value, v.token.start_loc());
            }
            "var" => {
                let name= top_group.child(0).unwrap().tokens().trimmed().first().unwrap().get_identifier().unwrap();
                let val= top_group.child(1);

                builder.decl_var_start(name.value,val.is_none());

                if let Some(val)=val {
                    builder.eval(val);
                }

                builder.decl_var_end();

                if val.is_some() {
                    builder.set_var(name.value);
                }
            }

            "func" => {
                let name= top_group.child(0).unwrap().tokens().trimmed().first().unwrap().get_identifier().unwrap();

                let params= top_group.child(1).unwrap().children()
                    .map(|x|x.tokens().trimmed().first().unwrap().get_identifier().unwrap().value);
                let variadic = top_group.child(2).map(|x|x.name()=="variadic").unwrap_or_default();
                let body= top_group.child(3).unwrap();

                builder
                    .decl_var_start(name.value,false)
                    .decl_var_end();

                //
                builder
                    .func_start(params,variadic)
                        .block_start(Some("func"))
                            .eval(body)
                            .result_void()
                        .block_end()
                    .func_end();

                builder.set_var(name.value);
            }
            "lambda" => {

                let params= top_group.child(0).unwrap().children()
                    .map(|x|x.tokens().trimmed().first().unwrap().get_identifier().unwrap().value);
                let variadic = top_group.child(1).map(|x|x.name()=="variadic").unwrap_or_default();
                let body= top_group.child(2).unwrap();

                //
                builder
                    .func_start(params,variadic)
                        .block_start(Some("func"))
                            .eval(body)
                            .result_void()
                        .block_end()
                    .func_end();
            }



            "array" => {
                let elements= top_group.children();

                builder
                    .block_start(None)
                    .call_method("array", 0)
                    .decl_anon_var("d", false)
                    .set_anon_var("d")
                    ;

                for e in elements {
                    builder
                        .eval(e)
                        .param_push()
                        .get_anon_var("d")
                        .param_push()
                        .call_method("push", 2)
                        ;
                }

                builder
                    .get_anon_var("d")
                    .block_end()
                    ;
            }

            "dict" => {
                let elements= top_group.children();

                builder
                    .block_start(None)
                    .call_method("dict", 0)
                    .decl_anon_var("d", false)
                    .set_anon_var("d")
                    ;

                for e in elements {
                    let k=e.child(0).unwrap();
                    let v=e.child(1).unwrap();


                    builder
                        .eval(v)
                        .param_push();

                    if k.name()=="name" {
                        builder.result_string(k.tokens().trimmed().first().unwrap().get_identifier().unwrap().value);
                    } else {
                        builder.eval(k);
                    }

                    builder
                        .param_push()
                        .get_anon_var("d")
                        .param_push()
                        .call_method("insert", 3)
                        ;
                }

                builder
                    .get_anon_var("d")
                    .block_end()
                    ;
            }

            _ => {
                panic!("{}",top_group.name());
                // builder.eval(primitive)
                // top_group.children()

            }
        }

        //

        Ok(())
    }

}

