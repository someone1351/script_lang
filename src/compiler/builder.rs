// pub mod builder;

// pub mod error;
// mod node;


use std::fmt::Debug;
use std::marker::PhantomData;



// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::fmt::Write;
// use std::path::Path;


// use error::AstError;

// use super::super::common::{*,instruction::JmpCond};
use super::super::common::*;
// use super::parser::*;
use super::super::ast::*;
// use error::*;
// use cmd_scope::*;
// use node::*;


// pub type Cmd = for<'a> fn(SExprContainer<'a>, &mut Builder<'a>) -> Result<(),BuilderError>;




//todo: allow return of both builderrs and asterrs, so that mistakes in cmds don't panic/crash the program
/*
todo
* instead of setting param loc, set optional loc for results, which the param loc is then gotten by
* * for params, if eval a code block, need to reset loc to the start of that block? no? eg b.loc(x).eval(y).push_param() the loc x will be used by push_param?
* is start/end loc really needed for params? would be simpler just to have the start loc

* make builder methods request file/line so on error can see where it came from
* should setfields be required to return the object they modified? which then can be used for the next set_field in the chain
** does that make sense? shouldn't it return the value it was set to?

* instead of try method, could just return undefined on missing methods?
*/


// #[derive(Debug,Clone)]
// pub enum BuilderError2<E:Clone+Debug> {
//     Builder(E),
//     Ast(AstError),
// }


#[derive(Debug,Clone)]
pub struct BuilderError<E:Clone+Debug> {
    // pub path:Option<PathBuf>,
    pub loc : Loc,
    pub error_type : E,
}

impl<E:Clone+Debug> BuilderError<E> {
    pub fn new(loc : Loc, error_type : E) -> Self {
        Self{loc,error_type}
    }
}



// use super::super::parser::*;

// #[derive(Debug,Clone)]
pub enum BuilderNodeType<'a,T:Clone+Debug+'a,E:Clone+Debug> {
    EvalPrimitive(
        // PrimitiveContainer<'a>
        T
    ),

    Ast(Box<dyn Fn(&mut Ast<'a>)->Result<(),BuilderError<E>>+'a>),

}


// #[derive(Debug)]
pub struct BuilderNode<'a,T:Clone+Debug+'a,E:Clone+Debug> {
    pub node_type:BuilderNodeType<'a,T,E>,
    pub loc : Option<Loc>,
}


pub struct Builder<'a,T:Clone+Debug+'a,E:Clone+Debug> {
    phantom_data:PhantomData<E>,
    temp_stk : Vec<BuilderNode<'a,T,E>>,
    cur_loc:Option<Loc>,
    // next_cmd_anon_id : usize,
    // in_cmd:bool,

    cur_anon_id:usize,

    // ast : BuilderAst<'a>,


    // nodes : Vec<BuilderNode<'a,T,E>>,
    nodes : Vec<(Box<dyn Fn(&mut Ast<'a>)->Result<(),BuilderError<E>>+'a>,Option<Loc>)>,
    // global_decls : HashSet<&'a str>,
    // depth : usize,

    temp_last_loc:Option<Loc>,
    temp_stk_last_len : usize,
}

// pub enum BuilderField<'a,T> {
//     String(&'a str),
//     // Symbol(&'a str),
//     // Int(i64),
//     Eval(T),
// }

impl<'a,T:Clone+Debug+'a,E:Clone+Debug+'a> Builder<'a,T,E> {
    pub fn get_fields<F>(&mut self, fields : F) -> &mut Self
    where
        // F : IntoIterator<Item = (BuilderField<'a,T>,Loc)>,
        F : IntoIterator<Item = (T,bool,Loc)>,
    {

        //fields
        for field in fields.into_iter() {
            self.param_push(); //last result

            // match field.0 {
            //     BuilderField::Eval(x) => { self.eval(x); }
            //     BuilderField::String(x) => { self.result_string(x); }
            // }

            self.eval(field.0.clone()); //what is this for??

            self
                .loc(field.2)
                .param_push()
                .swap()
                // .call_method("get_field", 2)
                .get_field(field.1)
                ;
        }

        self
    }

