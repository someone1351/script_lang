
/*

for err loc, should use end_loc of last token

TODO
* record only last fail, instead of all of them?
*/
use super::error::*;
use super::temp_data::*;
use core::panic;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
// use std::ops::Range;

use crate::build::Loc;
// use crate::clike::tokenizer::TokenContainer;
use super::super::grammar::data::{Walk,WalkGroup};
use super::super::tokenizer::{TokenIterContainer, ValueContainer};

use super::node::*;


// use data::*;
// use error::*;


pub struct GrammarWalker<'t,'g,G>
where
    G: Fn(&str)->Option<Rc<GrammarNode<'g>>>,
{
    non_term_cache:HashMap<&'g str, Rc<GrammarNode<'g>>>,

    hist_non_term_only:bool,
    // prev_non_term_only:bool,
    // stow_non_term_only:bool,

    top_tokens:TokenIterContainer<'t>,
    tokens_remaining: TokenIterContainer<'t>,
    expected_tokens_remaining: TokenIterContainer<'t>,
    grammar_func:G,
    stk: Vec<Work<'t,'g>>,
    step_count:usize,

    // expected_loc:Loc,

    // expected_news:Vec<TempExpectedNew<'g>>,
    // expecteds:Vec<TempExpected<'g>>,

    expecteds:Vec<TempExpected<'t,'g>>,

    debug:bool,
    // non_term_recursive_check:bool,
    // non_term_visiteds_stk:Vec<HashSet<(&'g str,usize)>>,
    // recurse_num:u64,

    groups:Vec<TempGroup<'t,'g>>,

    hist_news:Vec<TempHistNew<'t,'g>>,

    //simpler to use hist_stows_stk:Vec<Vec<TempHistBegin<'t,'g>>>
    //and maybe don't truncate it, instead use lens  to keep it
    // hist_stows_stk:Vec<Range<usize>>,
    // hist_stows_elements:Vec<TempHistBegin<'t,'g>>,
    // hist_stows_stk:Vec<TempHistBegins<'t,'g>>,

    //should rename hist to stow, so stow_success/stow_fails
    hist_stows:Vec<TempHistStow<'t,'g>>,
    hist_stows_groups:Vec<TempGroup<'t,'g>>,
    // hist_stows_prevs:Vec<TempHistPrev<'g>>,

    //
    // hist_prevs:Vec<TempHistPrev<'g>>,


    hist_fails:Vec<TempHistFail<'g>>,

    // hist_stows_stk:Vec<TempHistBegins<'t,'g>>,
    // hist_ends_stk:Vec<TempHistEnds<'g>>,

    was_news:Vec<TempWas<'g>>,
    wases:Vec<TempWas<'g>>,
    // wases:Vec<Option<TempWas<'g>>>,
    // hads:Vec<TempHad<'g>>,
    // had:Option<TempHad<'g>>,
}

