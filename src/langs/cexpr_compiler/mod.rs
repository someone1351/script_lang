
#![allow(unused_variables)]

mod error;


// use std::path::PathBuf;
use crate::StringVal;

use super::super::build::*;
use super::cexpr_parser::*;

use std::path::Path;

use super::{ast, builder,  };


pub use error::*;

use super::super::builder::*;



// pub type Cmd = Box<dyn for<'a> Fn(RecordContainer<'a>, &mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>>>;


pub struct Compiler {
    // cmds : HashMap<String,Vec<Cmd>>,
}

impl Compiler {
    pub fn new_empty() -> Self {
        Self{
            // cmds:Default::default(),
        }
    }
    pub fn new() -> Self {
        Self {

        }
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
        builder.eval(parsed.root_block_primitive());

        //
        let mut ast = ast::Ast::new(false,true);

        if let Err(e)=builder.generate_ast(&mut ast,|builder,primitive|{
            self.run(builder, primitive,&mut next_anon_id)
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
        builder:&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>,
        top_primitive:PrimitiveContainer<'a>,
        next_anon_id:&mut usize,
    ) -> Result<(),BuilderError<BuilderErrorType>> {
        builder.loc(top_primitive.start_loc());

        match top_primitive.primitive_type()
        {
            PrimitiveTypeContainer::Root(b) => { //root

            }
            PrimitiveTypeContainer::CurlyBlock(b) => { //code block

            }
            PrimitiveTypeContainer::SquareBlock(b) => { //array or dict
                let is_dict=b.primitives().find(|p|p.get_symbol().map(|s|s.eq(":")).unwrap_or(false)).is_some();


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
            PrimitiveTypeContainer::Symbol(x) => { //cmd or idn
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
                        builder.get_var(x);
                    }
                }
            }
            PrimitiveTypeContainer::End => {} //eol or eof //ignore
        }

        Ok(())
    }

}
