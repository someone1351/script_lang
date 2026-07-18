

use super::error::*;
use super::temp_data::*;
use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;

use crate::build::Loc;
// use crate::clike::tokenizer::TokenContainer;
use super::super::grammar::data::{Walk,WalkGroup};
use super::super::tokenizer::{TokenIterContainer, ValueContainer};

use super::node::*;


// use data::*;
// use error::*;


pub struct GrammarWalker<'t,'g,G>
where
    G: Fn(&str)->Option<GrammarNode<'g>>,
{
    prev_non_term_only:bool,
    stow_non_term_only:bool,

    top_primitives:TokenIterContainer<'t>,
    primitives_remaining: TokenIterContainer<'t>,
    grammar_func:G,
    stk: Vec<Work<'t,'g>>,
    c:usize,
    expected_loc:Loc,
    expected_news:Vec<TempExpectedNew<'g>>,
    expecteds:Vec<TempExpected<'g>>,

    debug:bool,
    non_term_recursive_check:bool,

    groups:Vec<TempGroupInfo<'t,'g>>,

    hist_news:Vec<TempHistNew<'t,'g>>,

    //simpler to use hist_begins_stk:Vec<Vec<TempHistBegin<'t,'g>>>
    //and maybe don't truncate it, instead use lens  to keep it
    // hist_begins_stk:Vec<Range<usize>>,
    // hist_begins_elements:Vec<TempHistBegin<'t,'g>>,
    // hist_begins_stk:Vec<TempHistBegins<'t,'g>>,

    hist_begins:Vec<TempHistBegin2<'t,'g>>,
    hist_begins_groups:Vec<TempGroupInfo<'t,'g>>,
    hist_begins_prevs:Vec<TempHistEnd<'g>>,

    //
    hist_ends:Vec<TempHistEnd<'g>>,

    // hist_begins_stk:Vec<TempHistBegins<'t,'g>>,
    // hist_ends_stk:Vec<TempHistEnds<'g>>,
}

