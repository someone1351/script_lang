

use super::error::*;
use super::temp_data::*;
use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::build::Loc;
use super::super::grammar::data::{Walk,WalkGroup};
use super::super::tokenizer::{TokenIterContainer, ValueContainer};

use super::node::*;


// use data::*;
// use error::*;


pub struct GrammarWalker<'t,'g,G>
where
    G: Fn(&str)->Option<GrammarNode<'g>>,
{
    top_primitives:TokenIterContainer<'t>,
    primitives_remaining: TokenIterContainer<'t>,
    grammar_func:G,
    stk: Vec<Work<'t,'g>>,
    c:usize,
    expected_loc:Loc,
    expecteds:Vec<(u32,GrammarNode<'g>)>, //(priority,gramamr)//(u64,GrammarNode<'g>) //(id,grammar) //todo change grammar to &'g
    expected_count:u64,
    debug:bool,
    non_term_recursive_check:bool,

    groups:Vec<TempGroupInfo<'t,'g>>,

    hist_news:Vec<TempHistNew<'t,'g>>,
    hist_begins_stk:Vec<TempHistBegins<'t,'g>>,
    hist_ends_stk:Vec<TempHistEnds<'t,'g>>,
}

impl<'t,'g,G> GrammarWalker<'t,'g,G>
where
    G: Fn(&str)->Option<GrammarNode<'g>>,
{

    pub fn new(top_primitives:TokenIterContainer<'t>, grammar_func:G,) -> Self {
        Self {
            stk:Default::default(),
            c:Default::default(),
            expected_loc:Loc::zero(),
            expecteds:Vec::new(),
            grammar_func,
            primitives_remaining:top_primitives.clone(),
            top_primitives,
            debug:false,
            non_term_recursive_check:true,
            expected_count: 0,

            groups:Default::default(),

            hist_news: Default::default(),
            hist_begins_stk: Default::default(),
            hist_ends_stk: Default::default(),
        }
    }

    pub fn set_non_term_recursive_check(&mut self,non_term_recursive_check:bool) {
        self.non_term_recursive_check=non_term_recursive_check;
    }

    fn init(&mut self,start_non_term:&'g str,) {
        self.stk.clear();

        //
        self.stk.push(Work{
            grammar:GrammarNode::Error(GrammarWalkError::FailedParse),
            success_len:0,fail_len:0,
            tokens:self.top_primitives,
            group_ind: 0, group_len: 1,
            visiteds:Default::default(),
            grammar_debug_len: 0,
            expected:Default::default(),
            and_id: 0,

            from_user:false,
            is_first:true,

            hist_news_len:0,
            hist_begins_stk_len:0,
            hist_ends_stk_len:1,
        });

        //
        let fail_len=self.stk.len();

        //no needed, but allows takeables2 to finish, for debugging purposes
        self.stk.push(Work{
            grammar : GrammarNode::Always,
            success_len:0,
            fail_len:0, //not used
            tokens:self.top_primitives,
            group_ind: 0, group_len: 1,
            visiteds:Default::default(),
            grammar_debug_len: 1,
            expected:Default::default(),
            and_id: 0,

            from_user:true,
            is_first:true,

            hist_news_len:0,
            hist_begins_stk_len:0,
            hist_ends_stk_len:1,
        });

        //
        let success_len=self.stk.len();

        //start
        {
            let grammar=if let Some(g)=(self.grammar_func)(start_non_term) {
                g
            } else {
                GrammarNode::Error(GrammarWalkError::MissingNonTerm(start_non_term))
            };

            self.stk.push(Work{
                grammar, //:(self.grammar_func)(start_non_term),
                // success_len:0,
                success_len,
                fail_len, //1
                tokens:self.top_primitives,
                group_ind: 0, group_len: 1,
                visiteds:Default::default(),
                grammar_debug_len: 1,
                expected:Default::default(),
                and_id: 0,

                from_user:true,
                is_first:true,

                hist_news_len:0,
                hist_begins_stk_len:0,
                hist_ends_stk_len:1,
            });
        }

        //
        self.groups=vec![TempGroupInfo{
            name: "",
            parent: 0,
            tokens:self.top_primitives,
        }];

        //
        self.hist_news.clear();
        self.hist_begins_stk.clear(); //don't need initial one because don't need to store begins before an Or exists
        // self.hist_ends_stk.clear();
        self.hist_ends_stk=vec![Default::default()]; //need an initial one because require ends regardless of an Or existing

        //
        self.c=0;
        self.expected_loc=Loc::zero();
        self.expecteds.clear();
        self.expected_count=0;
    }

    fn grammar_expect(&mut self,cur :Work<'t,'g>,priority:u32, name:&'g str,g:Box<GrammarNode<'g>>) {
        self.expected_count+=1;
        let expected=if cur.expected.id==0 {
            // (self.expected_count,name)
            let priority=priority+1; //so primitives/tokens are at priority 0, expected(s) are 1+
            WorkExpected{ id: self.expected_count, priority, name }
        } else {
            cur.expected
        };

        //
        let hist_news_len=self.hist_news_add(&cur);

        //TODO
        self.stk.push(Work {
            grammar: *g,
            success_len: cur.success_len,
            fail_len: cur.fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds:cur.visiteds,
            grammar_debug_len: cur.grammar_debug_len+1,
            expected,
            and_id:cur.and_id,

            from_user:true,
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,
        });
    }

    fn grammar_group(&mut self,cur :Work<'t,'g>,name:&'g str,g:Box<GrammarNode<'g>>) {
        //
        let new_group_ind=self.new_group(name, cur.group_ind, cur.tokens);

        //
        let groups=&self.groups;
        let new_group_len=groups.len();

        //
        let hist_news_len=self.hist_news_add(&cur);

        //
        self.stk.push(Work {
            grammar: *g,
            success_len: cur.success_len,
            fail_len: cur.fail_len,
            tokens: cur.tokens,
            group_ind: new_group_ind,
            group_len: new_group_len,
            visiteds:cur.visiteds,
            grammar_debug_len: cur.grammar_debug_len+1,
            expected:cur.expected,
            and_id:cur.and_id,

            from_user:true,
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,
        });
    }

    fn grammar_opt(&mut self,cur :Work<'t,'g>,g:Box<GrammarNode<'g>>,) {
        //
        let hist_news_len=self.hist_news_add(&cur);

        //
        self.stk.push(Work {
            grammar: GrammarNode::Always,
            success_len: cur.success_len,
            fail_len: 0, //fail is not used
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds:cur.visiteds.clone(),
            grammar_debug_len: cur.grammar_debug_len,
            expected:cur.expected,
            and_id:cur.and_id,

            from_user:false,
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,
        });

        //
        let fail_len=self.stk.len();

        //
        self.stk.push(Work {
            grammar: *g,
            success_len: cur.success_len,
            fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds:cur.visiteds,
            grammar_debug_len: cur.grammar_debug_len+1,
            expected:cur.expected,
            and_id:cur.and_id,

            from_user:true,
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,
        });
    }

    fn grammar_many(&mut self,cur :Work<'t,'g>,g:Box<GrammarNode<'g>>,) {
        //in always/prev they check if their success_ind is a many (which could be a problem if ands/ors were handled more efficiently),
        //  but could store maybe a many_id to check whether to exit? eg if id is eq, and/or tokens.inds.start is eq?
        //
        let hist_news_len=self.hist_news_add(&cur);

        //
        self.stk.push(Work {
            grammar: GrammarNode::Many(g.clone()),
            success_len: cur.success_len,
            fail_len: cur.fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds:cur.visiteds.clone(),
            grammar_debug_len: cur.grammar_debug_len,
            expected:cur.expected,
            and_id:cur.and_id,

            from_user:false,
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,
        });

        //
        let success_len2=self.stk.len();

        //
        self.stk.push(Work {
            grammar: GrammarNode::Always,
            success_len: cur.success_len,
            fail_len: 0, //fail is not used
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds:cur.visiteds.clone(),
            grammar_debug_len: cur.grammar_debug_len,
            expected:cur.expected,
            and_id:cur.and_id,

            from_user:false,
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,
        });

        //
        let fail_len=self.stk.len();

        //
        self.stk.push(Work {
            grammar: *g,
            success_len: success_len2,
            fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds:cur.visiteds,
            grammar_debug_len: cur.grammar_debug_len+1,
            expected:cur.expected,
            and_id:cur.and_id,

            from_user:true,
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,
        });
    }

    fn grammar_non_term(&mut self,cur :Work<'t,'g>,t:&'g str) -> Result<(),GrammarWalkError<'g>>{
        //
        let hist_news_len=self.hist_news_add(&cur);

        //
        let visiteds=self.do_non_term_visiteds(t,cur.tokens,cur.visiteds)?;

        //
        let grammar=if let Some(g)=(self.grammar_func)(t) {
            g
        } else {
            GrammarNode::Error(GrammarWalkError::MissingNonTerm(t))
        };

        //
        self.stk.push(Work {
            grammar, //: (self.grammar_func)(t), //should return err on not found, instead of grammar never, should have error
            success_len: cur.success_len,
            fail_len: cur.fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds,
            grammar_debug_len: cur.grammar_debug_len+1,
            expected:cur.expected,
            and_id:cur.and_id,

            from_user:true,
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,
        });

        Ok(())
    }

    fn grammar_error(&mut self,cur :Work<'t,'g>,e:GrammarWalkError<'g>) -> GrammarWalkError<'g> {
        if self.debug {
            println!("====error {:?} {:?}",self.expected_loc,self.expecteds,);
        }

        //necesaary? any point to it?
        // if self.expecteds.is_empty() { // self.expected.0.is_zero()
        //     self.expected_loc=cur.primitives.loc();
        // }

        //
        self.set_remaining_prims(cur.tokens);

        //
        return e;
    }

    fn grammar_string(&mut self,cur :Work<'t,'g>,) {
        if let Some(v)=self.do_primitive(cur,|ps|ps.pop_string(),) {
            if self.debug {
                println!("--- string {v:?}");
            }
        }
    }

    fn grammar_identifier(&mut self,cur :Work<'t,'g>,) {
        if let Some(v)=self.do_primitive(cur,|ps|ps.pop_identifier(),) {
            if self.debug {
                println!("--- identifier {v:?}");
            }
            // println!("==={}",self.grammar_debug_stk.last().map(|x|format!("{x}")).unwrap_or("None".to_string()));
        }
    }

    fn grammar_int(&mut self,cur :Work<'t,'g>,) {
        if let Some(v)=self.do_primitive(cur,|ps|ps.pop_int(),) {
            if self.debug {
                println!("--- int {v:?}");
            }
        }
    }

    fn grammar_float(&mut self,cur :Work<'t,'g>,) {
        if let Some(v)=self.do_primitive(cur,|ps|ps.pop_float(),) {
            if self.debug {
                println!("--- float {v:?}");
            }
        }
    }

    fn grammar_symbol(&mut self,cur :Work<'t,'g>,s:&'g str) {
        if let Some(v)=self.do_primitive(cur,|ps|ps.pop_with_symbol(s),) {
            if self.debug {
                println!("--- symbol {v:?}");
            }
        }
    }

    fn grammar_keyword(&mut self,cur :Work<'t,'g>,s:&'g str) {
        if let Some(v)=self.do_primitive(cur,|ps|ps.pop_with_keyword(s),) {
            if self.debug {
                println!("--- keyword {v:?}");
            }
        }
    }

    fn grammar_eol(&mut self,cur :Work<'t,'g>,) {
        if let Some(_)=self.do_primitive(cur,|ps|ps.pop_eol(),) {
            if self.debug {
                println!("--- eol");
            }
        }
    }

    fn grammar_and(&mut self,cur :Work<'t,'g>,gs:Vec<GrammarNode<'g>>,) {
        //
        let Some(first)=gs.first().cloned() else { return ; };

        //
        let hist_news_len=self.hist_news_add(&cur);

        //
        if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
            self.stk.push(Work {
                grammar: GrammarNode::And(rest.into()),
                success_len: cur.success_len,
                fail_len: cur.fail_len,
                tokens: cur.tokens, //not really necessary? since gets updated by always/primtitives
                group_ind: cur.group_ind,
                group_len: cur.group_len,
                visiteds:cur.visiteds.clone(),
                grammar_debug_len: cur.grammar_debug_len,
                expected:cur.expected,
                and_id:cur.and_id+1,

                from_user:false,
                is_first:false,

                hist_news_len,
                hist_begins_stk_len:cur.hist_begins_stk_len,
                hist_ends_stk_len:cur.hist_ends_stk_len,
            });
        }

        //
        let success_len=if gs.len()>1 {self.stk.len()}else{cur.success_len};

        //
        self.stk.push(Work {
            grammar: first,
            success_len,
            fail_len: cur.fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds:cur.visiteds,
            grammar_debug_len: cur.grammar_debug_len+1,
            expected:cur.expected,

            // and_id:cur.and_id+1,
            and_id:if gs.len()==1{cur.and_id}else{cur.and_id+1}, //don't need if single element in And?

            from_user:true,
            is_first:cur.is_first, //cur.from_user &&  //only want to know about grammars added by user, not the walker, could check from_user elsewhere,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,
        });
    }

    fn grammar_or(&mut self,cur :Work<'t,'g>,gs:Vec<GrammarNode<'g>>,) {
        //
        let Some(first)=gs.first().cloned() else { return; };

        //
        let hist_news_len=self.hist_news_add(&cur);

        //
        if  self.hist_begins_stk.is_empty() //reason? shouldn't it not be empty, as would be created by first? there was a reason ...
            || !cur.is_first //don't need to use cur.from_user, first is better, as could handle potential Or not added by user
        {
            self.hist_begins_stk.push(Default::default());
        }

        //
        if cur.is_first {
            self.hist_ends_stk.push(self.hist_ends_stk.last().cloned().unwrap());
        }

        //
        if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
            self.stk.push(Work {
                grammar: GrammarNode::Or(rest.into()),
                success_len: cur.success_len,
                fail_len: cur.fail_len,
                tokens: cur.tokens,
                group_ind: cur.group_ind,
                group_len: cur.group_len,
                visiteds:cur.visiteds.clone(),
                grammar_debug_len: cur.grammar_debug_len,
                expected:cur.expected,
                and_id:cur.and_id,

                from_user:false,
                is_first:cur.is_first,

                hist_news_len,
                hist_begins_stk_len:self.hist_begins_stk.len(),
                hist_ends_stk_len:self.hist_ends_stk.len(),
            });
        }

        //
        let fail_len=if gs.len()>1 {self.stk.len()}else{cur.fail_len};

        //
        self.stk.push(Work {
            grammar: first,
            success_len: cur.success_len,
            fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds:cur.visiteds,
            grammar_debug_len: cur.grammar_debug_len+1,
            expected:cur.expected,
            and_id:cur.and_id,

            from_user:true,
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:self.hist_begins_stk.len(),
            hist_ends_stk_len:self.hist_ends_stk.len(),
        });
    }

    fn grammar_prev(&mut self,cur :Work<'t,'g>,g:Box<GrammarNode<'g>>) {
        //
        let _hist_news_len=self.hist_news_add(&cur);

        let cur_hist_ends=&self.hist_ends_stk.last().unwrap().elements;

        //
        if cur_hist_ends.contains_key(&g) {

            //
            self.stk.truncate(cur.success_len);

            //
            if let Some(last)=self.stk.last_mut() {
                if last.grammar.is_many() && last.tokens.len()==cur.tokens.len() { //if not parsing anything, exit the many
                    last.grammar=GrammarNode::Always;
                }

                //
                last.tokens=cur.tokens;
                last.group_len=cur.group_len;

                // last.hist_ends=cur.hist_ends.clone(); //should use move
                last.hist_ends_stk_len=cur.hist_ends_stk_len;

                //
                if cur.expected.id!=last.expected.id {
                    last.expected=Default::default();
                }
            }

            //
            self.hist_news_truncate_to_last();


            //
            self.do_groups_primitives_clamp(cur.group_ind,cur.tokens);
            self.hist_news_add_to_last(&cur,true);
            self.set_remaining_prims(cur.tokens);
        } else {
            //
            if cur.expected.id==0{
                self.add_expected(
                    cur.tokens.first().map(|t|t.start_loc()).unwrap_or_default(),
                    0,cur.grammar.clone(),
                );
            }

            //
            self.stk.truncate(cur.fail_len);

            //
            self.hist_news_truncate_to_last();
        }
    }

    fn grammar_always(&mut self,cur :Work<'t,'g>,) {
        //
        self.stk.truncate(cur.success_len);

        //
        let _hist_news_len=self.hist_news_add(&cur);

        //
        if let Some(last)=self.stk.last_mut() {
            //
            if self.debug {
                println!("---- last.group_len={}, cur.group_len={}, last.group_ind={}, cur.group_ind={}",
                    last.group_len,cur.group_len,
                    last.group_ind, cur.group_ind,
                );
            }

            //
            if last.grammar.is_many() && last.tokens.len()==cur.tokens.len() { //to stop the many getting stuck in a loop
                last.grammar=GrammarNode::Always;
            }

            //
            last.tokens=cur.tokens;
            last.group_len=cur.group_len; //done below //not anymore

            //
            last.hist_ends_stk_len=cur.hist_ends_stk_len;

            //
            if cur.expected.id!=last.expected.id {
                last.expected=Default::default();
            }
        }

        //
        self.do_groups_primitives_clamp(cur.group_ind,cur.tokens); //here

        //
        self.hist_news_add_to_last(&cur,true);

        //
        self.set_remaining_prims(cur.tokens);

        //
        // self.clear_expected();
    }

    fn grammar_try_from_hist_begins(&mut self,cur :&Work<'t,'g>) -> bool {
        if cur.from_user && cur.is_first {
            if let Some(hist_begins)=self.hist_begins_stk.last() {
                if let Some(hist_begin)=hist_begins.elements.get(&cur.grammar) {
                    //
                    self.stk.truncate(cur.success_len);

                    //
                    if let Some(last)=self.stk.last_mut() {
                        //
                        // last.tokens=cur.tokens;
                        last.tokens=hist_begin.tokens_after;

                        //add groups
                        for g in hist_begin.groups.iter() {
                            self.groups.push(TempGroupInfo { parent: cur.group_ind+g.parent, ..g.clone()});
                        }

                        //
                        last.group_len=self.groups.len(); //cur.group_len+or_element.groups;

                        //
                        last.hist_ends_stk_len=cur.hist_ends_stk_len;

                        //
                        if cur.expected.id!=last.expected.id {
                            last.expected=Default::default();
                        }
                    }

                    //
                    let hist_begin=hist_begin.clone();

                    //
                    self.do_groups_primitives_clamp(cur.group_ind,cur.tokens);

                    //a
                    self.hist_news_add_to_last(&cur,false); //not needed? no.. if And(Z,Or(And(X,Y),X)), then will add that

                    //
                    self.set_remaining_prims(cur.tokens);

                    //
                    println!("---- grabbed from or {:?}, {hist_begin:?}",cur.grammar);

                    //
                    return true;
                }
            }
        }

        false
    }

    fn do_primitive<Q,P>(&mut self,mut cur:Work<'t,'g>,prim_func:Q) -> Option<ValueContainer<'t,P>>
    where
        P:Clone,
        Q:Fn(&mut TokenIterContainer<'t>)->Result<ValueContainer<'t,P>,Loc>,
    {
        //
        let _hist_news_len=self.hist_news_add(&cur);

        match prim_func(&mut cur.tokens) {
            Ok(v) => {
                //
                let vprim=v.token;


                //
                if vprim.start_loc() >= self.expected_loc {
                    self.clear_expected();
                }

                //
                self.stk.truncate(cur.success_len);

                //
                if let Some(last)=self.stk.last_mut() {
                    last.tokens=cur.tokens;
                    last.group_len=cur.group_len;
                    last.expected=Default::default();
                }

                //
                self.do_groups_primitives_clamp(cur.group_ind,cur.tokens);

                //
                self.last_hist_ends_remove_previous();
                self.hist_news_add_to_last(&cur,true);

                //
                if self.debug {
                    println!("--- hmm stk={:?}",self.stk.iter().map(|x|x.grammar.clone()).collect::<Vec<_>>());

                    if let Some(last)=self.stk.last() {
                        let last_hist_ends=&self.hist_ends_stk[last.hist_ends_stk_len-1].elements;
                        println!("---last_hist_ends={:?}, len={}",last_hist_ends,last_hist_ends.len());
                    }
                }

                //
                self.set_remaining_prims(cur.tokens);

                //
                Some(v)
            }
            Err(loc) => {
                //
                // if self.stk.last().map(|last|!last.expected_non_term.is_none() ).unwrap_or_default()

                //
                if cur.expected.id==0{
                    self.add_expected(loc,0,cur.grammar.clone());
                }

                //
                self.stk.truncate(cur.fail_len);

                //
                if let Some(last)=self.stk.last_mut() {


                    //
                    // if let Some(x)=cur.expected_non_term {
                    //     if last.expected_non_term.is_none() {
                    //         self.add_expected(loc, GrammarNode::NonTerm(x));
                    //     }
                    // //     last.expected_non_term=None;
                    // }

                    //
                    if cur.expected.id!=last.expected.id && cur.expected.id!=0 {
                        last.expected=Default::default();
                        self.add_expected(loc, cur.expected.priority,GrammarNode::NonTerm(cur.expected.name));
                    }
                }

                //
                self.hist_news_truncate_to_last();

                //
                self.set_remaining_prims(cur.tokens);

                //
                None
            }
        }
    }

    fn hist_news_add_to_last(&mut self,
        cur:&Work<'t,'g>,
        gotten:bool, //what was this for again? something to do with not adding cur grammar to hist_begins? it was for not adding cur grammar to hist_new?
    ) {
        let cur_tokens=cur.tokens;

        //
        if let Some(last)=self.stk.last_mut() {
            //
            let drained_hist_news=self.hist_news.drain(last.hist_news_len ..).collect::<Vec<_>>();

            //
            let hist_news=drained_hist_news.iter().map(|hist_new|{
                //
                let tokens_len=hist_new.tokens_start.len()-cur_tokens.len();
                let tokens=hist_new.tokens_start.get_amount(tokens_len).unwrap();

                //
                if self.debug {
                    println!("--- inserting hist_new {:?} {tokens:?}",hist_new.grammar);
                }

                //
                (  hist_new.grammar.clone(), TempHistEnd { tokens, } )
            }).collect::<Vec<_>>();

            //
            for i in 0..drained_hist_news.len() {
                let hist_new=&drained_hist_news[i];

                // //
                // if self.debug {
                //     println!("---going i={i}, is_first={}, g={:?} self.or_stk.len={}",
                //         hist_new.is_first,hist_new.grammar,
                //         self.hist_begins_stk.len(),
                //     );
                // }

                //
                if !hist_new.is_first { continue; }
                let Some(hist_begins)=self.hist_begins_stk.last_mut() else {continue;};

                //
                let mut groups=self.groups[cur.group_ind..cur.group_len].to_vec();

                //offset parents in group
                if !groups.is_empty() { //cur.group_ind!=cur.group_len
                    let first_parent=groups[0].parent;

                    for group in groups.iter_mut() {
                        group.parent-=first_parent;
                    }
                }

                //if or(and(A0,A1),A2)
                //  after A0, it gets added to the hist_begins, that A1 can see, but not use because it isn't a first

                //
                hist_begins.elements.entry(hist_new.grammar.clone()).or_insert_with(||TempHistBegin {
                    groups,
                    hist_ends: HashMap::from_iter(hist_news[i+1..].into_iter().cloned()),
                    tokens_after: cur_tokens,
                });
            }

            //
            let last_hist_ends=&mut self.hist_ends_stk[last.hist_ends_stk_len-1].elements;
            last_hist_ends.extend(hist_news.into_iter());
        }
    }

    fn last_hist_ends_remove_previous(&mut self) {
        if let Some(last)=self.stk.last_mut() {
            let last_tokens_start=last.tokens.inds().start;

            let last_hist_ends=&mut self.hist_ends_stk[last.hist_ends_stk_len-1].elements;
            last_hist_ends.retain(|_k,v|{
                v.tokens.inds().start>=last_tokens_start
                // last_tokens_start<v.tokens.inds().start
            });
        }
    }

    fn hist_news_truncate_to_last(&mut self) {
        if let Some(last)=self.stk.last() {
            self.hist_news.truncate(last.hist_news_len); //what is it for? clears failed takeable_starts?
        }
    }

    fn hist_news_add(&mut self,cur:&Work<'t,'g>) -> usize {
        if cur.from_user { //ignore grammars added by walker
            self.hist_news.push(TempHistNew {
                grammar: cur.grammar.clone(),
                tokens_start: cur.tokens.clone(),
                // group_ind: cur.group_ind,
                is_first:cur.is_first,
            });
        }

        self.hist_news.len()
    }

    fn step_truncates(&mut self,cur :&Work<'t,'g>) {
        self.groups.truncate(cur.group_len);
        self.hist_begins_stk.truncate(cur.hist_begins_stk_len);
        self.hist_ends_stk.truncate(cur.hist_ends_stk_len);
    }

    fn do_groups_primitives_clamp(&mut self,
        cur_group_ind:usize,
        cur_primitives:TokenIterContainer<'t>,
    ) {
        if let Some(last)=self.stk.last_mut() {
            //
            if self.debug {
                println!("==do_groups_primitives_clamp: cur_group_ind={cur_group_ind}, last.group_ind={}",last.group_ind);
            }

            //
            let mut g=cur_group_ind;

            //
            while g>last.group_ind {
                let group=&mut self.groups[g];

                //
                let n=group.tokens.len()-cur_primitives.len();

                //
                let group_prims=group.tokens.get_amount(n).unwrap();

                //
                group.tokens=group_prims;
                g=group.parent;
            }
        }
    }

    fn do_non_term_visiteds(&mut self,
        t:&'g str,
        cur_primitives:TokenIterContainer<'t>,
        cur_visiteds: HashSet<(&'g str, usize)>,
    ) -> Result<HashSet<(&'g str, usize)>,GrammarWalkError<'g>> {

        //
        if !self.non_term_recursive_check {
            return  Ok(Default::default());
        }

        //
        let v=(t,cur_primitives.inds().start);

        //
        if cur_visiteds.contains(&v) {
            return Err(GrammarWalkError::RecursiveNonTerm(t));
        }

        //
        let mut visiteds=cur_visiteds;
        visiteds.insert(v);

        //
        Ok(visiteds)
    }

    fn new_group(&mut self,name : &'g str, parent:usize, ps:TokenIterContainer<'t>) -> usize {
        let groups=&mut self.groups;
        let new_group_ind=groups.len();

        groups.push(TempGroupInfo { name, parent, tokens:ps, });
        new_group_ind
    }

    fn set_remaining_prims(&mut self,cur_primitives:TokenIterContainer<'t>,) {
        if self.stk.is_empty() {
            self.primitives_remaining=cur_primitives;
        }
    }

    //
    fn clear_expected(&mut self) {
        // println!("-------==== expected cleared, {}",self.expected_loc);

        //
        self.expected_loc=Loc::zero();
        self.expecteds.clear();
    }

    fn add_expected(&mut self,loc:Loc,p:u32,g:GrammarNode<'g>) {
        //
        if loc==self.expected_loc {
            //
            self.expecteds.push((p,g.clone()));

            //
            // println!("-------==== expected added {g:?}, {loc}=={}",self.expected_loc);
        } else if loc>self.expected_loc  { //|| self.expecteds.is_empty()
            //
            self.expected_loc=loc;
            self.expecteds=vec![(p,g.clone())];

            //
            // println!("-------==== expected new {g:?}, {loc}=={}",self.expected_loc);
        } else {
            // println!("-------==== expected not added {g:?}, {loc}=={}",self.expected_loc);
        }
    }

    //
    pub fn expecteds_string(&self) -> String {
        let max_priority=self.expecteds.iter().map(|&(p,_)|p).max().unwrap_or(0);

        self.expecteds.iter().filter_map(|(p,g)|(*p==max_priority).then_some(g)).map(|g|match g {
            GrammarNode::String => "string".to_string(),
            GrammarNode::Identifier => "identifier".to_string(),
            GrammarNode::Int => "int".to_string(),
            GrammarNode::Float => "float".to_string(),
            GrammarNode::Symbol(s) => format!("'{s}'"),
            GrammarNode::Keyword(s) => format!("'{s}'"),
            GrammarNode::Eol => "eol".to_string(),
            GrammarNode::NonTerm(s) => format!("{s}"),
            _ =>"".to_string(),
        }).collect::<Vec<_>>().join(", ")
    }

    pub fn last_loc(&self) -> Loc {
        if self.expected_loc.is_zero() {
            self.primitives_remaining.loc()
        } else {
            self.expected_loc
        }
    }

    //
    pub fn get_walk(&self) -> Walk<'t,'g> {
        //
        let mut groups_out: Vec<WalkGroup<'t,'g>>=Vec::new();//vec![WalkGroup{ name: "", children: 0..0, tokens: todo!() }];

        //
        let group_infos=&self.groups;

        //
        let mut group_infos2 = group_infos.iter().enumerate()
            .map(|(i,g)|(i,g.parent,))
            .collect::<Vec<_>>(); //(grouo_ind,parent_ind,child_num)

        //sort groups to breadth first
        group_infos2[1..].sort_by(|&(g1,p1,),&(g2,p2,)|{
            match p1.cmp(&p2) {
                std::cmp::Ordering::Equal => g1.cmp(&g2),
                x=>x,
            }
        });

        //
        println!("groups2 {:?}",group_infos2.iter().enumerate().collect::<Vec<_>>());

        //
        for (i,&(g,p,)) in group_infos2.iter().enumerate() {
            //
            let group_infos=&self.groups;

            //
            println!("\t{i}: g{g}, p{p}, {:?}, {:?}, {:?}",group_infos[g].name,group_infos[g].tokens.inds(),group_infos[g].tokens);
        }

        //
        // let mut csum=1;
        let ind_map: HashMap<usize, usize> = HashMap::from_iter(group_infos2.iter().enumerate().map(|(i,&(g,_p,))|(g,i)));

        //
        for (i,&(gind,p,)) in group_infos2.iter().enumerate() {
            //
            let group_infos=&self.groups;

            //
            let g=&group_infos[gind];

            //
            groups_out.push(WalkGroup { name: g.name,
                children: 0..0, // csum..csum+c
                tokens: g.tokens,
            });

            //
            if i!=0 { //as root's parent is 0, ie itself, which is incorrect
                let ind=ind_map.get(&p).cloned().unwrap();
                let c= &mut groups_out[ind].children;
                if c.start==0 {c.start=i;}
                c.start=c.start.min(i);
                c.end=c.end.max(i+1);
            }
        }

        //
        let walk=Walk{ groups: groups_out };
        walk
    }

    //
    pub fn run(&mut self,start_non_term:&'g str,) -> Result<(),GrammarWalkError<'g>> {
        //
        self.init(start_non_term);

        //
        let mut result: Result<(), GrammarWalkError<'g>>=Ok(());

        //
        while let Some(cur)=self.stk.pop() {
           if let Err(e)=self.step(cur) {
                if self.debug {
                    match e {
                        GrammarWalkError::RecursiveNonTerm(t) => {
                            println!("Recursive NonTerm {t:?}, At {}",self.expected_loc);
                        }
                        GrammarWalkError::MissingNonTerm(t) => {
                            println!("Missing NonTerm {t:?}, At {}",self.expected_loc);
                        }
                        GrammarWalkError::FailedParse => {
                            println!("Failed parse, At {}, expected {:?}",self.expected_loc,self.expecteds_string());
                        }
                        GrammarWalkError::Unfinished =>{}
                    }
                }

                result=Err(e);
                break;
           }
        }

        //
        if self.debug {
            println!("groups={:?}",self.groups);
        }

        //
        if !result.is_err() && !self.primitives_remaining.is_empty() {
            if self.debug {
                // println!("error, failed to parse all tokens {:?}",self.primitives_remaining);
                println!("error, failed to parse all tokens, at {}",self.expected_loc);
                println!("{:?}",self.expecteds); //self.expected.1 should be empty?
            }

            //
            if self.expecteds.is_empty() {
                result=Err(GrammarWalkError::Unfinished);
            } else {
                result=Err(GrammarWalkError::FailedParse);
            }

            //need to store grammar that was traversed ...
        } else {
            if self.debug {
                println!("parsed ok");
            }
        }

        //
        if self.debug {
            println!("===a {}",self.primitives_remaining.is_empty());
        }

        //
        if self.debug {
            for (i,g) in self.groups.iter().enumerate() {
                println!("g{i}: {:?} {:?}",g.name,g.tokens);
            }
        }

        //
        if self.debug {
            println!("top_primitives={:?}", self.top_primitives );
        }

        //
        result
    }

    fn step(&mut self,cur:Work<'t,'g>) -> Result<(),GrammarWalkError<'g>> {
        //
        if self.debug {
            if self.groups.len() != cur.group_len {
                println!("--- groups dif len, groups.len={}, cur.group_len={}",self.groups.len(),cur.group_len);
            }
            if self.hist_begins_stk.len() != cur.hist_begins_stk_len {
                println!("--- or_stk dif len, or_stk.len={}, cur.or_stk_len={}",self.hist_begins_stk.len(),cur.hist_begins_stk_len);
            }
        }

        //
        self.step_truncates(&cur);

        //
        if self.debug {
            self.c+=1;

            // // if c>30 {break;}
            // // println!(": {cur:?} || {} && {primitives:?}", self.stk.iter().rev().map(|x|format!("{:?}",x.0)).collect::<Vec<_>>().join(" << "), );

            {
                //
                let groups=&self.groups;

                //
                let c=self.c;

                //
                let Work {
                    grammar, success_len, fail_len, tokens,
                    group_ind, group_len,
                    // expected,
                    and_id,

                    // visiteds,grammar_debug_len, from_user,
                    hist_news_len,

                    is_first,
                    hist_begins_stk_len,
                    hist_ends_stk_len,
                    ..
                }=&cur;

                //
                let ps=tokens.inds();
                // let expected=if expected.id==0 {"None".to_string()}else{format!("{}:{}",expected.id,expected.name)};

                let temp_groups=groups.iter().enumerate().map(|(i,x)|format!("g{i}:p{}:{}",x.parent,x.name)).collect::<Vec<_>>();

                let groups_len2=groups.len();

                //
                println!("=>{c:4}: {grammar:?}, ps={ps:?}, success={success_len}, fail={fail_len}, ",);
                println!("        and_id={and_id}, groups.len={groups_len2}, group_ind={group_ind}, group_len={group_len}, gs={temp_groups:?}",);

                println!("        first={is_first}, hist_news_len={hist_news_len}, hist_begins_stk_len={hist_begins_stk_len}:{}, hist_ends_stk_len={hist_ends_stk_len}:{}, ",
                    self.hist_begins_stk.last().map(|x|x.elements.len()).unwrap_or_default(),
                    self.hist_ends_stk.last().map(|x|x.elements.len()).unwrap_or_default(),
                ); //hist_begins_stk_last_len={}, hist_ends_stk_last_len={}

                if true {
                    println!("        hist_news={}",
                        self.hist_news.iter().map(|x|x.grammar.clone())
                            .map(|x|format!("{x:?}"))
                            .map(|x|{let mut s=x; s.retain(|y|!['"',' '].contains(&y));s})
                            .collect::<Vec<_>>().join(", ")
                    );
                    println!("        hist_begins_last={}",
                        self.hist_begins_stk.last().map(|q|format!("{}",q.elements.iter()
                            .map(|x|{
                                format!("{}:{:?}",
                                    {let mut s=format!("{:?}",x.0.clone()); s.retain(|y|!['"',' '].contains(&y));s},
                                    x.1.tokens_after.inds(),
                                )
                            }).collect::<Vec<_>>().join(", ")
                        )).unwrap_or_default(),
                    );
                    println!("        hist_ends_last={}",
                        self.hist_ends_stk.last().map(|q|format!("{}",q.elements.iter()
                            .map(|x|{
                                format!("{}:{:?}",
                                    {let mut s=format!("{:?}",x.0.clone()); s.retain(|y|!['"',' '].contains(&y));s},
                                    x.1.tokens.inds(),
                                )
                            }).collect::<Vec<_>>().join(", ")
                        )).unwrap_or_default(),
                    );
                }

                //
                // println!("        expecteds {} : = {}", self.expected_loc,self.expecteds_string());
                println!("        tokens {tokens:?}");
            }

            //
            if false {
                for (i,Work {
                    grammar:g, success_len:s, fail_len:f, tokens, group_ind, group_len,
                    and_id,

                    // visiteds, grammar_debug_len, expected,
                    // from_user,
                    // hist_news_len,
                    // is_first,
                    // hist_begins_stk_len,
                    // hist_ends_stk_len,
                    ..
                }) in self.stk.iter()
                    // .rev()
                    .enumerate()
                {
                    //
                    println!("    {i:3}: ps={:?}, success={s}, fail={f}, and_id={and_id}, group_ind={group_ind}, group_len={group_len}, {g:?},",tokens.inds());
                }
            }
        }

        //
        if cur.group_ind>=self.groups.len() {
            panic!("invalid group_ind={}, groups_len={}",cur.group_ind,self.groups.len());
        }

        //try take from hist begins
        if self.grammar_try_from_hist_begins(&cur) {return Ok(());}

        //
        match cur.grammar.clone() {
            GrammarNode::Expected(priority, name, g) => {self.grammar_expect(cur, priority, name, g);}
            GrammarNode::Prev(g) => {self.grammar_prev(cur, g);}
            GrammarNode::Group(name, g) => {self.grammar_group(cur, name, g);}
            GrammarNode::And(gs) => {self.grammar_and(cur, gs);}
            GrammarNode::Or(gs) => {self.grammar_or(cur, gs);}
            GrammarNode::Opt(g) => {self.grammar_opt(cur,g);}
            GrammarNode::Many(g) => {self.grammar_many(cur,g);}
            GrammarNode::NonTerm(t) => {self.grammar_non_term(cur, t)?;}
            GrammarNode::Always => {self.grammar_always(cur);}
            GrammarNode::Error(e) => {return Err(self.grammar_error(cur, e));}

            GrammarNode::String => {self.grammar_string(cur);}
            GrammarNode::Identifier => {self.grammar_identifier(cur);}
            GrammarNode::Int => {self.grammar_int(cur);}
            GrammarNode::Float => {self.grammar_float(cur);}
            GrammarNode::Symbol(s) => {self.grammar_symbol(cur,s);}
            GrammarNode::Keyword(s) => {self.grammar_keyword(cur,s);}
            GrammarNode::Eol => {self.grammar_eol(cur);}
        }

        //
        Ok(())
    }

    //
    pub fn set_debug(&mut self,debug:bool) {
        self.debug=debug;
    }
}