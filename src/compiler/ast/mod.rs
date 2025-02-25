
// mod ast;
pub mod error;
mod node;
mod node_type;
mod symbol_manager;
mod include_manager;
mod misc;
// pub use super::common;

// pub use ast::*;
// pub use ast::Ast;

/*
TODO
* add gotos
- can't jump out of functions
- can jump out of blocks, will pop local vars
- can jump anywhere in current block or ancestor blocks
- can't jump into middle of sibling/uncle block?

- label can be in scope if declared before or after the goto position
- - can jump down/up to cur/ancestor blocks
- -
- can jump into block if label is at start of a block?
- - including a block multiple depths deep, as long as each block at start of parent block

- labels can be stored as a variable
- - but can only jump to it if the stored label is in scope from the goto call
- - goto_label goto_var
- - stored label's instr start/end range
- - label and vars can shadow/overwrite each other?

- either add call_method_or_get_global_var instr
- - or make it so any globals being used need to be declared in the script
- - - what about a global captures being used before they have been initialised in the script, but already added in the varscope?
- - - - would be nice to return undefined at runtime,
- - - - - but that is implemented with local vars by just preinitialising them with undefined, which the global won't be if it already has a value
- - - - - - also what even happens if you access a non init global, does it return nil or undefined? think returns undefined?
- - - global decls accessed from inside a func would be a captured var and not directly a global
- - - - assuming the global decl was decl'd at root, non root global decls might be added
- - - - - would capturing work differently on those? say a global decl'd from within a func, then a sub func captures that ...
- - - - so only need to handle globals used outside a func? no need to return loc of the captured global used in src for compile errors
- - - but won't return error on missing var, as it will assume its just a method call
- - might be better to add the instr, so if no var or method is found, can return: NoMethodOrVar "somevar" found
- - - if global access allowed without  decl
- - - - then instr can  be used to get global first, then try method
- - - - can have param bool on instr to mark whether to check for global var
- - - - - what about the option to capture non decld globals?
- - - - - - ...
- - - - - if global is declld, then compiler will know to capture,
- - - - - - if not, it will try for a global access, otherwise try a method
- reason to not replace "var" being returned with "get var", because method/func/cmds params get vars with just "var"
- - could just use "$var", but then things like "set var 5", should that be "set $var 5", or "var a 5" vs "var $a 5"

- need to catch undeclared anon var usage
- - problem is the get/set/call node funcs don't return errors
- - - as previously undeclared variables were assumed to be just globals
- - - also due to how captures work, checking if a var exists can't really be done from those funcs
- - - - but could just make anon variables not capturable, and then from those funcs if the anon variable is decl'd or not
- - also what happens with anon vars decl'd at depth 0? it is made a global
- - - should make it an error to declare one at root?
- - - - and make a panic on a decld one in the generate func
* - - or just allow anon (from the user, but not the cmd/instrs) globals
- - what about consts?

- make local vars declable at root?
- - could add additional funcs, decl_local, decl_global

- make global vars declable at non root?

- have global var checks at compile time where possible?
- - causes problems? as with some configurations, only possible at runtime
- - just set flag on instructions not to look in varscope at runtime

- could have methods be required to have atleast one param to differentiate between them and vars
- - though would be a hassle if the user wants to bind a rust func to just do something


=======
GOTO:
* can jump anywhere, except out of or in to functions
* for each goto, keep vec of reachable locations eg reachables[ind][label]=(pos,stack_pop_amount,stack_push_amount,stack_set_undefined_amount)
- goto_label(reachable_ind,label)
- goto_var(reachable_ind), have var on top of stack, which the goto will pop off
* can store label in a var, but can only jump to labels in the same build
* a var decl will shadow a label of the same name

* if jumping midway into a block, vars above the label will be undefined??
- but jumping in a block that already in ofcourse doesn't need to do that
- - actually if jumping up past decls, then they should be set to undefined
- what about a var captured below? should it not set them to undefined?
- -  or don't set refs (captured stack vars) to undefined? or don't set the ref val inside to undefined?

{
    var a {fn {} b}
    label x
    var b 5
    goto x
}

{
    var a {array}
    label x
    var b 5
    push a {fn {} b} #each one pushed should capture the same b
    goto x
}
{
    var c {fn {} b}
    var a {array}
    label x
    var b 5
    push a {fn {} b} #each one pushed should have a different b captured
    goto x
}

TODO:
* make separate decl_local and decl_global
* have flag to allow use of undeclared variables be capturable globals

TODO
* have closures captured by nearest scope first? eg:
    for scope in cur_scope .. global_scope
        check scope.above else check scope.below

* have option to make undeclared globals accessable and capturable


TODO
* have way to try method calls, and handle if failed
* check if pushed params but not used in call are popped when leaving a block, have a feeling it is set panic.

*/

use std::{collections::{HashMap,  HashSet},  path::Path};

// use super::super::common::{*,instruction::JmpCond};
use super::super::common::*;

use include_manager::*;
use symbol_manager::*;
use error::*;
use node::*;
use node_type::*;
use misc::*;


#[derive(Debug,Clone)]
pub struct Ast<'a> {
    pub nodes:Vec<AstNode<'a>>,
    pub body_node_ind : usize,
    pub last_node_ind : usize,
    pub funcs : Vec<AstFunc<'a>>,
    pub cur_loc : Option<Loc>,
    pub reuse_local_decls:bool,
    pub must_decl_globals:bool,

    pub decls_starts : Vec<(VarName<'a>,AstDeclVar,bool)>,

    // pub anon_var_decls : Vec<HashSet<VarName<'a>>>,
}

