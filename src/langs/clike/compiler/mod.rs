
// // #![allow(unused_variables)]
// #![allow(unused)]
mod error;
// mod cmds;
mod rules;

use crate::clike::grammar::walker::GrammarWalker;
use crate::clike::grammar::GrammarWalkError;
// use crate::ccexpr_compiler::grammar::grammar_run;
// use std::path::PathBuf;
// use super::parser::parse;
use crate::clike::tokenizer::{tokenize, TokenizerErrorType};
use crate::primitive_types::StringVal;

use crate::build::*;
// use super::ccexpr_tokenizer::*;

use std::path::Path;

use crate::compiler::ast;


pub use error::*;

// use super::super::builder::*;

// use cmds::*;

// pub type CExprBuilder<'a>=Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>;
// type CExprBuilderTaken<'a>=BuilderTaken<'a,PrimitiveIterContainer<'a>,BuilderErrorType>;

// pub type Cmd = for<'a> fn(&mut PrimitiveIterContainer<'a>, &mut CExprBuilder<'a>) -> Result<(),BuilderError<BuilderErrorType>>;

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
        walker.set_debug(true);

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

        //
        // let walk=walker.get_walk();
        //

        // let parsed=parse(src.as_str());

        // if let Err(e)=parsed {
        //     // return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::Tokenizer(e.error_type)});
        // }
        // let parsed=tokenize(src.as_str(),  );

        // if let Err(e)=parsed {
        //     return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::Tokenizer(e.error_type)});
        // }

        // let parsed=parsed.unwrap();

        // parsed.print();

        // println!("===");
        // let walk=parse(parsed.tokens());

        // if let Err(e)= walk {
        //     return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::Parser(e.msg)});
        // }
        // println!("===");

        // parsed.print();

        //
        // let mut builder = builder::Builder::new();
        // // // builder.eval(parsed.root_block_primitive().get_block().unwrap().primitives());
        // // builder.eval(parsed.root_primitives());


        //builder needs to be passed a primitive_iter instead of primitive?

        //
        let mut ast = ast::Ast::new(false,true);

        // if let Err(e)=builder.generate_ast(&mut ast,|builder,primitive_iter|{
        //     self.run(builder, primitive_iter,&mut next_anon_id)
        // }) {
        //     return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::CexprBuilder(e.error_type)});
        // }

        // if let Err(e)=ast.calc_vars(false) {
        //     return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::AstVar(e.error_type)});
        // }

        // if let Err(e)=ast.calc_labels_gotos() {
        //     return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::AstVar(e.error_type)});
        // }

        // // if print_ast { ast.print(); }

        //
        let kept_src=if keep_src {Some(src.clone())} else {None};
        let build = ast.compile(version, path, kept_src,true,true);
        Ok(BuildT::new(build))

        //
        // Ok(BuildT::new(Build::default()))
    }


    // pub fn run<'a>(&self,
    //     builder:&mut CExprBuilder<'a>,
    //     mut top_primitive_iter:PrimitiveIterContainer<'a>,
    //     next_anon_id:&mut usize,
    // ) -> Result<(),BuilderError<BuilderErrorType>> {
    //     let prefixes : HashSet<&'static str>=["+","-","!"].into();
    //     let infixes : HashSet<&'static str>=["+","-","*","/","&&","||","^","==","!=",">=","<=","<",">","^","%"].into();
    //     let setters  : HashSet<&'static str>=["=","+=","-=","*=","/="].into();

    //     enum ExprVal<'b> {
    //         Builder(CExprBuilderTaken<'b>),
    //         Symbol(PrimitiveContainer<'b>),
    //         Identifier(PrimitiveContainer<'b>),
    //     }

    //     let mut cur_exprs: Vec<ExprVal>= Vec::new();

    //     //
    //     // while let Ok(first_primitive)=top_primitive_iter.pop_front()
    //     loop
    //     {
    //         match top_primitive_iter.first().map(|p|p.primitive_type()) {
    //             Ok(PrimitiveTypeContainer::Eob) => {

    //             }
    //             Ok(PrimitiveTypeContainer::Eol) => {

    //             }
    //             Ok(PrimitiveTypeContainer::Symbol(";")) => {

    //             }
    //             Ok(PrimitiveTypeContainer::Identifier(idn)) => {
    //                 match cur_exprs.last() {
    //                     Some(ExprVal::Identifier(_)) => {

    //                     }
    //                     Some(ExprVal::Builder(_)) => {

    //                     }
    //                     _ => {

    //                     }
    //                 }
    //                 if let Some(ExprVal::Identifier(_))=cur_exprs.last() {

    //                 }

    //             }
    //             Err(_) => {break;}
    //             _ => {}
    //         }


    //         let Ok(first_primitive)=top_primitive_iter.pop_front() else {break;};
    //         // let Ok(first_primitive)=top_primitive_iter.first() else { return Ok(()) };
    //         builder.loc(first_primitive.start_loc());
    //         println!("hmm {first_primitive:?}",);

    //         let mut done=false;


    //         builder.mark();

    //         match first_primitive.primitive_type() {
    //             PrimitiveTypeContainer::CurlyBlock(b) => { //code block
    //                 builder.eval(b);
    //                 cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    //             }
    //             PrimitiveTypeContainer::SquareBlock(b) => { //array or dict
    //                 // let is_dict=b.children().find(|p|p.get_symbol().map(|s|s.eq(":")).unwrap_or(false)).is_some();
    //             }
    //             PrimitiveTypeContainer::ParenthesesBlock(b) => {} //expr

    //             PrimitiveTypeContainer::Float(x) => { //float
    //                 builder.result_float(x);
    //             }
    //             PrimitiveTypeContainer::Int(x) => { //int
    //                 builder.result_int(x);
    //             }
    //             PrimitiveTypeContainer::String(x) => { //string
    //                 builder.result_string(x);
    //             }
    //             PrimitiveTypeContainer::Symbol(x) => { //
    //                 match x {
    //                     ";" => {
    //                         // done=true;
    //                     }
    //                     _ => {
    //                         cur_exprs.push(ExprVal::Symbol(first_primitive));
    //                     }
    //                 }
    //             }
    //             PrimitiveTypeContainer::Identifier(x) => { //cmd or idn
    //                 // println("")
    //                 if let Some(cmds)=self.cmds.get(x) {
    //                     // let mut primitives=top_primitive_iter.clone();
    //                     let mut errors=Vec::<BuilderError<BuilderErrorType>>::new();

    //                     //
    //                     builder.set_anon_scope(*next_anon_id);

    //                     //
    //                     for cmd in cmds {
    //                         let mut primitives=top_primitive_iter.clone();

    //                         if let Err(e)=cmd(&mut primitives,builder) {
    //                             errors.push(e);
    //                             builder.discard_from_mark();
    //                         } else { //ok
    //                             errors.clear();
    //                             *next_anon_id+=1;
    //                             top_primitive_iter=primitives;
    //                             cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    //                             break;
    //                         }
    //                     }

    //                     //
    //                     builder.set_anon_scope(0);

    //                     //
    //                     if errors.len()>0 {
    //                         errors.sort_by(|a,b|a.loc.cmp(&b.loc));
    //                         return Err(errors.last().unwrap().clone());
    //                     }
    //                 } else {
    //                     match x {
    //                         "true" => {
    //                             builder.result_bool(true);
    //                             cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    //                         }
    //                         "false" => {
    //                             builder.result_bool(false);
    //                             cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    //                         }
    //                         "nil" => {
    //                             builder.result_nil();
    //                             cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    //                         }
    //                         "void" => {
    //                             builder.result_void();
    //                             cur_exprs.push(ExprVal::Builder(builder.take_from_mark()));
    //                         }
    //                         "var" => {
    //                             //var decls ...
    //                         }
    //                         _ => {
    //                             cur_exprs.push(ExprVal::Identifier(first_primitive));
    //                             // builder.get_var(x);
    //                         }
    //                     }
    //                 }
    //             }
    //             // PrimitiveTypeContainer::End => {} //eol or eof //ignore
    //             PrimitiveTypeContainer::Eol|PrimitiveTypeContainer::Eob => {
    //                 if let Some(ExprVal::Symbol(_))=cur_exprs.last() {
    //                     //there aren't any postfix symbols, so don't need to handle
    //                 } else {
    //                     done=true;
    //                 }
    //             }
    //         }

    //     }

    //     //handle exprs

    //     //
    //     Ok(())
    // }

}
