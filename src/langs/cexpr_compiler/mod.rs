
/*
* can't do something like if 3 < 2 {123} else {4}
** because would need to split cmds up into multiple functions, one for each of the: if_cond, elif_cond, else
*** the problem with that is, if a cmd fails, would need to backtrack, which would be a hassle ish to implement

* when going through expressions, need to go from left to right, handling


* check if idn has a cmd, and run that cmd on it

* if idn or (idn), check afterward
** if prefix/infix then eval
** if setter then (set idn val), (set idn (+ idn val)) etc

** does putting (x) or {x} still identify as a var? or just eval it?

* could convert all stmts to sexprs eg (if (cond) (then) (elif) (elif) (else))

* how to tell dif between (1;-2) and (1-2)?
    1
    -2
** could use brackets?
    (1
    -2)
** treat as single expr, if want to have it as two vals then do
    1;
    -2

** any infix expr before a val, will be consued by prev expr parsing, same with after
===

* could treat cmds like prefixes

* could convert whole thing into sexprs, and run the builder on that instead

* when "if" is used as a stmt, needs "end" before another stmt can be used, but not if used as expr
** could treat stmts as exprs that just return void
** so exprs need symbols between the vars, then everything would need an end eg eol/eob/semicolon

*/
#![allow(unused_variables)]

mod error;
mod cmds;

// use std::path::PathBuf;
use crate::StringVal;

use super::super::build::*;
use super::cexpr_parser::*;

use std::{collections::{HashMap, HashSet}, path::Path};

use super::{ast, builder,  };


pub use error::*;

use super::super::builder::*;

use cmds::*;

pub type Cmd = for<'a> fn(&mut PrimitiveIterContainer<'a>, &mut Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>>;

/*

infix
* add, sub, mul, div
* mod, pow
* gt,lt, ge,le,eq,ne
* set, set_add,set_sub,set_mul,set_div

prefix
* pos, neg
* not

expr
* if
* lambda

stmt
* for, while, break
* format, print, println
* func, return
* include
* var
 */

pub struct Compiler {
    cmds : HashMap<&'static str,Vec<Cmd>>,

}

impl Compiler {
    pub fn new_empty() -> Self {
        Self{
            cmds:Default::default(),

        }
    }
    pub fn new() -> Self {
        let mut cmds: HashMap<&'static str,Vec<Cmd>> = HashMap::new();

        cmds.insert("break", vec![break_cmd]);
        cmds.insert("continue", vec![continue_cmd]);
        cmds.insert("for", vec![for_cmd]);
        cmds.insert("format", vec![format_cmd,]);
        cmds.insert("fn", vec![func_cmd, lambda_cmd]);
        cmds.insert("if", vec![if_cmd]);
        cmds.insert("include", vec![include_cmd]);
        cmds.insert("print", vec![print_cmd]);
        cmds.insert("println", vec![println_cmd]);
        cmds.insert("return", vec![return_cmd]);
        cmds.insert("var", vec![var_cmd]);
        cmds.insert("while", vec![while_cmd]);

        Self { cmds }
    }







    pub fn compile(&self,src : &str, version:usize, path : Option<&Path>, keep_src : bool, ) -> Result<BuildT,CompileError> {
        let mut next_anon_id=1;

        let src= StringVal::new(src);
        let pathbuf=path.map(|x|x.to_path_buf());

        let parsed=parse(src.as_str(),  );

        if let Err(e)=parsed {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::CexprParser(e.error_type)});
        }

        let parsed=parsed.unwrap();

        parsed.print();

        //
        let mut builder = builder::Builder::new();
        // builder.eval(parsed.root_block_primitive().get_block().unwrap().primitives());
        builder.eval(parsed.root_primitives());


        //builder needs to be passed a primitive_iter instead of primitive?

        //
        let mut ast = ast::Ast::new(false,true);

        if let Err(e)=builder.generate_ast(&mut ast,|builder,primitive_iter|{
            self.run(builder, primitive_iter,&mut next_anon_id)
        }) {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::CexprBuilder(e.error_type)});
        }

        if let Err(e)=ast.calc_vars(false) {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::AstVar(e.error_type)});
        }

        if let Err(e)=ast.calc_labels_gotos() {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CompileErrorType::AstVar(e.error_type)});
        }

        // if print_ast { ast.print(); }

        //
        let kept_src=if keep_src {Some(src.clone())} else {None};
        let build = ast.compile(version, path, kept_src,true,true);
        Ok(BuildT::new(build))

        //
        // Ok(BuildT::new(Build::default()))
    }


    pub fn run<'a>(&self,
        builder:&mut Builder<'a,PrimitiveIterContainer<'a>,BuilderErrorType>,
        top_primitive_iter:PrimitiveIterContainer<'a>,
        next_anon_id:&mut usize,
    ) -> Result<(),BuilderError<BuilderErrorType>> {
        let prefixes : HashSet<&'static str>=["+","-","!"].into();
        let infixes : HashSet<&'static str>=["+","-","*","/","&&","||","^","==","!=",">=","<=","<",">","^","%"].into();
        let setters  : HashSet<&'static str>=["=","+=","-=","*=","/="].into();

        let Some(first_primitive)=top_primitive_iter.first() else { return Ok(()) };
        builder.loc(first_primitive.start_loc());

        match first_primitive.primitive_type() {
            PrimitiveTypeContainer::CurlyBlock(b) => { //code block
                builder.eval(b.children());
            }
            PrimitiveTypeContainer::SquareBlock(b) => { //array or dict
                // let is_dict=b.children().find(|p|p.get_symbol().map(|s|s.eq(":")).unwrap_or(false)).is_some();


            }
            PrimitiveTypeContainer::ParenthesesBlock(b) => {} //expr

            PrimitiveTypeContainer::Float(x) => { //float
                builder.result_float(x);
            }
            PrimitiveTypeContainer::Int(x) => { //int
                builder.result_int(x);
            }
            PrimitiveTypeContainer::String(x) => { //string
                builder.result_string(x);
            }
            PrimitiveTypeContainer::Symbol(x) => { //
            }
            PrimitiveTypeContainer::Identifier(x) => { //cmd or idn
                if let Some(cmds)=self.cmds.get(x) {
                    let mut primitives=top_primitive_iter.get_range(1..);
                    // let mut primitives=top_primitive_iter.clone();
	                let mut errors=Vec::<BuilderError<BuilderErrorType>>::new();

                    //
                    builder.temp_mark();
                    builder.set_anon_scope(*next_anon_id);

                    //
                    for cmd in cmds {
                        if let Err(e)=cmd(&mut primitives,builder) {
                            errors.push(e);
                            builder.temp_clear();
                        } else { //ok
                            errors.clear();
                            *next_anon_id+=1;
                            break;
                        }
                    }

                    //
	                builder.set_anon_scope(0);
                } else {
                    match x {
                        "true" => {
                            builder.result_bool(true);
                        }
                        "false" => {
                            builder.result_bool(false);
                        }
                        "nil" => {
                            builder.result_nil();
                        }
                        "void" => {
                            builder.result_void();
                        }
                        _ => {
                            // builder.get_var(x);
                        }
                    }
                }
            }
            // PrimitiveTypeContainer::End => {} //eol or eof //ignore
            PrimitiveTypeContainer::Eol => {}
            PrimitiveTypeContainer::Eob => {}
        }

        Ok(())
    }

}
