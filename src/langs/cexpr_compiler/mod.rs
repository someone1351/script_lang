
/*
label abc
goto abc
var a abc
goto a
*/
/*
set a :x 5
set a.x 5
set a["x"] 5
set x[0].y 5
set z[0].y[5] 7

*/

/*

 for something like
{fn {} 1; 123}

 {
    fn {} 1;
    123
 }


*/

// pub mod cexpr_parser;
// pub mod cmd_scope;
pub mod cmds;



use std::path::PathBuf;
use super::super::common::*;
use super::cexpr_parser;


use std::collections::HashMap;
use std::path::Path;

use super::cexpr_parser::*;
use super::{ast, builder,  };
use cmds::*;


use super::super::builder::*;


#[derive(Debug,Clone)]
pub enum BuilderErrorType {
    ExpectSymbol,
    // ExpectList,
    ExpectString,
    IncorrectParamsNum,
    NoParamsAllowed,
    InvalidParam,
    // ExpectExpr,
    // DeclFuncNotRoot,
    // ExpectValue,
    // ExpectParamName,
    VariadicMustBeAtEnd,
    // EmptySExpr, //
    // // BuilderAst(BuilderAstError),

    
    ContinueNotInLoop,
    BreakNotInLoop,
    ReturnNotInMethodOrLambda,

    ExpectBlock,
    NoSemiColonsAllowed,
    NoBlocksAllowed,
    NoFieldsAllowed,
    InvalidStringSymbol,
    // NoCmdFound,
    // NoArgsAllowed,
}





#[derive(Debug,Clone)]
pub enum CexprCompileErrorType {
    CexprBuilder(BuilderErrorType),
    CexprParser(cexpr_parser::ParserErrorType),
    AstVar(ast::error::AstVarErrorType),
}

#[derive(Debug,Clone)]
pub struct CompileError {
    pub src : StringT,
    pub path : Option<PathBuf>,
    pub error_type : CexprCompileErrorType,
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




pub type Cmd = for<'a> fn(RecordContainer<'a>, &mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>>;


pub struct Compiler {
    cmds : HashMap<String,Vec<Cmd>>,
    // next_anon_id:usize,
}

impl Compiler {
    pub fn new_empty() -> Self {
        Self{
            cmds:Default::default(),
            // next_anon_id:1,
        }
    }
    pub fn new() -> Self {
        let mut cmd_scope = Self::new_empty();
        cmd_scope.add_cmd("while", while_cmd);
        cmd_scope.add_cmd("for", for_cmd);
        cmd_scope.add_cmd("continue", continue_cmd);
        cmd_scope.add_cmd("break", break_cmd);
        cmd_scope.add_cmd("return", return_cmd);
        cmd_scope.add_cmd("var", var_cmd);
        cmd_scope.add_cmd("set", set_var_cmd);
        // cmd_scope.add_cmd("set", set_field_cmd);
        // cmd_scope.add_cmd("get", get_field_cmd);
        cmd_scope.add_cmd("if", if_cmd);
        cmd_scope.add_cmd("and", and_cmd);
        cmd_scope.add_cmd("or", or_cmd);
        cmd_scope.add_cmd("+", add_cmd);
        cmd_scope.add_cmd("-", sub_cmd);
        cmd_scope.add_cmd("*", mul_cmd);
        cmd_scope.add_cmd("/", div_cmd);
        cmd_scope.add_cmd("include", include_cmd);
        cmd_scope.add_cmd("format", format_cmd);
        cmd_scope.add_cmd("print", print_cmd);
        cmd_scope.add_cmd("println", println_cmd);
        cmd_scope.add_cmd("fn", func_cmd);
        cmd_scope.add_cmd("fn", lambda_cmd);
        cmd_scope.add_cmd("call", call_func_cmd);
        cmd_scope.add_cmd("?", ternary_cmd);
        

        cmd_scope
    }

    pub fn add_cmd(&mut self,k:&str,cmd : Cmd) {
        self.cmds.entry(k.to_string()).or_insert_with(Default::default).push(cmd);
    }

