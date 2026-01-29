
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

====
* have var return/eval use $ ?
* so can tell dif between calling a method and returning a val
* so can tell if a global (and not a method) without needing to declare them
* set v 5; shouldn't use the $ prefix ??

var a {func {} 1}
call $a 5
fn myfunc {a b c} { + $a $b $c }
call $myfunc $a $b $c
call {call $mygetfunc} $a $b $c
fn abc {} { $g }
var a $b;
fn abc {g} { call $g }
call $myfunc $a $b $c
set a 5

abc.$i
abc.{$i}

*/
/*
TODO
* make anything in parenthesis an expression? eg (1+2 * 5 -9) => (1 +2 * 5 -9) => {- {+ 1 {* 2 555}} 9}

* pass cmds an iter of primitives, and let them take as much as they like,
** possibly even partial blocks? somehow
** so can do things like have val decl span multiple blocks (lines)

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
    ExpectSymbol(u32),
    NoSymbolPrefixAllowed,
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
    NoSemiColonsAllowed, //only used within func param decl
    NoBlocksAllowed,
    NoFieldsAllowed,
    InvalidStringSymbol,
    InvalidSymbol,
    // NoCmdFound,
    // NoArgsAllowed,
    CannotCallGetVar,
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




// pub type Cmd = for<'a> fn(RecordContainer<'a>, &mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>>;
pub type Cmd = Box<dyn for<'a> Fn(RecordContainer<'a>, &mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>>>;
// pub type Cmd = Box<dyn Fn(RecordContainer, &mut Builder<PrimitiveContainer,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>>>;


pub struct Compiler {
    cmds : HashMap<String,Vec<Cmd>>,
    get_var_prefix : Option<&'static str>, //prefix for getting vars
    optional_get_var_prefix : bool, //if using get var prefix, make it optional when using vars as a params eg func $a $b => func a b
    funcs_without_call:bool, //allow calling funcs the same way methods are called eg mymethod 123 and myfunc 123 instead of: call myfunc 123
}

impl Compiler {
    pub fn new_empty() -> Self {
        Self{
            cmds:Default::default(),
            // get_var_prefix : Some("$"),
            get_var_prefix : None,
            optional_get_var_prefix:true, //not used when no prefix
            funcs_without_call:false, //true, //
        }
    }
    pub fn new() -> Self { //denote_get_var:bool,
        let mut cmd_scope = Self::new_empty(); //denote_get_var
        let get_var_prefix=cmd_scope.get_var_prefix;

        cmd_scope.add_cmd("while", while_cmd);
        cmd_scope.add_cmd("for", for_cmd);
        cmd_scope.add_cmd("continue", continue_cmd);
        cmd_scope.add_cmd("break", break_cmd);
        cmd_scope.add_cmd("return", return_cmd);
        cmd_scope.add_cmd("var", var_cmd);
        cmd_scope.add_cmd("set", move|r,b|set_var_cmd(r,b,get_var_prefix));
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
        cmd_scope.add_cmd("fn", move|r,b|func_cmd(r,b,get_var_prefix));
        cmd_scope.add_cmd("fn", lambda_cmd);
        cmd_scope.add_cmd("call", call_func_cmd);
        cmd_scope.add_cmd("?", ternary_cmd);

        cmd_scope.add_cmd("dict", dict_cmd);

        cmd_scope
    }

    pub fn add_cmd<F>(&mut self,k:&str,cmd : F)
    where
        // F : Fn(RecordContainer, &mut Builder<PrimitiveContainer,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> + 'static,
        F : for<'a> Fn(RecordContainer<'a>, &mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>) -> Result<(),BuilderError<BuilderErrorType>> + 'static,
    {
        self.cmds.entry(k.to_string()).or_insert_with(Default::default).push(Box::new(cmd));
    }

    fn get(&self,k:&str) -> Option<std::slice::Iter<'_, Cmd>> {
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
        let hasnt_fields=top_primitive.as_param().map(|x|x.fields_num()==0).unwrap_or(true);

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

                        if let Some(symbol)=first_param.as_primitive().as_symbol() {
                            if ["true","false","nil","void"].contains(&symbol) || symbol.starts_with(":") {
                                if record.params_num()==1 {
                                    builder.eval(first_param.as_primitive());
                                } else {
                                    return Err(BuilderError { loc: record.start_loc(), error_type: BuilderErrorType::NoParamsAllowed });
                                }
                            } else if [","].contains(&symbol) {
                                return Err(BuilderError { loc: record.start_loc(), error_type: BuilderErrorType::InvalidSymbol });
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
                                    if let Some(get_var_prefix)=self.get_var_prefix {
                                        if let Some(symbol)=symbol.strip_prefix(get_var_prefix) {
                                            builder.get_var(symbol);
                                        } else if self.funcs_without_call {
                                            builder.call(symbol, 0);
                                        } else {
                                            builder.call_method(symbol,0);
                                        }
                                    } else {
                                        builder.get_var_or_call_method(symbol);
                                    }
                                } else if self.get_var_prefix.is_some() && symbol.starts_with(self.get_var_prefix.unwrap()) { //has args, no fields, using var prefix, has prefix
                                    return Err(BuilderError { loc: record.start_loc(), error_type: BuilderErrorType::NoParamsAllowed });
                                } else { //has args, no fields
                                    for i in (1 .. record.params_num()).rev() {
                                        let x=record.param(i).unwrap();
                                        // builder.param_loc(x.start_loc(),x.end_loc());
                                        builder.eval(x.as_primitive());
                                        builder.param_push();
                                    }

                                    // builder.commit_param_locs();
                                    builder.loc(first_param.start_loc());

                                    //
                                    if self.funcs_without_call {
                                        builder.call(symbol, record.params_num()-1);
                                    } else {
                                        builder.call_method(symbol, record.params_num()-1);
                                    }
                                }
                            } else if self.funcs_without_call && (self.get_var_prefix.is_none() || !symbol.starts_with(self.get_var_prefix.unwrap()))
                            { //no prefix(ie for variables, should just remove that, as not using it), has fields
                                for i in (1 .. record.params_num()).rev() {
                                    let x=record.param(i).unwrap();
                                    // builder.param_loc(x.start_loc(),x.end_loc());
                                    builder.eval(x.as_primitive());
                                    builder.param_push();
                                }

                                // builder.commit_param_locs();
                                builder.loc(first_param.start_loc());

                                builder.eval(first_param.as_primitive());
                                builder.call_result(record.params_num()-1);
                            } else if record.params_num()==1 { //has fields, no args
                                if self.get_var_prefix.is_none() || symbol.starts_with(self.get_var_prefix.unwrap()) || self.optional_get_var_prefix {
                                    builder.eval(first_param.as_primitive());
                                } else {
                                    return Err(BuilderError { loc: record.start_loc(), error_type: BuilderErrorType::CannotCallGetVar });
                                }
                                // println!("dffsd {:?}",first_param.primitive());
                            } else { //args and fields
                                let mut field_iter=first_param.fields();
                                let last_field=field_iter.next_back().unwrap();

                                // println!("hmm {:?}",field_iter.collect::<Vec<_>>());

                                let Some(last_field_symbol)=last_field.as_primitive().as_symbol() else {
                                    return Err(BuilderError { loc: record.start_loc(), error_type: BuilderErrorType::NoParamsAllowed });
                                };

                                //
                                for i in (1 .. record.params_num()).rev() {
                                    let x=record.param(i).unwrap();
                                    // builder.param_loc(x.start_loc(),x.end_loc());
                                    builder.eval(x.as_primitive());
                                    builder.param_push();
                                }

                                builder.loc(first_param.start_loc());

                                //

                                // let mid_fields=first_param.fields_range(0..first_param.fields_num()-1);

                                // builder.result_float(1.23);
                                // builder.get_fields(field_iter.map(|field|(field.primitive(),field.primitive().symbol().is_some(),field.start_loc())));
                                builder.get_var(first_param.as_primitive().as_symbol().unwrap());

                                self.get_fields(builder, field_iter)?;
                                // builder.result_float(2.23);
                                builder.param_push();
                                builder.call_method(last_field_symbol, record.params_num());
                            }
                        } //end if symbol
                        else if record.params_num()==1 { //no args, first not symbol, could have fields
                            builder.eval(first_param.as_primitive()); //not symbol, no args
                        // } else if {
                        } else { //has args, first not symbol, could have fields
                            // println!("hasnt_fields {hasnt_fields}, primitive_ind={}, r={}, fnum={}",top_primitive.primitive_ind,record.record_ind, first_param.fields_num());
                            // return Err(BuilderError { loc: record.start_loc(), error_type: BuilderErrorType::ExpectSymbol(3) });
                            // // first_param.primitive().
                            // // builder.eval_without_fields(first_param.primitive());

                            // builder.eval(first_param.to_fieldless().as_primitive());



                            //

                            let mut field_iter=first_param.fields();
                            let last_field=field_iter.next_back().unwrap();

                            //

                            let Some(last_field_symbol)=last_field.as_primitive().as_symbol() else {
                                return Err(BuilderError { loc: record.start_loc(), error_type: BuilderErrorType::NoParamsAllowed });
                            };

                            //
                            for i in (1 .. record.params_num()).rev() {
                                let x=record.param(i).unwrap();
                                // builder.param_loc(x.start_loc(),x.end_loc());
                                builder.eval(x.as_primitive());
                                builder.param_push();
                            }

                            builder.loc(first_param.start_loc());
                            // builder.get_var(first_param.as_primitive().as_symbol().unwrap());
                            builder.eval(first_param.to_fieldless().as_primitive());

                            self.get_fields(builder, field_iter)?;
                            builder.param_push();

                            builder.call_method(last_field_symbol, record.params_num());
                        }
                    } //end first param
                } //end for record

                if !hasnt_fields {
                    self.get_fields(builder,top_primitive.as_param().unwrap().fields())?;
                }
            } //end block match
            PrimitiveTypeContainer::Float(f,_) => {
                builder.result_float(f as FloatT);
            }
            PrimitiveTypeContainer::Int(i,_) => {
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
                    // "," => {
                    //     return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::InvalidSymbol });
                    // }
                    _=>{
                        if let Some(symbol)=symbol.strip_prefix(":") {
                            if hasnt_fields {
                                if symbol.is_empty() {
                                    return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::InvalidStringSymbol });
                                }

                                builder.result_string(symbol);
                            } else {
                                return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::NoFieldsAllowed });
                            }
                        } else {
                            let symbol=if let Some(get_var_prefix)=self.get_var_prefix {
                                if let Some(symbol)=symbol.strip_prefix(get_var_prefix) {
                                    if symbol.is_empty() {
                                        return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::InvalidSymbol });
                                    }

                                    symbol
                                } else if self.optional_get_var_prefix { //var prefix optional for everything except returns
                                    symbol
                                } else {
                                    return Err(BuilderError { loc: top_primitive.start_loc(), error_type: BuilderErrorType::InvalidSymbol });
                                }
                            } else {
                                symbol
                            };

                            builder.get_var(symbol);

                            if !hasnt_fields {
                                self.get_fields(builder,top_primitive.as_param().unwrap().fields())?;
                            }
                        }
                    }
                }
            }

        }

        Ok(())
    }

    // fn get_fields<'a>(&self,
    //     builder:&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>,
    //     top_primitive:PrimitiveContainer<'a>,
    // ) -> Result<(),BuilderError<BuilderErrorType>> {
    //     self.get_fields_ext(builder,top_primitive)
    // }
    fn get_fields<'a>(&self,
        builder:&mut Builder<'a,PrimitiveContainer<'a>,BuilderErrorType>,
        // top_primitive:PrimitiveContainer<'a>,
        fields : FieldIter<'a>,
    ) -> Result<(),BuilderError<BuilderErrorType>> {

        //requires object fields coming from to be on result register

        // let fields=top_primitive.param().unwrap().fields();

        builder.get_fields(fields.map(|field|{
            let s=field.as_primitive().as_symbol();
            let (f,is_field_symbol)=if s.is_none()||self.get_var_prefix.map(|prefix|s.unwrap().starts_with(prefix)).unwrap_or_default(){
                //if it's not a symbol or is a symbol with a var prefix
                //  these primitives will be evaluated, so a variable will be gotten, an expr evaluated etc?
                (field.as_primitive(),false)
            } else {
                //is a symbol, symbols can be "+", "-", but also "abc"
                //  use string primitive, so that the symbol abc will be converted to the string "abc" ?
                (field.as_string_primitive(),true)
            };

            (f,is_field_symbol,field.start_loc(),)
        }));

        // // let mut last_start_loc=top_primitive.start_loc();
        // // let mut last_end_loc=top_primitive.end_loc();

        // //fields
        // for field in top_primitive.param().unwrap().fields() {
        //     builder
        //         // .param_loc(last_start_loc,last_end_loc)
        //         .param_push(); //last result

        //     // let field=symbol.field(field_ind).unwrap();

        //     if let Some(symbol)=field.primitive().symbol() {
        //         if let Some(get_var_prefix)=self.get_var_prefix {
        //             if symbol.starts_with(get_var_prefix) {
        //                 builder.eval(field.primitive());
        //             } else { //is string
        //                 builder.result_string(symbol);
        //             }
        //         } else { //is string
        //             builder.result_string(symbol);
        //         }
        //     } else { //not a symbol
        //         builder.eval(field.primitive());
        //     }

        //     builder
        //         // .param_loc(field.start_loc(),field.end_loc())
        //         .param_push()
        //         .swap()
        //         ;

        //     builder.loc(field.start_loc());

        //     //
        //     builder.call_method("get_field", 2);

        //     // last_start_loc=field.start_loc();
        //     // last_end_loc=field.end_loc();
        // }

        Ok(())
    }

    // pub fn compile2(&self,src : &str, version:usize, path : Option<&Path>, keep_src : bool, ) -> Result<BuildT,CompileError> {
    // }

    pub fn compile(&self,src : &str, version:usize, path : Option<&Path>, keep_src : bool, ) -> Result<BuildT,CompileError> {
        let mut next_anon_id=1;

        let src= StringT::new(src);
        let pathbuf=path.map(|x|x.to_path_buf());

        let parsed=cexpr_parser::parse(src.as_str(),  );

        if let Err(e)=parsed {
            return Err(CompileError{path:pathbuf,src,loc:e.loc,error_type:CexprCompileErrorType::CexprParser(e.error_type)});
        }

        let parsed=parsed.unwrap();

        parsed.print();

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