    pub fn set_fields_begin<F>(&mut self, fields : F) -> &mut Self
    where
        F : IntoIterator<Item = (T,bool,Loc)>,
    {

        self.block_start(None); //needed for try_call_method

        let fields=fields.into_iter().collect::<Vec<_>>();

        //fields
        for field_ind in 0 .. fields.len() {
            //push last result
            if fields.len()>1 {
                self.param_push();
            }

            //push last result
            self.param_push();

            //push last result
            if field_ind!=0 && field_ind!=fields.len()-1 { //not first or last field
                self.param_push();
            }

            // //on last field : push to
            // if field_ind==fields.len()-1 {
            //     //push to_val
            //     self
            //         .eval(to_val.0.clone())
            //         .param_push()
            //         .swap()
            //         ;
            // }

            //result toval => toval result
            //toval result field => toval field result
            //----
            //result
            //result field => field result

            //
            let field=fields.get(field_ind).unwrap();

            self.eval(field.0.clone());
            self.loc(field.2);


            //push field, swap
            self
                .param_push()
                .swap();

            //on not last field
            if field_ind!=fields.len()-1 {
                //push field, swap
                self
                    .param_push()
                    .swap();

                //get_field
                // self.call_method("get_field", 2);
                self.get_field(field.1);
            }
        }

        //
        self
    }
    pub fn set_fields_end<F>(&mut self,
        //fields_len:usize
        fields : F) -> &mut Self
    where
        F : DoubleEndedIterator<Item = (T,bool,Loc)>,
    {
        let mut fields=fields.into_iter().rev();
        let last_field=fields.next().unwrap();
        // for f in fields {

        // }
        //push to_val
        self //=> field result
            .param_push() //=> field result to_val
            .rot_right() //=> result to_val field
            .rot_right(); //=> to_val field result

        //
        // self.call_method("set_field", 3);
        self.set_field(last_field.1,true);

        //sometimes is unecessary to call, for things like arrays and dicts, since they hold "pointer" like values,
        //  and not copies, but for get_field's that return a copy and not a "pointer", then
        //  it must be modified and then copied back to its original owner

        //if a set_field method doesn't exist, want to abandon the set_field chain?
        //could try to call a special set_field_end (and also a get_field_end),
        //  might be useful for special fields like thing.0.color vs thing.0.color.on_press
        //  but problem is: set thing.0.color.r 0.5 vs: set thing.0.color.on_press .r 0.5
        // println!("fields num is {fields_num}");


        // for _ in 0 .. fields_len-1
        for field in fields
        {
            self
                .rot_right()
                .rot_right()
                .swap()

                //
                // // .try_call_method("set_field", 3) //allowed to fail if no set_field method
                // // .block_start(None)
                // //     .to_block_end(JmpCond::NotUndefined, 0)
                // //     .pop_params() //will pop all params though ...
                // //     .to_block_end(JmpCond::None, 1) //todo:need to make try_call_method return undefined on fail
                // // .block_end()
                // // //todo: also need a way to store result of prev set_field, and re set it as result on try_call_method fail
                // // //  why? don't use the result of a set_field anyway?
                // // //     because it can eg var r {set a.x 5}, could optionally return void or something else
                // // //       in that case just return undefined when method missing?
                // // //todo: on try method fail, pop off unused params? don't need to, ast handles that?

                // .call_method("set_field", 3)
                .set_field(field.1,false)
                ;
        }

        self.block_end(); //needed for try_call_method

        self
    }
    pub fn temp_mark(&mut self) {
        self.temp_last_loc=self.cur_loc;
        self.temp_stk_last_len = self.temp_stk.len();
    }
    pub fn temp_clear(&mut self) {
        self.cur_loc=self.temp_last_loc;
        self.temp_stk.truncate(self.temp_stk_last_len);
    }
    pub fn new() -> Self {
        Self {
            phantom_data:Default::default(),
            temp_stk : Default::default(),
            // ast : BuilderAst::new(),
            nodes : Default::default(),
            // global_decls : Default::default(),
            cur_loc:None,
            // next_cmd_anon_id:1,
            // in_cmd : false,
            cur_anon_id:0,

            temp_last_loc:None,
            temp_stk_last_len : 0,
        }
    }
    pub fn anon_scope(&mut self,anon_scope:usize) {
        self.cur_anon_id=anon_scope;
    }