    fn get(&self,k:&str) -> Option<std::slice::Iter<Cmd>> {
        if let Some(v)=self.cmds.get(k) {
            // let it: std::slice::Iter<Cmd>=;
            Some(v.iter())
        } else {
            None
        }
    }

    pub fn run<'a>(&self,
        builder:&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>,
        top_primitive:PrimitiveContainer<'a>,
        next_anon_id:&mut usize,
    ) -> Result<(),BuilderError<BuilderErrorType>> {
        builder.loc(top_primitive.start_loc());
        let hasnt_fields=top_primitive.param().map(|x|x.fields_num()==0).unwrap_or(true);

        match top_primitive.primitive_type() //sexpr.val() 
        {
            PrimitiveTypeContainer::Block(block) => {
                //on no records, should to to void or nil?
                //  set x {}
                //  set x {{}}
                //  for i 0 5 {}
                //void would be better, I think the interpreter will check for void, so it is handled at runtime
                // each empty block returns void, eg {{}}, the inner block will return the void to the outer block

                if block.records_num()==0 {
                    builder.result_void();
                }

                //
                for record in block.records() {
                    builder.loc(record.start_loc());
                
                    if record.params_num()==0 {
                        builder.result_void();
                    }

                    //check if cmd,
                    //else if symbol with params is a method call
                    //else if symbol with no params, then is either a method call or a return of a variable

                    //{a} //a is either a method or a var

                    // let Some(first_primitive)=record.first_param().map(|x|x.primitive()) else {continue;};

                    if let Some(first_param)=record.first_param() {

                        if let Some(symbol)=first_param.primitive().symbol() {
                            if ["true","false","nil","void"].contains(&symbol) || symbol.starts_with(":") {
                                if record.params_num()==1 {
                                    builder.eval(first_param.primitive());
                                } else {
                                    return Err(BuilderError { loc: record.start_loc(), error_type: BuilderErrorType::NoParamsAllowed });
                                }
                            } else if first_param.fields_num()==0 { //no fields
                                if let Some(cmds)=self.get(symbol) {  //command
                                    let mut errors=Vec::<BuilderError<BuilderErrorType>>::new();
        
                                    // let last_loc=builder.cur_loc;
                                    // let temp_stk_last_len = builder.temp_stk.len();
                                    builder.temp_mark();
                                    
                                    // builder.in_cmd=true;
                                    builder.anon_scope(*next_anon_id);
        
                                    for cmd in cmds {
                                        //
                                        if let Err(e)=cmd(record,builder) {
                                            errors.push(e);
        
                                            //reset
                                            // builder.cur_loc=last_loc;
                                            // builder.temp_stk.truncate(temp_stk_last_len);
                                            builder.temp_clear();
                                        } else { //ok
                                            errors.clear();
                                            // builder.next_cmd_anon_id+=1;
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
                                } else if record.params_num()==1 { //no args, no fields
                                    builder.get_var_or_call_method(symbol);
                                } else { //has args, no fields
                                    for i in (1 .. record.params_num()).rev() {
                                        let x=record.param(i).unwrap();
                                        builder.param_loc(x.start_loc(),x.end_loc());
                                        builder.eval(x.primitive());
                                        builder.param_push();
                                    }
                                    
                                    builder.commit_param_locs();
                                    builder.loc(first_param.start_loc());
                                    builder.call_method(symbol, record.params_num()-1);
                                }
                            } else if record.params_num()==1 { //has fields, no args
                                builder.eval(first_param.primitive()); 
                            } else { //args and fields
                                return Err(BuilderError { loc: record.start_loc(), error_type: BuilderErrorType::NoParamsAllowed });
                            }
                        } else if record.params_num()==1 { //no args, first not symbol
                            builder.eval(first_param.primitive()); //not symbol, no args
                        } else { //has args, first not symbol
                            return Err(BuilderError { loc: record.start_loc(), error_type: BuilderErrorType::ExpectSymbol });
                        }
                    }

                }

                
                if !hasnt_fields {
                    Self::get_fields(builder,top_primitive);
                }
            }
            PrimitiveTypeContainer::Float(f) => {
                builder.result_float(f as FloatT);
            }
            PrimitiveTypeContainer::Int(i) => {
                builder.result_int(i as IntT);
            }
            PrimitiveTypeContainer::String(s) => {
                builder.result_string(s);
            }
            PrimitiveTypeContainer::Symbol(symbol) => {

                match symbol {
                    "true" =>{
                        if hasnt_fields {
                            builder.result_bool(true);
                        } else {
                            return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::NoFieldsAllowed });
                        }
                    }
                    "false" =>{
                        if hasnt_fields {
                            builder.result_bool(false);
                        } else {
                            return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::NoFieldsAllowed });
                        }
                    }
                    "nil" =>{
                        if hasnt_fields {
                            builder.result_nil();
                        } else {
                            return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::NoFieldsAllowed });
                        }
                    }
                    "void" =>{
                        if hasnt_fields {
                            builder.result_void();
                        } else {
                            return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::NoFieldsAllowed });
                        }
                    }
                    _=>{
                        if symbol.starts_with(":") { //what is with the s + and_then ?
                            if hasnt_fields {
                                let s=&symbol[":".len()..];

                                if !s.is_empty() {
                                    builder.result_string(s);
                                } else {
                                    return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::InvalidStringSymbol });
                                }
                            } else {
                                return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::NoFieldsAllowed });
                            }
                        } else {
                            builder.get_var(symbol);

                            if !hasnt_fields {
                                Self::get_fields(builder,top_primitive);
                            }
                        }
                    }
                }
            }

        }

        Ok(())
    }

    fn get_fields<'a>(
        builder:&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>,
        top_primitive:PrimitiveContainer<'a>,
        // fields:FieldIter<'a>,
    ) {
        
        let mut last_start_loc=top_primitive.start_loc();
        let mut last_end_loc=top_primitive.end_loc();

        //fields
        for field in top_primitive.param().unwrap().fields() {
            builder
                .param_loc(last_start_loc,last_end_loc)
                .param_push(); //last result

            // let field=symbol.field(field_ind).unwrap();

            builder.eval(field.primitive());

            
            builder
                .param_loc(field.start_loc(),field.end_loc())
                .param_push()
                .swap()
                ;

            builder.loc(field.start_loc());

            //
            builder.call_method("get_field", 2);
            
            last_start_loc=field.start_loc();
            last_end_loc=field.end_loc();
        }
    }

    pub fn compile(&self,src : &str, version:usize, path : Option<&Path>, keep_src : bool) -> Result<BuildT,CompileError> {
        let mut next_anon_id=1;

        let src= StringT::new(src);
        let pathbuf=path.map(|x|x.to_path_buf());

        let parsed=cexpr_parser::parse(src.as_str(),  );

        if let Err(e)=parsed {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CexprCompileErrorType::CexprParser(e.error_type)});
        }

        let parsed=parsed.unwrap();

        // println!("{:?}",parsed.root_block_primitive());
        // let mut cmd_scope= CmdScope::new_core();
        let mut builder = builder::Builder::new();

        builder.eval(parsed.root_block_primitive());
        
        let mut ast = ast::Ast::new(false,true);
        
        if let Err(e)=builder.generate_ast(&mut ast,|builder,primitive|{
            self.run(builder, primitive,&mut next_anon_id)
        }) {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CexprCompileErrorType::CexprBuilder(e.error_type)});
        }
        
        if let Err(e)=ast.calc_vars(false) {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CexprCompileErrorType::AstVar(e.error_type)});
        }

        // ast.calc_vars(false,true);

        // if print_ast {
        //     ast.print();
        // }

        // let kept_src=if keep_src {Some(common::StringType::new(src))} else {None};
        let kept_src=if keep_src {Some(src.clone())} else {None};
        
        let build = ast.compile(version, path, kept_src,true,true);                    
        Ok(BuildT::new(build))
    }
}