impl<'a> Ast<'a> {
    pub fn new(reuse_local_decls:bool,must_decl_globals:bool) -> Self {
        Self {
            nodes:vec![AstNode::new(AstNodeType::Root, None,0,None,0)],
            body_node_ind:0,
            last_node_ind:0,
            funcs:Vec::new(),
            cur_loc: None,
            reuse_local_decls,
            must_decl_globals,
            decls_starts : Vec::new(),
            // anon_var_decls:Vec::new(),
        }
    }
    pub fn set_cur_loc(&mut self, loc : Option<Loc>) {
        self.cur_loc=loc;
    }
    fn body_node_mut(&mut self) -> &mut AstNode<'a> {
        self.nodes.get_mut(self.body_node_ind).unwrap()
    }
    fn body_node(&self) -> &AstNode<'a> {
        self.nodes.get(self.body_node_ind).unwrap()
    }
    fn last_sibling_node(&self) -> Option<&AstNode<'a>> {
        self.body_node().children.last().and_then(|&child_node_ind|Some(self.get_node(child_node_ind)))
    }
    fn last_node_mut(&mut self) -> &mut AstNode<'a> {
        self.nodes.get_mut(self.last_node_ind).unwrap()
    }
    fn last_node(&self) -> &AstNode<'a> {
        self.nodes.get(self.last_node_ind).unwrap()
    }
    // fn get_parent_node(&self,node_ind:usize) -> Option<&AstNode<'a>> { //never used
    //     self.get_node(node_ind).parent.and_then(|parent_node_ind|self.nodes.get(parent_node_ind))
    // }
    fn get_node(&self,node_ind:usize) -> &AstNode<'a> {
        self.nodes.get(node_ind).unwrap()
    }
    fn get_node_mut(&mut self,node_ind:usize) -> &mut AstNode<'a> {
        self.nodes.get_mut(node_ind).unwrap()
    }

    fn add_next(&mut self,node_type:AstNodeType<'a>) {
        let mut stack_pushed_num=0;

        if let AstNodeType::Function{..}=&node_type {
        } else {
            for child_ind in (0 .. self.body_node().children.len()).rev() {
                let &child_node_ind=self.body_node().children.get(child_ind).unwrap();

                if let AstNodeType::Function{..}=self.get_node(child_node_ind).node_type {
                } else {
                    stack_pushed_num=self.get_node(child_node_ind).stack_pushed_num;
                    break;
                }
            }
        }

        //
        let child_ind=self.body_node().children.len();

        let mut node=AstNode::new(node_type, Some(self.body_node_ind), child_ind, self.cur_loc,self.body_node().depth+1);
        node.stack_pushed_num=stack_pushed_num;

        //
        let node_ind=self.nodes.len();
        self.nodes.push(node);

        //
        self.body_node_mut().children.push(node_ind);
        self.last_node_ind=node_ind;
    }

    fn add_body(&mut self,node_type:AstNodeType<'a>,
        // from_last:bool
    ) {
        self.add_next(node_type);
        // self.last_node_mut().stack_pushed_num=0;

        //
        self.body_node_ind=self.last_node_ind;
    }

    fn end_body(&mut self) -> Result<(),AstError> {
        if let Some(&child_node_ind) = self.body_node().children.last() {
            let stack_pushed_num=self.get_node(child_node_ind).stack_pushed_num;

            if stack_pushed_num!=0 {
                return Err(AstError::LocalPushValuesNotZero(stack_pushed_num));
            }
        }

        let parent_ind = self.body_node_mut().parent.unwrap();
        self.last_node_ind=self.body_node_ind;
        self.body_node_ind=parent_ind;

        //
        Ok(())
    }

    /////////////////

    pub fn block_start(&mut self,block_label:Option<&'a str>) {
        self.add_body(AstNodeType::Block{label:block_label,}); //,true
    }

    pub fn block_end(&mut self) -> Result<(),AstError> {
        let AstNodeType::Block{..} = self.body_node().node_type else {
            return Err(AstError::ClosingBlockNotMatching);
        };

        self.end_body()?;

        Ok(())
    }

    pub fn pop_params(&mut self) {
        let params = self.last_node().stack_pushed_num;

        if params > 0 {
            self.add_next(AstNodeType::StackPop(params));
            self.last_node_mut().stack_pushed_num=0;
        }
    }
    pub fn func_start(&mut self,in_params:Vec<&'a str>,variadic:bool) -> Result<(), AstError> {
        if variadic && in_params.len()==0 {
            // panic!("scriptlang, ast, funcstart, params 0 and variadic");
            return Err(AstError::FuncParamsZeroAndVariadic);
        }

        //
        let node_ind=self.nodes.len();

        //vararg isn't counted in params_num, nor does captured_params record a captured vararg

        let func_ind=self.funcs.len();

        self.add_body(AstNodeType::Function {func_ind,}); //, false

        let func_params = in_params.iter().enumerate()
            .filter_map(|(i,&name)|(!variadic || i!=in_params.len()-1).then_some(AstFuncParam{
                name,
                captured:false,
                stack_ind:0,
            }))
            .collect::<_>(); //params minus the variadic

        self.funcs.push(AstFunc {
            params:func_params,
            captures: Vec::new(),
            variadic,
            node_ind,
        });

        if variadic {
            if let Some(&name)=in_params.last() {
                self.decl_var_start(name,false,None);
                self.decl_var_end().unwrap();
                self.add_next(AstNodeType::ResultVararg);
                self.set_var(name,None).unwrap();
            }
        }

        self.result_void();

        Ok(())
    }

    pub fn func_end(&mut self) -> Result<(),AstError> {
        let &AstNodeType::Function{
            // func_ind,
        ..}= &self.body_node().node_type else {
            return Err(AstError::ClosingFunctionNotMatching);
        };

        self.end_body()?;

        Ok(())
    }

    pub fn to_block_start(&mut self, cond:JmpCond, block_offset:usize) -> Result<(),AstError> { //Result<bool,AstError>
        let stack_pushed_num=self.last_sibling_node().and_then(|x|Some(x.stack_pushed_num)).unwrap_or(0);

        if stack_pushed_num!=0 {
            return Err(AstError::LocalPushValuesNotZero(stack_pushed_num));
        }

        //
        let mut cur_node_ind = self.body_node_ind;
        let mut i =0;

        while let AstNodeType::Block{..} = self.nodes.get(cur_node_ind).unwrap().node_type {
            if i==block_offset {
                self.add_next(AstNodeType::ToBlockStart{cond,block_node_ind:cur_node_ind});
                // return Ok(true);
                return Ok(());
            } else {
                cur_node_ind = self.nodes.get(cur_node_ind).unwrap().parent.unwrap();
                i+=1;
            }
        }

        // Ok(false)
        Err(AstError::BlockOffsetNotFound(block_offset))
    }

    pub fn to_block_end(&mut self, cond:JmpCond, block_offset:usize) -> Result<(),AstError> {
        let stack_pushed_num=self.last_sibling_node().and_then(|x|Some(x.stack_pushed_num)).unwrap_or(0);

        if stack_pushed_num!=0 {
            return Err(AstError::LocalPushValuesNotZero(stack_pushed_num));
        }

        //
        let mut cur_node_ind = self.body_node_ind;
        let mut i =0;

        while let AstNodeType::Block{..} = self.nodes.get(cur_node_ind).unwrap().node_type {
            if i==block_offset {
                self.add_next(AstNodeType::ToBlockEnd{cond,block_node_ind:cur_node_ind});
                // return Ok(true);
                return Ok(());
            } else {
                cur_node_ind = self.nodes.get(cur_node_ind).unwrap().parent.unwrap();
                i+=1;
            }
        }

        // Ok(false)
        Err(AstError::BlockOffsetNotFound(block_offset))
    }

    pub fn to_label_block_start(&mut self, cond:JmpCond, block_label:&'a str) -> Result<bool,AstError> {
        let stack_pushed_num=self.last_sibling_node().and_then(|x|Some(x.stack_pushed_num)).unwrap_or(0);

        if stack_pushed_num!=0 {
            return Err(AstError::LocalPushValuesNotZero(stack_pushed_num));
        }

        //
        // let mut cur_node_ind = self.body_node().parent.unwrap();
        let mut cur_node_ind = self.body_node_ind;

        while let AstNodeType::Block{label} = self.nodes.get(cur_node_ind).unwrap().node_type {
            if let Some(label)=label {
                if block_label==label {
                    self.add_next(AstNodeType::ToBlockEnd{cond,block_node_ind:cur_node_ind});
                    return Ok(true);
                }
            }

            cur_node_ind=self.nodes.get(cur_node_ind).unwrap().parent.unwrap();
        }

        Ok(false)
    }

    pub fn to_label_block_end(&mut self, cond:JmpCond, block_label:&'a str) -> Result<bool,AstError> {
        let stack_pushed_num=self.last_sibling_node().and_then(|x|Some(x.stack_pushed_num)).unwrap_or(0);

        if stack_pushed_num!=0 {
            return Err(AstError::LocalPushValuesNotZero(stack_pushed_num));
        }

        //
        let mut cur_node_ind = self.body_node_ind;

        while let AstNodeType::Block{label} = self.nodes.get(cur_node_ind).unwrap().node_type {
            if let Some(label)=label {
                if block_label==label {
                    self.add_next(AstNodeType::ToBlockEnd{cond,block_node_ind:cur_node_ind});
                    return Ok(true);
                }
            }

            cur_node_ind=self.nodes.get(cur_node_ind).unwrap().parent.unwrap();
        }

        Ok(false)
    }

    pub fn stack_swap(&mut self) {
        //todo check stack size? return err?

        //swap two top values on stack
        self.add_next(AstNodeType::StackSwap);
    }
    pub fn stack_rot(&mut self) {
        //todo check stack size? return err?

        self.add_next(AstNodeType::StackRot);
    }
    pub fn stack_dup(&mut self) {
        //todo check stack size? return err?

        self.add_next(AstNodeType::StackDup);
        self.last_node_mut().stack_pushed_num+=1;
    }
    pub fn stack_pop(&mut self,amount:usize) -> Result<(),AstError> {
        if self.last_node().stack_pushed_num == 0 {
            return Err(AstError::StackSizeAlreadyZero);
        }

        self.add_next(AstNodeType::StackPop(amount));
        self.last_node_mut().stack_pushed_num-=amount;

        Ok(())
    }
    pub fn stack_param_push(&mut self) {
        //pushes result onto stack
        self.add_next(AstNodeType::StackPush);
        self.last_node_mut().stack_pushed_num+=1;
    }
    pub fn result_string(&mut self,v: &'a str) {
        self.add_next(AstNodeType::ResultString(v));
    }
    pub fn result_bool(&mut self,v: bool) {
        self.add_next(AstNodeType::ResultBool(v));
    }
    pub fn result_int(&mut self,v: IntT) {
        self.add_next(AstNodeType::ResultInt(v));
    }
    pub fn result_float(&mut self,v: FloatT) {
        self.add_next(AstNodeType::ResultFloat(v));
    }
    pub fn result_void(&mut self) {
        self.add_next(AstNodeType::ResultVoid);
    }
    pub fn result_nil(&mut self) {
        self.add_next(AstNodeType::ResultNil);
    }

    pub fn decl_var_start(&mut self,name:&'a str, init_nil:bool, anon_id:Option<usize>) {
        if self.body_node().depth==0 && anon_id.is_none() {
            let decl=AstDeclVar::Global;

            self.add_next(AstNodeType::DeclVarStart { name, decl, anon_id:None,});

            {
                let name=VarName{name,anon_id:None,};
                self.decls_starts.push((name,decl,false));
            }
        } else {
            let func_and_param_ind=if self.reuse_local_decls {
                if let AstNodeType::Function { func_ind }=self.body_node().node_type {
                    self.funcs.get(func_ind)
                        .and_then(|func|func.params.iter().position(|param|param.name==name))
                        .map(|param_ind|(func_ind,param_ind))
                } else {
                    None
                }
            } else {
                None
            };

            if let Some((func_ind,param_ind))=func_and_param_ind {
                let decl=AstDeclVar::Param { func_ind, param_ind, };
                self.add_next(AstNodeType::DeclVarStart { name, decl, anon_id, });
                self.add_next(AstNodeType::DeclVarEnd { name, decl, anon_id, });
            } else {
                let existing_local_ind= if self.reuse_local_decls {
                    self.body_node().local_decls.iter().position(|local_decl|{local_decl.name==VarName{name,anon_id}})
                } else {
                    None
                };

                let is_first_decl = existing_local_ind.is_none();

                let local_ind=if let Some(local_ind)=existing_local_ind {
                    local_ind
                } else {
                    let local_ind=self.body_node().local_decls.len();

                    self.body_node_mut().local_decls.push(AstLocalDecl{
                        name : VarName { name, anon_id },
                        // name,
                        // anon_id,
                        captured:false,
                        stack_ind:0,
                    });

                    local_ind
                };

                self.add_next(AstNodeType::DeclVarStart { name, decl:AstDeclVar::Local { local_ind, }, anon_id, });

                {
                    let name=VarName{name,anon_id,};
                    let decl=AstDeclVar::Local { local_ind, };
                    let init_nil = init_nil && is_first_decl;
                    self.decls_starts.push((name,decl,init_nil));
                }

                // if init_nil && is_first_decl {
                //     self.result_nil();
                //     self.set_var(name,anon_id);
                // }

                //
            }
        }
    }

    pub fn decl_var_end(&mut self) -> Result<(),AstError> {
        if let Some((name,decl,init_nil))=self.decls_starts.pop() {
            self.add_next(AstNodeType::DeclVarEnd { name:name.name,decl,anon_id:name.anon_id, });

            if init_nil {
                self.result_nil();
                self.set_var(name.name,name.anon_id).unwrap();
            }

            Ok(())
        } else {
            Err(AstError::DeclStartNotMatching)
        }
    }

    pub fn set_var(&mut self,name:&'a str, anon_id:Option<usize>) -> Result<(),AstError> { //sets global/local var to result
        self.add_next(AstNodeType::SetVar{name,var:AstAccessVar::None, anon_id});

        // if anon_id.is_some() && self.{

        // }

        Ok(())
    }

    pub fn get_var(&mut self,name:&'a str, anon_id:Option<usize>) -> Result<(),AstError> { //gets global/local var to and stores in result
        self.add_next(AstNodeType::GetVar{name,var:AstAccessVar::None, anon_id});
        Ok(())
    }

    pub fn call_var_or_method(&mut self,name:&'a str,params_num:usize, anon_id:Option<usize>) -> Result<(),AstError> {
        //don't need anon_id? since when would you want to call an anon var or a method?

        //uses and pops off params_num amount off stack

        if self.last_node().stack_pushed_num < params_num {
            return Err(AstError::CallNotEnoughParamsPushedOnStack);
        }

        self.add_next(AstNodeType::CallVarOrMethod{name,params_num,var:AstAccessVar::None, anon_id});

        self.last_node_mut().stack_pushed_num-=params_num;

        Ok(())
    }

    // pub fn call_var(&mut self,name:&'a str,params_num:usize, anon_id:Option<usize>) -> Result<(),AstError> {
    //     //uses and pops off params_num amount off stack

    //     if self.last_node().stack_pushed_num < params_num {
    //         return Err(AstError::CallNotEnoughParamsPushedOnStack);
    //     }

    //     self.add_next(AstNodeType::CallVarOrMethod{name,params_num,var:AstAccessVar::None, anon_id});

    //     self.last_node_mut().stack_pushed_num-=params_num;

    //     Ok(())
    // }

    pub fn get_var_or_call_method(&mut self,name:&'a str) -> Result<(),AstError> {
        self.add_next(AstNodeType::GetVarOrCallMethod { name, var: AstAccessVar::None });

        Ok(())
    }
    pub fn call_method(&mut self,name:&'a str,params_num:usize) -> Result<(),AstError> {
        //uses and pops off params_num amount off stack

        if self.last_node().stack_pushed_num < params_num {
            return Err(AstError::CallNotEnoughParamsPushedOnStack);
        }

        self.add_next(AstNodeType::CallMethod{name,params_num});

        self.last_node_mut().stack_pushed_num-=params_num;

        Ok(())
    }

    pub fn try_call_method(&mut self,name:&'a str,params_num:usize) -> Result<(),AstError> {
        //uses and pops off params_num amount off stack

        if self.last_node().stack_pushed_num < params_num {
            return Err(AstError::CallNotEnoughParamsPushedOnStack);
        }

        self.add_next(AstNodeType::TryCallMethod{name,params_num});

        self.last_node_mut().stack_pushed_num-=params_num;

        Ok(())
    }
    // pub fn has_method(&mut self,name:&'a str) -> Result<(),AstError> {
    //     //checks if method exists, store bool in result?

    //     self.add_next(AstNodeType::HasMethod{name,});


    //     Ok(())
    // }
    pub fn call_result(&mut self,params_num:usize) -> Result<(),AstError> {
        //uses and pops off params_num amount off stack

        if self.last_node().stack_pushed_num < params_num {
            return Err(AstError::CallNotEnoughParamsPushedOnStack);
        }

        self.add_next(AstNodeType::CallResult{params_num});

        self.last_node_mut().stack_pushed_num-=params_num;

        Ok(())
    }

    pub fn include(&mut self,name:&'a str) {
        self.add_next(AstNodeType::Include(name));
    }

    //////////////////

    pub fn print(&self) {
        let mut stk = vec![0];

        while let Some(cur_node_ind)=stk.pop() {
            let cur_node = self.nodes.get(cur_node_ind).unwrap();
            stk.extend(cur_node.children.iter().rev());

            println!("{:0>3} [{}=>{} : {}] {}{:?}",
                cur_node_ind,
                cur_node.last_stack_size,
                cur_node.stack_size,
                cur_node.stack_pushed_num,
                " ".repeat(cur_node.depth*4),cur_node.node_type,
            );

            // let stack_inds=cur_node.local_decls.iter().map(|x|x.stack_ind).collect::<Vec<_>>();

            // if stack_inds.len()>0 {
            //     println!("* local_stack_inds {stack_inds:?}");
            // }

            // match cur_node.node_type {
            //     AstNodeType::Function { func_ind } => {
            //         let param_stack_inds=self.funcs.get(func_ind).unwrap().params.iter().map(|x|x.stack_ind).collect::<Vec<_>>();
            //         let capture_stack_inds=self.funcs.get(func_ind).unwrap().captures.iter().map(|x|x.stack_ind).collect::<Vec<_>>();

            //         if param_stack_inds.len()>0 {
            //             println!("* param_stack_inds {param_stack_inds:?}");
            //         }
            //         if capture_stack_inds.len()>0 {
            //             println!("* capture_stack_inds {capture_stack_inds:?}");
            //         }
            //     }
            //     _=>{}
            // }
        }
    }

    fn clear_calc_vars(&mut self) { //clear changes done by calc_vars()
        for node in self.nodes.iter_mut() {
            match &mut node.node_type {
                AstNodeType::GetVar{var,..}
                | AstNodeType::SetVar{var, ..}
                | AstNodeType::CallVarOrMethod{var, ..}
                | AstNodeType::GetVarOrCallMethod { var, .. }
                => {
                    *var=AstAccessVar::None
                }
                _ => {}
            }

            for local_decl in node.local_decls.iter_mut() {
                local_decl.captured=false; //
                local_decl.stack_ind=0;
            }

            node.stack_size=0; //
            node.last_stack_size=0; //
        }

        for func in self.funcs.iter_mut() {
            for param in func.params.iter_mut() {
                param.captured=false; //
                param.stack_ind=0;
            }

            func.captures.clear(); //
        }
    }

    pub fn calc_vars(&mut self, capture_undecl_globals:bool
            // , must_decl_globals:bool,
        ) -> Result<(),AstVarError>{
        // let must_decl_globals=self.must_decl_globals;

        self.clear_calc_vars();

        let mut root_after_env: HashMap<VarName, AstAccessVar> = HashMap::new();
        let mut bodies_before_envs: Vec<HashMap<VarName, AstAccessVar>> = Vec::new(); //[body_depth][var_name]=stack_var
        let mut funcs_after_envs: Vec<HashMap<VarName, AstAccessVar>> = Vec::new(); //[func_ind][var_name]=stack_var

        //cant put access var in capturables, as wont know from which env they came from ie the cur on a prev
        let mut funcs_capturables: Vec<HashSet<VarName>> = Vec::new(); //[func_ind][capturable_name],
        let mut funcs_captures: Vec<Vec<AstCapture>> = Vec::new(); //[func_ind]=captures,
        let mut funcs_body_depths = Vec::<usize>::new(); //[func_ind]=body_depth,

        funcs_after_envs.resize(self.funcs.len(), HashMap::new());
        funcs_capturables.resize(self.funcs.len(), HashSet::new());
        funcs_captures.resize(self.funcs.len(), Vec::new());
        funcs_body_depths.resize(self.funcs.len(), 0);

        //
        let mut node_stk = vec![0]; //(node_ind,on_exit) //,(0,true)

        while let Some(cur_node_ind)=node_stk.pop() {
            node_stk.extend(self.get_node(cur_node_ind).children.iter().rev());

            let mut func_ind_stk = Vec::<usize>::new(); //[func_stk_depth]=func_ind

            //get func ind stk
            {
                let mut cur_node_ind2 = Some(cur_node_ind);

                while let Some(node_ind)=cur_node_ind2 {
                    let node = self.get_node(node_ind);

                    if let AstNodeType::Function { func_ind }=node.node_type {
                        func_ind_stk.push(func_ind);
                    }

                    cur_node_ind2=node.parent;
                }

                func_ind_stk.reverse();
            }

            //
            let cur_node_depth= self.get_node(cur_node_ind).depth;
            let parent_node_ind= self.get_node(cur_node_ind).parent;

            //clear bodies_before_envs to ..
            bodies_before_envs.truncate(cur_node_depth);

            //create root after env
            if cur_node_depth==1 {
                root_after_env.clear();

                let sibling_node_inds=self.get_node(0).children.clone();
                let next_sibling_start=sibling_node_inds.iter().position(|&x|x==cur_node_ind).unwrap()+1;

                for &sibling_node_ind in &sibling_node_inds[next_sibling_start ..] {
                    if let AstNodeType::DeclVarStart { name, decl, anon_id } = self.get_node(sibling_node_ind).node_type {
                        root_after_env.entry(VarName{name,anon_id,}).or_insert(match decl {
                            AstDeclVar::Global => AstAccessVar::Global,
                            AstDeclVar::Local { local_ind } => AstAccessVar::Local {
                                node_ind: parent_node_ind.unwrap(),
                                local_ind,
                                // anon_id,
                            },
                            AstDeclVar::Param { func_ind, param_ind } => AstAccessVar::Param {
                                func_ind,
                                param_ind,
                            },
                        });
                    }
                }
            }

            //
            match self.get_node(cur_node_ind).node_type {
                AstNodeType::Root => {
                    bodies_before_envs.push(HashMap::new());
                }
                AstNodeType::Block { .. } => {
                    bodies_before_envs.push(bodies_before_envs.last().cloned().unwrap_or_default());
                }
                AstNodeType::Function { func_ind } => {
                    funcs_body_depths[func_ind]=cur_node_depth;

                    let parent_before_envs=bodies_before_envs.last().unwrap().clone();
                    bodies_before_envs.push(HashMap::new());

                    //add func params to env
                    {
                        let func=self.funcs.get(func_ind).unwrap();

                        for (param_ind,param) in func.params.iter().enumerate() {
                            bodies_before_envs.last_mut().unwrap().insert(
                                VarName{name:param.name,anon_id:None},
                                AstAccessVar::Param { func_ind, param_ind },
                            );
                        }
                    }

                    //after sibling/root decls
                    let sibling_node_inds=self.get_node(parent_node_ind.unwrap()).children.clone();
                    let next_sibling_start=sibling_node_inds.iter().position(|&x|x==cur_node_ind).unwrap()+1;

                    for &sibling_node_ind in &sibling_node_inds[next_sibling_start ..] {
                        if let AstNodeType::DeclVarStart { name, decl, anon_id } = self.get_node(sibling_node_ind).node_type {
                            funcs_after_envs[func_ind].entry(VarName{name,anon_id})
                                .or_insert(match decl
                            {
                                AstDeclVar::Global => AstAccessVar::Global,
                                AstDeclVar::Local { local_ind } => AstAccessVar::Local {
                                    node_ind: parent_node_ind.unwrap(),
                                    local_ind,
                                    // anon_id,
                                },
                                AstDeclVar::Param { func_ind, param_ind } => AstAccessVar::Param {
                                    func_ind,
                                    param_ind,
                                },
                            });
                        }
                    }

                    //add parent before_env to capturables
                    funcs_capturables[func_ind].extend(parent_before_envs.iter().map(|kv|kv.0));

                    //add func after_env to capturables
                    funcs_capturables[func_ind].extend(funcs_after_envs[func_ind].iter().map(|kv|kv.0));

                    //add root after_env to capturables
                    funcs_capturables[func_ind].extend(root_after_env.iter().map(|kv|kv.0));

                    //add prev funcs capturables to capturables
                    if func_ind_stk.len()>1 {
                        let prev_func_ind=*func_ind_stk.get(func_ind_stk.len()-2).unwrap();
                        let prev_capturables=funcs_capturables[prev_func_ind].clone();

                        funcs_capturables[func_ind].extend(prev_capturables);
                    }
                }
                AstNodeType::DeclVarEnd { name, decl, anon_id } => {
                    let var=match decl {
                        AstDeclVar::Global => AstAccessVar::Global,
                        AstDeclVar::Local { local_ind } => AstAccessVar::Local {
                            node_ind: parent_node_ind.unwrap(),
                            local_ind,
                            // anon_id,
                        },
                        AstDeclVar::Param { func_ind, param_ind } => AstAccessVar::Param {
                            func_ind,
                            param_ind,
                        },
                    };

                    bodies_before_envs.last_mut().unwrap().insert(VarName{name,anon_id},var);
                }

                node_type => {
                    match (node_type,None) {
                        (AstNodeType::GetVar{name,anon_id,..},_)
                        | (AstNodeType::SetVar{name,anon_id, ..},_)
                        | (AstNodeType::CallVarOrMethod{name,anon_id, ..},_)
                        | (AstNodeType::GetVarOrCallMethod { name, .. },anon_id)
                    => {
                        let mut end_depth = cur_node_depth;

                        //create/add captures to before_envs
                        for (func_stk_ind,&func_ind) in func_ind_stk.iter().enumerate().rev() {
                            let func_depth = funcs_body_depths[func_ind];

                            //
                            if bodies_before_envs.get(end_depth-1).unwrap().contains_key(&VarName{name,anon_id})  {
                                break;
                            }

                            //
                            if !capture_undecl_globals {
                                if !funcs_capturables.get(func_ind).unwrap().contains(&VarName{name,anon_id}) {
                                    break;
                                }
                            }

                            //
                            let capture_ind = {
                                //need func's parent depth for beforeenvs
                                let capture_var=if let Some(x)=bodies_before_envs.get(func_depth-1).unwrap().get(&VarName{name,anon_id}) { //func's parent env
                                    *x
                                } else if let Some(x)=funcs_after_envs.get(func_ind).unwrap().get(&VarName{name,anon_id}) {
                                    *x
                                } else if func_stk_ind==0 { //undecl global var, or lib constant/method
                                    AstAccessVar::None
                                    // AstAccessVar::Global
                                } else {
                                    //dont need to worry about func_stk_ind being 0, since an if above will resolve first

                                    //has to be a func capture var, otherwise would of been caught in the prev if's

                                    let prev_func_stk_ind=func_stk_ind-1;
                                    let prev_func_ind=*func_ind_stk.get(prev_func_stk_ind).unwrap();
                                    let prev_func_captures=funcs_captures.get(prev_func_ind).unwrap();

                                    //capture var from ancestor func, doesnt create it, but capture_ind to the future created one
                                    let capture_ind2=prev_func_captures.len();
                                    AstAccessVar::Capture { func_ind:prev_func_ind,capture_ind:capture_ind2 }
                                };

                                //add capture to func
                                let func_captures=funcs_captures.get_mut(func_ind).unwrap();
                                let capture_ind=func_captures.len();

                                func_captures.push(AstCapture {
                                    name,
                                    var: capture_var,
                                    stack_ind:0,
                                });

                                //
                                capture_ind
                            };

                            let capture_var=AstAccessVar::Capture { func_ind,capture_ind };

                            for depth in (func_depth..end_depth).rev() {
                                bodies_before_envs.get_mut(depth).unwrap()
                                    .insert(VarName{name,anon_id},capture_var);
                            }

                            end_depth=func_depth;

                            //
                            if funcs_after_envs[func_ind].contains_key(&VarName{name,anon_id}) {
                                break;
                            }
                        }

                        //
                        let cur_loc=self.get_node(cur_node_ind).loc;


                        //
                        match &mut self.get_node_mut(cur_node_ind).node_type {
                            AstNodeType::GetVar{name,var,..}
                            | AstNodeType::SetVar{name,var, ..}
                            | AstNodeType::CallVarOrMethod{name,var, ..}
                            | AstNodeType::GetVarOrCallMethod { name,var, .. }
                            => {
                                if let Some(&env_var)=bodies_before_envs.get(cur_node_depth-1).unwrap().get(&VarName{name,anon_id}) {
                                    *var=env_var; //modification here!
                                // } else { //what was this originally here for? as it seems it is set to global from somewhere above?
                                    // *var=AstAccessVar::Global; //modification here!
                                }
                                // else if must_decl_globals {
                                //     println!("it is {name:?} {var:?} {cur_loc:?}");

                                //     let loc = cur_loc.unwrap_or(Loc::zero());//should usually be set, but if a mistake was made in the builder,

                                //     return Err(AstVarError{error_type:AstVarErrorType::GlobalNotDecl(name.to_string()),loc});
                                // }
                            }
                            _ => {}
                        }

                        // if let AstNodeType::CallMethodOrGetVar{name,var,..}=self.get_node_mut(cur_node_ind).node_type {

                        // }
                        match &self.get_node(cur_node_ind).node_type {
                            // AstNodeType::GetVar{name,var,..}   |
                            AstNodeType::SetVar{name,var, ..}
                            // | AstNodeType::CallVarOrMethod{name,var, ..}
                            => {
                                if self.must_decl_globals && *var==AstAccessVar::None{
                                    // println!("it is {name:?} {var:?} {cur_loc:?}");

                                    let loc = cur_loc.unwrap_or(Loc::zero());//should usually be set, but if a mistake was made in the builder,

                                    return Err(AstVarError{error_type:AstVarErrorType::GlobalNotDecl(name.to_string()),loc});
                                }
                            }

                            _ => {}
                        }
                    }
                        _ => {}
                    }

                }
            }
        }

        //mark captured locals and params as captured
        for captures in funcs_captures.iter() {
            for capture in captures.iter() {
                match capture.var {
                    AstAccessVar::Param { func_ind, param_ind } => {
                        self.funcs.get_mut(func_ind).unwrap()
                            .params.get_mut(param_ind).unwrap()
                            .captured=true; //modification here!
                    }
                    AstAccessVar::Local { node_ind, local_ind, //anon_id
                    } => {
                        self.nodes.get_mut(node_ind).unwrap()
                            .local_decls.get_mut(local_ind).unwrap()
                            .captured=true; //modification here!
                    }
                    AstAccessVar::Global => {
						//not needed, uses get_global_var_ref
                    }
                    // AstAccessVar::OuterGlobal => {  }
                    AstAccessVar::Capture { .. } => {
						//already a refvar
                    }
                    AstAccessVar::None => {
                        // panic!("");
                        //not needed, uses get_global_access_ref
                    }
                }
            }
        }

        //store captures in self.funcs
        for (func_ind,func) in self.funcs.iter_mut().enumerate() {
            func.captures=funcs_captures.get(func_ind).unwrap().clone(); //modification here!
        }

        //calc stack sizes
        {
            let mut stk = vec![0];

            while let Some(cur_node_ind)=stk.pop() {
                stk.extend(self.get_node(cur_node_ind).children.iter().rev());

                //sibling/parent stack_size
                let last_stack_size = if let Some(parent_node_ind)=self.get_node(cur_node_ind).parent {
                    let parent_node=self.get_node(parent_node_ind);
                    let child_ind=self.get_node(cur_node_ind).child_ind;

                    if child_ind!=0 {
                        let sibling_node_ind=*parent_node.children.get(child_ind-1).unwrap();
                        let sibling_node=self.get_node(sibling_node_ind);

                        match sibling_node.node_type {
                            AstNodeType::Function { .. } => {
                                sibling_node.last_stack_size
                            }
                            AstNodeType::Block { .. } => {
                                sibling_node.last_stack_size
                            }
                            _ => {
                                sibling_node.stack_size
                            }
                        }
                    } else {
                        parent_node.stack_size
                    }
                } else {
                    0
                };

                //
                self.get_node_mut(cur_node_ind).last_stack_size=last_stack_size;

                //

                if let AstNodeType::Function { func_ind } = self.get_node(cur_node_ind).node_type {
                    let params_num=self.funcs.get(func_ind).unwrap().params.len();
                    let captures_num=self.funcs.get(func_ind).unwrap().captures.len();

                    self.get_node_mut(cur_node_ind).stack_size = params_num+captures_num;
                } else {
                    self.get_node_mut(cur_node_ind).stack_size = last_stack_size;
                }

                //push/pop params
                match self.get_node(cur_node_ind).node_type {
                    AstNodeType::StackPush|AstNodeType::StackDup => {
                        self.get_node_mut(cur_node_ind).stack_size+=1;
                    }
                    AstNodeType::StackPop(amount) => {
                        self.get_node_mut(cur_node_ind).stack_size-=amount;
                    }
                    AstNodeType::CallMethod { params_num, .. }
                    | AstNodeType::TryCallMethod { params_num, .. }
                    | AstNodeType::CallResult { params_num }
                    | AstNodeType::CallVarOrMethod { params_num, .. } =>
                    {
                        self.get_node_mut(cur_node_ind).stack_size-=params_num;
                    }
                    _ => {}
                }

                //local decls
                let local_decls_num=self.get_node(cur_node_ind).local_decls.len();

                self.get_node_mut(cur_node_ind).stack_size += local_decls_num;
            }
        }

        //calc local decl stack_inds
        for node_ind in 0 .. self.nodes.len() {
            //local decls already in stack_size, so need to subtract
            let stack_size=self.get_node(node_ind).stack_size - self.get_node(node_ind).local_decls.len();

            for (local_decl_ind,local_decl) in self.get_node_mut(node_ind).local_decls.iter_mut().enumerate() {
                local_decl.stack_ind = stack_size + local_decl_ind;
            }
        }

        //calc func param/capture stack_inds
        for //(func_ind,func)
            func in self.funcs.iter_mut() //.enumerate()
        {
            let params_num=func.params.len(); //non variadic num

            //calc param stack inds
            for (param_ind,param) in func.params.iter_mut().enumerate() {
                param.stack_ind=params_num-param_ind-1;
            }

            //calc capture stack inds
            for (capture_ind,capture) in func.captures.iter_mut().enumerate() {
                capture.stack_ind=params_num+capture_ind;
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.funcs.clear();
        self.body_node_ind = 0;
        self.last_node_ind = 0;
        self.cur_loc = None;
    }

    fn calc_var_stack_offset(&self, var : AstAccessVar, stack_size:usize) -> Option<(usize,bool)> {
        match var {
            AstAccessVar::Global | AstAccessVar::None => { //AstAccessVar::OuterGlobal |
                None
            }
            AstAccessVar::Local { node_ind, local_ind, //anon_id
            } => {
                let local_decl=self.get_node(node_ind).local_decls.get(local_ind).unwrap();
                let stack_offset=stack_size-local_decl.stack_ind-1;
                Some((stack_offset,local_decl.captured))
            }
            AstAccessVar::Param { func_ind, param_ind } => {
                let param=self.funcs.get(func_ind).unwrap().params.get(param_ind).unwrap();
                let stack_offset=stack_size-param.stack_ind-1;
                Some((stack_offset,param.captured))
            }
            AstAccessVar::Capture { func_ind, capture_ind } => {
                let capture=self.funcs.get(func_ind).unwrap().captures.get(capture_ind).unwrap();
                let stack_offset=stack_size-capture.stack_ind-1;
                Some((stack_offset,true))
            }
        }
    }

    pub fn compile(&self, version:usize, path : Option<&Path>, src : Option<StringT>,
        _store_instr_locs : bool,
        _store_stack_names : bool,
    ) -> Build {
        //
        let mut include_inds = IncludeManager::new();
        let mut symbol_inds = SymbolManager::new();

        let mut block_start_instr_inds = HashMap::<usize,usize>::new();
        let mut block_to_ends = HashMap::<usize,Vec<usize>>::new();

        let //mut
            instr_stack_var_names=HashMap::new();

        //instructions are stored main first, then functions in order after that
        let mut instrs_stk : Vec<Vec<Instruction>> = Vec::new();
        let mut instr_locs_stk : Vec<Vec<Option<Loc>>> = vec![Vec::new()];
        let mut instr_stk_inds : Vec<usize> = vec![0];

        //
        instrs_stk.resize(self.funcs.len()+1, Vec::new());
        instr_locs_stk.resize(self.funcs.len()+1, Vec::new());

        //
        //instructions is where main instructions are inserted permanently, and
        //  function instructions temporarily before being move to func_instructions

        let mut node_stk = vec![(0,false)]; //root enter

        while let Some((cur_node_ind,exited))=node_stk.pop() {
            let cur_node = self.nodes.get(cur_node_ind).unwrap();

            let mut instructions=instrs_stk.get_mut(*instr_stk_inds.last().unwrap()).unwrap();

            match cur_node.node_type {
                AstNodeType::Root if exited => { //on exit
                    //pop locals
                    if cur_node.local_decls.len()>0 {
                        instructions.push(Instruction::StackPop(cur_node.local_decls.len()));
                    }
                }
                AstNodeType::Root => { //on enter
                    node_stk.push((cur_node_ind,true)); //root exit
                    node_stk.extend(cur_node.children.iter().rev().map(|&x|(x,false)));

                    //push locals
                    if cur_node.local_decls.len()>0 {
                        instructions.push(Instruction::StackLocals(cur_node.local_decls.len()));
                    }

                    for local_decl in cur_node.local_decls.iter() {
                        if local_decl.captured {
                            let stack_offset = cur_node.stack_size-local_decl.stack_ind-1;
                            instructions.push(Instruction::MakeStackVarRef(stack_offset));
                        }
                    }
                }
                AstNodeType::Block{..} if exited => { //on exit
                    //pop locals
                    if cur_node.local_decls.len()>0 {
                        instructions.push(Instruction::StackPop(cur_node.local_decls.len()));
                    }

                    //
                    if let Some(to_end_instr_inds)=block_to_ends.get(&cur_node_ind) {
                        let cur_instr_ind = instructions.len();

                        for &to_end_instr_ind in to_end_instr_inds.iter() {


                            let _instr_offset_down: usize=cur_instr_ind-to_end_instr_ind;
                            // println!("cur_instr_ind={cur_instr_ind}, to_end_instr_ind={to_end_instr_ind}, _instr_offset_down={_instr_offset_down}");

                            let Instruction::Jmp {
                                instr_pos ,
                                debug,
                                ..
                            } = instructions.get_mut(to_end_instr_ind).unwrap() else {
                                panic!("scriptlang,builder,ast, expected Jmp instr");
                            };

                            *instr_pos=cur_instr_ind; //ok2

                            *debug=(4,_instr_offset_down as i64);
                        }
                    }

                    block_to_ends.remove(&cur_node_ind); //pointless, but cleans memory up a bit
                }
                AstNodeType::Block{..} => { //on enter
                    node_stk.push((cur_node_ind,true)); //block exit
                    node_stk.extend(cur_node.children.iter().rev().map(|&x|(x,false)));

                    //
                    block_start_instr_inds.insert(cur_node_ind, instructions.len());

                    //push locals
                    if cur_node.local_decls.len()>0 {
                        instructions.push(Instruction::StackLocals(cur_node.local_decls.len()));
                    }

                    //setup refvars
                    for local_decl in cur_node.local_decls.iter() {
                        if local_decl.captured {
                            let stack_offset = cur_node.stack_size-local_decl.stack_ind-1;
                            instructions.push(Instruction::MakeStackVarRef(stack_offset));
                        }
                    }

                    //
                    // block_start_instr_inds.insert(cur_node_ind, instructions.len());
                }
                AstNodeType::Function{func_ind,..} if exited => { //on exit
                    // let params_num=self.funcs.get(func_ind).unwrap().params.len();//non_variadic_params_num;
                    let captures_num=self.funcs.get(func_ind).unwrap().captures.len();

                    //pop locals
                    if cur_node.local_decls.len()>0 {
                        instructions.push(Instruction::StackPop(cur_node.local_decls.len()));
                    }

                    //
                    instr_stk_inds.pop().unwrap();
                    instructions=instrs_stk.get_mut(*instr_stk_inds.last().unwrap()).unwrap();

                    //push captures
                    // println!("captures {:?}",self.funcs.get(func_ind).unwrap().captures);

                    for (capture_ind,capture) in self.funcs.get(func_ind).unwrap().captures.iter().enumerate() {
                        let stack_size = capture_ind + cur_node.last_stack_size;

                        if let AstAccessVar::None = capture.var { //an undeclared global (call/get/set)?
                            // panic!("");
                            instructions.push(Instruction::GetGlobalAccessRef(symbol_inds.get(capture.name)));

                        } else if let Some((stack_offset,_))=self.calc_var_stack_offset(capture.var,stack_size) {
                            instructions.push(Instruction::GetStackVar(stack_offset));
                        } else { //from a global decl in script, as wouldn't be in captures list otherwise
                            instructions.push(Instruction::GetGlobalVarRef(symbol_inds.get(capture.name)));
                        }

                        instructions.push(Instruction::StackPush);
                    }

                    //
                    instructions.push(Instruction::ResultFunc(func_ind,captures_num)); //build_func_ind

                    //
                    continue; //not really necessary?
                }
                AstNodeType::Function {func_ind, ..} => { //on enter
                    //params_num,captures_num,captured_params, variadic
                    let func=self.funcs.get(func_ind).unwrap();
                    // let params_num=func.params.len();//func.non_variadic_params_num;
                    // let captures_num=func.captures.len();

                    instr_stk_inds.push(func_ind + 1);
                    instructions=instrs_stk.get_mut(*instr_stk_inds.last().unwrap()).unwrap();

                    //
                    node_stk.push((cur_node_ind,true)); //lambda exit
                    node_stk.extend(cur_node.children.iter().rev().map(|&x|(x,false)));

                    //push locals
                    if cur_node.local_decls.len()>0 {
                        instructions.push(Instruction::StackLocals(cur_node.local_decls.len()));
                    }

                    //convert captured params to refvars
                    for param in func.params.iter() {
                        if param.captured {
                            let stack_offset = cur_node.stack_size-param.stack_ind-1;
                            instructions.push(Instruction::MakeStackVarRef(stack_offset));
                        }
                    }

                    //
                    for local_decl in cur_node.local_decls.iter() {
                        if local_decl.captured {
                            let stack_offset = cur_node.stack_size-local_decl.stack_ind-1;
                            instructions.push(Instruction::MakeStackVarRef(stack_offset));
                        }
                    }
                }
                AstNodeType::ToBlockStart{cond,block_node_ind} => {
                    let to_node=self.get_node(block_node_ind);
                    // let to_stack_size = to_node.stack_size;
                    let to_stack_size = to_node.last_stack_size;
                    let dif_stack_size=cur_node.stack_size - to_stack_size;

                    if dif_stack_size>0 {
                        if //cond.is_some()
                            cond!=JmpCond::None
                        { //(not_cond,2)
                            // let not_cond=cond.and_then(|b|Some(!b));
                            let not_cond=cond.not();
                            let _instr_offset_down:usize = 3;

                            instructions.push(Instruction::Jmp { cond: not_cond,
                                instr_pos: instructions.len()+3, //ok2
                                debug:(0,_instr_offset_down as i64),
                            });
                        }

                        instructions.push(Instruction::StackPop(dif_stack_size));
                    }

                    //
                    let jmp_instr_ind = block_start_instr_inds.get(&block_node_ind).unwrap();

                    let _instr_offset_up: usize = instructions.len()- *jmp_instr_ind;

                    instructions.push(Instruction::Jmp{cond,
                        instr_pos:*jmp_instr_ind , //ok2
                        debug:(1,-(_instr_offset_up as i64)),
                        }); //(cond,jmp_instr_offset)
                }
                AstNodeType::ToBlockEnd{cond,block_node_ind} => {
                    let to_node=self.get_node(block_node_ind);
                    let to_node_parent=self.get_node(to_node.parent.unwrap());
                    let to_stack_size = to_node_parent.stack_size+to_node.stack_pushed_num;
                    let dif_stack_size=cur_node.stack_size - to_stack_size;

                    if dif_stack_size>0 {
                        if //cond.is_some()
                            cond!=JmpCond::None
                        {
                            // let not_cond=cond.and_then(|b|Some(!b));
                            let not_cond=cond.not();

                            let _instr_offset_down:usize = 3;

                            instructions.push(Instruction::Jmp{cond:not_cond,
                                instr_pos:instructions.len()+3, //ok2
                                debug:(2,_instr_offset_down as i64),
                                }); //(not_cond,2)
                        }

                        instructions.push(Instruction::StackPop(dif_stack_size));
                    }

                    //
                    block_to_ends.entry(block_node_ind).or_default().push(instructions.len());

                    instructions.push(Instruction::Jmp {
                        cond,
                        instr_pos: 0,
                        debug:(3,0),
                    }); //(cond,0)
                }
                AstNodeType::StackSwap => {
                    instructions.push(Instruction::StackSwap);
                }
                AstNodeType::StackRot => {
                    instructions.push(Instruction::StackRot);
                }
                AstNodeType::StackPush => {
                    instructions.push(Instruction::StackPush);
                }
                AstNodeType::StackDup => {
                    //cant dupl things outside cur block?
                    // let stack_offset = if cur_node.stack_pushed_num==0 {
                    //     0
                    // } else {
                    //     let parent_node=cur_node.parent.map(|p|self.nodes.get(p).unwrap());
                    //     parent_node.map(|parent_node|parent_node.stack_size).unwrap_or(0)
                    // };

                    // instructions.push(Instruction::StackDup(stack_offset));
                    instructions.push(Instruction::StackDup);
                }
                AstNodeType::StackPop(amount) => {
                    instructions.push(Instruction::StackPop(amount));
                }
                AstNodeType::ResultString(x) => {
                    instructions.push(Instruction::ResultSymbol(symbol_inds.get(x)));
                }
                AstNodeType::ResultBool(x) => {
                    instructions.push(Instruction::ResultBool(x));
                }
                AstNodeType::ResultInt(x) => {
                    instructions.push(Instruction::ResultInt(x));
                }
                AstNodeType::ResultFloat(x) => {
                    instructions.push(Instruction::ResultFloat(x));
                }
                AstNodeType::ResultNil => {
                    instructions.push(Instruction::ResultNil);
                }
                AstNodeType::ResultVoid => {
                    instructions.push(Instruction::ResultVoid);
                }
                AstNodeType::ResultVararg => {
                    instructions.push(Instruction::ResultVararg);
                }
                // AstNodeType::GetGlobalVarRef { name, }=> {
                //     instructions.push(Instruction::GetGlobalVarRef(symbol_inds.get(name), ));
                // }
                AstNodeType::CallMethod{name,params_num} => {
                    instructions.push(Instruction::CallMethod(symbol_inds.get(name),params_num));
                }
                AstNodeType::TryCallMethod{name,params_num} => {
                    instructions.push(Instruction::TryCallMethod(symbol_inds.get(name),params_num));
                }
                AstNodeType::CallResult{params_num} => {
                    instructions.push(Instruction::CallResult(params_num));
                }
                AstNodeType::Include(name) => {
                    instructions.push(Instruction::Include(include_inds.get(name,cur_node.loc)));
                }
                AstNodeType::DeclVarStart { name, decl, anon_id,} => {
                    if anon_id.is_none() { //anon_id vars are local
                        if let AstDeclVar::Global = decl {
                            instructions.push(Instruction::DeclGlobalVar(symbol_inds.get(name)));
                        }
                    }
                }
                AstNodeType::DeclVarEnd {
                    // name, decl, anon_id,
                    ..
                } => {
                }
                AstNodeType::GetVar{name,var,
                    // anon_id,
                    ..
                } => { //copy result ?
                    if let Some((stack_offset,captured))=self.calc_var_stack_offset(var,cur_node.stack_size) {
                        if captured {
                            instructions.push(Instruction::GetStackVarDeref(stack_offset));
                        } else {
                            instructions.push(Instruction::GetStackVar(stack_offset));
                        }
                    } else if let AstAccessVar::Global=var { //global declared
                        instructions.push(Instruction::GetGlobalVarOrConst(symbol_inds.get(name),true));
                    } else {
                        let get_global=!self.must_decl_globals;
                        instructions.push(Instruction::GetGlobalVarOrConst(symbol_inds.get(name),get_global));
                    }
                }
                AstNodeType::SetVar{name,var,
                    // anon_id
                    ..
                } => { //copy result ?
                    if let Some((stack_offset,captured))=self.calc_var_stack_offset(var,cur_node.stack_size) {
                        if captured {
                            let init = if let AstAccessVar::Local { .. } = var {true}else{false};
                            instructions.push(Instruction::SetStackVarDeref(stack_offset,init));
                        } else {
                            instructions.push(Instruction::SetStackVar(stack_offset));
                        }
                    // } else if anon_id.is_some() {
                    } else {
                        instructions.push(Instruction::SetGlobalVar(symbol_inds.get(name)));
                    }
                }
                AstNodeType::CallVarOrMethod{name,params_num,var,
                    // anon_id
                    ..
                } => {
                    if let Some((stack_offset,captured))=self.calc_var_stack_offset(var,cur_node.last_stack_size) { //why does this use last_stack_size and not stack_size?
                        if captured {
                            // instructions.push(Instruction::GetStackVarDeref(stack_offset));
                            // instructions.push(Instruction::CallResult(params_num));

                            instructions.push(Instruction::CallStackVarDeref(stack_offset,params_num));
                        } else {
                            instructions.push(Instruction::GetStackVar(stack_offset));
                            instructions.push(Instruction::CallResult(params_num));
                        }

                    } else {
                        instructions.push(Instruction::CallGlobalOrMethod(symbol_inds.get(name),params_num));
                    }
                }
                AstNodeType::GetVarOrCallMethod { name, var } => {
                    if let Some((stack_offset,captured))=self.calc_var_stack_offset(var,cur_node.stack_size) {
                        if captured {
                            instructions.push(Instruction::GetStackVarDeref(stack_offset));
                        } else {
                            instructions.push(Instruction::GetStackVar(stack_offset));
                        }
                    } else if let AstAccessVar::Global=var { //global declared
                        instructions.push(Instruction::GetGlobalVarOrConst(symbol_inds.get(name),true));
                    } else {
                        let get_global=!self.must_decl_globals;
                        instructions.push(Instruction::GetGlobalOrConstOrCallMethod(symbol_inds.get(name),get_global));
                        // instructions.push(Instruction::CallMethod(symbol_inds.get(name),0));
                    }

                }
            }

            //
            for (instr_stk_ind,instrs) in instrs_stk.iter().enumerate() {
                let instr_locs= instr_locs_stk.get_mut(instr_stk_ind).unwrap();

                if instrs.len()!=instr_locs.len() {
                    for _ in instr_locs.len() .. instrs.len() {
                        instr_locs.push(cur_node.loc);
                    }
                }
            }
        } //end while

        //instr jmp pos's are local to the function, so need to offset
        {
            let mut offset = instrs_stk.first().unwrap().len();

            for instrs_stk_ind in 1..instrs_stk.len() {
                let instrs= instrs_stk.get_mut(instrs_stk_ind).unwrap();

                for instr in instrs.iter_mut() {
                    let Instruction::Jmp { instr_pos, ..}=instr else {continue;};
                    // println!("=> {instr_pos}+{offset}={}",*instr_pos+offset);
                    *instr_pos+=offset;
                }

                offset+=instrs.len();
            }
        }

        //
        let instructions = instrs_stk.iter().flatten().map(|x|x.clone()).collect::<Vec<_>>();

        let locs = instr_locs_stk.iter().flatten().map(|x|x.clone()).collect::<Vec<_>>();

        //
        let mut build_funcs = Vec::<BuildFunc>::new();
        let main_instruct_len= instrs_stk.first().unwrap().len();
        let mut instruct_start_pos=main_instruct_len;

        for (func_ind,func) in self.funcs.iter().enumerate() {
            let instruct_len=instrs_stk.get(func_ind+1).unwrap().len();
            let params_num = func.params.len() + func.captures.len();

            build_funcs.push(BuildFunc{instruct_start_pos,instruct_len,params_num,});

            instruct_start_pos+=instruct_len;
        }

        //
        let mut instr_locs_map = HashMap::<usize,Loc>::new();
        let mut instr_locs_alt = Vec::<(usize,Option<Loc>)>::new();

        for (instr_ind,loc) in locs.iter().enumerate() {
            if let Some(loc)=loc {
                instr_locs_map.insert(instr_ind,*loc);
            }

            //alternate idea
            let prev_loc = instr_locs_alt.last().and_then(|(_prev_start_instr_ind,prev_loc)|prev_loc.clone());

            if *loc!=prev_loc {
                instr_locs_alt.push((instr_ind,*loc));
            }
        }

        //
        Build {
            includes : include_inds.to_paths(),
            symbols : symbol_inds.to_vec(),
            instructions,
            functions: build_funcs,
            main_instruct_len,
            path:path.and_then(|p|Some(p.to_path_buf())),
            src,
            version,
            instr_locs : instr_locs_map,
            include_first_locs : include_inds.to_locs(),
            instr_stack_var_names,
            instr_locs_alt,
            // func_names,
            // func_locs,
        }
    }
}