    pub fn loc(&mut self, loc:Loc) -> &mut Self {
        self.cur_loc=Some(loc);
        self
    }

    pub fn no_loc(&mut self) -> &mut Self {
        self.cur_loc=None;
        self
    }
    // pub fn param_loc(&mut self, _start_loc:Loc, _end_loc:Loc) -> &mut Self {
    //     //todo
    //     self
    // }
    // pub fn param_loc_swap(&mut self) -> &mut Self {
    //     //todo
    //     self
    // }
    // pub fn param_loc_rot(&mut self) -> &mut Self {
    //     //todo
    //     self
    // }

    // pub fn commit_param_locs(&mut self) {
    //     //todo
    // }


    // pub fn result_value(&mut self,v:&Value) -> &mut Self {
    //     self.temp_stk.push(BuilderInputNode::ResultValue(v.clone()));
    //     self
    // }

    fn add_node<F>(&mut self,func:F) -> &mut Self
    where
        F:Fn(&mut Ast<'a>)->Result<(),BuilderError<E>>+'a,
    {
        // self.temp_stk.push(BuilderNode{node_type,loc:self.cur_loc});
        self.temp_stk.push(BuilderNode{
            node_type:BuilderNodeType::Ast(Box::new(func)),
            loc:self.cur_loc,
        });


        self
    }

    pub fn result_void(&mut self) -> &mut Self {
        // self.add_node(BuilderNodeType::ResultVoid)

        self.add_node(|ast|{
            ast.result_void();
            Ok(())
        })
    }

    pub fn result_nil(&mut self) -> &mut Self {
        // self.add_node(BuilderNodeType::ResultNil)

        self.add_node(|ast|{
            ast.result_nil();
            Ok(())
        })
    }

    pub fn result_bool(&mut self,x:bool) -> &mut Self {
        // self.add_node(BuilderNodeType::ResultBool(x))

        self.add_node(move|ast|{
            ast.result_bool(x);
            Ok(())
        })
    }

    pub fn result_int<Q:Into<IntVal>>(&mut self,x:Q) -> &mut Self {
        // self.add_node(BuilderNodeType::ResultInt(x))
        let x=x.into();

        self.add_node(move|ast|{
            ast.result_int(x);
            Ok(())
        })
    }

    pub fn result_float<Q:Into<FloatVal>>(&mut self,x:Q) -> &mut Self {
        // self.add_node(BuilderNodeType::ResultFloat(x))
        let x=x.into();

        self.add_node(move|ast|{
            ast.result_float(x);
            Ok(())
        })
    }

    pub fn result_string(&mut self,x:&'a str) -> &mut Self {
        // self.add_node(BuilderNodeType::ResultString(x))
        self.add_node(|ast|{
            ast.result_string(x);
            Ok(())
        })
    }

    pub fn param_push(&mut self) -> &mut Self {
        // self.add_node(BuilderNodeType::StackParamPush)
        self.add_node(|ast|{
            ast.stack_param_push();
            Ok(())
        })
    }

    pub fn swap(&mut self) -> &mut Self {
        // self.param_loc_swap();
        // self.add_node(BuilderNodeType::StackSwap)

        self.add_node(|ast|{
            ast.stack_swap();
            Ok(())
        })
    }

    pub fn rot_right(&mut self) -> &mut Self {
        // self.param_loc_rot();
        // self.add_node(BuilderNodeType::StackRot)

        self.add_node(|ast|{
            ast.stack_rot_right();
            Ok(())
        })
    }

    pub fn rot_left(&mut self) -> &mut Self {
        self.add_node(|ast|{
            ast.stack_rot_left();
            Ok(())
        })
    }
    pub fn dup(&mut self) -> &mut Self {
        // self.param_loc_dup();
        // self.add_node(BuilderNodeType::StackDup)

        self.add_node(|ast|{
            ast.stack_dup();
            Ok(())
        })
    }

    // not necessary? (can renable if needed)
    pub fn pop(&mut self) -> &mut Self {
        // self.param_loc_rot();
        // self.add_node(BuilderNodeType::StackPop)

        self.add_node(|ast|{
            ast.stack_pop(1).unwrap();
            Ok(())
        })
    }

    //
    // pub fn decl_global_var(&mut self,n:&'a str, init:bool) -> &mut Self {
    //     self.add_node(BuilderNodeType::DeclGlobalVar(n,init))
    // }

    pub fn decl_var_start(&mut self,name:&'a str,init_nil:bool) -> &mut Self {
        // self.add_node(BuilderNodeType::DeclVarStart{name:n,init_nil,anon_id:None,})

        self.add_node(move|ast|{
            ast.decl_var_start(name,init_nil,None);
            Ok(())
        })
    }

    pub fn decl_var_end(&mut self) -> &mut Self {
        // self.add_node(BuilderNodeType::DeclVarEnd)

        self.add_node(|ast|{
            ast.decl_var_end().unwrap();
            Ok(())
        })
    }

    pub fn set_var(&mut self,name:&'a str) -> &mut Self {
        // self.add_node(BuilderNodeType::SetVar{name:n,anon_id:None,})

        self.add_node(|ast|{
            ast.set_var(name,None).unwrap();
            Ok(())
        })
    }

    pub fn get_var(&mut self,name:&'a str) -> &mut Self {
        // self.add_node(BuilderNodeType::GetVar{name:n,anon_id:None,})

        self.add_node(|ast|{
            ast.get_var(name,None).unwrap();
            Ok(())
        })
    }

    pub fn decl_anon_var(&mut self,name:&'a str,init_nil:bool) -> &mut Self {
        // let anon_id=if self.in_cmd{Some(self.next_cmd_anon_id)}else{Some(0)}; //why 0 if outside cmd? 0 for root?
        let anon_id=Some(self.cur_anon_id);
        // self.add_node(BuilderNodeType::DeclVarStart{name:n,init_nil,anon_id,});
        // self.add_node(BuilderNodeType::DeclVarEnd)

        self.add_node(move|ast|{
            ast.decl_var_start(name,init_nil,anon_id);
            ast.decl_var_end().unwrap();
            Ok(())
        })
    }

    pub fn set_anon_var(&mut self,name:&'a str) -> &mut Self {
        // let anon_id=if self.in_cmd{Some(self.next_cmd_anon_id)}else{Some(0)};
        let anon_id=Some(self.cur_anon_id);
        // self.add_node(BuilderNodeType::SetVar{name:n,anon_id,})

        self.add_node(move|ast|{
            ast.set_var(name,anon_id).unwrap();
            Ok(())
        })
    }

    pub fn get_anon_var(&mut self,name:&'a str) -> &mut Self {
        // let anon_id=if self.in_cmd{Some(self.next_cmd_anon_id)}else{Some(0)};
        let anon_id=Some(self.cur_anon_id);
        // self.add_node(BuilderNodeType::GetVar{name:n,anon_id,})

        self.add_node(move|ast|{
            ast.get_var(name,anon_id).unwrap();
            Ok(())
        })
    }
    pub fn call_anon(&mut self,name:&'a str,params_num:usize) -> &mut Self {
        // let anon_id=if self.in_cmd{Some(self.next_cmd_anon_id)}else{Some(0)};
        let anon_id=Some(self.cur_anon_id);
        // self.commit_param_locs();
        // self.add_node(BuilderNodeType::Call{name:n,params_num,anon_id})


        self.add_node(move|ast|{
            ast.get_var(name, anon_id).unwrap();
            ast.call_result(params_num).unwrap();
            // ast.call_var_or_method(name, params_num, anon_id).unwrap();
            Ok(())
        })
    }

    pub fn set_field(&mut self,is_field_symbol:bool,is_last: bool) -> &mut Self {
        self.add_node(move|ast|{
            // ast.call_method("set_field", 3).unwrap();
            ast.set_field(is_field_symbol,is_last).unwrap();
            Ok(())
        })
    }

    pub fn get_field(&mut self,is_field_symbol:bool) -> &mut Self {
        self.add_node(move|ast|{
            // ast.call_method("get_field", 2).unwrap();
            ast.get_field(is_field_symbol).unwrap();
            Ok(())
        })
    }

    pub fn call_method(&mut self,name:&'a str,params_num:usize) -> &mut Self {
        // self.commit_param_locs();
        // self.add_node(BuilderNodeType::CallMethod(n,params_num))

        self.add_node(move|ast|{
            ast.call_method(name, params_num).unwrap();
            Ok(())
        })
    }

    pub fn try_call_method(&mut self,name:&'a str,params_num:usize) -> &mut Self {
        // self.commit_param_locs();
        // self.add_node(BuilderNodeType::TryCallMethod(n,params_num))


        self.add_node(move|ast|{
            ast.try_call_method(name, params_num).unwrap();
            Ok(())
        })
    }

    pub fn call(&mut self,name:&'a str,params_num:usize) -> &mut Self {
        // self.commit_param_locs();
        // self.add_node(BuilderNodeType::Call{name:n,params_num,anon_id:None})

        self.add_node(move|ast|{
            ast.call_var_or_method(name, params_num, None).unwrap();
            Ok(())
        })
    }

    pub fn call_result(&mut self,params_num:usize) -> &mut Self {
        // self.commit_param_locs();
        // self.add_node(BuilderNodeType::CallResult(params_num))

        self.add_node(move|ast|{
            ast.call_result(params_num).unwrap();
            Ok(())
        })
    }

    pub fn get_var_or_call_method(&mut self,name:&'a str) -> &mut Self {
        // self.commit_param_locs();
        // self.add_node(BuilderNodeType::CallMethodOrGetVar{name},)

        self.add_node(|ast|{
            ast.get_var_or_call_method(name).unwrap();
            Ok(())
        })
    }




    pub fn to_block_start(&mut self,cond:JmpCond,block_offset:usize) -> &mut Self {
        // self.add_node(BuilderNodeType::BlockToStart(cond,block_offset))

        self.add_node(move|ast|{
            ast.to_block_start(cond, block_offset).unwrap();
            Ok(())
        })
    }

    pub fn to_block_end(&mut self,cond:JmpCond,block_offset:usize) -> &mut Self {
        // self.add_node(BuilderNodeType::BlockToEnd(cond,block_offset))

        self.add_node(move|ast|{
            ast.to_block_end(cond, block_offset).unwrap();
            Ok(())
        })
    }

    pub fn to_block_start_label(&mut self,cond:JmpCond,label:&'a str, err : Option<BuilderError<E>>) -> &mut Self {
        // self.add_node(BuilderNodeType::BlockToStartLabel(cond,label,err))

        self.add_node(move|ast|{
            match ast.to_label_block_start(cond, label) {
                Ok(x) => {
                    if !x {
                        if let Some(err)=err.clone() {
                            return Err(err);
                        }
                    }
                    Ok(())
                }
                Err(e) => {Err(e)}
            }.unwrap();

            Ok(())
        })
    }

    pub fn to_block_end_label(&mut self,cond:JmpCond,label:&'a str, err : Option<BuilderError<E>>) -> &mut Self {
        // self.add_node(BuilderNodeType::BlockToEndLabel(cond,label,err))


        self.add_node(move|ast|{
            match ast.to_label_block_end(cond, label) {
                Ok(x) => {
                    if !x {
                        if let Some(err)=err.clone() {
                            return Err(err);
                        }
                    }

                    Ok(())
                }
                Err(e) => {
                    Err(e)
                }
            }.unwrap();

            Ok(())
        })
    }

    // pub fn block_start_label(&mut self,label:&'a str) -> &mut Self {
    //     self.add_node(BuilderNodeType::BlockStart(Some(label)))
    // }

    pub fn block_start(&mut self,label:Option<&'a str>) -> &mut Self {
        // self.add_node(BuilderNodeType::BlockStart(label))

        self.add_node(move|ast|{
            ast.block_start(label);
            Ok(())
        })
    }

    pub fn block_end(&mut self) -> &mut Self {
        // self.add_node(BuilderNodeType::BlockEnd)

        self.add_node(|ast|{
            ast.block_end().unwrap();
            Ok(())
        })
    }

    pub fn pop_params(&mut self) -> &mut Self {
        self.add_node(|ast|{
            ast.pop_params();
            Ok(())
        })
    }
    pub fn func_start(&mut self,params:Vec<&'a str>,variadic:bool) -> &mut Self {
        // self.add_node(BuilderNodeType::FuncStart(params,variadic))
        self.add_node(move|ast|{
            ast.func_start(params.clone(), variadic).unwrap();
            Ok(())
        })

    }

    pub fn func_end(&mut self) -> &mut Self {
        // self.add_node(BuilderNodeType::FuncEnd)

        self.add_node(|ast|{
            ast.func_end().unwrap();
            Ok(())
        })
    }

    pub fn include(&mut self,n:&'a str, _loc:Loc) -> &mut Self {
        // self.add_node(BuilderNodeType::Include(n))
        self.add_node(|ast|{
            ast.include(n);
            Ok(())
        })
    }

    pub fn eval(&mut self, sexpr : T) -> &mut Self {
        self.temp_stk.push(BuilderNode{node_type:BuilderNodeType::EvalPrimitive(sexpr),loc:self.cur_loc});
        self
    }

    // pub fn eval_sexprs<I>(&mut self, sexprs : I) -> &mut Self
    // where
    //     I: IntoIterator<Item=PrimitiveContainer<'a>>
    // {
    //     for sexpr in sexprs.into_iter() {
    //         self.add_node(BuilderNodeType::EvalSexpr(sexpr));
    //     }

    //     self
    // }

    // fn param_eval_push_sexprs<I>(&mut self, sexprs : I) -> &mut Self
    // where
    //     I: IntoIterator<Item=PrimitiveContainer<'a>>
    // {
    //     for sexpr in sexprs.into_iter() {
    //         self.add_node(BuilderNodeType::EvalPrimitive(sexpr));
    //         self.add_node(BuilderNodeType::StackParamPush);
    //     }

    //     self
    // }




    // ===

    pub fn generate_ast(&mut self,
        // cmd_scope : &CmdScope,
        ast :&mut Ast<'a>,
        mut callback:impl FnMut (&mut Builder<'a,T,E>,T) -> Result<(),BuilderError<E>>+'a,

    ) -> Result<(),BuilderError<E>> {
        // self.generate_nodes(cmd_scope)?;

        let mut working_stk = self.temp_stk.drain(0..).rev().collect::<Vec<_>>();

        while let Some(node)=working_stk.pop() {
            match node.node_type {
                BuilderNodeType::EvalPrimitive(primitive)=> {
                    callback(self,primitive)?;
                }
				BuilderNodeType::Ast(x)=>{
                    self.nodes.push((x,node.loc));
                }
            }

            working_stk.extend(self.temp_stk.drain(0 ..).rev());
        }
        // for (node_ind,node) in self.nodes.iter().enumerate() {
        //     println!("{node_ind} {node:?}");
        // }

        // for &global_decl in self.global_decls.iter() {
        //     ast.add_capturable_global(&global_decl);
        // }

        let mut nodes=self.nodes.drain(0 ..).rev().collect::<Vec<_>>();

        // for node in nodes
        while let Some((node,loc))=nodes.pop()
        {
            ast.set_cur_loc(loc);

            // let x=match &node.node_type {
            //     BuilderNodeType::EvalPrimitive(_) =>{panic!("")}
            //     BuilderNodeType::Ast(_) =>{}

            //     // _=>{
            //     //     panic!("");
            //     // }
            // };
            // let x=node(ast);
            node(ast)?;
            // if let Err(e) = x {
            //     ast.print();
            //     // for node in nodes.iter().rev() {
            //     //     println!("{node:?}");
            //     // }
            //     panic!("{:?} : {e:?}",node.node_type);
            // }
        }

        // ast.commit();

        //

        // ast.print();




        // Ok(ast.compile(version,path))
        Ok(())
    }

}