impl<'t,'g,G> GrammarWalker<'t,'g,G>
where
    G: Fn(&str)->Option<Rc<GrammarNode<'g>>>,
{

    pub fn new(top_primitives:TokenIterContainer<'t>, grammar_func:G,) -> Self {
        Self {
            non_term_cache:Default::default(),
            // prev_non_term_only:true,
            // stow_non_term_only:true,
            hist_non_term_only:false,

            stk:Default::default(),
            step_count:Default::default(),

            // expected_loc:Loc::zero(),
            // expected_news:Default::default(),
            // expecteds:Default::default(),
            expecteds:Default::default(),

            grammar_func,
            tokens_remaining:top_primitives.clone(),
            expected_tokens_remaining:top_primitives.clone(),
            top_tokens: top_primitives,
            debug:false,
            // non_term_recursive_check:true,
            // non_term_visiteds_stk:Default::default(),
            // recurse_num:0,

            groups:Default::default(),

            hist_news: Default::default(),

            // hist_stows_stk:Default::default(),
            hist_stows:Default::default(),
            hist_stows_groups:Default::default(),
            // hist_stows_prevs:Default::default(),
            // hist_stows_elements: Default::default(),
            // hist_prevs: Default::default(),
            hist_fails:Default::default(),

            // hist_stows_stk: Default::default(),
            // hist_ends_stk: Default::default(),

            was_news:Default::default(),
            wases:Default::default(),
            // wases:vec![None],
            // hads:Default::default(),
            // had:None,
        }
    }

    // pub fn set_non_term_recursive_check(&mut self,non_term_recursive_check:bool) {
    //     self.non_term_recursive_check=non_term_recursive_check;
    // }

    fn init(&mut self,start_non_term:&'g str,) {
        self.non_term_cache.clear(); //not necessary ...

        self.stk.clear();

        //
        self.tokens_remaining=self.top_tokens;
        self.expected_tokens_remaining=self.top_tokens;

        //
        self.stk.push(Work{
            grammar:Rc::new(GrammarNode::Error(GrammarWalkError::FailedParse)),
            // grammar_ind:0,
            work_stk_success_len:0,work_stk_fail_len:0,
            tokens:self.top_tokens,
            group_ind: 0, group_len: 1,
            // visiteds:Default::default(),
            // non_term_visiteds_stk_len:0,
            // grammar_debug_len: 0,
            // and_id: 0,

            from_user:false,
            first:false,
            or_id:0,
            // and_first:false,

            hist_news_len:0,


            // hist_stows_stk_len:0,
            // hist_ends_stk_len:1,

            // hist_stows_stk_len:0,

            // hist_stows_ind: 0,
            hist_stows_len: 0,

            hist_fails_len:0,

            // in_or:false,
            // can_hist_stow:false,

            // hist_prevs_ind: 0,
            // hist_prevs_len: 0,


            // expected_news_len:0,
            // expecteds_len:0,

            expected_ind:None,
            expecteds_len:0,

            // was_start_ind:0,
            // was_ind:0,
            // was_len:0,

            // had_ind:0,
            // had_len:0,

            was_new_len2:0,
            was_ind2:0,

        });

        //
        let fail_len=self.stk.len();

        //no needed, but allows takeables2 to finish, for debugging purposes
        self.stk.push(Work{
            grammar : Rc::new(GrammarNode::Always),
            // grammar_ind:0,
            work_stk_success_len:0,
            work_stk_fail_len:0, //not used
            tokens:self.top_tokens,
            group_ind: 0, group_len: 1,
            // visiteds:Default::default(),
            // non_term_visiteds_stk_len:0,
            // grammar_debug_len: 1,
            // and_id: 0,

            from_user:true,
            first:false,
            or_id:0,
            // and_first:false,

            // in_or:false,
            // can_hist_stow:false,

            hist_news_len:0,


            // hist_stows_stk_len:0,
            // hist_ends_stk_len:1,

            // hist_stows_stk_len:0,

            // hist_stows_ind: 0,
            hist_stows_len: 0,

            hist_fails_len:0,

            // hist_prevs_ind: 0,
            // hist_prevs_len: 0,

            // expected_news_len:0,
            // expecteds_len:0,


            expected_ind:None,
            expecteds_len:0,


            // was_start_ind:0,
            // was_ind:0,
            // was_len:0,
            // had_ind:0,
            // had_len:0,

            was_new_len2:0,
            was_ind2:0,
        });

        //
        let success_len=self.stk.len();

        //start
        {
            let grammar=self.get_non_term(start_non_term);
            // let grammar=if let Some(g)=(self.grammar_func)(start_non_term) {
            //     g
            // } else {
            //     Rc::new(GrammarNode::Error(GrammarWalkError::MissingNonTerm(start_non_term)))
            // };

            self.stk.push(Work{
                grammar, //:(self.grammar_func)(start_non_term),
                // grammar_ind:0,
                // success_len:0,
                work_stk_success_len: success_len,
                work_stk_fail_len: fail_len, //1
                tokens:self.top_tokens,
                group_ind: 0, group_len: 1,
                // visiteds:Default::default(),
                // non_term_visiteds_stk_len:0,
                // grammar_debug_len: 1,
                // and_id: 0,

                from_user:true,
                first:false,
                or_id:0,
                // and_first:false,

                // in_or:false,
                // can_hist_stow:false,

                hist_news_len:0,
                // hist_stows_stk_len:0,
                // hist_ends_stk_len:1,

                // hist_stows_stk_len:0,

                // hist_stows_ind: 0,
                hist_stows_len: 0,

                hist_fails_len:0,

                // hist_prevs_ind: 0,
                // hist_prevs_len: 0,

                // expected_news_len:0,
                // expecteds_len:0,

                expected_ind:None,
                expecteds_len:0,


                // was_start_ind:0,
                // was_ind:0,
                // was_len:0,
                // had_ind:0,
                // had_len:0,

                was_new_len2:0,
                was_ind2:0,
            });
        }

        //
        self.groups=vec![TempGroup{
            name: "",
            parent: 0,
            tokens:self.top_tokens,
        }];

        //
        // self.non_term_visiteds_stk.clear(); //not necessary really...

        //
        self.hist_news.clear();

        // self.hist_stows_stk.clear();
        self.hist_stows.clear();
        self.hist_stows_groups.clear();
        // self.hist_stows_prevs.clear();

        // self.hist_stows_elements.clear();
        // self.hist_prevs.clear();
        self.hist_fails.clear();


        self.was_news.clear();
        self.wases.clear();
        // self.had=None;
        // self.hads.clear();

        // self.hist_stows_stk.clear(); //don't need initial one because don't need to store begins before an Or exists
        // // self.hist_ends_stk.clear();
        // self.hist_ends_stk=vec![Default::default()]; //need an initial one because require ends regardless of an Or existing

        //
        self.step_count=0;

        //
        // self.expected_loc=Loc::zero();
        // self.expected_news.clear();
        // self.expecteds.clear();
        self.expecteds.clear();

    }

    fn grammar_was(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Was(g,name, )=cur.grammar.as_ref() else{panic!("");};

        //
        let hist_news_len=self.hist_news_add(&cur);

        //
        // let was_ind=self.wases.len();
        // let was_ind=cur.was_ind+1;
        // let was_ind=cur.was_len;
        // self.wases.push(TempWas{name});
        // let was_len=cur.was_len+1; //self.wases.len();


        self.was_news.push(TempWas{name});
        // let was_new_len2=cur.was_new_len2+1;//self.was_news.len();
        let was_new_len2=self.was_news.len();

        //
        self.stk.push(Work {
            grammar: g.clone(),

            work_stk_success_len: cur.work_stk_success_len,
            work_stk_fail_len: cur.work_stk_fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,

            from_user:true,
            first:cur.first,
            or_id:cur.or_id,

            hist_news_len,

            hist_stows_len: cur.hist_stows_len,

            hist_fails_len: cur.hist_fails_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            expected_ind:cur.expected_ind,
            expecteds_len:cur.expecteds_len,

            // was_start_ind:cur.was_start_ind,
            // // was_ind:cur.was_ind,
            // was_ind,
            // was_len,
            // had_ind:cur.had_ind,
            // had_len:cur.had_len,

            was_new_len2,
            was_ind2:cur.was_ind2,
        });
    }

    fn grammar_had(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Had(name,)=cur.grammar.as_ref() else{panic!("");};

      //
        let _hist_news_len=self.hist_news_add(&cur);
        // self.hist_stows_clear(&cur);


        //
        if
            // self.had.as_ref().map(|x|x.name) == Some(*name)
            // self.wases[cur.was_ind..].iter().find(|x|x.name==*name).map(|x|x.name)==Some(*name)
            // self.wases.get(cur.was_ind).map(|x|x.name)==Some(*name)
            self.wases.last().map(|x|x.name)==Some(*name)
            // self.hist_ends_stk.last().unwrap().elements
            //     .iter().find(|x|x.grammar.eq(g)).is_some()
            //     // .contains_key(&g)
            // self.hist_prevs[cur.hist_prevs_ind..].iter().find(|x|x.grammar.eq(g)).is_some()
        {
            // self.stk.truncate(cur.success_len);
            self.work_on_success(&cur);

            //whats to stop a and(X, many(prev(X))) ?
            self.handle_exit_last_many(&cur); //this

            //
            // self.hist_news_truncate_to_last(); //why on success??
            self.update_tokens(&cur,true);
            self.groups_on_success(&cur);
            self.was_on_success(false); //before hist
            self.hist_on_success(&cur,false,);
            self.expected_on_success();
        } else {
            // self.stk.truncate(cur.fail_len);
            self.work_on_fail(&cur);
            self.update_tokens(&cur,false);
            // // self.revert_last_hist_news();
            self.hist_on_fail();
            self.was_on_fail(&cur);
            self.groups_on_fail();
            //don't add expected, let user manually add one
            // // // let _expected_news_len=self.add_expected_new(&cur);
            // // // let (_expected_ind,_expecteds_len)=self.add_expected2(&cur);

            // // self.submit_expected_news(&cur);
            self.expected_on_fail();
        }
    }

    fn grammar_expect(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Expected(g,_, )=cur.grammar.as_ref() else{panic!("");};

        //
        // let expected_news_len=self.add_expected_new(&cur);
        let (expected_ind,expecteds_len)=self.add_expected2(&cur);
        let hist_news_len=self.hist_news_add(&cur);

        //
        self.stk.push(Work {
            grammar: g.clone(),
            // grammar_ind:0,
            work_stk_success_len: cur.work_stk_success_len,
            work_stk_fail_len: cur.work_stk_fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len+1,
            // and_id:cur.and_id,

            from_user:true,
            first:cur.first,
            or_id:cur.or_id,
            // and_first:cur.and_first,

            // in_or:cur.in_or,
            // can_hist_stow:false,

            hist_news_len,
            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            hist_stows_len: cur.hist_stows_len,

            hist_fails_len: cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len,
            // // expected_news_len:cur.expected_news_len,
            // expecteds_len:cur.expecteds_len,

            expected_ind,
            expecteds_len,


            // expected_ind:cur.expected_ind,
            // expecteds_len:cur.expecteds_len,


            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,
            // had_ind:cur.had_ind,
            // had_len:cur.had_len,

            was_new_len2:cur.was_new_len2,
            was_ind2:cur.was_ind2,
        });
    }

    fn grammar_group(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Group(g,_, )=cur.grammar.as_ref() else{panic!("");};

        //
        let (group_ind,group_len)=self.new_group(&cur); //name, cur.group_ind, cur.tokens
        let hist_news_len=self.hist_news_add(&cur);

        //
        self.stk.push(Work {
            grammar: g.clone(),
            // grammar_ind:0,
            work_stk_success_len: cur.work_stk_success_len,
            work_stk_fail_len: cur.work_stk_fail_len,
            tokens: cur.tokens,
            group_ind,
            group_len,
            // visiteds:cur.visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len+1,
            // and_id:cur.and_id,

            from_user:true,
            first:cur.first,
            or_id:cur.or_id,
            // and_first:cur.and_first,

            // can_hist_stow:false,
            hist_news_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            hist_stows_len: cur.hist_stows_len,

            hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expecteds_len:cur.expecteds_len,

            expected_ind:cur.expected_ind,
            expecteds_len:cur.expecteds_len,


            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,

            was_new_len2:cur.was_new_len2,
            was_ind2:cur.was_ind2,
        });
    }

    fn grammar_many(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Many(g)=cur.grammar.as_ref() else{panic!("");};

        //in always/prev they check if their success_ind is a many (which could be a problem if ands/ors were handled more efficiently),
        //  but could store maybe a many_id to check whether to exit? eg if id is eq, and/or tokens.inds.start is eq?
        //
        let hist_news_len=self.hist_news_add(&cur);


        let was_ind2=self.wases.len();

        //
        self.stk.push(Work {
            // grammar: Rc::new(GrammarNode::Many(g.clone())),
            grammar:cur.grammar.clone(),
            // grammar_ind:0,
            work_stk_success_len: cur.work_stk_success_len,
            work_stk_fail_len: cur.work_stk_fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds.clone(),
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len,
            // and_id:cur.and_id,

            from_user:false,
            first:false, //grmmar in many, after the frist one is parsed are no longer firsts
            or_id:cur.or_id,
            // and_first:false,
            // can_hist_stow:false,

            hist_news_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            hist_stows_len: cur.hist_stows_len,

            hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expecteds_len:cur.expecteds_len,

            expected_ind:cur.expected_ind,
            expecteds_len:cur.expecteds_len,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,


            was_new_len2:cur.was_new_len2,
            was_ind2:cur.was_ind2,
        });

        //
        let success_len2=self.stk.len();

        //
        self.stk.push(Work {
            grammar: Rc::new(GrammarNode::Always),
            // grammar_ind:0,
            work_stk_success_len: cur.work_stk_success_len,
            work_stk_fail_len: 0, //fail is not used
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds.clone(),
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len,
            // and_id:cur.and_id,

            from_user:false,
            first:false,
            or_id:cur.or_id,
            // and_first:false,
            // can_hist_stow:false,

            hist_news_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            hist_stows_len: cur.hist_stows_len,

            hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expecteds_len:cur.expecteds_len,

            expected_ind:cur.expected_ind,
            expecteds_len:cur.expecteds_len,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,


            was_new_len2:cur.was_new_len2,
            // was_ind2:cur.was_ind2,
            was_ind2,
        });

        //
        let fail_len=self.stk.len();

        //
        self.stk.push(Work {
            grammar: g.clone(),
            // grammar_ind:0,
            work_stk_success_len: success_len2,
            work_stk_fail_len: fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len+1,
            // and_id:cur.and_id,

            from_user:true,
            first:cur.first,
            or_id:cur.or_id,
            // and_first:cur.and_first,
            // can_hist_stow:false,

            hist_news_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            hist_stows_len: cur.hist_stows_len,

            hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expecteds_len:cur.expecteds_len,

            expected_ind:cur.expected_ind,
            expecteds_len:cur.expecteds_len,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,


            was_new_len2:cur.was_new_len2,
            // was_ind2:cur.was_ind2,
            was_ind2,
        });
    }

    fn grammar_non_term(&mut self,cur :Work<'t,'g>,) -> Result<(),GrammarWalkError<'g>>{
        let GrammarNode::NonTerm(t)=cur.grammar.as_ref() else{panic!("");};

        //
        let hist_news_len=self.hist_news_add(&cur);
        // let visiteds=self.do_non_term_visiteds(t,cur.tokens,cur.visiteds)?;

        //
        // let grammar=if let Some(g)=(self.grammar_func)(t) {
        //     g
        // } else {
        //     Rc::new(GrammarNode::Error(GrammarWalkError::MissingNonTerm(t)))
        // };

        let grammar=self.get_non_term(t);

        //
        self.stk.push(Work {
            grammar, //: (self.grammar_func)(t), //should return err on not found, instead of grammar never, should have error
            // grammar_ind:0,
            work_stk_success_len: cur.work_stk_success_len,
            work_stk_fail_len: cur.work_stk_fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len+1,
            // grammar_debug_len: cur.grammar_debug_len+1,
            // and_id:cur.and_id,

            from_user:true,
            first:cur.first,
            or_id:cur.or_id,
            // and_first:cur.and_first,

            // can_hist_stow:false,

            hist_news_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            hist_stows_len: cur.hist_stows_len,

            hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expecteds_len:cur.expecteds_len,

            expected_ind:cur.expected_ind,
            expecteds_len:cur.expecteds_len,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,


            was_new_len2:cur.was_new_len2,
            was_ind2:cur.was_ind2,
        });

        Ok(())
    }

    fn grammar_error(&mut self,cur :Work<'t,'g>,) -> GrammarWalkError<'g> {
        let GrammarNode::Error(e)=cur.grammar.as_ref() else{panic!("");};

        // if self.debug {
        //     println!("====error {:?} ",self.expected_loc,); //self.expecteds,
        // }

        //necesaary? any point to it?
        // if self.expecteds.is_empty() { // self.expected.0.is_zero()
        //     self.expected_loc=cur.primitives.loc();
        // }

        //
        self.update_tokens(&cur,false); //could be true, but would do nothing

        //
        // self.expect_news_drain(&cur); //necessary here? no since it is finishing here?

        //
        return e.clone();
    }

    fn grammar_and(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::And(gs,grammar_ind)=cur.grammar.as_ref() else{panic!("");};
        //

        if gs.is_empty() {return;}

        // let Some(head)=gs.first().cloned() else { return ; };
        let head=gs.get(*grammar_ind).unwrap().clone();

        //
        let hist_news_len=self.hist_news_add(&cur);

        //
        // println!("{}, {:?}",cur.grammar_ind,cur.grammar.as_ref(),);

        //
        // if cur.grammar_ind+1!=gs.len()
        // if gs.len()>1

        // // let gg=GrammarNode::And(Rc::new([&G]));
        // let mut gg:&[&i32]=&[&123];
        // gg=&[&123,&43];

        if *grammar_ind!=gs.len()
        // if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r.clone()))
        {
            // let rest=gs[1..];
            self.stk.push(Work {
                // grammar: Rc::new(GrammarNode::And(rest.into(),*grammar_ind+1)),
                grammar:Rc::new(GrammarNode::And(gs.clone(),*grammar_ind+1)),
                // grammar_ind:0,

                // grammar:cur.grammar.clone(),
                // grammar_ind:cur.grammar_ind+1,

                work_stk_success_len: cur.work_stk_success_len,
                work_stk_fail_len: cur.work_stk_fail_len,
                tokens: cur.tokens, //not really necessary? since gets updated by always/primtitives
                group_ind: cur.group_ind,
                group_len: cur.group_len,
                // visiteds:cur.visiteds.clone(),
                // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
                // grammar_debug_len: cur.grammar_debug_len,
                // and_id:cur.and_id+1,

                from_user:false,
                first:false,
                or_id:cur.or_id,
                // and_first:false,
                // can_hist_stow:false,

                hist_news_len,

                // hist_stows_stk_len:cur.hist_stows_stk_len,
                // hist_ends_stk_len:cur.hist_ends_stk_len,

                // hist_stows_ind: cur.hist_stows_ind,
                hist_stows_len: cur.hist_stows_len,

                hist_fails_len:cur.hist_fails_len,

                // hist_stows_stk_len:cur.hist_stows_stk_len,

                // hist_prevs_ind: cur.hist_prevs_ind,
                // hist_prevs_len: cur.hist_prevs_len,

                // expected_news_len:cur.expected_news_len,
                // expecteds_len:cur.expecteds_len,

                expected_ind:cur.expected_ind,
                expecteds_len:cur.expecteds_len,

                // was_start_ind:cur.was_start_ind,
                // was_ind:cur.was_ind,
                // was_len:cur.was_len,

                // had_ind:cur.had_ind,
                // had_len:cur.had_len,


                was_new_len2:cur.was_new_len2,
                was_ind2:cur.was_ind2,
            });
        }

        //
        // let not_end= gs.len() > 1;
        let not_end= *grammar_ind+1 != gs.len();
        let success_len=if not_end {self.stk.len()}else{cur.work_stk_success_len};

        //
        self.stk.push(Work {
            grammar: head,
            // grammar_ind:0,
            work_stk_success_len: success_len,
            work_stk_fail_len: cur.work_stk_fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len+1,

            // // and_id:cur.and_id+1,
            // and_id:if gs.len()==1{cur.and_id}else{cur.and_id+1}, //don't need if single element in And?

            from_user:true,
            first:cur.first, //cur.from_user &&  //only want to know about grammars added by user, not the walker, could check from_user elsewhere,
            or_id:cur.or_id,
            // and_first:true,

            // can_hist_stow:cur.or_first,

            hist_news_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            hist_stows_len: cur.hist_stows_len,

            hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expecteds_len:cur.expecteds_len,

            expected_ind:cur.expected_ind,
            expecteds_len:cur.expecteds_len,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,

            was_new_len2:cur.was_new_len2,
            was_ind2:cur.was_ind2,
        });
    }

    fn grammar_or(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Or(gs,grammar_ind)=cur.grammar.as_ref() else{panic!("");};

        if gs.is_empty() {return;}

        //
        // let Some(head)=gs.first().cloned() else { return; };
        // let Some(head)=gs.get(*grammar_ind).cloned() else { return; };
        let head=gs.get(*grammar_ind).unwrap().clone();
        //
        // if cur.grammar_ind==gs.len() {return;}
        // let head=gs.get(cur.grammar_ind).unwrap().clone();


        //
        let hist_news_len=self.hist_news_add(&cur);
        // let hist_stows_stk_len=self.hist_stows_stk_push(&cur);
        let hist_stows_len=self.hist_stows_push(&cur);


        let hist_fails_len=self.hist_fails_push(&cur);
        // let hist_ends_stk_len=self.hist_ends_stk_push(&cur);
        // let hist_stows_ind=if !cur.is_first{cur.hist_stows_len}else{cur.hist_stows_ind};

        //
        // // let was_ind = if cur.from_user && !cur.first {cur.was_len} else {cur.was_ind}; //self.wases.len()
        // let was_start_ind = if cur.from_user && !cur.first {cur.was_len} else {cur.was_start_ind}; //self.wases.len()

        let was_ind2 = if cur.from_user && !cur.first {self.wases.len()} else {cur.was_ind2}; //self.wases.len()


        // //on a new OR first, this sets a new hist_prevs_ind
        // let (hist_prevs_ind,hist_prevs_len) =
        //     if cur.from_user && !cur.first //cur.hist_prevs_ind is already 0 for the first, so only need for further ORs
        //     {(self.hist_prevs.len(),self.hist_prevs.len())}
        //     else{(cur.hist_prevs_ind,cur.hist_prevs_len)};

        // println!("cur.from_user={}, cur.first={} : {:?}, {:?}",
        //     cur.from_user, cur.first,
        //     (self.hist_prevs.len(),self.hist_prevs.len()),
        //     (cur.hist_prevs_ind,cur.hist_prevs_len),
        // );
        // let hist_ends_ind=if cur.is_first{}else{};

        //
        if grammar_ind+1!=gs.len()
        // if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r))
        {
            self.stk.push(Work {
                grammar: Rc::new(GrammarNode::Or(gs.clone(),*grammar_ind+1)),
                // grammar_ind:0,
                // grammar:cur.grammar.clone(),
                // grammar_ind:cur.grammar_ind+1,

                work_stk_success_len: cur.work_stk_success_len,
                work_stk_fail_len: cur.work_stk_fail_len,
                tokens: cur.tokens,
                group_ind: cur.group_ind,
                group_len: cur.group_len,
                // visiteds:cur.visiteds.clone(),
                // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
                // grammar_debug_len: cur.grammar_debug_len,
                // and_id:cur.and_id,

                from_user:false,
                first:cur.first,
                or_id:cur.or_id,
                // and_first:cur.and_first,
                // can_hist_stow:false,

                hist_news_len,
                hist_fails_len,

                // hist_stows_stk_len,
                // hist_ends_stk_len,

                // hist_stows_ind,
                // hist_stows_len: cur.hist_stows_len,

                // hist_stows_stk_len,
                hist_stows_len,

                // hist_prevs_ind: cur.hist_prevs_ind,
                // hist_prevs_len: cur.hist_prevs_len,
                // hist_prevs_ind,
                // hist_prevs_len,

                // expected_news_len:cur.expected_news_len,
                // expecteds_len:cur.expecteds_len,

                expected_ind:cur.expected_ind,
                expecteds_len:cur.expecteds_len,

                // // was_start_ind:cur.was_start_ind,
                // was_start_ind,
                // // was_ind,
                // was_ind:cur.was_ind,
                // was_len:cur.was_len,
                // had_ind:cur.had_ind,
                // had_len:cur.had_len,

                was_new_len2:cur.was_new_len2,
                // was_ind2:cur.was_ind2,
                was_ind2,
            });
        }

        //

        // let not_end= gs.len() > 1;
        let not_end= *grammar_ind+1 != gs.len();

        let fail_len=if not_end {self.stk.len()}else{cur.work_stk_fail_len};

        //
        self.stk.push(Work {
            grammar: head,
            // grammar_ind:0,
            work_stk_success_len: cur.work_stk_success_len,
            work_stk_fail_len: fail_len,
            tokens: cur.tokens,
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len+1,
            // and_id:cur.and_id,

            from_user:true,
            first:true,
            or_id:cur.or_id,
            // and_first:cur.and_first,
            // can_hist_stow:false,

            hist_news_len,

            // hist_stows_stk_len,
            hist_stows_len,
            hist_fails_len,
            // hist_ends_stk_len,

            // hist_stows_ind,
            // hist_stows_len: cur.hist_stows_len,
            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,
            // hist_prevs_ind,
            // hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expecteds_len:cur.expecteds_len,

            expected_ind:cur.expected_ind,
            expecteds_len:cur.expecteds_len,

            // // was_start_ind:cur.was_start_ind,
            // was_start_ind,
            // // was_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,
            // had_ind:cur.had_ind,
            // had_len:cur.had_len,


            was_new_len2:cur.was_new_len2,
            // was_ind2:cur.was_ind2,
            was_ind2,
        });
    }



    // fn grammar_prev(&mut self,cur :Work<'t,'g>,) {
    //     let GrammarNode::Prev(g)=cur.grammar.as_ref() else {panic!("");};
    //     //
    //     let _hist_news_len=self.hist_news_add(&cur);
    //     // self.hist_stows_clear(&cur);


    //     //
    //     if
    //         // self.hist_ends_stk.last().unwrap().elements
    //         //     .iter().find(|x|x.grammar.eq(g)).is_some()
    //         //     // .contains_key(&g)
    //         self.hist_prevs[cur.hist_prevs_ind..].iter().find(|x|x.grammar.eq(g)).is_some()
    //     {
    //         // self.stk.truncate(cur.success_len);
    //         self.work_on_success(&cur);

    //         //whats to stop a and(X, many(prev(X))) ?
    //         self.handle_exit_last_many(&cur); //this

    //         //
    //         // self.hist_news_truncate_to_last(); //why on success??
    //         self.update_tokens(&cur,true);
    //         self.groups_on_success(&cur);
    //         self.hist_on_success(&cur,false,);
    //         self.expected2_on_success();
    //     } else {
    //         // self.stk.truncate(cur.fail_len);
    //         self.work_on_fail(&cur);
    //         self.update_tokens(&cur,false);
    //         // // self.revert_last_hist_news();
    //         self.hist_on_fail();
    //         self.groups_on_fail();
    //         //don't add expected, let user manually add one
    //         // // // let _expected_news_len=self.add_expected_new(&cur);
    //         // // // let (_expected_ind,_expecteds_len)=self.add_expected2(&cur);

    //         // // self.submit_expected_news(&cur);
    //         self.expected2_on_fail();
    //     }
    // }

    fn grammar_always(&mut self,cur :Work<'t,'g>,) {
        // self.stk.truncate(cur.success_len);
        self.work_on_success(&cur);
        let _hist_news_len=self.hist_news_add(&cur);
        // self.hist_stows_clear(&cur);
        self.handle_exit_last_many(&cur);
        self.update_tokens(&cur,true);
        self.groups_on_success(&cur); //here
        self.was_on_success(false); //before hist
        self.hist_on_success(&cur,false);

        //why was this previously commented out?
        //  because grammar could finish without parsing anything due to optionals
        //  and then not report any errors
        // self.expected2_on_success();
    }

    fn grammar_try_from_hist_fails(&mut self,cur :&Work<'t,'g>) -> bool {
         //
        if !cur.from_user || !cur.first {return false;} // !(cur.from_user && cur.is_first)
        if cur.hist_fails_len==0 {return false;}

        //
        let hist_fail=&self.hist_fails[cur.hist_fails_len-1];

        if !hist_fail.grammers.contains(&cur.grammar) {
            return false;
        }

        //
        // self.stk.truncate(cur.fail_len);
        self.work_on_fail(&cur);
        self.update_tokens(&cur,false);
        self.hist_on_fail();
        self.was_on_fail(&cur);
        self.expected_on_fail();
        self.groups_on_fail();

        //
        if self.debug
        {
            println!("---- grabbed fail from or {:?},",cur.grammar);
        }

        //
        true
    }

    fn grammar_try_from_hist_stows(&mut self,cur :&Work<'t,'g>) -> bool {
        //
        if !cur.from_user || !cur.first {return false;} // !(cur.from_user && cur.is_first)
        // if cur.hist_stows_stk_len==0 {return false;}
        if cur.hist_stows_len==0 {return false;}

        // if self.hist_non_term_only && !cur.grammar.is_non_term() {return false;}


        let hist_stow=&self.hist_stows[cur.hist_stows_len-1];
        let Some(hist_stow_val)=&hist_stow.val else {return false;};

        //
        if hist_stow_val.grammar!=cur.grammar {
            return false;
        }

        //
        // self.stk.truncate(cur.success_len);

        let temp_groups_end=hist_stow.val.as_ref().map(|x|x.stow_groups_end).unwrap_or(hist_stow.stow_groups_start);
        // let temp_prevs_end=hist_stow.val.as_ref().map(|x|x.stow_prevs_end).unwrap_or(hist_stow.stow_prevs_start);

        let stow_groups=&self.hist_stows_groups[hist_stow.stow_groups_start .. temp_groups_end];
        // // let stow_prevs=&self.hist_stows_prevs[hist_stow.stow_prevs_start..temp_prevs_end];

        // //
        // // let group_ind_offset=stow_groups.first().map(|g|g.parent).unwrap_or_default();



        //
        let glen=self.groups.len();
        // println!("===--- glen={glen}, cur.group_ind={}, cur.group_len={}",cur.group_ind,cur.group_len);

        //add groups
        self.groups.extend(stow_groups.iter().enumerate().map(|(_i,g)|{
            // println!("===--- {i}:p{}:{} = {}",g.parent,g.name, cur.group_ind+g.parent + if g.parent==0{0}else{glen});
            TempGroup{
                parent:if g.parent==0{cur.group_ind}else{glen+g.parent-1},
                ..g.clone()
            }
        }));

        // self.hist_prevs.extend_from_slice(stow_prevs);

        //
        if let TempHistStowWas::Was(temp_was)=&hist_stow_val.was {
            self.was_news.push(temp_was.clone());
        }

        //
        let was_prim= if let TempHistStowWas::Primitive=&hist_stow_val.was {true}else{false};

        //
        let cur=Work {
            group_len:self.groups.len(),
            tokens:hist_stow_val.tokens_after,
            // was_new_len2,
            // hist_ends_stk_len:todo!(),
            ..cur.clone()
        };

        //

        self.work_on_success(&cur);
        self.update_tokens(&cur,true);
        self.groups_on_success(&cur,);
        self.was_on_success(was_prim); //before hist
        self.hist_on_success(&cur,true,); //not needed? no.. if And(Z,Or(And(X,Y),X)), then will add that
        self.expected_on_success();

        //
        if self.debug {
            println!("---- grabbed success from or {:?},",cur.grammar);
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
        // self.hist_stows_clear(&cur);

        //
        match prim_func(&mut cur.tokens) {
            Ok(v) => {
                //
                // self.stk.truncate(cur.success_len);
                self.work_on_success(&cur);
                self.update_tokens(&cur,true);
                self.groups_on_success(&cur);
                self.was_on_success(true); //before hist
                self.hist_on_success(&cur,false);
                self.expected_on_success();

                //
                // if self.debug {
                //     println!("--- hmm stk={:?}",self.stk.iter().map(|x|x.grammar.clone()).collect::<Vec<_>>());

                //     // if let Some(last)=self.stk.last() {
                //     //     let last_hist_ends=&self.hist_ends_stk[last.hist_ends_stk_len-1].elements;
                //     //     println!("---last_hist_ends={:?}, len={}",last_hist_ends,last_hist_ends.len());
                //     // }
                // }

                //
                Some(v)
            }
            Err(_loc) => {
                // self.stk.truncate(cur.fail_len);
                self.work_on_fail(&cur);
                self.update_tokens(&cur,false);
                self.hist_on_fail();
                self.was_on_fail(&cur);
                // // self.revert_last_hist_news();
                // self.update_hist_on_fail(&cur);
                // // let _expected_news_len=self.add_expected_new(&cur);
                // // self.submit_expected_news(&cur);

                self.groups_on_fail();
                let (_expected_ind,_expecteds_len)=self.add_expected2(&cur);
                self.expected_on_fail();

                //
                None
            }
        }
    }

    fn work_on_success(&mut self, cur:&Work<'t,'g>,) {
        self.stk.truncate(cur.work_stk_success_len);
    }

    fn work_on_fail(&mut self, cur:&Work<'t,'g>,) {
        self.stk.truncate(cur.work_stk_fail_len);
    }



    fn add_expected2(&mut self, cur:&Work<'t,'g>,) -> (Option<usize>,usize) {
        // return (cur.expected_ind,cur.expecteds_len);

        //check if prim and parent pos is same as cur pos
        //

        let parent_start=cur.expected_ind.map(|i|self.expecteds[i].tokens_start.inds().start) ;

        if parent_start==Some(cur.tokens.inds().start) {
            return (cur.expected_ind,cur.expecteds_len);
        }

        // if cur.expected_ind.is_some() && cur.grammar.is_primtive() { //(cur.grammar.is_prev() || )
        //     return (cur.expected_ind,cur.expecteds_len);
        // }

        //
        let expected_type=match cur.grammar.as_ref() {
            GrammarNode::Expected(_, name) => TempExpectedType::Expected(name),
            // GrammarNode::Prev(_) => TempExpectedType::Prev,
            GrammarNode::String => TempExpectedType::String,
            GrammarNode::Identifier => TempExpectedType::Identifier,
            GrammarNode::Int => TempExpectedType::Int,
            GrammarNode::Float => TempExpectedType::Float,
            GrammarNode::Symbol(s) => TempExpectedType::Symbol(s),
            GrammarNode::Keyword(s) => TempExpectedType::Keyword(s),
            GrammarNode::Eol => TempExpectedType::Eol,
           _ => {panic!("");}
        };

        //
        if self.debug {
            println!("----- added expected {expected_type:?}");
        }

        //
        let expected_ind=self.expecteds.len();

        //
        self.expecteds.push(TempExpected {
            expected_type,
            parent: cur.expected_ind,
            tokens_start: cur.tokens,
        });

        //
        (Some(expected_ind),self.expecteds.len())
    }


    fn expected_on_success(&mut self, ) {
        let Some(last)=self.stk.last() else {return;}; //the func, not run on always... does now
        self.expecteds.truncate(last.expecteds_len);
    }

    fn expected_on_fail(&mut self, ) {
        let Some(last)=self.stk.last_mut() else {panic!("");}; //the func, not run on always
        last.expecteds_len=self.expecteds.len();
    }



    fn was_on_success(&mut self, //cur:&Work<'t,'g>,
        is_prim:bool,

    ) {
        //run before hist_on_success

        let Some(last)=self.stk.last_mut() else {return;};


                //
        let drained_was_new=self.was_news.drain(last.was_new_len2 ..).next();

        if let Some(drained_was_new)=drained_was_new {

            // last.was_ind2=self.wases.len();
            self.wases.truncate(last.was_ind2); //remove prev WAS
            self.wases.push(drained_was_new);
            // last.was_len2=self.wases.len();
            println!("----- was 0");
        } else if is_prim { //cur.grammar.is_primtive() //should be not? no
            // let b=cur.grammar.is_primtive() || (cur.grammar.is_had()||cur.grammar.is_always());
            // self.wases.truncate(last.was_ind2+1); //keep previous WAS, if there was one
            self.wases.truncate(last.was_ind2);
            println!("----- was 1");
        } else if last.was_ind2!=self.wases.len() { //for had/always
            // println!("=======---===");
            self.wases.drain(last.was_ind2..self.wases.len()-1);
            println!("----- was 2");

            // let d=self.wases.drain(last.was_ind2..self.wases.len()-1).map(|x|x.name).collect::<Vec<_>>();
            // println!("----- was 2: {d:?}");
        } else {
            println!("----- was 3");
        }

    }

    fn was_on_fail(&mut self, cur:&Work<'t,'g>,) {
        let Some(last)=self.stk.last_mut() else {panic!("");}; //the func, not run on always

            println!("----- was 4 faill");
        self.was_news.truncate(last.was_new_len2);

        // if cur.was_ind2==last.was_ind2 {
        //     self.wases.truncate(last.was_ind2+1); //kee prev
        // } else {
            self.wases.truncate(last.was_ind2);
        // }

        // self.wases.truncate(last.was_len);
        // // self.wases.truncate(last.was_ind+1);
    }

    fn hist_on_fail(&mut self,
        // cur:&Work<'t,'g>,
    ){
        let Some(last)=self.stk.last_mut() else {return;};
        // self.hist_prevs.truncate(last.hist_prevs_len);

        // println!("---- hist on fail {:?}",
        //     self.hist_prevs.drain(last.hist_prevs_len..).map(|x|x.grammar.clone()).collect::<Vec<_>>(),
        // );

        if last.hist_fails_len!=0 {
            //
            let drained_hist_news=self.hist_news.drain(last.hist_news_len ..).collect::<Vec<_>>();

            //
            if self.debug && !drained_hist_news.is_empty() {
                // println!("----- adding to hist fails");
            }

            //
            for x in drained_hist_news {
                if !x.is_first {continue;}
                if x.hist_fails_len!=last.hist_fails_len {continue;}
                if !(x.grammar.is_non_term() || x.grammar.is_and() || x.grammar.is_many()) {continue;}

                self.hist_fails[last.hist_fails_len-1].grammers.insert(x.grammar.clone());
            }
        } else {
            self.hist_news.truncate(last.hist_news_len);
        }


        let hist_stows_len=last.hist_stows_len;
        self.hist_stows_truncate(hist_stows_len);

    }

    fn hist_on_success(&mut self,
        cur:&Work<'t,'g>,
        //what was this for again? something to do with not adding cur grammar to hist_stows?
        //  it was for not adding cur grammar to hist_new?
        gotten:bool,
        // _hist_ends_remove_previous:bool,
    ) {

        //TODO: dont convert drained drained_hist_news to vec, copy straight to

        //should always be some (due to init), use panic instead of ret? no, it will end on an always if successful
        let Some(last)=self.stk.last_mut() else {return;};

        //
        let drained_hist_news=self.hist_news.drain(last.hist_news_len ..).collect::<Vec<_>>();

        //add hist stows
        if last.hist_stows_len!=0 { //cur.hist_stows_len!=0 // && cur.hist_stows_len==last.hist_stows_len //that the hist_stows[ind] still exists
            //
            let drained_hist_new2=drained_hist_news.iter().find(|x|{
                x.is_first
                && x.hist_stows_len==last.hist_stows_len
                && (x.grammar.is_non_term() || x.grammar.is_and() || x.grammar.is_many())

            });

            // println!("-------found {:?} : {:?}",drained_hist_new2,drained_hist_news.iter().map(|x|&x.grammar).collect::<Vec<_>>());
            //

            if let Some(drained_hist_new2)=drained_hist_new2 {

                //
                if self.debug {
                    println!("------ hist_stows_set {}: {:?}", self.hist_stows.len(), drained_hist_new2.grammar, );
                }
                // println!("------ stowed {:?}",drained_hist_new2.grammar.clone());

                //
                // let hist_stow=self.hist_stows.last_mut().unwrap();
                let hist_stow=&mut self.hist_stows[last.hist_stows_len-1];

                //
                if !gotten {
                    // self.hist_stows_prevs.truncate(hist_stow.stow_prevs_start);
                }

                // self.hist_stows_prevs.extend(added_hist_prevs.iter().rev().cloned());

                //
                self.hist_stows_groups.truncate(hist_stow.stow_groups_start);

                //
                if self.groups.len()!=drained_hist_new2.group_len {

                    //
                    let group_ind_offset=self.groups[drained_hist_new2.group_len].parent;

                    self.hist_stows_groups.extend(self.groups[drained_hist_new2.group_len..cur.group_len].iter().map(|x|TempGroup{
                        parent: x.parent
                        -group_ind_offset
                        , ..x.clone()
                    }));
                }

                //
                hist_stow.val=Some(TempHistStowVal {
                    grammar: drained_hist_new2.grammar.clone(),
                    tokens_after: cur.tokens,
                    stow_groups_end: self.hist_stows_groups.len(),
                    // stow_prevs_end: self.hist_stows_prevs.len(),
                    // was:self.wases.get(cur.was_ind2).cloned(),
                    was: //self.wases[last.was_ind2..].last().m
                    if last.was_ind2!=self.wases.len() {
                        TempHistStowWas::Was(self.wases.last().cloned().unwrap())
                    } else if cur.grammar.is_primtive() {
                        TempHistStowWas::Primitive
                    } else {
                        TempHistStowWas::None
                    },
                })
            }


        }



        //
        self.hist_news.truncate(last.hist_news_len);

        let hist_stows_len=last.hist_stows_len;
        self.hist_stows_truncate(hist_stows_len);

        // //

        // // last.hist_stows_len=self.hist_stows_elements.len();

    }

    fn hist_fails_push(&mut self,cur:&Work<'t,'g>) -> usize {
        if cur.from_user
            && ( !cur.first || cur.hist_fails_len==0
        ) {
            //
            if self.hist_fails.len() < cur.hist_fails_len+1 {
                if self.hist_fails.len() != cur.hist_fails_len {panic!("");}

                //
                self.hist_fails.push(Default::default());
            }

            //
            self.hist_fails[cur.hist_fails_len].grammers.clear();

            //
            cur.hist_fails_len+1
        } else {
            cur.hist_fails_len
        }

        //

    }
    fn hist_stows_push(&mut self,cur:&Work<'t,'g>) -> usize {
        if cur.from_user //so not an added OR for rest,
            && ( !cur.first || //not part of current OR, eg: or(A, and(B,or(C,D))) A in dif OR stk than C,D
            // self.hist_stows_stk.is_empty()
            cur.hist_stows_len==0 //init first, for if all part of same OR stk, eg: or(A,or(B,C))
            //if not need to init first, then it just reuses existing one
        ) //add current/initial OR
            // && (!self.hist_non_term_only ||)
        {
            if self.debug {
                println!("------ hist_stows_push ind={}", self.hist_stows.len());
            }

            //
            self.hist_stows.push(TempHistStow {
                val: None,
                stow_groups_start: self.hist_stows_groups.len(),
                // stow_prevs_start: self.hist_stows_prevs.len(),
            });

            if self.hist_stows.len()!=cur.hist_stows_len+1 {
                panic!("");
            }

        }

        self.hist_stows.len()
    }


    fn hist_news_add(&mut self,cur:&Work<'t,'g>) -> usize {
        // return self.hist_news.len();

        if cur.from_user
            && (!self.hist_non_term_only || cur.grammar.is_non_term())
            // // && (cur.grammar.is_primtive() || cur.grammar.is_non_term())
            // && cur.grammar.is_non_term() //should only do nonterms?

            && cur.first //no longer using prevs, only stows/fails
        { //ignore grammars added by walker
            self.hist_news.push(TempHistNew {
                grammar: cur.grammar.clone(),
                tokens_start: cur.tokens.clone(),
                // group_ind: cur.group_ind,
                group_len:cur.group_len,
                is_first:cur.first
                    // &&cur.and_first
                ,
                hist_stows_len:cur.hist_stows_len,
                hist_fails_len:cur.hist_fails_len,
            });

            return self.hist_news.len();
        }

        cur.hist_news_len

        // self.hist_news.len()
    }

    fn hist_stows_truncate(&mut self,hist_stows_len:usize) {

        //
        if self.debug {
            if self.hist_stows.len() != hist_stows_len {
                println!("------ hist_stows_truncate {}=>{}", self.hist_stows.len(), hist_stows_len);
            }
        }

        //
        self.hist_stows.truncate(hist_stows_len);

        //
        if let Some(hist_stow)=self.hist_stows.last() {
            // let (groups_len,prevs_len)=if let Some(hist_stow_val)= &hist_stow.val {
            //     (hist_stow_val.stow_groups_end,hist_stow_val.stow_prevs_end)
            // } else {
            //     (hist_stow.stow_groups_start,hist_stow.stow_prevs_start)
            // };
            let groups_len=if let Some(hist_stow_val)= &hist_stow.val {
                hist_stow_val.stow_groups_end
            } else {
                hist_stow.stow_groups_start
            };

            self.hist_stows_groups.truncate(groups_len);
            // self.hist_stows_prevs.truncate(prevs_len);
        }
    }

    // fn step_truncates(&mut self,cur :&Work<'t,'g>) {
    //     //should move all these to run on success/fails of prims/prev/always/take

    //     // self.groups.truncate(cur.group_len);
    //     // self.hist_news.truncate(cur.hist_news_len);

    //     // self.hist_stows_stk.truncate(cur.hist_stows_stk_len);
    //     // self.hist_ends_stk.truncate(cur.hist_ends_stk_len);

    //     // self.hist_stows_elements.truncate(cur.hist_stows_len);

    //     //
    //     // self.hist_prevs.truncate(cur.hist_prevs_len);

    // }


    fn groups_on_fail(&mut self,) {
        let Some(last)=self.stk.last() else {panic!("");};

        self.groups.truncate(last.group_len);
    }

    fn groups_on_success(&mut self,cur :&Work<'t,'g>,
        // cur_group_ind:usize,
        // cur_primitives:TokenIterContainer<'t>,
    ) {
        let Some(last)=self.stk.last_mut() else {return;};

        //
        last.group_len=cur.group_len;

        //
        // if self.debug {
        //     println!("==do_groups_primitives_clamp: cur_group_ind={}, last.group_ind={}",cur.group_ind,last.group_ind);
        // }

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

        //
        self.groups.truncate(last.group_len);

    }

    // fn do_non_term_visiteds(&mut self,
    //     t:&'g str,
    //     cur_primitives:TokenIterContainer<'t>,
    //     cur_visiteds: HashSet<(&'g str, usize)>,
    // ) -> Result<HashSet<(&'g str, usize)>,GrammarWalkError<'g>> {
    //     //
    //     if !self.non_term_recursive_check { return  Ok(Default::default()); }

    //     //
    //     let v=(t,cur_primitives.inds().start);

    //     //
    //     if cur_visiteds.contains(&v) { return Err(GrammarWalkError::RecursiveNonTerm(t)); }

    //     //
    //     let mut visiteds=cur_visiteds;
    //     visiteds.insert(v);

    //     //
    //     Ok(visiteds)
    // }

    fn new_group(&mut self,cur:&Work<'t,'g>) -> (usize,usize) {
        let GrammarNode::Group(_,name)=cur.grammar.as_ref() else {panic!("");};
        let parent=cur.group_ind;
        let tokens=cur.tokens;

        let new_group_ind=self.groups.len();
        self.groups.push(TempGroup { name, parent, tokens, });
        (new_group_ind,self.groups.len())
    }

    fn update_tokens(&mut self,cur:&Work<'t,'g>, set_last_tokens:bool) {
        if self.stk.is_empty() {
            self.tokens_remaining=cur.tokens;
        } else if set_last_tokens {
            let Some(last)=self.stk.last_mut() else {panic!("");};
            last.tokens=cur.tokens;
        }
    }

    fn handle_exit_last_many(&mut self,cur:&Work<'t,'g>) { //if not parsing anything, exit the many
        let Some(last)=self.stk.last_mut() else {return;};
        if !last.grammar.is_many() || last.tokens.len()!=cur.tokens.len() {return;}
        last.grammar=Rc::new(GrammarNode::Always);
    }

    pub fn last_loc(&self) -> Loc {
        println!("l1 {:?} {:?} || {:?}",self.tokens_remaining.loc(),self.tokens_remaining.last_loc(),self.tokens_remaining);
        println!("l2 {:?} {:?} || {:?}",self.expected_tokens_remaining.loc(),self.expected_tokens_remaining.last_loc(),self.expected_tokens_remaining);
        println!("{:?}:{}:{}",self.top_tokens,self.top_tokens.loc(),self.top_tokens.last_loc());
        println!("{:?}:{}:{}",self.tokens_remaining,self.tokens_remaining.loc(),self.tokens_remaining.last_loc());
        println!("{:?}:{}:{}",self.expected_tokens_remaining,self.expected_tokens_remaining.loc(),self.expected_tokens_remaining.last_loc());

        for t in self.top_tokens {
            println!("t {t:?} :: {} to {}",t.start_loc(),t.end_loc());
        }
        let out_loc=if self.expecteds.is_empty() {
            self.tokens_remaining.loc()
        } else {
            self.expected_tokens_remaining.loc()
        };
        println!("l3 {out_loc:?}");
        out_loc
    }

    //

    fn organise_expecteds(&mut self) {
        println!("dsfsd");
        for (i,x) in self.expecteds.iter().enumerate() {
            // println!("e {:?} || {:?} || {} => {} || {:?}",x.expected_type,x.tokens_start.inds().start,x.tokens_start.loc(),x.tokens_start.last_loc(),x.tokens_start.inds());

            println!("e{i}:p{}:t{} {:?} :: {:?}",
                x.parent.map(|q|format!("{q}")).unwrap_or("_".to_string()),
                x.tokens_start.inds().start,
                x.expected_type,
                x.tokens_start,
            );

        }
        let max_token = self.expecteds.iter().map(|x|x.tokens_start).max_by(|x,y|x.inds().start.cmp(&y.inds().start)).unwrap_or(self.tokens_remaining);
        let max_token_start_ind=max_token.inds().start;

        self.expected_tokens_remaining=max_token;
        // self.expected_loc=max_token.loc();

        let parents= self.expecteds.iter().filter_map(|x|x.parent).collect::<HashSet<_>>();

        let expecteds=self.expecteds.iter().enumerate().rev().filter_map(|(i,x)|(
            x.tokens_start.inds().start == max_token_start_ind &&
            !parents.contains(&i)
        ).then(||(x.expected_type.clone(),x.clone()))).collect::<BTreeMap<_,_>>();

        // if self.debug {
        //     println!("hmmm {:?}",expecteds);
        // }


        self.expecteds=expecteds.iter().map(|(_k,v)|v.clone()).collect();
        // self.expecteds=self.expecteds.iter().enumerate().rev().filter_map(|(i,x)|(
        //     x.tokens_start.inds().start == max_token_start_ind && !parents.contains(&i)
        // ).then(||x.clone())).collect();

        println!("--");
        for x in self.expecteds.iter() {
            println!("e2 {:?} {:?}",x.expected_type,x.tokens_start.inds().start);
        }
    }

    //
    pub fn expecteds_string(&self) -> String {
        self.expecteds.iter().rev().map(|x|match &x.expected_type {
            TempExpectedType::Expected(n) => n,
            TempExpectedType::Int => "int",
            TempExpectedType::Float => "float",
            TempExpectedType::String => "string",
            TempExpectedType::Identifier => "identifier",
            TempExpectedType::Symbol(s) => s,
            TempExpectedType::Keyword(s) => s,
            TempExpectedType::Eol => "eol",
        }).collect::<Vec<_>>().join(", ")
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
        if self.debug {
            println!("groups2 {:?}",group_infos2.iter().enumerate().collect::<Vec<_>>());

                //
                for (i,&(g,p,)) in group_infos2.iter().enumerate() {
                    //
                    let group_infos=&self.groups;

                    //
                    println!("\t{i}: g{g}, p{p}, {:?}, {:?}, {:?}",group_infos[g].name,group_infos[g].tokens.inds(),group_infos[g].tokens);
                }
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


                //
                if self.debug {
                    // let err_loc=self.last_loc();

                    match e {
                        GrammarWalkError::RecursiveNonTerm(t) => {
                            println!("Recursive NonTerm {t:?}, At {}",self.tokens_remaining.loc());
                        }
                        GrammarWalkError::MissingNonTerm(t) => {
                            println!("Missing NonTerm {t:?}, At {}",self.tokens_remaining.loc());
                        }
                        GrammarWalkError::FailedParse => {

                            println!("Failed parse, At {}, expected {:?}",self.last_loc(),"self.expecteds_string()");
                        }
                        GrammarWalkError::Unfinished =>{}
                    }
                }

                result=Err(e);
                break;
           } else {
           }
        }

        //
        if self.debug {
            println!("groups={:?}",self.groups);
        }

        //


        //
        if !result.is_err() && !self.tokens_remaining.is_empty() {
            if self.debug {
                // println!("error, failed to parse all tokens {:?}",self.primitives_remaining);
                println!("error, failed to parse all tokens, at {}",self.last_loc());
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
        if result.is_err() {
            self.organise_expecteds();
        }
        //
        if self.debug {
            println!("===a {}",self.tokens_remaining.is_empty());
        }

        //
        if self.debug {
            for (i,g) in self.groups.iter().enumerate() {
                println!("g{i}: {:?} {:?}",g.name,g.tokens);
            }
        }

        //
        if self.debug {
            println!("top_primitives={:?}", self.top_tokens );
        }

        //
        result
    }

    fn step(&mut self,cur:Work<'t,'g>) -> Result<(),GrammarWalkError<'g>> {
        //
        // if self.debug {
        //     if self.groups.len() != cur.group_len {
        //         println!("--- groups dif len, groups.len={}, cur.group_len={}",self.groups.len(),cur.group_len);
        //     }
        //     // if self.hist_stows_stk.len() != cur.hist_stows_stk_len {
        //     //     println!("--- or_stk dif len, or_stk.len={}, cur.or_stk_len={}",self.hist_stows_stk.len(),cur.hist_stows_stk_len);
        //     // }
        // }

        //
        // self.step_truncates(&cur);

        //
        self.step_count+=1;

        //
        if self.debug {

            {
                //
                let groups=&self.groups;

                //
                let c=self.step_count;

                //
                let Work {
                    grammar, work_stk_success_len: success_len, work_stk_fail_len: fail_len, tokens,
                    group_ind, group_len,
                    // and_id,
                    first,
                    hist_news_len,
                    // hist_stows_stk_len,hist_ends_stk_len,
                    // hist_stows_ind,
                    hist_stows_len,
                    // hist_stows_stk_len,
                    hist_fails_len,
                    // hist_prevs_ind,
                    // hist_prevs_len,
                    // expected_news_len,expecteds_len,
                    expected_ind,expecteds_len,
                    // was_start_ind,was_ind,was_len,
                    was_new_len2,was_ind2,
                    ..
                }=&cur;

                //
                let ps=tokens.inds();
                let temp_groups=groups.iter().enumerate().map(|(i,x)|format!("g{i}:p{}:{}",x.parent,x.name)).collect::<Vec<_>>();
                let groups_len2=groups.len();

                // let grammar2= match grammar.as_ref() {
                //     GrammarNode::And(gs) => {
                //         format!("And({:?})",&gs[cur.grammar_ind..])
                //     }
                //     GrammarNode::Or(gs) => {
                //         format!("Or({:?})",&gs[cur.grammar_ind..])
                //     }
                //     _ => {
                //         format!("{grammar:?}")
                //     }
                // };

                //
                println!("");
                println!("=>{c:4}: {grammar:?}, ps={ps:?}, success={success_len}, fail={fail_len}, ",);

                if false {
                    // println!("        and_id={and_id}, groups.len={groups_len2}, group_ind={group_ind}, group_len={group_len}, gs={temp_groups:?}",);
                    println!("        groups.len={groups_len2}, group_ind={group_ind}, group_len={group_len}, gs={temp_groups:?}",);


                    // println!("        first={is_first}, hist_news_len={hist_news_len}, hist_stows_stk_len={hist_stows_stk_len}:{}, hist_ends_stk_len={hist_ends_stk_len}:{}, ",
                    //     self.hist_stows_stk.last().map(|x|x.elements.len()).unwrap_or_default(),
                    //     self.hist_ends_stk.last().map(|x|x.elements.len()).unwrap_or_default(),
                    // );
                    // let hist_stows_len=if *hist_stows_stk_len==0{None}else{
                    //     self.hist_stows_stk.get(hist_stows_stk_len-1).map(|x|x.elements.len())
                    // };
                    // println!("        first={first}, hist_news_len={hist_news_len}, hist_stows_len={hist_stows_len:?}, hist_prevs_ind={hist_prevs_ind}, hist_prevs_len={hist_prevs_len}",);
                    // println!("        first={first}, hist news_len={hist_news_len}, stows_len={hist_stows_len:?}, prevs_ind={hist_prevs_ind}, fails_len={hist_fails_len},",);
                    // println!("        actual: hist news_len={}, stows_len={:?}, prevs_len={}, fails_len={}",
                    //     self.hist_news.len(),self.hist_stows.len(),self.hist_prevs.len(),self.hist_fails.len(),
                    // );
                    println!("        first={first}, hist news_len={hist_news_len}, stows_len={hist_stows_len:?}, fails_len={hist_fails_len},",);
                    println!("        actual: hist news_len={}, stows_len={:?}, fails_len={}",
                        self.hist_news.len(),self.hist_stows.len(),self.hist_fails.len(),
                    );
                    // println!("        hist_stows_ind={hist_stows_ind}, hist_stows_len={hist_stows_len},",
                    //     self.stk.get(cur.)
                    // );

                }

                //
                if false {
                    println!("        expected_ind={expected_ind:?}, expecteds_len={expecteds_len}, expecteds.len={}",self.expecteds.len());

                     println!("        expecteds=[{}]",
                        self.expecteds.iter().enumerate()
                            .map(|(i,x)|format!("e{i}:p{}:t{}:{:?}",
                                x.parent.map(|q|format!("{q}")).unwrap_or("_".to_string()),
                                x.tokens_start.inds().start,
                                x.expected_type,
                            ))
                            .collect::<Vec<_>>().join(", "),
                    );
                }

                //
                if true {
                    //

                     println!("        was_news: len={was_new_len2} ({})",self.was_news.len());

                    for (i,w) in self.was_news.iter().enumerate().rev() {
                        println!("            {i}: {:?}",w.name);
                    }

                    //
                     println!("        wases: ind={was_ind2}, len= ({})",self.wases.len());

                    for (i,w) in self.wases.iter().enumerate().rev() {
                        println!("            {i}: {:?}",w.name);
                    }
                }
                // //
                // if true {
                //      println!("        wases: start_ind={was_start_ind}, ind={was_ind}, len={was_len} ({})",self.wases.len());

                //     for (i,w) in self.wases.iter().enumerate() {
                //         println!("            {i}: {:?}",w.name);
                //     }
                // }

                //
                // if false {
                //     println!("        expected_news_len={expected_news_len}, expecteds_len={expecteds_len}");
                //     println!("        expected news={:?}",
                //         self.expected_news.iter().map(|x|&x.expected_type).collect::<Vec<_>>(),
                //     );
                //      println!("        expected={:?}",
                //         self.expecteds.iter().map(|x|&x.expected_type).collect::<Vec<_>>(),
                //     );

                // }

                //
                if false {
                    //
                    println!("        hist_news");

                    for (i,h) in self.hist_news.iter().enumerate() {
                        println!("            {i}:[{:?}]: {:?}",h.tokens_start.inds(),h.grammar)
                    }

                    //
                    println!("        hist_stows {hist_stows_len} ({})",self.hist_stows.len());

                    if true {
                        for (i,x) in self.hist_stows.iter().enumerate().rev() {
                            println!("            {i}:{}",x.val.as_ref().map(|y|format!("{:?}",&y.grammar)).unwrap_or("_".to_string()));
                        }

                    } else
                    if *hist_stows_len!=0 {
                        let hist_stow=self.hist_stows.last().unwrap();
                        if let Some(hist_stow_val)=&hist_stow.val {
                            let hist_stow_groups=&self.hist_stows_groups[
                                hist_stow.stow_groups_start..hist_stow_val.stow_groups_end
                            ];
                            // let hist_stow_prev=&self.hist_stows_prevs[
                            //     hist_stow.stow_prevs_start..hist_stow_val.stow_prevs_end
                            // ];

                            println!("            grammar={:?}",hist_stow_val.grammar);
                            println!("            groups={:?}",hist_stow_groups.iter().enumerate().map(|(i,g)|format!("{i}:p{}:{}",
                                g.parent,
                                g.name,
                            )).collect::<Vec<_>>());
                        }
                    }

                    // //
                    // println!("        hist_prevs_last {hist_prevs_ind}..{hist_prevs_len} : {}",
                    //     self.hist_prevs.len(),
                    // );

                    // // for i in *hist_prevs_ind.. self.hist_prevs.len() //*hist_prevs_len
                    // for (i,x) in self.hist_prevs[*hist_prevs_ind..].iter().enumerate()
                    // {
                    //     // let x=&self.hist_prevs[i];
                    //     println!("            {i}:[{:?}]: {:?}",x.tokens_start_ind,x.grammar)
                    // }

                    //
                    println!("        hist_fails {} : ({})",cur.hist_fails_len,self.hist_fails.len());

                    if false {

                        for (i,x) in self.hist_fails.iter().enumerate().rev() {
                            println!("            {i}: {:?}",x.grammers);
                        }
                    } else
                    if cur.hist_fails_len!=0 {
                        let his_fails_last=&self.hist_fails[cur.hist_fails_len-1].grammers;

                        for (i,x) in his_fails_last.iter().enumerate() {
                            println!("            {i}: {x:?}",);
                        }

                    }

                }

                //
                println!("        tokens {tokens:?}");
            }

            //
            if false {
                for (i,Work {grammar:g, work_stk_success_len:s, work_stk_fail_len:f, tokens,
                    group_ind, group_len,..}) in self.stk.iter().enumerate()
                {
                    // println!("    {i:3}: ps={:?}, success={s}, fail={f}, and_id={and_id}, group_ind={group_ind}, group_len={group_len}, {g:?},",tokens.inds());
                    println!("    {i:3}: ps={:?}, success={s}, fail={f}, group_ind={group_ind}, group_len={group_len}, {g:?},",tokens.inds());
                }
            }
        }

        //
        if cur.group_ind>=self.groups.len() {
            panic!("invalid group_ind={}, groups_len={}",cur.group_ind,self.groups.len());
        }

        // //try take from hist fails
        if self.grammar_try_from_hist_fails(&cur) {return Ok(());}

        // //try take from hist begins
        if self.grammar_try_from_hist_stows(&cur) {return Ok(());}

        //
        match cur.grammar.as_ref() {
            GrammarNode::Expected(..) => {self.grammar_expect(cur);}
            GrammarNode::Was(..) => {self.grammar_was(cur);}
            GrammarNode::Had(..) => {self.grammar_had(cur);}
            // GrammarNode::Stow(..) => {self.grammar_stow(cur);}
            // GrammarNode::Prev(..) => {self.grammar_prev(cur);}
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
                let s= *s;
                let Some(v)=self.grammar_primitive(cur,|ps|ps.pop_with_symbol(s),) else{return Ok(());};
                if self.debug {println!("--- symbol {v:?}");}
            }
            GrammarNode::Keyword(s) => {
                let s= *s;
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

    pub fn set_hist_non_term_only(&mut self,hist_non_term_only:bool) {
        self.hist_non_term_only=hist_non_term_only;
    }


    fn get_non_term(&mut self,n:&'g str) -> Rc<GrammarNode<'g>> {
        if let Some(g)=self.non_term_cache.get(n) {
            g.clone()
        } else if let Some(g)=(self.grammar_func)(n) {
            self.non_term_cache.insert(n, g.clone());
            g
        } else {
            Rc::new(GrammarNode::Error(GrammarWalkError::MissingNonTerm(n)))
        }
    }
    pub fn step_count(&self) -> usize {
        self.step_count
    }
}