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



use std::collections::HashMap;
use std::path::{Path, PathBuf};
use builder::{Builder, BuilderError};
use cmds::*;

use super::super::common::*;
use super::sexpr_parser::{self, SExprContainer, SExprValContainer};
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



pub type Cmd = for<'a> fn(SExprContainer<'a>, &mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>) -> Result<(),BuilderError<SexprBuilderErrorType>>;


#[derive(Copy,Clone)]
pub struct CmdIter<'a> {
    index : usize,
    v : &'a Vec<Cmd>,
}

impl<'a> Iterator for CmdIter<'a> {
    type Item = Cmd;

    fn next(&mut self) -> Option<Self::Item> {
        self.index+=1;
        self.v.get(self.index-1).and_then(|x|Some(*x))
    }
}

pub struct Compiler {
    cmds : HashMap<String,Vec<Cmd>>,
    next_anon_id:usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self{
            cmds:Default::default(),
            next_anon_id:1,
        }
    }
    pub fn new_core() -> Self {
        let mut cmd_scope = Self::new();

        cmd_scope.insert("while", while_cmd);
        cmd_scope.insert("for", for_cmd);
        cmd_scope.insert("continue", continue_cmd);
        cmd_scope.insert("break", break_cmd);
        cmd_scope.insert("return", return_cmd);
        cmd_scope.insert("var", decl_var_cmd);
        cmd_scope.insert("set", set_var_cmd);
        cmd_scope.insert("set", set_field_cmd);
        cmd_scope.insert("get", get_field_cmd);
        cmd_scope.insert("if", if_cmd);
        cmd_scope.insert("ternary", ternary_cmd);
        cmd_scope.insert("and", and_cmd);
        cmd_scope.insert("or", or_cmd);
        cmd_scope.insert("fn", decl_func_cmd);
        cmd_scope.insert("fn", lambda_func_cmd);
        cmd_scope.insert("+", add_cmd);
        cmd_scope.insert("-", sub_cmd);
        cmd_scope.insert("*", mul_cmd);
        cmd_scope.insert("/", div_cmd);
        cmd_scope.insert("block", block_cmd);
        cmd_scope.insert("include", include_cmd);
        cmd_scope.insert("print", print_cmd);
        cmd_scope.insert("println", println_cmd);
        cmd_scope.insert("format", format_cmd);
        // cmd_scope.insert("method", decl_method_cmd);

        cmd_scope
    }

    pub fn insert(&mut self,k:&str,cmd : Cmd) {
        let v=self.cmds.entry(k.to_string()).or_insert_with(Default::default);
        v.push(cmd);
    }

    pub fn get(&self,k:&str) -> Option<CmdIter> {
        if let Some(v)=self.cmds.get(k) {
            Some(CmdIter { index: 0, v })
        } else {
            None
        }
    }

    pub fn run<'a>(
        & self,
        builder:&mut Builder<'a,SExprContainer<'a>,SexprBuilderErrorType>,
        sexpr:SExprContainer<'a>,
        next_anon_id:&mut usize,
    ) -> Result<(),BuilderError<SexprBuilderErrorType>> {
        builder.loc(sexpr.start_loc());

        match sexpr.val() {
            SExprValContainer::List(_)=>{
                let Some(v0)=sexpr.get(0) else {
                    return Err(BuilderError::new(sexpr.start_loc(), SexprBuilderErrorType::EmptySExpr)); //empty list
                };
            
                builder.loc(v0.start_loc());

                if let Some(symbol)=v0.symbol() {
                    if let Some(cmds)=self.get(symbol) {  //command
                        let mut errors=Vec::<BuilderError<SexprBuilderErrorType>>::new();

                        // let last_loc=builder.cur_loc;
                        // let temp_stk_last_len = builder.temp_stk.len();
                        // builder.in_cmd=true;
                        builder.temp_mark();
                        builder.anon_scope(self.next_anon_id);

                        for cmd in cmds {
                            //
                            if let Err(e)=cmd(sexpr,builder) {
                                errors.push(e);

                                //reset
                                // builder.cur_loc=last_loc;
                                // builder.temp_stk.truncate(temp_stk_last_len);
                                builder.temp_clear();
                            } else { //ok
                                errors.clear();
                                *next_anon_id+=1;
                                break;
                            }
                        }

                        //
                        // builder.in_cmd=false;
                        builder.anon_scope(0);

                        //
                        if errors.len()>0 {
                            errors.sort_by(|a,b|a.loc.cmp(&b.loc));
                            return Err(errors.last().unwrap().clone());
                        }
                    } else { // call
                        // builder.param_eval_push_sexprs(sexpr.list_iter_from(1).rev());
                        for sexpr in sexpr.list_iter_from(1).rev() {
                            // self.add_node(BuilderNodeType::EvalSexpr(sexpr));
                            // self.add_node(BuilderNodeType::StackParamPush);
                            builder.eval(sexpr);
                            builder.param_push();
                        }
                
                        // .eval_push_sexprs(sexpr.list_iter_from(1))
                        
                        // for x in sexpr.list_iter_from(1).rev() {
                        //     // builder.param_loc(x.start_loc(),x.end_loc());
                        // }

                        builder.loc(v0.start_loc());
                        builder.call(symbol,sexpr.len()-1);
                    }
                } else { //call value
                    // builder.param_eval_push_sexprs(sexpr.list_iter_from(1).rev()); //params

                    for sexpr in sexpr.list_iter_from(1).rev() {
                        builder.eval(sexpr);
                        builder.param_push();
                    }
                    builder.eval(v0); //func;
                    // .call(symbol,sexpr.len()-1,v0.start_loc())
                        
                    
                
                    // for x in sexpr.list_iter_from(1).rev() {
                    //     // builder.param_loc(x.start_loc(),x.end_loc());
                    // }

                    // builder.commit_param_locs();

                    builder.loc(v0.start_loc());
                    // builder.add_node(BuildersNodeType::CallResult(sexpr.len()-1));
                    builder.call_result(sexpr.len()-1);
                    // return Err(BuilderError::new(sexpr.start_loc(), BuilderErrorType::InvalidSyntax)); //first element of list not a symbol
                }
                
            }
            SExprValContainer::Symbol(symbol)=> { //get var
                match symbol {
                    "true"=>{
                        builder.result_bool(true);
                    }
                    "false"=>{
                        builder.result_bool(false);
                    }
                    "nil"=>{
                        builder.result_nil();
                    }
                    _=>{
                        if let Some(s)=symbol.starts_with(":").then_some(symbol).and_then(|_|{ //whats with the and_then?
                            let s=&symbol[":".len()..];
                            (!s.is_empty()).then_some(s)
                        }) {
                            builder.result_string(s);
                        } else {
                            builder.get_var(symbol);
                        }
                    }
                }
            }
            SExprValContainer::Bool(x)=> {
                builder.result_bool(x);
            }
            SExprValContainer::Int(x)=>{
                builder.result_int(x as IntT);
            }
            SExprValContainer::Float(x)=> {
                builder.result_float(x as FloatT);
            }
            SExprValContainer::String(x)=>{
                builder.result_string(x);
            }
        }

        Ok(())
    }


    pub fn compile(&self,src : &str, version:usize, path : Option<&Path>, keep_src : bool, print_ast:bool) -> Result<BuildT,CompileError> {
        let mut next_anon_id=1;
        let src= StringT::new(src);
        let pathbuf=path.map(|x|x.to_path_buf());
        
        match sexpr_parser::parse(src.as_str(), false, path) {
            Ok(sexpr_tree)=> {
                let mut builder = builder::Builder::new();
    
                for x in sexpr_tree.sexprs() {
                    builder.eval(x);
                }
    
                // builder.eval_sexprs(sexpr_tree.sexprs());
                
                let mut ast = ast::Ast::new(false,false);
                    
                match builder.generate_ast(&mut ast,|builder,primitive|{
                    self.run(builder, primitive,&mut next_anon_id)
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
    
}