impl<'t,'g,G> GrammarWalker<'t,'g,G>
where
    G: Fn(&str)->Option<GrammarNode<'g>>,
{

    pub fn new(top_primitives:TokenIterContainer<'t>, grammar_func:G,) -> Self {
        Self {
            prev_non_term_only:true,
            stow_non_term_only:true,

            stk:Default::default(),
            c:Default::default(),

            expected_loc:Loc::zero(),
            expected_news:Default::default(),
            expecteds:Default::default(),

            grammar_func,
            primitives_remaining:top_primitives.clone(),
            top_primitives,
            debug:false,
            non_term_recursive_check:true,

            groups:Default::default(),

            hist_news: Default::default(),

            // hist_begins_stk:Default::default(),
            hist_begins:Default::default(),
            hist_begins_groups:Default::default(),
            hist_begins_prevs:Default::default(),
            // hist_begins_elements: Default::default(),
            hist_ends: Default::default(),

            // hist_begins_stk: Default::default(),
            // hist_ends_stk: Default::default(),
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
            and_id: 0,

            from_user:false,
            or_first:true,

            hist_news_len:0,


            // hist_begins_stk_len:0,
            // hist_ends_stk_len:1,

            // hist_begins_stk_len:0,

            // hist_begins_ind: 0,
            hist_begins_len: 0,

            // in_or:false,
            can_hist_begin:false,

            hist_ends_ind: 0,
            hist_ends_len: 0,

            expected_news_len:0,
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
            and_id: 0,

            from_user:true,
            or_first:true,

            // in_or:false,
            can_hist_begin:false,

            hist_news_len:0,


            // hist_begins_stk_len:0,
            // hist_ends_stk_len:1,

            // hist_begins_stk_len:0,

            // hist_begins_ind: 0,
            hist_begins_len: 0,

            hist_ends_ind: 0,
            hist_ends_len: 0,

            expected_news_len:0,
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
                and_id: 0,

                from_user:true,
                or_first:true,

                // in_or:false,
                can_hist_begin:false,

                hist_news_len:0,
                // hist_begins_stk_len:0,
                // hist_ends_stk_len:1,

                // hist_begins_stk_len:0,

                // hist_begins_ind: 0,
                hist_begins_len: 0,

                hist_ends_ind: 0,
                hist_ends_len: 0,

                expected_news_len:0,
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

        // self.hist_begins_stk.clear();
        self.hist_begins.clear();
        self.hist_begins_groups.clear();
        self.hist_begins_prevs.clear();

        // self.hist_begins_elements.clear();
        self.hist_ends.clear();

        // self.hist_begins_stk.clear(); //don't need initial one because don't need to store begins before an Or exists
        // // self.hist_ends_stk.clear();
        // self.hist_ends_stk=vec![Default::default()]; //need an initial one because require ends regardless of an Or existing

        //
        self.c=0;
        self.expected_loc=Loc::zero();
        self.expected_news.clear();
        self.expecteds.clear();

    }

    fn grammar_expect(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Expected(g,_, )=cur.grammar.clone() else{panic!("");};

        //
        let expected_news_len=self.add_expected_new(&cur);
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
            and_id:cur.and_id,

            from_user:true,
            or_first:cur.or_first,

            // in_or:cur.in_or,
            can_hist_begin:false,

            hist_news_len,
            // hist_begins_stk_len:cur.hist_begins_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_begins_ind: cur.hist_begins_ind,
            hist_begins_len: cur.hist_begins_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,

            hist_ends_ind: cur.hist_ends_ind,
            hist_ends_len: cur.hist_ends_len,

            expected_news_len,
        });
    }

    fn grammar_group(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Group(g,_, )=cur.grammar.clone() else{panic!("");};

        //
        let (group_ind,group_len)=self.new_group(&cur); //name, cur.group_ind, cur.tokens
        let hist_news_len=self.hist_news_add(&cur);

        //
        self.stk.push(Work {
            grammar: *g,
            success_len: cur.success_len,
            fail_len: cur.fail_len,
            tokens: cur.tokens,
            group_ind,
            group_len,
            visiteds:cur.visiteds,
            grammar_debug_len: cur.grammar_debug_len+1,
            and_id:cur.and_id,

            from_user:true,
            or_first:cur.or_first,

            can_hist_begin:false,
            hist_news_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_begins_ind: cur.hist_begins_ind,
            hist_begins_len: cur.hist_begins_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,

            hist_ends_ind: cur.hist_ends_ind,
            hist_ends_len: cur.hist_ends_len,

            expected_news_len:cur.expected_news_len,
        });
    }

    fn grammar_many(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Many(g)=cur.grammar.clone() else{panic!("");};

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
            and_id:cur.and_id,

            from_user:false,
            or_first:cur.or_first,
            can_hist_begin:false,

            hist_news_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_begins_ind: cur.hist_begins_ind,
            hist_begins_len: cur.hist_begins_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,

            hist_ends_ind: cur.hist_ends_ind,
            hist_ends_len: cur.hist_ends_len,

            expected_news_len:cur.expected_news_len,
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
            and_id:cur.and_id,

            from_user:false,
            or_first:cur.or_first,
            can_hist_begin:false,

            hist_news_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_begins_ind: cur.hist_begins_ind,
            hist_begins_len: cur.hist_begins_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,

            hist_ends_ind: cur.hist_ends_ind,
            hist_ends_len: cur.hist_ends_len,

            expected_news_len:cur.expected_news_len,
        });

        //
        let fail_len=self.stk.len();

        //
        self.stk.push(Work {
            grammar: *g.clone(),
            success_len: success_len2,
            fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds:cur.visiteds,
            grammar_debug_len: cur.grammar_debug_len+1,
            and_id:cur.and_id,

            from_user:true,
            or_first:cur.or_first,
            can_hist_begin:false,

            hist_news_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_begins_ind: cur.hist_begins_ind,
            hist_begins_len: cur.hist_begins_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,

            hist_ends_ind: cur.hist_ends_ind,
            hist_ends_len: cur.hist_ends_len,

            expected_news_len:cur.expected_news_len,
        });
    }

    fn grammar_non_term(&mut self,cur :Work<'t,'g>,) -> Result<(),GrammarWalkError<'g>>{
        let GrammarNode::NonTerm(t)=cur.grammar.clone() else{panic!("");};

        //
        let hist_news_len=self.hist_news_add(&cur);
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
            and_id:cur.and_id,

            from_user:true,
            or_first:cur.or_first,
            can_hist_begin:false,

            hist_news_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_begins_ind: cur.hist_begins_ind,
            hist_begins_len: cur.hist_begins_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,

            hist_ends_ind: cur.hist_ends_ind,
            hist_ends_len: cur.hist_ends_len,

            expected_news_len:cur.expected_news_len,
        });

        Ok(())
    }

    fn grammar_error(&mut self,cur :Work<'t,'g>,) -> GrammarWalkError<'g> {
        let GrammarNode::Error(e)=cur.grammar.clone() else{panic!("");};

        if self.debug {
            println!("====error {:?} ",self.expected_loc,); //self.expecteds,
        }

        //necesaary? any point to it?
        // if self.expecteds.is_empty() { // self.expected.0.is_zero()
        //     self.expected_loc=cur.primitives.loc();
        // }

        //
        self.update_tokens(&cur,false); //could be true, but would do nothing

        //
        // self.expect_news_drain(&cur); //necessary here? no since it is finishing here?

        //
        return e;
    }

    fn grammar_and(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::And(gs)=&cur.grammar else{panic!("");};
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
                and_id:cur.and_id+1,

                from_user:false,
                or_first:false,
                can_hist_begin:false,

                hist_news_len,

                // hist_begins_stk_len:cur.hist_begins_stk_len,
                // hist_ends_stk_len:cur.hist_ends_stk_len,

                // hist_begins_ind: cur.hist_begins_ind,
                hist_begins_len: cur.hist_begins_len,

                // hist_begins_stk_len:cur.hist_begins_stk_len,

                hist_ends_ind: cur.hist_ends_ind,
                hist_ends_len: cur.hist_ends_len,

                expected_news_len:cur.expected_news_len,
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

            // and_id:cur.and_id+1,
            and_id:if gs.len()==1{cur.and_id}else{cur.and_id+1}, //don't need if single element in And?

            from_user:true,
            or_first:cur.or_first, //cur.from_user &&  //only want to know about grammars added by user, not the walker, could check from_user elsewhere,
            can_hist_begin:cur.or_first,

            hist_news_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_begins_ind: cur.hist_begins_ind,
            hist_begins_len: cur.hist_begins_len,

            // hist_begins_stk_len:cur.hist_begins_stk_len,

            hist_ends_ind: cur.hist_ends_ind,
            hist_ends_len: cur.hist_ends_len,

            expected_news_len:cur.expected_news_len,
        });
    }

    fn grammar_or(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Or(gs)=&cur.grammar else{panic!("");};

        //
        let Some(g_first)=gs.first().cloned() else { return; };

        //
        let hist_news_len=self.hist_news_add(&cur);
        // let hist_begins_stk_len=self.hist_begins_stk_push(&cur);
        let hist_begins_len=self.hist_begins_push(&cur);
        // let hist_ends_stk_len=self.hist_ends_stk_push(&cur);
        // let hist_begins_ind=if !cur.is_first{cur.hist_begins_len}else{cur.hist_begins_ind};


        // let hist_ends_ind=if cur.is_first{}else{};

        //
        if let Some(g_rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
            self.stk.push(Work {
                grammar: GrammarNode::Or(g_rest.into()),
                success_len: cur.success_len,
                fail_len: cur.fail_len,
                tokens: cur.tokens,
                group_ind: cur.group_ind,
                group_len: cur.group_len,
                visiteds:cur.visiteds.clone(),
                grammar_debug_len: cur.grammar_debug_len,
                and_id:cur.and_id,

                from_user:false,
                or_first:cur.or_first,
                can_hist_begin:false,

                hist_news_len,

                // hist_begins_stk_len,
                // hist_ends_stk_len,

                // hist_begins_ind,
                // hist_begins_len: cur.hist_begins_len,

                // hist_begins_stk_len,
                hist_begins_len,

                hist_ends_ind: cur.hist_ends_ind,
                hist_ends_len: cur.hist_ends_len,

                expected_news_len:cur.expected_news_len,
            });
        }

        //
        let fail_len=if gs.len()>1 {self.stk.len()}else{cur.fail_len};

        //
        self.stk.push(Work {
            grammar: g_first,
            success_len: cur.success_len,
            fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            visiteds:cur.visiteds,
            grammar_debug_len: cur.grammar_debug_len+1,
            and_id:cur.and_id,

            from_user:true,
            or_first:cur.or_first,
            can_hist_begin:false,

            hist_news_len,

            // hist_begins_stk_len,
            hist_begins_len,
            // hist_ends_stk_len,

            // hist_begins_ind,
            // hist_begins_len: cur.hist_begins_len,
            hist_ends_ind: cur.hist_ends_ind,
            hist_ends_len: cur.hist_ends_len,

            expected_news_len:cur.expected_news_len,
        });
    }


    // fn grammar_stow(&mut self,cur :Work<'t,'g>,) {
    //     let GrammarNode::Stow(g)=&cur.grammar else {panic!("");};




    //     // //
    //     // let hist_news_len=self.hist_news_add(&cur);

    //     // //
    //     // self.stk.push(Work {
    //     //     grammar: *g.clone(),
    //     //     success_len: cur.success_len,
    //     //     fail_len: cur.fail_len,
    //     //     tokens: cur.tokens,
    //     //     group_ind:cur.group_ind,
    //     //     group_len:cur.group_len,
    //     //     visiteds:cur.visiteds,
    //     //     grammar_debug_len: cur.grammar_debug_len+1,
    //     //     and_id:cur.and_id,

    //     //     from_user:true,
    //     //     is_first:cur.is_first,

    //     //     hist_news_len,
    //     //     hist_begins_stk_len:cur.hist_begins_stk_len,
    //     //     hist_ends_stk_len:cur.hist_ends_stk_len,

    //     //     expected_news_len:cur.expected_news_len,
    //     // });
    // }

    fn grammar_prev(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Prev(g)=&cur.grammar else {panic!("");};
        //
        let _hist_news_len=self.hist_news_add(&cur);
        // self.hist_begins_clear(&cur);


        //
        if
            // self.hist_ends_stk.last().unwrap().elements
            //     .iter().find(|x|x.grammar.eq(g)).is_some()
            //     // .contains_key(&g)
            self.hist_ends[cur.hist_ends_ind..].iter().find(|x|x.grammar.eq(g)).is_some()
        {
            self.stk.truncate(cur.success_len);
            self.handle_exit_last_many(&cur);
            // self.hist_news_truncate_to_last(); //why on success??
            self.update_tokens(&cur,true);
            self.update_groups(&cur);
            self.submit_hist_news(&cur,true,false);
            self.revert_last_expected_news();
        } else {
            self.stk.truncate(cur.fail_len);
            self.update_tokens(&cur,false);
            // self.revert_last_hist_news();
            self.update_hist_on_fail(&cur);
            let _expected_news_len=self.add_expected_new(&cur);
            self.submit_expected_news(&cur);
        }
    }

    fn grammar_always(&mut self,cur :Work<'t,'g>,) {
        self.stk.truncate(cur.success_len);
        let _hist_news_len=self.hist_news_add(&cur);
        // self.hist_begins_clear(&cur);
        self.handle_exit_last_many(&cur);
        self.update_tokens(&cur,true);
        self.update_groups(&cur); //here
        self.submit_hist_news(&cur,true,false);
        self.revert_last_expected_news();
    }

    fn grammar_try_from_hist_begins(&mut self,cur :&Work<'t,'g>) -> bool {
        //
        if !cur.from_user || !cur.or_first {return false;} // !(cur.from_user && cur.is_first)
        // if cur.hist_begins_stk_len==0 {return false;}
        if cur.hist_begins_len==0 {return false;}

        // // let Some(hist_begins)=self.hist_begins_stk.last() else {return false;};
        // let hist_begins=&self.hist_begins_stk[cur.hist_begins_stk_len-1];

        // let Some(hist_begin)=
        //     // hist_begins.elements
        //     // // .get(&cur.grammar)
        //     // .iter().rev().find(|x|x.grammar.eq(&cur.grammar))
        //     // self.hist_begins_elements[cur.hist_begins_ind..]
        //     hist_begins.elements.iter().rev().find(|x|x.grammar.eq(&cur.grammar))
        //     else {return false;};

        // let hist_begins=&self.hist_begins_stk[cur.hist_begins_stk_len-1];

        let hist_begin=&self.hist_begins[cur.hist_begins_len-1];
        let Some(hist_begin_val)=&hist_begin.val else {return false;};

        //
        if hist_begin_val.grammar!=cur.grammar {
            return false;
        }

        //
        self.stk.truncate(cur.success_len);

        // println!("=====gr {:?}, {:?}",hist_begin.inner_groups_range.clone(),hist_begins.inner_groups);
        // let groups=&hist_begins.inner_groups[hist_begin.inner_groups_range.clone()].to_vec();

        let temp_groups_end=hist_begin.val.as_ref().map(|x|x.temp_groups_end).unwrap_or(hist_begin.temp_groups_start);
        let temp_prevs_end=hist_begin.val.as_ref().map(|x|x.temp_prevs_end).unwrap_or(hist_begin.temp_prevs_start);

        let stow_groups=&self.hist_begins_groups[hist_begin.temp_groups_start .. temp_groups_end];
        let stow_prevs=&self.hist_begins_prevs[hist_begin.temp_prevs_start..temp_prevs_end];

        //
        //add groups
        self.groups.extend(stow_groups.iter().map(|g|TempGroupInfo{ parent:cur.group_ind+g.parent,..g.clone()}));
        self.hist_ends.extend_from_slice(stow_prevs);

        //
        let cur=Work {
            group_len:self.groups.len(),
            tokens:hist_begin_val.tokens_after,
            // hist_ends_stk_len:todo!(),
            ..cur.clone()
        };

        //
        self.update_tokens(&cur,true);
        self.update_groups(&cur);
        self.submit_hist_news(&cur,false, false); //not needed? no.. if And(Z,Or(And(X,Y),X)), then will add that
        self.revert_last_expected_news();

        //
        if self.debug {
            println!("---- grabbed from or {:?},",cur.grammar);
        }

        //
        true
    }

    fn grammar_primitive<Q,P>(&mut self,mut cur:Work<'t,'g>,prim_func:Q) -> Option<ValueContainer<'t,P>>
    where
        P:Clone,
        Q:Fn(&mut TokenIterContainer<'t>)->Result<ValueContainer<'t,P>,Loc>,
    {
        //
        let _hist_news_len=self.hist_news_add(&cur);
        // self.hist_begins_clear(&cur);

        //
        match prim_func(&mut cur.tokens) {
            Ok(v) => {
                //
                self.stk.truncate(cur.success_len);
                self.update_tokens(&cur,true);
                self.update_groups(&cur);
                self.submit_hist_news(&cur,true,true);
                self.revert_last_expected_news();

                //
                if self.debug {
                    println!("--- hmm stk={:?}",self.stk.iter().map(|x|x.grammar.clone()).collect::<Vec<_>>());

                    // if let Some(last)=self.stk.last() {
                    //     let last_hist_ends=&self.hist_ends_stk[last.hist_ends_stk_len-1].elements;
                    //     println!("---last_hist_ends={:?}, len={}",last_hist_ends,last_hist_ends.len());
                    // }
                }

                //
                Some(v)
            }
            Err(loc) => {
                self.stk.truncate(cur.fail_len);
                self.update_tokens(&cur,false);
                // self.revert_last_hist_news();
                self.update_hist_on_fail(&cur);
                let _expected_news_len=self.add_expected_new(&cur);
                self.submit_expected_news(&cur);

                //
                None
            }
        }
    }

    fn add_expected_new(&mut self, cur:&Work<'t,'g>,) -> usize {
        let expected_type=match cur.grammar {
            GrammarNode::Expected(_, name) => TempExpectedType::Expected(name),
            GrammarNode::Prev(_) => TempExpectedType::Prev,
            GrammarNode::String => TempExpectedType::String,
            GrammarNode::Identifier => TempExpectedType::Identifier,
            GrammarNode::Int => TempExpectedType::Int,
            GrammarNode::Float => TempExpectedType::Float,
            GrammarNode::Symbol(s) => TempExpectedType::Symbol(s),
            GrammarNode::Keyword(s) => TempExpectedType::Keyword(s),
            GrammarNode::Eol => TempExpectedType::Eol,
           _ => {panic!("");}
        };

        self.expected_news.push(TempExpectedNew { expected_type, });
        self.expected_news.len()
    }

    fn submit_expected_news(&mut self, cur:&Work<'t,'g>,) {
        let Some(last)=self.stk.last_mut() else {panic!("");};
        let drained_expected_news=self.expected_news.drain(last.expected_news_len ..).collect::<Vec<_>>();

        //
        for drained_expected_new in drained_expected_news {
            self.expecteds.push(TempExpected { expect_type: drained_expected_new.expected_type });
        }
    }

    fn revert_last_expected_news(&mut self) {
        let Some(last)=self.stk.last() else {return;};
        self.expected_news.truncate(last.expected_news_len);
    }

    fn submit_hist_news(&mut self,
        cur:&Work<'t,'g>,
        //what was this for again? something to do with not adding cur grammar to hist_begins?
        //  it was for not adding cur grammar to hist_new?
        _gotten:bool,
        _hist_ends_remove_previous:bool,
    ) {

        //should always be some (due to init), use panic instead of ret? no, it will end on an always if successful
        let Some(last)=self.stk.last_mut() else {return;};

        //
        // last.hist_ends_stk_len=cur.hist_ends_stk_len;

        //
        last.hist_ends_len=cur.hist_ends_len;

        // //why only use for primitives?
        // // if hist_ends_remove_previous {
        // //     let last_tokens_start=last.tokens.inds().start;

        // //     self.hist_ends_stk[last.hist_ends_stk_len-1].elements.retain(|_k,v|{
        // //         //could be removed later, use dif way, through and/or ids ?
        // //         // v.tokens.inds().start
        // //         v.tokens_start_ind
        // //         >=last_tokens_start
        // //         // last_tokens_start<v.tokens.inds().start
        // //     });
        // // }

        //whipe previous hist
        if cur.from_user { //why only if from_user??
            // self.hist_ends_stk[last.hist_ends_stk_len-1].elements.clear();
            self.hist_ends.truncate(cur.hist_ends_ind);
        }

        //
        if self.hist_news.len()==last.hist_news_len { panic!(""); } //should always be some

        //
        let drained_hist_news=self.hist_news.drain(last.hist_news_len ..).collect::<Vec<_>>();

        //
        let added_hist_ends=drained_hist_news.iter().map(|hist_new|{
            //
            let tokens_len=hist_new.tokens_start.len()-cur.tokens.len();
            let tokens=hist_new.tokens_start.get_amount(tokens_len).unwrap();

            //
            if self.debug {
                println!("--- inserting hist_end {:?} {tokens:?}",hist_new.grammar);
            }

            // // //
            // // (  hist_new.grammar.clone(), TempHistEnd {
            // //     tokens_start_ind:tokens.start,
            // //     // tokens,
            // // } )

            TempHistEnd {
                grammar:hist_new.grammar.clone(),
                tokens_start_ind:tokens.start,
                // tokens,
            }
        }).collect::<Vec<_>>();

        //add hist begins
        if cur.hist_begins_len!=0 {

            //
            let drained_hist_new2=drained_hist_news.iter().find(|x|{
                let b=x.grammar.is_always() || x.grammar.is_prev() || x.grammar.is_primtive();
                println!("---hm? {:?} {b} {}",x.grammar,x.is_first);
                !b && x.is_first
            });

            println!("-------found {:?} : {:?}",drained_hist_new2,drained_hist_news.iter().map(|x|&x.grammar).collect::<Vec<_>>());
            //
            if let Some(drained_hist_new2)=drained_hist_new2 {

                //
                let hist_begin=self.hist_begins.last_mut().unwrap();

                //
                self.hist_begins_prevs.truncate(hist_begin.temp_prevs_start);
                self.hist_begins_prevs.extend(added_hist_ends.iter().cloned());

                //
                self.hist_begins_groups.truncate(hist_begin.temp_groups_start);
                self.hist_begins_groups.extend_from_slice(&self.groups[drained_hist_new2.group_len..cur.group_len]);

                //
                hist_begin.val=Some(TempHistStowVal {
                    grammar: drained_hist_new2.grammar.clone(),
                    tokens_after: cur.tokens,
                    temp_groups_end: self.hist_begins_groups.len(),
                    temp_prevs_end: self.hist_begins_prevs.len(),
                })
            }


        }

        //
        // self.hist_ends_stk[last.hist_ends_stk_len-1].elements.extend(added_hist_ends.into_iter());

        //
        self.hist_ends.extend(added_hist_ends.into_iter());

        //

        // last.hist_begins_len=self.hist_begins_elements.len();
        last.hist_ends_len=self.hist_ends.len();
    }


    fn update_hist_on_fail(&mut self,cur:&Work<'t,'g>,) {
        let Some(last)=self.stk.last_mut() else {panic!("");};

        // last.hist_begins_len=cur.hist_begins_len;
    }
    // fn revert_last_hist_news(&mut self) {
    //     let Some(last)=self.stk.last() else {panic!("");};
    //     self.hist_news.truncate(last.hist_news_len); //what is it for? clears failed takeable_starts?
    // }
fn hist_begins_push(&mut self,cur:&Work<'t,'g>) -> usize {
        if cur.from_user //so not an added OR for rest,
            && ( !cur.or_first || //not part of current OR, eg: or(A, and(B,or(C,D))) A in dif OR stk than C,D
            // self.hist_begins_stk.is_empty()
            cur.hist_begins_len==0 //init first, for if all part of same OR stk, eg: or(A,or(B,C))
            //if not need to init first, then it just reuses existing one
        ) //add current/initial OR
        {
            println!("------ hist_begins_push");
            self.hist_begins.push(TempHistBegin2 {
                val: None,
                temp_groups_start: self.hist_begins_groups.len(),
                temp_prevs_start: self.hist_begins_prevs.len(),
            });

            if self.hist_begins.len()!=cur.hist_begins_len+1 {
                panic!("");
            }

        }

        self.hist_begins.len()
    }
    // fn hist_begins_stk_push(&mut self,cur:&Work<'t,'g>) -> usize {
    //     if cur.from_user //so not an added OR for rest,
    //         && ( !cur.or_first || //not part of current OR, eg: or(A, and(B,or(C,D))) A in dif OR stk than C,D
    //         // self.hist_begins_stk.is_empty()
    //         cur.hist_begins_stk_len==0 //init first, for if all part of same OR stk, eg: or(A,or(B,C))
    //         //if not need to init first, then it just reuses existing one
    //     ) //add current/initial OR
    //     {
    //         println!("------ hist_begins_stk_push");
    //         if self.hist_begins_stk.len()<cur.hist_begins_stk_len+1 { //make room
    //             if cur.hist_begins_stk_len-self.hist_begins_stk.len() > 1 {panic!("");} //shouldn't happen
    //             self.hist_begins_stk.push(Default::default());
    //         } else { //clear existing room

    //         }

    //         //
    //         println!("--=-=-=-= {:?} {:?}",
    //             self.hist_begins_stk.len(),
    //             cur.hist_begins_stk_len,

    //         );

    //         //

    //         let hist_begins=&mut self.hist_begins_stk[cur.hist_begins_stk_len];

    //         hist_begins.group_len=cur.group_len;

    //         //

    //         hist_begins.elements.clear();
    //         hist_begins.inner_groups.clear();
    //         hist_begins.hist_ends.clear();
    //         hist_begins.group_ind=cur.group_ind;
    //         hist_begins.last_group_len=hist_begins.group_len;

    //         //
    //         // self.hist_begins_clear(cur);

    //         //
    //         // let hist_begins=&mut self.hist_begins_stk[cur.hist_begins_stk_len];
    //         // hist_begins.elements.clear();
    //         // hist_begins.inner_groups.clear();
    //         // hist_begins.hist_ends.clear();
    //         // hist_begins.group_ind=cur.group_ind;
    //         // hist_begins.group_len=cur.group_len;
    //         // hist_begins.last_group_len=cur.group_len;
    //     }

    //     self.hist_begins_stk.len()
    // }

    // fn hist_begins_clear(&mut self,cur:&Work<'t,'g>) {

    //     if cur.hist_begins_stk_len==0
    //         // || !cur.from_user //dont care, eg or(Y,opt(X)) even if X fails, throw away anything from inside Y or even X
    //         || !cur.or_first
    //     {
    //         return;
    //     }

    //         println!("------ hist_begins_clear");

    //     // if !(cur.grammar.is_primtive() || cur.grammar.is_always() || cur.grammar.is_prev()) { //just call from these places
    //     //     return;
    //     // }

    //     // self.hist_begins_stk[cur.hist_begins_stk_len-1].clear();

    //     let hist_begins=&mut self.hist_begins_stk[cur.hist_begins_stk_len-1];

    //     hist_begins.elements.clear();
    //     hist_begins.inner_groups.clear();
    //     hist_begins.hist_ends.clear();
    //     hist_begins.group_ind=cur.group_ind;
    //     hist_begins.last_group_len=hist_begins.group_len;
    // }

    // fn hist_begins_stk_push(&mut self,cur:&Work<'t,'g>) -> usize {

    //     if  self.hist_begins_stk.is_empty() //reason? shouldn't it not be empty, as would be created by first? there was a reason ...
    //         || !cur.is_first //don't need to use cur.from_user, first is better, as could handle potential Or not added by user
    //     {
    //         self.hist_begins_stk.push(Default::default());
    //     }

    //     self.hist_begins_stk.len()

    // }
    // fn hist_ends_stk_push(&mut self,cur:&Work<'t,'g>) -> usize {
    //     if cur.is_first {
    //         self.hist_ends_stk.push(self.hist_ends_stk.last().cloned().unwrap());
    //     }

    //     self.hist_ends_stk.len()
    // }

    fn hist_news_add(&mut self,cur:&Work<'t,'g>) -> usize {
        if cur.from_user
            // // && (cur.grammar.is_primtive() || cur.grammar.is_non_term())
            // && cur.grammar.is_non_term() //should only do nonterms?
        { //ignore grammars added by walker
            self.hist_news.push(TempHistNew {
                grammar: cur.grammar.clone(),
                tokens_start: cur.tokens.clone(),
                group_ind: cur.group_ind,
                group_len:cur.group_len,
                is_first:cur.or_first,
            });
        }

        self.hist_news.len()
    }

    fn step_truncates(&mut self,cur :&Work<'t,'g>) {
        self.groups.truncate(cur.group_len);
        self.hist_news.truncate(cur.hist_news_len);

        // self.hist_begins_stk.truncate(cur.hist_begins_stk_len);
        // self.hist_ends_stk.truncate(cur.hist_ends_stk_len);

        // self.hist_begins_elements.truncate(cur.hist_begins_len);

        //
        self.hist_ends.truncate(cur.hist_ends_len);

        //
        self.hist_begins.truncate(cur.hist_begins_len);

        //
        if let Some(hist_begin)=self.hist_begins.last() {
            let (groups_len,prevs_len)=if let Some(hist_begin_val)= &hist_begin.val {
                (hist_begin_val.temp_groups_end,hist_begin_val.temp_prevs_end)
            } else {
                (hist_begin.temp_groups_start,hist_begin.temp_prevs_start)
            };

            self.hist_begins_groups.truncate(groups_len);
            self.hist_begins_prevs.truncate(prevs_len);
        }
    }

    fn update_groups(&mut self,cur :&Work<'t,'g>,
        // cur_group_ind:usize,
        // cur_primitives:TokenIterContainer<'t>,
    ) {
        let Some(last)=self.stk.last_mut() else {return;};

        //
        last.group_len=cur.group_len;

        //
        if self.debug {
            println!("==do_groups_primitives_clamp: cur_group_ind={}, last.group_ind={}",cur.group_ind,last.group_ind);
        }

        //clamp groups tokens (for groups that have ended)
        let mut g=cur.group_ind;

        //
        while g>last.group_ind {
            let group=&mut self.groups[g];
            let n=group.tokens.len()-cur.tokens.len();
            let group_prims=group.tokens.get_amount(n).unwrap();

            //
            group.tokens=group_prims;
            g=group.parent;
        }

    }

    fn do_non_term_visiteds(&mut self,
        t:&'g str,
        cur_primitives:TokenIterContainer<'t>,
        cur_visiteds: HashSet<(&'g str, usize)>,
    ) -> Result<HashSet<(&'g str, usize)>,GrammarWalkError<'g>> {
        //
        if !self.non_term_recursive_check { return  Ok(Default::default()); }

        //
        let v=(t,cur_primitives.inds().start);

        //
        if cur_visiteds.contains(&v) { return Err(GrammarWalkError::RecursiveNonTerm(t)); }

        //
        let mut visiteds=cur_visiteds;
        visiteds.insert(v);

        //
        Ok(visiteds)
    }

    fn new_group(&mut self,cur:&Work<'t,'g>) -> (usize,usize) {
        let GrammarNode::Group(_,name)=cur.grammar else {panic!("");};
        let parent=cur.group_ind;
        let tokens=cur.tokens;

        let new_group_ind=self.groups.len();
        self.groups.push(TempGroupInfo { name, parent, tokens, });
        (new_group_ind,self.groups.len())
    }

    fn update_tokens(&mut self,cur:&Work<'t,'g>, set_last_tokens:bool) {
        if self.stk.is_empty() {
            self.primitives_remaining=cur.tokens;
        } else if set_last_tokens {
            let Some(last)=self.stk.last_mut() else {panic!("");};
            last.tokens=cur.tokens;
        }
    }

    fn handle_exit_last_many(&mut self,cur:&Work<'t,'g>) { //if not parsing anything, exit the many
        let Some(last)=self.stk.last_mut() else {return;};
        if !last.grammar.is_many() || last.tokens.len()!=cur.tokens.len() {return;}
        last.grammar=GrammarNode::Always;
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
                            println!("Failed parse, At {}, expected {:?}",self.expected_loc,"self.expecteds_string()");
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
                // println!("{:?}",self.expecteds); //self.expected.1 should be empty?
            }

            //
            // if self.expecteds.is_empty() {
            //     result=Err(GrammarWalkError::Unfinished);
            // } else
            {
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
            // if self.hist_begins_stk.len() != cur.hist_begins_stk_len {
            //     println!("--- or_stk dif len, or_stk.len={}, cur.or_stk_len={}",self.hist_begins_stk.len(),cur.hist_begins_stk_len);
            // }
        }

        //
        self.step_truncates(&cur);

        //
        if self.debug {
            self.c+=1;

            {
                //
                let groups=&self.groups;

                //
                let c=self.c;

                //
                let Work {
                    grammar, success_len, fail_len, tokens,
                    group_ind, group_len,and_id,or_first: is_first,
                    hist_news_len,
                    // hist_begins_stk_len,hist_ends_stk_len,
                    // hist_begins_ind,
                    hist_begins_len,
                    // hist_begins_stk_len,
                    hist_ends_ind,hist_ends_len,
                    ..
                }=&cur;

                //
                let ps=tokens.inds();
                let temp_groups=groups.iter().enumerate().map(|(i,x)|format!("g{i}:p{}:{}",x.parent,x.name)).collect::<Vec<_>>();
                let groups_len2=groups.len();

                //
                println!("=>{c:4}: {grammar:?}, ps={ps:?}, success={success_len}, fail={fail_len}, ",);
                println!("        and_id={and_id}, groups.len={groups_len2}, group_ind={group_ind}, group_len={group_len}, gs={temp_groups:?}",);

                // println!("        first={is_first}, hist_news_len={hist_news_len}, hist_begins_stk_len={hist_begins_stk_len}:{}, hist_ends_stk_len={hist_ends_stk_len}:{}, ",
                //     self.hist_begins_stk.last().map(|x|x.elements.len()).unwrap_or_default(),
                //     self.hist_ends_stk.last().map(|x|x.elements.len()).unwrap_or_default(),
                // );
                // let hist_begins_len=if *hist_begins_stk_len==0{None}else{
                //     self.hist_begins_stk.get(hist_begins_stk_len-1).map(|x|x.elements.len())
                // };
                println!("        first={is_first}, hist_news_len={hist_news_len}, hist_begins_len={hist_begins_len:?}, hist_ends_ind={hist_ends_ind}, hist_ends_len={hist_ends_len}",);
                // println!("        hist_begins_ind={hist_begins_ind}, hist_begins_len={hist_begins_len},",
                //     self.stk.get(cur.)
                // );
                if true {
                    println!("        hist_news");
                    for (i,h) in self.hist_news.iter().enumerate() {
                        println!("            {i}:[{:?}]: {:?}",h.tokens_start.inds(),h.grammar)
                    }

                    // println!("        hist_begins_last");
                    // if let Some(h)=self.hist_begins_stk.last() {
                    //     for (i,
                    //         // (g,x)
                    //         x
                    //         ) in h.elements.iter().enumerate() {
                    //         println!("            {i}:[{:?}]: {:?}",x.tokens_after.inds(),x.grammar)
                    //     }
                    // }
                    // println!("        hist_ends_last");
                    // if let Some(h)=self.hist_ends_stk.last() {
                    //     for (i,
                    //         // (g,x)
                    //         x
                    //     ) in h.elements.iter().enumerate() {
                    //         println!("            {i}:[{:?}]: {:?}",x.tokens_start_ind,x.grammar)
                    //     }
                    // }

                    //
                    println!("        hist_begins",);

                    if *hist_begins_len!=0 {
                        let hist_begin=self.hist_begins.last().unwrap();
                        if let Some(hist_begin_val)=&hist_begin.val {
                            let hist_begin_groups=&self.hist_begins_groups[
                                hist_begin.temp_groups_start..hist_begin_val.temp_groups_end
                            ];
                            let hist_begin_prev=&self.hist_begins_prevs[
                                hist_begin.temp_prevs_start..hist_begin_val.temp_prevs_end
                            ];

                            println!("            grammar={:?}",hist_begin_val.grammar);
                            println!("            groups={:?}",hist_begin_groups.iter().map(|g|g.name).collect::<Vec<_>>());
                            println!("            prevs={:?}",hist_begin_prev.iter().map(|p|&p.grammar).collect::<Vec<_>>());

                        }

                    }

                    println!("        hist_ends_last");
                    for i in *hist_ends_ind..*hist_ends_len {
                        let x=&self.hist_ends[i];
                        println!("            {i}:[{:?}]: {:?}",x.tokens_start_ind,x.grammar)
                    }

                    // println!("        hist_news=[{}]",
                    //     self.hist_news.iter()
                    //         .map(|x|format!("{}:{}",x.tokens_start.inds().start,x.grammar.get_non_term_name().unwrap()))
                    //         .collect::<Vec<_>>().join(", ")
                    // );
                    // println!("        hist_begins_last={}",
                    //     self.hist_begins_stk.last().map(|q|format!("[{}]",q.elements.iter()
                    //         .map(|x|format!("{}:{}", //: gs={:?}
                    //             x.1.tokens_start_ind,
                    //             x.0.get_non_term_name().unwrap(),
                    //             // x.1.groups,
                    //         ))
                    //         .collect::<Vec<_>>().join(", ")
                    //     )).unwrap_or_default(),
                    // );
                    // println!("        hist_ends_last={}",
                    //     self.hist_ends_stk.last().map(|q|format!("[{}]",q.elements.iter()
                    //         .map(|x|format!("{}:{}",x.1.tokens_start_ind,x.0.get_non_term_name().unwrap()))
                    //         .collect::<Vec<_>>().join(", ")
                    //     )).unwrap_or_default(),
                    // );
                }

                //
                println!("        tokens {tokens:?}");
            }

            //
            if false {
                for (i,Work {grammar:g, success_len:s, fail_len:f, tokens,
                    group_ind, group_len,and_id,..}) in self.stk.iter().enumerate()
                {
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
            GrammarNode::Expected(..) => {self.grammar_expect(cur);}
            // GrammarNode::Stow(..) => {self.grammar_stow(cur);}
            GrammarNode::Prev(..) => {self.grammar_prev(cur);}
            GrammarNode::Group(..) => {self.grammar_group(cur);}
            GrammarNode::And(..) => {self.grammar_and(cur);}
            GrammarNode::Or(..) => {self.grammar_or(cur);}
            GrammarNode::Many(..) => {self.grammar_many(cur);}
            GrammarNode::NonTerm(..) => {self.grammar_non_term(cur)?;}
            GrammarNode::Error(..) => {return Err(self.grammar_error(cur));}
            GrammarNode::Always => {self.grammar_always(cur);}

            GrammarNode::String => {
                let Some(v)=self.grammar_primitive(cur,|ps|ps.pop_string(),) else{return Ok(());};
                if self.debug {println!("--- string {v:?}");}
            }
            GrammarNode::Identifier => {
                let Some(v)=self.grammar_primitive(cur,|ps|ps.pop_identifier(),) else{return Ok(());};
                if self.debug {println!("--- identifier {v:?}");}
            }
            GrammarNode::Int => {
                let Some(v)=self.grammar_primitive(cur,|ps|ps.pop_int(),) else{return Ok(());};
                if self.debug {println!("--- int {v:?}");}
            }
            GrammarNode::Float => {
                let Some(v)=self.grammar_primitive(cur,|ps|ps.pop_float(),) else{return Ok(());};
                if self.debug {println!("--- float {v:?}");}
            }
            GrammarNode::Symbol(s) => {
                let Some(v)=self.grammar_primitive(cur,|ps|ps.pop_with_symbol(s),) else{return Ok(());};
                if self.debug {println!("--- symbol {v:?}");}
            }
            GrammarNode::Keyword(s) => {
                let Some(v)=self.grammar_primitive(cur,|ps|ps.pop_with_keyword(s),) else{return Ok(());};
                if self.debug {println!("--- keyword {v:?}");}
            }
            GrammarNode::Eol => {
                let Some(_)=self.grammar_primitive(cur,|ps|ps.pop_eol(),) else{return Ok(());};
                if self.debug {println!("--- eol");}
            }
        }

        //
        Ok(())
    }

    //
    pub fn set_debug(&mut self,debug:bool) {
        self.debug=debug;
    }

    pub fn set_prev_non_term_only(&mut self,prev_non_term_only:bool) {
        self.prev_non_term_only=prev_non_term_only;
    }

    pub fn set_stow_non_term_only(&mut self,stow_non_term_only:bool) {
        self.stow_non_term_only=stow_non_term_only;
    }
}