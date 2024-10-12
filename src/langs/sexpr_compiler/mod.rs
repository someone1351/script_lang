// pub mod sexpr_parser;
// pub mod builder;

pub mod cmds;
pub mod cmd_scope;

#[derive(Debug,Clone)]
pub enum SexprBuilderErrorType {
    ExpectSymbol,
    ExpectList,
    ExpectString,
    IncorrectParamsNum,
    NoParamsAllowed,
    ExpectExpr,
    DeclFuncNotRoot,
    ElseMustBeAtEnd,
    ExpectParamName,
    VariadicMustBeAtEnd,
    EmptySExpr, //
    ContinueNotInLoop,
    BreakNotInLoop,
    ReturnNotInMethodOrLambda,
    // BuilderAst(BuilderAstError),

}



use std::path::{Path, PathBuf};
use super::super::common::*;
use super::sexpr_parser;
use super::super::compiler::*;




#[derive(Debug,Clone)]
pub enum CompileErrorType {
    SexprBuilder(SexprBuilderErrorType),
    SexprParser(sexpr_parser::ParserErrorType),
    AstVar(ast::error::AstVarErrorType),
}

#[derive(Debug,Clone)]
pub struct CompileError {
    pub src : StringT,
    pub path : Option<PathBuf>,
    pub error_type : CompileErrorType,
    pub loc : Loc,
}

impl CompileError {
    pub fn msg(&self) -> String {
        error_msg(&self.error_type, self.loc, Some(self.src.as_str()), self.path.as_ref().map(|p|p.as_path()))
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",error_msg(&self.error_type,self.loc,Some(self.src.as_str()),self.path.as_ref().map(|p|p.as_path())))
    }
}

impl std::error::Error for CompileError {
    fn description(&self) -> &str {
        "scriptlang compile error"
    }
}


pub fn sexpr_compile(src : &str, version:usize, path : Option<&Path>, keep_src : bool, print_ast:bool) -> Result<BuildT,CompileError> {
    let src= StringT::new(src);
    let pathbuf=path.map(|x|x.to_path_buf());
    
    match sexpr_parser::parse(src.as_str(), false, path) {
        Ok(sexpr_tree)=> {
            let mut cmd_scope= cmd_scope::CmdScope::new_core();
            let mut builder = builder::Builder::new();

            for x in sexpr_tree.sexprs() {
                builder.eval(x);
            }

            // builder.eval_sexprs(sexpr_tree.sexprs());
            
            let mut ast = ast::Ast::new(false,false);
                
            match builder.generate_ast(&mut ast,|builder,primitive|{
                cmd_scope.run(builder, primitive)
            }) {
                Ok(_)=>{
                    if let Err(e)=ast.calc_vars(false) {
                        return Err(CompileError{
                            path:pathbuf,
                            src,
                            loc:e.loc,
                            error_type:CompileErrorType::AstVar(e.error_type),
                        });
                    }

                    if print_ast {
                        ast.print();
                    }

                    // let kept_src=if keep_src {Some(common::StringType::new(src))} else {None};
                    let kept_src=if keep_src {Some(src.clone())} else {None};
                    
                    let build = ast.compile(version, path, kept_src,true,true);                    
                    Ok(BuildT::new(build))
                }
                Err(builder_error)=> {
                    Err(CompileError{
                        path:pathbuf,
                        src,
                        loc:builder_error.loc,
                        error_type:CompileErrorType::SexprBuilder(builder_error.error_type),
                    })
                }
            }
        }
        Err(parser_error)=>{
            Err(CompileError{
                path:pathbuf,
                src,
                loc:parser_error.loc,
                error_type:CompileErrorType::SexprParser(parser_error.error_type),
            })
        }
    }
}

