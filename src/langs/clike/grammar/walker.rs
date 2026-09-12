
/*

for err loc, should use end_loc of last token

TODO
* record only last fail, instead of all of them?

NOTE
* problem with expects, if have or(and(A,B), Eol)
    if A succeeds, but B fails, so have B in expects
    but for input there is an Eol before A, then the
    Eol in the OR will succeed, clearing the B from expects

    currently the solution is to do  or(and(A,or(B, Error)), Eol)
    so that it will stop if A succeeds, but B fails

    slight issues if or(and(A,B), and(A,C), Eol)
        and if you wanted it to report B or C expected
        could do
            X => and(A, or(B,C,Error))
        then
            or(
                groupB(and(X hadB)),
                groupC(and(X hadC)),
            )

NOTE
* need to point to errors at end of token, at token or maybe before?
** like if expecting semicolon, should point to end of token it ws epxected after
** if unknown token eg no expects, then should point at

NOTE
* a parse might actually end up on a success, but fail to parse all the input
** due to things in the grammar being optional

TODO
* make skipped eol's not be included in start of a group?
** so then don't need filtered_token_iter


TODO
* replace non terms with generic, so can use enums for them
** could have NonTerms{Start,NonTerm(NT)}
*** so can declare enum hidden inside of rules func

* also allow group name to be a generic, ie enum
** or a func to handle putting it in an enum or whatever

TODO
* on grammar_primitive bein run
** if the primitive wasn't trimmable, and tokens needed to be trimmed
*** need to some how when ending the group, to trim the group;s token
**** maybe set a flag in work, and on success, pass it on

TODO
* have option to add groups for tokens that are inbetween sibinling groups
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
use crate::clike::grammar;
use crate::clike::grammar::GrammarPrimitiveTrait;
use crate::clike::grammar::TokenIterGetTrait;
use crate::clike::grammar::TokenIterTrait;
// use crate::clike::tokenizer::TokenContainer;
use super::super::grammar::data::{Walk,WalkGroup};
use super::super::tokenizer::{TokenIterContainer, ValueContainer};

use super::node::*;
use std::fmt::Debug;

// use data::*;
// use error::*;


pub struct GrammarWalker<'g,P,T,TS,G>
where
    P:Clone+core::hash::Hash+PartialEq+Eq,
    TS:Iterator<Item=T>+Clone,
    G: Fn(&str)->Option<Rc<GrammarNode<'g,P>>>,
{
    non_term_cache:HashMap<&'g str, Rc<GrammarNode<'g,P>>>,

    work_error_len:usize, //what is this?

    top_tokens:TS,
    tokens_remaining: TS,
    tokens_furthest: TS,
    expected_tokens_remaining1: TS,
    expect_token_start2: TS,

    grammar_func:G,
    stk: Vec<Work<'g,P,TS>>,
    step_count:usize,

    use_expect1:bool,
    use_expect2:bool,

    expects1:Vec<TempExpect1<'g,TS>>,

    expect_news2:Vec<TempExpectNew2<'g,TS>>,
    expects2:Vec<TempExpect2<'g,TS>>,
    expects_temp2:Vec<TempExpect2<'g,TS>>,

    debug:bool,

    groups:Vec<TempGroup<'g,TS>>,

    stow_news:Vec<TempStowNew<'g,P,TS>>,
    stows:Vec<TempStow<'g,P,TS>>,
    stow_groups:Vec<TempGroup<'g,TS>>,

    was_news:Vec<TempWas<'g>>,
    wases:Vec<TempWas<'g>>,
}

impl<'g,P,T,TS,G> GrammarWalker<'g,P,T,TS,G>
where
    P:Clone+core::hash::Hash+PartialEq+Eq+Debug+GrammarPrimitiveTrait,
    T:Clone,
    TS: Iterator<Item=T>+Clone+TokenIterTrait+Debug + TokenIterGetTrait<P> ,
    G: Fn(&str)->Option<Rc<GrammarNode<'g,P>>>,
{

    pub fn new(top_primitives:TS, grammar_func:G,) -> Self {
        Self {
            work_error_len:0,
            // always:Rc::new(GrammarNode::Always),
            non_term_cache:Default::default(),
            // // prev_non_term_only:true,
            // // stow_non_term_only:true,
            // hist_non_term_only:false,

            stk:Default::default(),
            step_count:Default::default(),

            use_expect1:false, //old implementation
            use_expect2:true, //new implementation

            // expected_loc:Loc::zero(),
            expected_tokens_remaining1:top_primitives.clone(),
            expects1:Default::default(),

            expect_token_start2:top_primitives.clone(),
            expect_news2:Default::default(),
            expects2:Default::default(),
            expects_temp2:Default::default(),

            grammar_func,
            tokens_remaining:top_primitives.clone(),

            tokens_furthest:top_primitives.clone(),
            top_tokens: top_primitives,
            debug:false,
            // non_term_recursive_check:true,
            // non_term_visiteds_stk:Default::default(),
            // recurse_num:0,

            groups:Default::default(),

            stow_news: Default::default(),

            // hist_stows_stk:Default::default(),
            stows:Default::default(),
            stow_groups:Default::default(),
            // // hist_stows_prevs:Default::default(),
            // // hist_stows_elements: Default::default(),
            // // hist_prevs: Default::default(),
            // hist_fails:Default::default(),

            // // hist_stows_stk: Default::default(),
            // // hist_ends_stk: Default::default(),

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

    fn init(&mut self,start_non_term:&'g str,) -> Result<(),GrammarWalkError<'g>> {
        self.non_term_cache.clear(); //not necessary ...

        self.stk.clear();

        //
        self.tokens_remaining=self.top_tokens.clone();
        self.tokens_furthest=self.top_tokens.clone();
        self.expected_tokens_remaining1=self.top_tokens.clone();

        //
        self.stk.push(Work{
            // grammar:Rc::new(GrammarNode::Error(GrammarWalkError::FailedParse)),
            grammar:Rc::new(GrammarNode::Error),
            // grammar_ind:0,
            work_success_len:0,work_fail_len:0,
            tokens:self.top_tokens.clone(),
            group_ind: 0, group_len: 1,
            // visiteds:Default::default(),
            // non_term_visiteds_stk_len:0,
            // grammar_debug_len: 0,
            // and_id: 0,

            grammar_ind:0,
            user:false,
            first:false,
            stow:false,
            // or_id:0,
            // and_first:false,

            stow_new_len:0,


            // hist_stows_stk_len:0,
            // hist_ends_stk_len:1,

            // hist_stows_stk_len:0,

            // hist_stows_ind: 0,
            stow_len: 0,

            // hist_fails_len:0,

            // in_or:false,
            // can_hist_stow:false,

            // hist_prevs_ind: 0,
            // hist_prevs_len: 0,


            in_expect:false,

            expect_ind1:None,
            expect_len1:0,

            expect_new_len2:0,
            expect_len2:0,

            // was_start_ind:0,
            // was_ind:0,
            // was_len:0,

            // had_ind:0,
            // had_len:0,

            was_new_len:0,
            was_ind:0,

            // trim:false,

        });

        //
        self.work_error_len=self.stk.len();

        //
        let fail_len=self.stk.len();

        //no needed, but allows takeables2 to finish, for debugging purposes
        self.stk.push(Work{
            grammar : Rc::new(GrammarNode::Always), //self.always.clone(),
            // grammar_ind:0,
            work_success_len:0,
            work_fail_len:0, //not used
            tokens:self.top_tokens.clone(),
            group_ind: 0, group_len: 1,
            // visiteds:Default::default(),
            // non_term_visiteds_stk_len:0,
            // grammar_debug_len: 1,
            // and_id: 0,

            grammar_ind:0,
            user:true,
            first:false,
            stow:false,
            // or_id:0,
            // and_first:false,

            // in_or:false,
            // can_hist_stow:false,

            stow_new_len:0,


            // hist_stows_stk_len:0,
            // hist_ends_stk_len:1,

            // hist_stows_stk_len:0,

            // hist_stows_ind: 0,
            stow_len: 0,

            // hist_fails_len:0,

            // hist_prevs_ind: 0,
            // hist_prevs_len: 0,

            in_expect:false,

            expect_ind1:None,
            expect_len1:0,

            expect_new_len2:0,
            expect_len2:0,

            // was_start_ind:0,
            // was_ind:0,
            // was_len:0,
            // had_ind:0,
            // had_len:0,

            was_new_len:0,
            was_ind:0,

            // trim:false,
        });

        //
        let success_len=self.stk.len();

        //start
        {
            let grammar=self.get_non_term(start_non_term)?;
            // let grammar=if let Some(g)=(self.grammar_func)(start_non_term) {
            //     g
            // } else {
            //     Rc::new(GrammarNode::Error(GrammarWalkError::MissingNonTerm(start_non_term)))
            // };

            self.stk.push(Work{
                grammar, //:(self.grammar_func)(start_non_term),
                // grammar_ind:0,
                // success_len:0,
                work_success_len: success_len,
                work_fail_len: fail_len, //1
                tokens:self.top_tokens.clone(),
                group_ind: 0, group_len: 1,
                // visiteds:Default::default(),
                // non_term_visiteds_stk_len:0,
                // grammar_debug_len: 1,
                // and_id: 0,

                grammar_ind:0,
                user:true,
                first:false,
                stow:false,
                // or_id:0,
                // and_first:false,

                // in_or:false,
                // can_hist_stow:false,

                stow_new_len:0,
                // hist_stows_stk_len:0,
                // hist_ends_stk_len:1,

                // hist_stows_stk_len:0,

                // hist_stows_ind: 0,
                stow_len: 0,

                // hist_fails_len:0,

                // hist_prevs_ind: 0,
                // hist_prevs_len: 0,

                in_expect:false,

                expect_ind1:None,
                expect_len1:0,

                expect_new_len2:0,
                expect_len2:0,

                // was_start_ind:0,
                // was_ind:0,
                // was_len:0,
                // had_ind:0,
                // had_len:0,

                was_new_len:0,
                was_ind:0,

                // trim:false,
            });
        }

        //
        self.groups=vec![TempGroup{
            name: "",
            parent: 0,
            tokens:self.top_tokens.clone(),
            trim:false,
        }];

        //
        // self.non_term_visiteds_stk.clear(); //not necessary really...

        //
        self.stow_news.clear();

        // self.hist_stows_stk.clear();
        self.stows.clear();
        self.stow_groups.clear();
        // // self.hist_stows_prevs.clear();

        // // self.hist_stows_elements.clear();
        // // self.hist_prevs.clear();
        // self.hist_fails.clear();


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
        self.expects1.clear();

        self.expect_token_start2=self.top_tokens.clone();
        self.expect_news2.clear();
        self.expects2.clear();
        // self.expects_temp2.clear(); //not needed

        Ok(())

    }

    // fn grammar_stow(&mut self,cur :Work<'g,P,TS>,) {
    //     let GrammarNode::Stow(g, )=cur.grammar.as_ref() else{panic!("");};

    //     //
    //     let stow_new_len=self.hist_news_add(&cur);

    //     //
    //     self.stk.push(Work {
    //         grammar: g.clone(),

    //         work_success_len: cur.work_success_len,
    //         work_fail_len: cur.work_fail_len,
    //         tokens: cur.tokens,
    //         group_ind: cur.group_ind,
    //         group_len: cur.group_len,

    //         grammar_ind:0,
    //         user:true,
    //         first:cur.first,
    //         stow:cur.stow,
    //         // or_id:cur.or_id,

    //         stow_new_len,

    //         stow_len: cur.stow_len,

    //         in_expect:cur.in_expect,

    //         expect_ind1:cur.expect_ind1,
    //         expect_len1:cur.expect_len1,

    //         expect_new_len2:cur.expect_new_len2,
    //         expect_len2:cur.expect_len2,

    //         was_new_len:cur.was_new_len,
    //         was_ind:cur.was_ind,
    //     });
    // }

    fn grammar_was(&mut self,cur :Work<'g,P,TS>,) {
        let GrammarNode::Was(g,name, )=cur.grammar.as_ref() else{panic!("");};

        //
        // let stow_new_len=self.hist_news_add(&cur);

        //
        // let was_ind=self.wases.len();
        // let was_ind=cur.was_ind+1;
        // let was_ind=cur.was_len;
        // self.wases.push(TempWas{name});
        // let was_len=cur.was_len+1; //self.wases.len();


        self.was_news.push(TempWas{name});
        // let was_new_len=cur.was_new_len+1;//self.was_news.len();
        let was_new_len=self.was_news.len();

        //
        self.stk.push(Work {
            grammar: g.clone(),

            work_success_len: cur.work_success_len,
            work_fail_len: cur.work_fail_len,
            tokens: cur.tokens.clone(),
            group_ind: cur.group_ind,
            group_len: cur.group_len,

            grammar_ind:0,
            user:true,
            first:cur.first,
            stow:cur.stow,
            // or_id:cur.or_id,

            // stow_new_len,
            stow_new_len:cur.stow_new_len,

            stow_len: cur.stow_len,

            // hist_fails_len: cur.hist_fails_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            in_expect:cur.in_expect,

            expect_ind1:cur.expect_ind1,
            expect_len1:cur.expect_len1,

            expect_new_len2:cur.expect_new_len2,
            expect_len2:cur.expect_len2,

            // was_start_ind:cur.was_start_ind,
            // // was_ind:cur.was_ind,
            // was_ind,
            // was_len,
            // had_ind:cur.had_ind,
            // had_len:cur.had_len,

            was_new_len,
            was_ind:cur.was_ind,

            // trim:cur.trim,
        });
    }

    fn grammar_had(&mut self,cur :Work<'g,P,TS>,) {
        let GrammarNode::Had(name,)=cur.grammar.as_ref() else{panic!("");};

      //
        // let _hist_news_len=self.hist_news_add(&cur);
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
            self.work_stk_truncate(cur.work_success_len);

            //whats to stop a and(X, many(prev(X))) ?
            self.handle_exit_last_many(&cur); //this

            //
            // self.hist_news_truncate_to_last(); //why on success??
            self.update_tokens(cur.tokens.clone(),true);
            self.groups_on_success(cur.group_ind,cur.group_len,cur.tokens.clone());
            self.was_on_success(false); //before hist
            self.hist_on_success(cur.grammar.clone(),cur.group_len,cur.tokens.clone(),false,);
            self.expect_on_success2();
            self.expect_on_success1();
        } else {
            // self.stk.truncate(cur.fail_len);
            self.work_stk_truncate(cur.work_fail_len);
            self.update_tokens(cur.tokens.clone(),false);
            // // self.revert_last_hist_news();
            self.hist_on_fail();
            self.was_on_fail();
            self.groups_on_fail();
            //don't add expected, let user manually add one
            // // // let _expected_news_len=self.add_expected_new(&cur);
            // // // let (_expected_ind,_expecteds_len)=self.add_expected2(&cur);

            // // self.submit_expected_news(&cur);
            self.expect_on_fail2();
            self.expect_on_fail1();
        }
    }

    fn grammar_expect(&mut self,cur :Work<'g,P,TS>,) {
        let GrammarNode::Expect(g,_, )=cur.grammar.as_ref() else{panic!("");};

        //
        // let expected_news_len=self.add_expected_new(&cur);
        let (expect_ind1,expect_len1)=self.add_expect1(&cur);
        // let stow_new_len=self.hist_news_add(&cur);

        let expect_new_len2=self.add_expect_new2(&cur);

        //
        self.stk.push(Work {
            grammar: g.clone(),
            // grammar_ind:0,
            work_success_len: cur.work_success_len,
            work_fail_len: cur.work_fail_len,
            tokens: cur.tokens.clone(),
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len+1,
            // and_id:cur.and_id,

            grammar_ind:0,
            user:true,
            first:cur.first,
            stow:cur.stow,

            // or_id:cur.or_id,
            // and_first:cur.and_first,

            // in_or:cur.in_or,
            // can_hist_stow:false,

            // stow_new_len,
            stow_new_len:cur.stow_new_len,
            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            stow_len: cur.stow_len,

            // hist_fails_len: cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len,
            // // expected_news_len:cur.expected_news_len,
            // expect_len:cur.expect_len,

            in_expect:true,

            expect_ind1,
            expect_len1,



            expect_new_len2,
            expect_len2:cur.expect_len2,

            // expect_ind:cur.expect_ind,
            // expect_len:cur.expect_len,


            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,
            // had_ind:cur.had_ind,
            // had_len:cur.had_len,

            was_new_len:cur.was_new_len,
            was_ind:cur.was_ind,

            // trim:cur.trim,
        });
    }

    fn grammar_group(&mut self,cur :Work<'g,P,TS>,) {
        let GrammarNode::Group(g,_, )=cur.grammar.as_ref() else{panic!("");};

        //
        let (group_ind,group_len)=self.new_group(&cur); //name, cur.group_ind, cur.tokens
        // let stow_new_len=self.hist_news_add(&cur);

        //
        self.stk.push(Work {
            grammar: g.clone(),
            // grammar_ind:0,
            work_success_len: cur.work_success_len,
            work_fail_len: cur.work_fail_len,
            tokens: cur.tokens.clone(),
            group_ind,
            group_len,
            // visiteds:cur.visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len+1,
            // and_id:cur.and_id,

            grammar_ind:0,
            user:true,
            first:cur.first,
            stow:cur.stow,

            // or_id:cur.or_id,
            // and_first:cur.and_first,

            // can_hist_stow:false,
            // stow_new_len,
            stow_new_len:cur.stow_new_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            stow_len: cur.stow_len,

            // hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expect_len:cur.expect_len,

            in_expect:cur.in_expect,

            expect_ind1:cur.expect_ind1,
            expect_len1:cur.expect_len1,

            expect_new_len2:cur.expect_new_len2,
            expect_len2:cur.expect_len2,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,

            was_new_len:cur.was_new_len,
            was_ind:cur.was_ind,

            // trim:cur.trim,
        });
    }

    fn grammar_many(&mut self,cur :Work<'g,P,TS>,) {
        let GrammarNode::Many(g)=cur.grammar.as_ref() else{panic!("");};

        //in always/prev they check if their success_ind is a many (which could be a problem if ands/ors were handled more efficiently),
        //  but could store maybe a many_id to check whether to exit? eg if id is eq, and/or tokens.inds.start is eq?
        //
        // let stow_new_len=self.hist_news_add(&cur);


        let was_ind=self.wases.len();

        //
        self.stk.push(Work {
            // grammar: Rc::new(GrammarNode::Many(g.clone())),
            grammar:cur.grammar.clone(),
            // grammar_ind:0,
            work_success_len: cur.work_success_len,
            work_fail_len: cur.work_fail_len,
            tokens: cur.tokens.clone(),
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds.clone(),
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len,
            // and_id:cur.and_id,

            grammar_ind:cur.grammar_ind+1,
            user:false,
            first:false, //grmmar in many, after the frist one is parsed are no longer firsts
            stow:cur.stow,

            // or_id:cur.or_id,
            // and_first:false,
            // can_hist_stow:false,

            // stow_new_len,
            stow_new_len:cur.stow_new_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            stow_len: cur.stow_len,

            // hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expect_len:cur.expect_len,

            in_expect:cur.in_expect,

            expect_ind1:cur.expect_ind1,
            expect_len1:cur.expect_len1,

            expect_new_len2:cur.expect_new_len2,
            expect_len2:cur.expect_len2,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,


            was_new_len:cur.was_new_len,
            was_ind:cur.was_ind,

            // trim:cur.trim,
        });

        //
        let success_len2=self.stk.len();

        //
        self.stk.push(Work {
            grammar: Rc::new(GrammarNode::Always), //self.always.clone(),
            // grammar_ind:0,
            work_success_len: cur.work_success_len,
            work_fail_len: 0, //fail is not used
            tokens: cur.tokens.clone(),
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds.clone(),
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len,
            // and_id:cur.and_id,

            grammar_ind:0,
            user:false,
            first:false,
            stow:cur.stow,

            // or_id:cur.or_id,
            // and_first:false,
            // can_hist_stow:false,

            // stow_new_len,
            stow_new_len:cur.stow_new_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            stow_len: cur.stow_len,

            // hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expect_len:cur.expect_len,

            in_expect:cur.in_expect,

            expect_ind1:cur.expect_ind1,
            expect_len1:cur.expect_len1,

            expect_new_len2:cur.expect_new_len2,
            expect_len2:cur.expect_len2,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,


            was_new_len:cur.was_new_len,
            // was_ind:cur.was_ind,
            was_ind,

            // trim:cur.trim,
        });

        //
        let fail_len=self.stk.len();

        //
        self.stk.push(Work {
            grammar: g.clone(),
            // grammar_ind:0,
            work_success_len: success_len2,
            work_fail_len: fail_len,
            tokens: cur.tokens.clone(),
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len+1,
            // and_id:cur.and_id,

            grammar_ind:0,
            user:true,
            first:cur.first,
            stow:cur.stow,

            // or_id:cur.or_id,
            // and_first:cur.and_first,
            // can_hist_stow:false,

            // stow_new_len,
            stow_new_len:cur.stow_new_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            stow_len: cur.stow_len,

            // hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expect_len:cur.expect_len,

            in_expect:cur.in_expect,

            expect_ind1:cur.expect_ind1,
            expect_len1:cur.expect_len1,

            expect_new_len2:cur.expect_new_len2,
            expect_len2:cur.expect_len2,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,


            was_new_len:cur.was_new_len,
            // was_ind:cur.was_ind,
            was_ind,

            // trim:cur.trim,
        });
    }

    fn grammar_non_term(&mut self,cur :Work<'g,P,TS>,) -> Result<(),GrammarWalkError<'g>>{
        let GrammarNode::NonTerm(t)=cur.grammar.as_ref() else{panic!("");};

        //
        let stow_new_len=self.hist_news_add(&cur);
        // let visiteds=self.do_non_term_visiteds(t,cur.tokens,cur.visiteds)?;

        //
        // let grammar=if let Some(g)=(self.grammar_func)(t) {
        //     g
        // } else {
        //     Rc::new(GrammarNode::Error(GrammarWalkError::MissingNonTerm(t)))
        // };

        let grammar=self.get_non_term(t)?;

        //
        self.stk.push(Work {
            grammar, //: (self.grammar_func)(t), //should return err on not found, instead of grammar never, should have error
            // grammar_ind:0,
            work_success_len: cur.work_success_len,
            work_fail_len: cur.work_fail_len,
            tokens: cur.tokens.clone(),
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len+1,
            // grammar_debug_len: cur.grammar_debug_len+1,
            // and_id:cur.and_id,

            grammar_ind:0,
            user:true,
            first:cur.first,
            stow:cur.stow,

            // or_id:cur.or_id,
            // and_first:cur.and_first,

            // can_hist_stow:false,

            stow_new_len,
            // stow_new_len:cur.stow_new_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            stow_len: cur.stow_len,

            // hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expect_len:cur.expect_len,

            in_expect:cur.in_expect,

            expect_ind1:cur.expect_ind1,
            expect_len1:cur.expect_len1,

            expect_new_len2:cur.expect_new_len2,
            expect_len2:cur.expect_len2,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,


            was_new_len:cur.was_new_len,
            was_ind:cur.was_ind,

            // trim:cur.trim,
        });

        Ok(())
    }

    fn grammar_error(&mut self,cur :Work<'g,P,TS>,) -> GrammarWalkError<'g> {
        let GrammarNode::Error=cur.grammar.as_ref() else{panic!("");};

        // if self.debug {
        //     println!("====error {:?} ",self.expected_loc,); //self.expecteds,
        // }

        //necesaary? any point to it?
        // if self.expecteds.is_empty() { // self.expected.0.is_zero()
        //     self.expected_loc=cur.primitives.loc();
        // }

        //
        self.expect_on_fail2();
        self.expect_on_fail1();

        // self.expect_on_error2(&cur);
        // self.expect_on_error1(&cur);
        self.update_tokens(cur.tokens.clone(),false); //could be true, but would do nothing

        //
        // self.expect_news_drain(&cur); //necessary here? no since it is finishing here?

        //
        // return e.clone();
        GrammarWalkError::FailedParse
    }

    fn grammar_and(&mut self,cur :Work<'g,P,TS>,) {
        let GrammarNode::And(gs,  error_ind)=cur.grammar.as_ref() else{panic!("");};
        //

        if gs.is_empty() {return;}

        // let Some(head)=gs.first().cloned() else { return ; };
        let head=gs.get(cur.grammar_ind).unwrap().clone();

        //
        // let stow_new_len=self.hist_news_add(&cur);

        //
        // println!("{}, {:?}",cur.grammar_ind,cur.grammar.as_ref(),);

        //
        // if cur.grammar_ind+1!=gs.len()
        // if gs.len()>1

        // // let gg=GrammarNode::And(Rc::new([&G]));
        // let mut gg:&[&i32]=&[&123];
        // gg=&[&123,&43];

        //
        // let work_fail_len= if cur.user && (
        //     (!(*stow) && gs.len()!=1)
        //     || (*stow && gs.len()!=1)
        // ) {
        //     //
        //     let work_fail_len=self.stk.len();

        //     //
        //     self.stk.push(Work {

        //         grammar:Rc::new(GrammarNode::Error),

        //         work_success_len: cur.work_success_len,
        //         work_fail_len: cur.work_fail_len,

        //         tokens: cur.tokens,

        //         group_ind: cur.group_ind,
        //         group_len: cur.group_len,

        //         user:false,
        //         first:false,

        //         stow_new_len:cur.stow_new_len,
        //         stow_len: cur.stow_len,

        //         // expect_ind:cur.expect_ind,
        //         // expect_len:cur.expect_len,

        //         expect_ind1:cur.expect_ind1,
        //         expect_len1:cur.expect_len1,

        //         expect_new_len2:cur.expect_new_len2,
        //         expect_len2:cur.expect_len2,


        //         was_new_len:cur.was_new_len,
        //         was_ind:cur.was_ind,
        //     });

        //     work_fail_len
        // } else {
        //     cur.work_fail_len
        // };

        // let work_fail_len= if *error_ind!=0 && cur.grammar_ind>=*error_ind {self.work_error_len}else{cur.work_fail_len};

        //

        if cur.grammar_ind!=gs.len()
        // if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r.clone()))
        {


            // let rest=gs[1..];
            self.stk.push(Work {

                // grammar:Rc::new(GrammarNode::And(gs.clone(),*grammar_ind+1,*stow)),
                grammar:cur.grammar.clone(),

                work_success_len: cur.work_success_len,
                // work_fail_len, //: cur.work_fail_len,
                work_fail_len: cur.work_fail_len,


                tokens: cur.tokens.clone(), //not really necessary? since gets updated by always/primtitives

                group_ind: cur.group_ind,
                group_len: cur.group_len,

                grammar_ind:cur.grammar_ind+1,
                user:false,
                first:false,
                stow:false,

                stow_new_len:cur.stow_new_len,
                stow_len: cur.stow_len,

                in_expect:cur.in_expect,

                expect_ind1:cur.expect_ind1,
                expect_len1:cur.expect_len1,

                expect_new_len2:cur.expect_new_len2,
                expect_len2:cur.expect_len2,

                was_new_len:cur.was_new_len,
                was_ind:cur.was_ind,

                // trim:cur.trim,
            });
        }

        //
        // let not_end= gs.len() > 1;
        let not_end= cur.grammar_ind+1 != gs.len();
        let success_len=if not_end {self.stk.len()}else{cur.work_success_len};

        //
        self.stk.push(Work {
            grammar: head,
            // grammar_ind:0,
            work_success_len: success_len,
            // work_fail_len: cur.work_fail_len,
            work_fail_len : if *error_ind!=0 && cur.grammar_ind>=*error_ind {self.work_error_len}else{cur.work_fail_len},
            tokens: cur.tokens.clone(),
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len+1,

            // // and_id:cur.and_id+1,
            // and_id:if gs.len()==1{cur.and_id}else{cur.and_id+1}, //don't need if single element in And?

            grammar_ind:0,
            user:true,
            first:cur.first, //cur.from_user &&  //only want to know about grammars added by user, not the walker, could check from_user elsewhere,
            stow:cur.stow, //if *stow_first {true} else {cur.stow},
            // or_id:cur.or_id,
            // and_first:true,

            // can_hist_stow:cur.or_first,

            // stow_new_len,
            stow_new_len:cur.stow_new_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,
            // hist_ends_stk_len:cur.hist_ends_stk_len,

            // hist_stows_ind: cur.hist_stows_ind,
            stow_len: cur.stow_len,

            // hist_fails_len:cur.hist_fails_len,

            // hist_stows_stk_len:cur.hist_stows_stk_len,

            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expect_len:cur.expect_len,

            in_expect:cur.in_expect,

            expect_ind1:cur.expect_ind1,
            expect_len1:cur.expect_len1,

            expect_new_len2:cur.expect_new_len2,
            expect_len2:cur.expect_len2,

            // was_start_ind:cur.was_start_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,

            // had_ind:cur.had_ind,
            // had_len:cur.had_len,

            was_new_len:cur.was_new_len,
            was_ind:cur.was_ind,

            // trim:cur.trim,
        });
    }

    fn grammar_or(&mut self,cur :Work<'g,P,TS>,) {
        let GrammarNode::Or(gs,)=cur.grammar.as_ref() else{panic!("");};

        if gs.is_empty() {return;}

        //
        // let Some(head)=gs.first().cloned() else { return; };
        // let Some(head)=gs.get(*grammar_ind).cloned() else { return; };
        let head=gs.get(cur.grammar_ind).unwrap().clone();
        //
        // if cur.grammar_ind==gs.len() {return;}
        // let head=gs.get(cur.grammar_ind).unwrap().clone();


        //
        // let stow_new_len=self.hist_news_add(&cur);
        // let hist_stows_stk_len=self.hist_stows_stk_push(&cur);
        let stow_len=self.hist_stows_push(&cur);


        // let hist_fails_len=self.hist_fails_push(&cur);
        // // let hist_ends_stk_len=self.hist_ends_stk_push(&cur);
        // // let hist_stows_ind=if !cur.is_first{cur.stow_len}else{cur.hist_stows_ind};

        //
        // // let was_ind = if cur.from_user && !cur.first {cur.was_len} else {cur.was_ind}; //self.wases.len()
        // let was_start_ind = if cur.from_user && !cur.first {cur.was_len} else {cur.was_start_ind}; //self.wases.len()

        let was_ind = if cur.user && !cur.first {self.wases.len()} else {cur.was_ind}; //self.wases.len()


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
        if cur.grammar_ind+1!=gs.len()
        // if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r))
        {
            self.stk.push(Work {
                // grammar: Rc::new(GrammarNode::Or(gs.clone(),*grammar_ind+1)),
                grammar:cur.grammar.clone(),
                // grammar_ind:0,
                // grammar:cur.grammar.clone(),
                // grammar_ind:cur.grammar_ind+1,

                work_success_len: cur.work_success_len,
                work_fail_len: cur.work_fail_len,
                tokens: cur.tokens.clone(),
                group_ind: cur.group_ind,
                group_len: cur.group_len,
                // visiteds:cur.visiteds.clone(),
                // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
                // grammar_debug_len: cur.grammar_debug_len,
                // and_id:cur.and_id,

                grammar_ind:cur.grammar_ind+1,
                user:false,
                first:cur.first,
                stow:cur.stow,

                // or_id:cur.or_id,
                // and_first:cur.and_first,
                // can_hist_stow:false,

                // stow_new_len,
                stow_new_len:cur.stow_new_len,
                // hist_fails_len:stow_len,

                // hist_stows_stk_len,
                // hist_ends_stk_len,

                // hist_stows_ind,
                // stow_len: cur.stow_len,

                // hist_stows_stk_len,
                stow_len,

                // hist_prevs_ind: cur.hist_prevs_ind,
                // hist_prevs_len: cur.hist_prevs_len,
                // hist_prevs_ind,
                // hist_prevs_len,

                // expected_news_len:cur.expected_news_len,
                // expect_len:cur.expect_len,

                in_expect:cur.in_expect,

                expect_ind1:cur.expect_ind1,
                expect_len1:cur.expect_len1,

                expect_new_len2:cur.expect_new_len2,
                expect_len2:cur.expect_len2,

                // // was_start_ind:cur.was_start_ind,
                // was_start_ind,
                // // was_ind,
                // was_ind:cur.was_ind,
                // was_len:cur.was_len,
                // had_ind:cur.had_ind,
                // had_len:cur.had_len,

                was_new_len:cur.was_new_len,
                // was_ind:cur.was_ind,
                was_ind,

                // trim:cur.trim,
            });
        }

        //

        // let not_end= gs.len() > 1;
        let not_end= cur.grammar_ind+1 != gs.len();

        let fail_len=if not_end {self.stk.len()}else{cur.work_fail_len};

        //
        self.stk.push(Work {
            grammar: head,
            // grammar_ind:0,
            work_success_len: cur.work_success_len,
            work_fail_len: fail_len,
            tokens: cur.tokens.clone(),
            group_ind: cur.group_ind,
            group_len: cur.group_len,
            // visiteds:cur.visiteds,
            // non_term_visiteds_stk_len:cur.non_term_visiteds_stk_len,
            // grammar_debug_len: cur.grammar_debug_len+1,
            // and_id:cur.and_id,

            grammar_ind:0,
            user:true,
            first:true,
            stow:cur.stow,

            // or_id:cur.or_id,
            // and_first:cur.and_first,
            // can_hist_stow:false,

            // stow_new_len,
            stow_new_len:cur.stow_new_len,

            // hist_stows_stk_len,
            stow_len,
            // hist_fails_len:stow_len,
            // hist_ends_stk_len,

            // hist_stows_ind,
            // stow_len: cur.stow_len,
            // hist_prevs_ind: cur.hist_prevs_ind,
            // hist_prevs_len: cur.hist_prevs_len,
            // hist_prevs_ind,
            // hist_prevs_len,

            // expected_news_len:cur.expected_news_len,
            // expect_len:cur.expect_len,

            in_expect:cur.in_expect,

            expect_ind1:cur.expect_ind1,
            expect_len1:cur.expect_len1,

            expect_new_len2:cur.expect_new_len2,
            expect_len2:cur.expect_len2,

            // // was_start_ind:cur.was_start_ind,
            // was_start_ind,
            // // was_ind,
            // was_ind:cur.was_ind,
            // was_len:cur.was_len,
            // had_ind:cur.had_ind,
            // had_len:cur.had_len,


            was_new_len:cur.was_new_len,
            // was_ind:cur.was_ind,
            was_ind,

            // trim:cur.trim,
        });
    }



    // fn grammar_prev(&mut self,cur :Work<'g,P,TS>,) {
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
    //         self.work_stk_truncate(cur.work_success_len);

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
    //         self.work_stk_truncate(cur.work_fail_len);
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

    fn grammar_always(&mut self,cur :Work<'g,P,TS>,) {
        // self.stk.truncate(cur.success_len);
        self.work_stk_truncate(cur.work_success_len);
        // let _hist_news_len=self.hist_news_add(&cur);
        // self.hist_stows_clear(&cur);
        self.handle_exit_last_many(&cur);
        self.update_tokens(cur.tokens.clone(),true);
        self.groups_on_success(cur.group_ind,cur.group_len,cur.tokens.clone()); //here
        self.was_on_success(false); //before hist
        self.hist_on_success(cur.grammar.clone(),cur.group_len,cur.tokens.clone(),false);

        //why was this previously commented out?
        //  because grammar could finish without parsing anything due to optionals
        //  and then not report any errors
        // self.expected2_on_success();
    }

    fn grammar_try_from_hist_fails(&mut self,cur :&Work<'g,P,TS>) -> bool {
         //
        if !cur.user || !cur.first {return false;} // !(cur.from_user && cur.is_first)
        if cur.stow_len==0 {return false;}

        //
        // let hist_fail=&self.hist_fails[cur.hist_fails_len-1];

        let hist_stow=&self.stows[cur.stow_len-1];
        // if !hist_stow.fail_vals.grammers.contains(&cur.grammar) {
        //     return false;
        // }

        let Some(stow_fail)=&hist_stow.fail else {return false;};

        // let Some(hist_fail)=&hist_stow.fail_val else {return false;};
        // let TempStowVal::Fail { grammar: hist_fail_grammar }=&hist_stow.val else {return false;};
        // if hist_fail.grammar!=cur.grammar {
        //     return false;
        //     // println!("--- checkkk")
        // }

        if stow_fail.grammar!=cur.grammar { return false; }

        //
        // self.stk.truncate(cur.fail_len);
        self.work_stk_truncate(cur.work_fail_len);
        self.update_tokens(cur.tokens.clone(),false);
        self.hist_on_fail();
        self.was_on_fail();
        self.expect_on_fail2();
        self.expect_on_fail1();
        self.groups_on_fail();

        //
        if self.debug
        {
            println!("---- grabbed fail from or {:?},",cur.grammar);
        }

        //
        true
    }

    fn grammar_try_from_hist_stows(&mut self,cur :&Work<'g,P,TS>) -> bool {
        //
        if !cur.user || !cur.first {return false;} // !(cur.from_user && cur.is_first)
        // if cur.hist_stows_stk_len==0 {return false;}
        if cur.stow_len==0 {return false;}

        // if self.hist_non_term_only && !cur.grammar.is_non_term() {return false;}


        let hist_stow=&self.stows[cur.stow_len-1];
        // let Some(hist_stow_val)=&hist_stow.success_val else {return false;};


        let Some(stow_success)=&hist_stow.success else {return false;};

        // let TempStowVal::Success {
        //     grammar:hist_success_gramar,
        //     tokens_after:tokens_after,
        //     stow_groups_end,
        //     was:hist_was,
        // }=&hist_stow.val else {return false;};

        // //
        // if hist_stow_val.grammar!=cur.grammar {
        //     return false;
        // }

        //
        if stow_success.grammar!=cur.grammar { return false; }
        //
        // self.stk.truncate(cur.success_len);

        // let temp_groups_end=hist_stow.success_val.as_ref().map(|x|x.stow_groups_end).unwrap_or(hist_stow.stow_groups_start);
        // // let temp_prevs_end=hist_stow.val.as_ref().map(|x|x.stow_prevs_end).unwrap_or(hist_stow.stow_prevs_start);
        // let temp_groups_end=hist_stow.success_val.as_ref().map(|x|x.stow_groups_end).unwrap_or(hist_stow.stow_groups_start);
        // let temp_groups_end=*stow_groups_end;

        let stow_groups=&self.stow_groups[hist_stow.stow_groups_start .. stow_success.stow_groups_end];
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
        if let TempStowWas::Was(temp_was)=&stow_success.was //&hist_stow_val.was
        {
            self.was_news.push(temp_was.clone());
        }

        //
        let was_prim= if let TempStowWas::Primitive=&stow_success.was //&hist_stow_val.was
            {true}else{false};

        //
        // let cur=Work {
        //     group_len:self.groups.len(),
        //     // tokens:hist_stow_val.tokens_after,
        //     tokens:stow_success.tokens_after.clone(),
        //     // was_new_len,
        //     // hist_ends_stk_len:todo!(),
        //     ..cur.clone()
        // };
        // let cur=0;


        let tokens_after=stow_success.tokens_after.clone();
        let new_group_len=self.groups.len();
        //

        self.work_stk_truncate(cur.work_success_len);
        self.update_tokens(tokens_after.clone(),true);
        self.groups_on_success(cur.group_ind,new_group_len,tokens_after.clone());
        self.was_on_success(was_prim); //before hist
        self.hist_on_success(cur.grammar.clone(),new_group_len,tokens_after.clone(),true,); //not needed? no.. if And(Z,Or(And(X,Y),X)), then will add that
        self.expect_on_success2();
        self.expect_on_success1();

        //
        if self.debug {
            println!("---- grabbed success from or {:?},",cur.grammar);
        }

        //
        true
    }

    fn grammar_primitive(&mut self, cur:Work<'g,P,TS>,)
    {
        //
        // let _hist_news_len=self.hist_news_add(&cur);
        // self.hist_stows_clear(&cur);

        // let is_group_token_start=if cur.group_ind!=0 {cur.tokens.inds2().start==self.groups[cur.group_ind].tokens.inds2().start}
        //     else{false};

        //

        // if  !cur.grammar.is_eol() {
        //     while !cur.tokens.is_empty() {
        //         let x=cur.tokens.first().unwrap();
        //         if !x.is_eol() { break; }
        //         cur.tokens.next().unwrap();
        //     }
        // }

        //
        // let tokens_start=cur.tokens;

        //
        // if !cur.grammar.is_eol() {
        //     cur.tokens.trim();
        // }

        let GrammarNode::Primitive(p)=cur.grammar.as_ref() else {panic!("");};

        let mut after_tokens=cur.tokens.clone();


        if !p.is_trimmable() {
            // if
            //     self.groups[cur.group_ind].tokens.inds2().start == cur.tokens.inds2().start
            //     // cur.first
            //     b
            // {
            //     cur.trim=true;
            // }

                // cur.trim=true;

            after_tokens.trim2();
        }

        //

        let result=after_tokens.pop_primitive(p);

        // //
        // let result=match cur.grammar.as_ref() {
        //     GrammarNode::String => { cur.tokens.pop_string().is_ok() },
        //     GrammarNode::Identifier => { cur.tokens.pop_identifier().is_ok() },
        //     GrammarNode::Int => {cur.tokens.pop_int().is_ok() },
        //     GrammarNode::Float => { cur.tokens.pop_float().is_ok() },
        //     GrammarNode::Symbol(s) => { cur.tokens.pop_with_symbol(s).is_ok() },
        //     GrammarNode::Keyword(s) => { cur.tokens.pop_with_keyword(s).is_ok() },
        //     GrammarNode::Eol => {cur.tokens.pop_eol().is_ok() },
        //     _ => {panic!("");}
        // };

        if result {
            self.group_set_trim(&p, cur.group_ind,cur.tokens.clone());

            // if is_group_token_start {
            //     let g=&mut self.groups[cur.group_ind];
            //     g.tokens=tokens_start;
            // }

            //
            self.work_stk_truncate(cur.work_success_len);
            self.update_tokens(after_tokens.clone(),true);
            self.groups_on_success(cur.group_ind,cur.group_len,after_tokens.clone());
            self.was_on_success(true); //before hist
            self.hist_on_success(cur.grammar.clone(),cur.group_len,after_tokens.clone(),false);

            self.expect_on_success2();
            self.expect_on_success1();

        } else {
            // self.stk.truncate(cur.fail_len);
            self.work_stk_truncate(cur.work_fail_len);
            self.update_tokens(cur.tokens.clone(),false);
            self.hist_on_fail();
            self.was_on_fail();
            // // self.revert_last_hist_news();
            // self.update_hist_on_fail(&cur);
            // // let _expected_news_len=self.add_expected_new(&cur);
            // // self.submit_expected_news(&cur);

            self.groups_on_fail();
            let (_expected_ind,_expecteds_len)=self.add_expect1(&cur);
            let _expect_new_len2=self.add_expect_new2(&cur);

            self.expect_on_fail2();
            self.expect_on_fail1();

        }
    }

    // fn work_on_success(&mut self,
    //     cur:&Work<'g,P,TS>,
    // ) {
    //     self.stk.truncate(cur.work_success_len);
    // }

    fn work_stk_truncate(&mut self,len:usize) {
        self.stk.truncate(len);
    }

    // fn work_on_fail(&mut self, cur:&Work<'g,P,TS>,) {
    //     self.stk.truncate(cur.work_fail_len);
    // }

    fn add_expect_new2(&mut self, cur:&Work<'g,P,TS>,) -> usize {
        if !self.use_expect2 {return cur.expect_new_len2;}


        //
        // println!("----- isp={} e={}, cur.in_expect={}",cur.grammar.is_primtive(), self.expect_news2.is_empty(),cur.in_expect);
        //don't add primitives if in expect
        if cur.grammar.is_primtive() //disable primitive expects
            // && !self.expect_news2.is_empty()
            // // && cur.in_expect
        {
        //     // println!("")
            return cur.expect_new_len2; //self.expect_news2.len()
        }

        //do it here or in on fail?
        // if self.expect_news2.last().map(|x|x.tokens_start.inds2().start)==Some(cur.tokens.inds2().start) {
        //     return cur.expect_new_len2; //self.expect_news2.len()
        // }

        //
        // if cur.grammar.is_expect() && self.expect_news2.last().map(|x|x.expect_type.is_expect()).unwrap_or_default() {

        // }

        //
        let expect_type=match cur.grammar.as_ref() {
            GrammarNode::Expect(_, name) => TempExpectType::Expect(name),
            // // GrammarNode::Prev(_) => TempExpectedType::Prev,
            // GrammarNode::String => TempExpectType::String,
            // GrammarNode::Identifier => TempExpectType::Identifier,
            // GrammarNode::Int => TempExpectType::Int,
            // GrammarNode::Float => TempExpectType::Float,
            // GrammarNode::Symbol(s) => TempExpectType::Symbol(s),
            // GrammarNode::Keyword(s) => TempExpectType::Keyword(s),
            // GrammarNode::Eol => TempExpectType::Eol,
           _ => {panic!("");}
        };

        //
        if self.debug {
            println!("----- expect added2  {expect_type:?}");
        }

        self.expect_news2.push(TempExpectNew2 {
            expect_type, tokens_start: cur.tokens.clone(), expect_len: cur.expect_len2,
        });
        self.expect_news2.len()
    }

    fn expect_on_error2(&mut self, ) {
        // println!("here---");
        self.expects2.retain(|x|x.tokens_start.inds2().start==self.expect_token_start2.inds2().start
           && if let TempExpectType::Expect("")=&x.expect_type {false} else {true}
        );
    }
    fn expect_on_success2(&mut self, ) {
        if !self.use_expect2 {return;}

        let Some(last)=self.stk.last_mut() else {return;}; //the func, not run on always... does now


        // let drained=self.expect_news2.drain(last.expect_new_len2 ..).collect::<Vec<_>>();

        self.expect_news2.truncate(last.expect_new_len2);
        // self.expects2.truncate(last.expect_len2);

        //
        // let drained_expects=self.expects2.drain(last.expect_len2 ..)
        //     // .filter(|x|x.tokens_start.inds2().start>=cur.tokens.inds2().start)
        //     .filter(|x|x.tokens_start.inds2().start==self.expect_token_start2.inds2().start)
        //     .collect::<Vec<_>>();

        // println!("----- drained expects2 [{}]",drained_expects.iter().map(|x|format!("t{}:{:?}",x.tokens_start.inds2().start,x.expect_type,)).collect::<Vec<_>>().join(", "));
        // println!("----- expects2 [{}]",self.expects2.iter().map(|x|format!("t{}:{:?}",x.tokens_start.inds2().start,x.expect_type,)).collect::<Vec<_>>().join(", "));
        // // drained_expects.retain(|x|x.tokens_start.inds2().start>=cur.tokens.inds2().start); //use >= or just == ?

                // self.expect_token_start2=self.expect_token_start2.max(cur.tokens);


        //
        let drained_expects=self.expects2.drain(last.expect_len2 ..)
            .filter(|x|x.tokens_start.inds2().start==self.expect_token_start2.inds2().start)
            ;

        //
        self.expects_temp2.extend(drained_expects);
        self.expects2.extend(self.expects_temp2.drain(0..));

        //
        last.expect_len2=self.expects2.len();

    }

    fn expect_on_fail2(&mut self, ) {
        if !self.use_expect2 {return;}

        let Some(last)=self.stk.last_mut() else {return;}; //the func, not run on always

        // let draineds=self.expect_news2.drain(last.expect_new_len2 ..).collect::<Vec<_>>();


        // // if cur.grammar.is_expect() && self.expect_news2.last().map(|x|x.expect_type.is_expect()).unwrap_or_default() {
        // // }

        // let token_start_ind_max=draineds.iter().map(|x|x.tokens_start.inds2().start).max().unwrap_or(self.expect_token_start2.inds2().start);

        // let draineds=draineds.into_iter().filter(|x|x.tokens_start.inds2().start==token_start_ind_max).collect::<Vec<_>>();

        // if self.expect_token_start2.inds2().start< token_start_ind_max {
        //     self.expects2.clear();
        // }

        // self.expects2.extend(drained.map(|x|TempExpect2 { expect_type: x.expect_type, tokens_start: x.tokens_start }));

        //
        let draineds=self.expect_news2.drain(last.expect_new_len2 ..); //here

        //first element with max token_ind
        let drained=draineds.rev().max_by(|x,y|x.tokens_start.inds2().start.cmp(&y.tokens_start.inds2().start));

        //
        if let Some(drained)=drained {
            if let TempExpectType::Expect(_expect_name)=&drained.expect_type {
                if let Some(last_token_ind)= self.expects2
                    .get(drained.expect_len)
                    .map(|x|x.tokens_start.inds2().start)
                { //has self.expect.last
                    if //expect_name.is_empty() &&
                        drained.tokens_start.inds2().start== last_token_ind
                    { //replace
                        self.expects2.truncate(drained.expect_len);

                        if drained.tokens_start.inds2().start >= self.expect_token_start2.inds2().start {
                            self.expects2.push(TempExpect2 { expect_type: drained.expect_type, tokens_start: drained.tokens_start.clone() });
                        }
                    }
                } else {
                    if drained.tokens_start.inds2().start >= self.expect_token_start2.inds2().start {
                        self.expects2.push(TempExpect2 { expect_type: drained.expect_type, tokens_start: drained.tokens_start.clone() });
                    }
                }
            } else {
                if drained.tokens_start.inds2().start >= self.expect_token_start2.inds2().start {
                    self.expects2.push(TempExpect2 { expect_type: drained.expect_type, tokens_start: drained.tokens_start.clone() });
                }
            }

            if drained.tokens_start.inds2().start > self.expect_token_start2.inds2().start {
                self.expect_token_start2=drained.tokens_start;
            }
        }

        //
        last.expect_len2=self.expects2.len();

    }


    fn add_expect1(&mut self, cur:&Work<'g,P,TS>,) -> (Option<usize>,usize) {
        if !self.use_expect1 {return (cur.expect_ind1,cur.expect_len1);}

        // return (cur.expect_ind,cur.expect_len);

        //check if prim and parent pos is same as cur pos
        //

        let parent_start=cur.expect_ind1.map(|i|self.expects1[i].tokens_start.inds2().start) ;

        if parent_start==Some(cur.tokens.inds2().start) {
            return (cur.expect_ind1,cur.expect_len1);
        }

        // if cur.expect_ind.is_some() && cur.grammar.is_primtive() { //(cur.grammar.is_prev() || )
        //     return (cur.expect_ind,cur.expect_len);
        // }

        //
        let expected_type=match cur.grammar.as_ref() {
            GrammarNode::Expect(_, name) => TempExpectType::Expect(name),
            // // GrammarNode::Prev(_) => TempExpectedType::Prev,
            // GrammarNode::String => TempExpectType::String,
            // GrammarNode::Identifier => TempExpectType::Identifier,
            // GrammarNode::Int => TempExpectType::Int,
            // GrammarNode::Float => TempExpectType::Float,
            // GrammarNode::Symbol(s) => TempExpectType::Symbol(s),
            // GrammarNode::Keyword(s) => TempExpectType::Keyword(s),
            // GrammarNode::Eol => TempExpectType::Eol,
           _ => {panic!("");}
        };

        //
        if self.debug {
            println!("----- expect added  {expected_type:?}");
        }

        //
        let expect_ind=self.expects1.len();

        //
        self.expects1.push(TempExpect1 {
            expect_type: expected_type,
            parent: cur.expect_ind1,
            tokens_start: cur.tokens.clone(),
            // last:false,
        });

        //
        (Some(expect_ind),self.expects1.len())
    }


    fn expect_on_error1(&mut self,  ) {
        if !self.use_expect1 {return;}

        //
        let max_token = self.expects1.iter().map(|x|x.tokens_start.clone()).max_by(|x,y|x.inds2().start.cmp(&y.inds2().start)).unwrap_or(self.tokens_remaining.clone());

        self.expected_tokens_remaining1=max_token;

        //
        let max_token_start_ind=self.expected_tokens_remaining1.inds2().start;

        let parents= self.expects1.iter().filter_map(|x|x.parent).collect::<HashSet<_>>();

        let expecteds=self.expects1.iter().enumerate().rev().filter_map(|(i,x)|(
            x.tokens_start.inds2().start == max_token_start_ind &&
            !parents.contains(&i)
        ).then(||(x.expect_type.clone(),x.clone()))).collect::<BTreeMap<_,_>>();

        self.expects1=expecteds.iter().map(|(_k,v)|v.clone()).collect::<Vec<_>>();
    }

    fn expect_on_success1(&mut self, ) {
        if !self.use_expect1 {return;}

        let Some(last)=self.stk.last() else {return;}; //the func, not run on always... does now
        self.expects1.truncate(last.expect_len1);
    }

    fn expect_on_fail1(&mut self, ) {
        if !self.use_expect1 {return;}

        let Some(last)=self.stk.last_mut() else {return;}; //the func, not run on always
        last.expect_len1=self.expects1.len();
    }



    fn was_on_success(&mut self, //cur:&Work<'g,P,TS>,
        is_prim:bool,

    ) {
        //run before hist_on_success

        let Some(last)=self.stk.last_mut() else {return;};


        //
        // // let drained_was_new=self.was_news.drain(last.was_new_len ..).next();
        // self.was_news.truncate(last.was_new_len+1);
        // let drained_was_new=if self.was_news.len()==last.was_new_len {self.was_news.pop()} else {None};

        let drained_was_new=self.was_news.get(last.was_new_len).cloned();
        self.was_news.truncate(last.was_new_len);

        if let Some(drained_was_new)= drained_was_new {

            // last.was_ind=self.wases.len();
            self.wases.truncate(last.was_ind); //remove prev WAS
            self.wases.push(drained_was_new);
            // last.was_len2=self.wases.len();

            if self.debug {
                println!("----- was 0");
            }
        } else if is_prim { //cur.grammar.is_primtive() //should be not? no
            // let b=cur.grammar.is_primtive() || (cur.grammar.is_had()||cur.grammar.is_always());
            // self.wases.truncate(last.was_ind+1); //keep previous WAS, if there was one
            self.wases.truncate(last.was_ind);

            if self.debug {
                println!("----- was 1");
            }
        } else if last.was_ind!=self.wases.len() { //for had/always
            // println!("=======---===");

            //
            // self.wases.drain(last.was_ind..self.wases.len()-1);
            let x=self.wases.pop().unwrap();
            self.wases.truncate(last.was_ind);
            self.wases.push(x);

            //
            if self.debug {
                println!("----- was 2");
            }
            // let d=self.wases.drain(last.was_ind..self.wases.len()-1).map(|x|x.name).collect::<Vec<_>>();
            // println!("----- was 2: {d:?}");
        } else {
            if self.debug {
                println!("----- was 3");
            }
        }
        // self.was_news.truncate(last.was_new_len);

    }

    fn was_on_fail(&mut self, ) {
        let Some(last)=self.stk.last_mut() else {panic!("");}; //the func, not run on always

        if self.debug {
            println!("----- was 4 faill");
        }

        self.was_news.truncate(last.was_new_len);

        // if cur.was_ind==last.was_ind {
        //     self.wases.truncate(last.was_ind+1); //kee prev
        // } else {
            self.wases.truncate(last.was_ind);
        // }

        // self.wases.truncate(last.was_len);
        // // self.wases.truncate(last.was_ind+1);
    }

    fn hist_on_fail(&mut self,
        // cur:&Work<'g,P,TS>,
    ){
        let Some(last)=self.stk.last_mut() else {return;};
        // self.hist_prevs.truncate(last.hist_prevs_len);

        // println!("---- hist on fail {:?}",
        //     self.hist_prevs.drain(last.hist_prevs_len..).map(|x|x.grammar.clone()).collect::<Vec<_>>(),
        // );

        if last.stow_len!=0 {
            //
            // let mut drained_hist_news=self.hist_news.drain(last.stow_new_len ..);
            let mut drained_hist_news=self.stow_news[last.stow_new_len ..].iter();



            //
            // if self.debug && !drained_hist_news.is_empty() {
            //     // println!("----- adding to hist fails");
            // }

            //
            let hist_stow=&mut self.stows[last.stow_len-1];
            // //
            // for x in drained_hist_news.iter() {
            //     // if !x.is_first {continue;}
            //     if x.stow_len!=last.stow_len {continue;}
            //     if !(x.grammar.is_non_term() || x.grammar.is_and() || x.grammar.is_many()) {continue;}

            //     // self.hist_fails[last.hist_fails_len-1].grammers.insert(x.grammar.clone());
            //     hist_stow.fail_vals.grammers.insert(x.grammar.clone());
            // }


            // //
            if let Some(drained_hist_new)=drained_hist_news
                // .iter()
                .find(|x|{
                // x.is_first &&
                x.stow_len==last.stow_len
                && (x.grammar.is_non_term() || x.grammar.is_and() || x.grammar.is_many())
            }) {
                // // self.hist_fails[last.hist_fails_len-1].grammar=drained_hist_new.grammar.clone();
                // hist_stow.fail_val=Some(TempHistFail{grammar:drained_hist_new.grammar.clone()});
                hist_stow.fail=Some(TempStowFail { grammar: drained_hist_new.grammar.clone() });
            }

            // self.hist_news.truncate(last.stow_new_len);
        } else {
            // self.hist_news.truncate(last.stow_new_len);
        }

        //
        self.stow_news.truncate(last.stow_new_len);

        //
        let stow_len=last.stow_len;
        self.hist_stows_truncate(stow_len);

    }

    fn hist_on_success(&mut self,
        cur_grammar:Rc<GrammarNode<'g,P>>,
        cur_group_len:usize,
        cur_tokens:TS,
        // cur:&Work<'g,P,TS>,
        //what was this for again? something to do with not adding cur grammar to hist_stows?
        //  it was for not adding cur grammar to hist_new?
        gotten:bool,
        // _hist_ends_remove_previous:bool,
    ) {

        //TODO: dont convert drained drained_hist_news to vec, copy straight to

        //should always be some (due to init), use panic instead of ret? no, it will end on an always if successful
        let Some(last)=self.stk.last_mut() else {return;};



        //add hist stows
        if last.stow_len!=0 { //cur.stow_len!=0 // && cur.stow_len==last.stow_len //that the hist_stows[ind] still exists
            //
            // let mut drained_hist_news=self.hist_news.drain(last.stow_new_len ..);
            let mut drained_hist_news=self.stow_news[last.stow_new_len ..].iter();
            //
            let drained_hist_new2=drained_hist_news
                // .iter()
                .find(|x|{

                // x.is_first
                 x.stow_len==last.stow_len &&
                (x.grammar.is_non_term() || x.grammar.is_and() || x.grammar.is_many())

            });

            // println!("-------found {:?} : {:?}",drained_hist_new2,drained_hist_news.iter().map(|x|&x.grammar).collect::<Vec<_>>());
            //

            if let Some(drained_stow_new)=drained_hist_new2 {

                //
                if self.debug {
                    println!("------ hist_stows_set {}: {:?}", self.stows.len(), drained_stow_new.grammar, );
                }
                // println!("------ stowed {:?}",drained_hist_new2.grammar.clone());

                //
                // let hist_stow=self.hist_stows.last_mut().unwrap();
                let hist_stow=&mut self.stows[last.stow_len-1];

                //
                if !gotten {
                    // self.hist_stows_prevs.truncate(hist_stow.stow_prevs_start);
                }

                // self.hist_stows_prevs.extend(added_hist_prevs.iter().rev().cloned());

                //
                self.stow_groups.truncate(hist_stow.stow_groups_start);

                //
                if self.groups.len()!=drained_stow_new.group_len {

                    //
                    let group_ind_offset=self.groups[drained_stow_new.group_len].parent;

                    self.stow_groups.extend(self.groups[drained_stow_new.group_len..cur_group_len].iter().map(|x|TempGroup{
                        parent: x.parent
                        -group_ind_offset
                        , ..x.clone()
                    }));
                }

                //
                // hist_stow.success_val=Some(TempHistStowVal {
                //     grammar: drained_hist_new2.grammar.clone(),
                //     tokens_after: cur.tokens,
                //     stow_groups_end: self.hist_stows_groups.len(),
                //     // stow_prevs_end: self.hist_stows_prevs.len(),
                //     // was:self.wases.get(cur.was_ind).cloned(),
                //     was: //self.wases[last.was_ind..].last().m
                //     if last.was_ind!=self.wases.len() {
                //         TempHistStowWas::Was(self.wases.last().cloned().unwrap())
                //     } else if cur.grammar.is_primtive() {
                //         TempHistStowWas::Primitive
                //     } else {
                //         TempHistStowWas::None
                //     },
                // });

                hist_stow.success=Some(TempStowSuccess {
                    grammar: drained_stow_new.grammar.clone(),
                    tokens_after: cur_tokens.clone(),
                    stow_groups_end: self.stow_groups.len(),
                    // stow_prevs_end: self.hist_stows_prevs.len(),
                    // was:self.wases.get(cur.was_ind).cloned(),
                    was: //self.wases[last.was_ind..].last().m
                    if last.was_ind!=self.wases.len() {
                        TempStowWas::Was(self.wases.last().cloned().unwrap())
                    } else if cur_grammar.is_primtive() {
                        TempStowWas::Primitive
                    } else {
                        TempStowWas::None
                    },
                    trim:drained_stow_new.trim,
                });
            }


            // self.hist_news.truncate(last.stow_new_len);
        } else {
            // self.hist_news.truncate(last.stow_new_len);
        }

        //
        self.stow_news.truncate(last.stow_new_len);



        //

        let stow_len=last.stow_len;
        self.hist_stows_truncate(stow_len);

        // //

        // // last.stow_len=self.hist_stows_elements.len();

    }

    // fn hist_fails_push(&mut self,cur:&Work<'g,P,TS>) -> usize {
    //     if cur.from_user
    //         && ( !cur.first || cur.hist_fails_len==0
    //     ) {
    //         //
    //         if self.hist_fails.len() < cur.hist_fails_len+1 {
    //             if self.hist_fails.len() != cur.hist_fails_len {panic!("");}

    //             //
    //             self.hist_fails.push(Default::default());
    //         }

    //         //
    //         self.hist_fails[cur.hist_fails_len].grammers.clear();

    //         //
    //         cur.hist_fails_len+1
    //     } else {
    //         cur.hist_fails_len
    //     }

    //     //

    // }
    fn hist_stows_push(&mut self,cur:&Work<'g,P,TS>) -> usize {
        if cur.user //so not an added OR for rest,
            && ( !cur.first || //not part of current OR, eg: or(A, and(B,or(C,D))) A in dif OR stk than C,D
            // self.hist_stows_stk.is_empty()
            cur.stow_len==0 //init first, for if all part of same OR stk, eg: or(A,or(B,C))
            //if not need to init first, then it just reuses existing one
        ) //add current/initial OR
            // && (!self.hist_non_term_only ||)
        {
            if self.debug {
                println!("------ hist_stows_push ind={}", self.stows.len());
            }

            //
            self.stows.push(TempStow {
                stow_groups_start: self.stow_groups.len(),
                // success_val: None,
                // // stow_prevs_start: self.hist_stows_prevs.len(),
                // fail_val:None,
                // // fail_vals:Default::default(),


                success:None,
                fail:None,
                tokens_start_ind:cur.tokens.inds2().start,
            });

            if self.stows.len()!=cur.stow_len+1 {
                panic!("");
            }

        }

        self.stows.len()
    }


    fn hist_news_add(&mut self,cur:&Work<'g,P,TS>) -> usize {
        // return self.hist_news.len();
        //
        // let GrammarNode::Stow(g, )=cur.grammar.as_ref() else{panic!("");};

        //
        if
            // cur.from_user
            // && (!self.hist_non_term_only || cur.grammar.is_non_term())
            // // // && (cur.grammar.is_primtive() || cur.grammar.is_non_term())
            // // && cur.grammar.is_non_term() //should only do nonterms?

            // &&
            cur.grammar.is_non_term() &&
            // cur.grammar.is_stow() &&
            cur.first //no longer using prevs, only stows/fails
        { //ignore grammars added by walker
            // let grammar=if  let GrammarNode::Stow(g, )=cur.grammar.as_ref() {g.clone()}else{cur.grammar.clone()};
            self.stow_news.push(TempStowNew {
                grammar:cur.grammar.clone(),
                tokens_start: cur.tokens.clone(),
                // group_ind: cur.group_ind,
                group_len:cur.group_len,
                // is_first:cur.first
                    // &&cur.and_first
                // ,
                stow_len:cur.stow_len,
                // hist_fails_len:cur.hist_fails_len,
                trim:false,
            });

            return self.stow_news.len();
        }

        cur.stow_new_len

        // self.hist_news.len()
    }

    fn hist_stows_truncate(&mut self,stow_len:usize) {

        //
        if self.debug {
            if self.stows.len() != stow_len {
                println!("------ hist_stows_truncate {}=>{}", self.stows.len(), stow_len);
            }
        }

        //
        self.stows.truncate(stow_len);

        //
        if let Some(hist_stow)=self.stows.last() {
            // let (groups_len,prevs_len)=if let Some(hist_stow_val)= &hist_stow.val {
            //     (hist_stow_val.stow_groups_end,hist_stow_val.stow_prevs_end)
            // } else {
            //     (hist_stow.stow_groups_start,hist_stow.stow_prevs_start)
            // };
            let groups_len=if let Some(stow_success)=&hist_stow.success {
                stow_success.stow_groups_end
            } else {
                hist_stow.stow_groups_start
            };
            // let groups_len=if let Some(hist_stow_val)= &hist_stow.success_val {
            //     hist_stow_val.stow_groups_end
            // } else {
            //     hist_stow.stow_groups_start
            // };

            self.stow_groups.truncate(groups_len);
            // self.hist_stows_prevs.truncate(prevs_len);
        }
    }

    // fn step_truncates(&mut self,cur :&Work<'g,P,TS>) {
    //     //should move all these to run on success/fails of prims/prev/always/take

    //     // self.groups.truncate(cur.group_len);
    //     // self.hist_news.truncate(cur.stow_new_len);

    //     // self.hist_stows_stk.truncate(cur.hist_stows_stk_len);
    //     // self.hist_ends_stk.truncate(cur.hist_ends_stk_len);

    //     // self.hist_stows_elements.truncate(cur.stow_len);

    //     //
    //     // self.hist_prevs.truncate(cur.hist_prevs_len);

    // }


    fn groups_on_fail(&mut self,) {
        let Some(last)=self.stk.last() else {panic!("");};

        self.groups.truncate(last.group_len);
    }

    fn group_set_trim(&mut self,

        primitive:&P,
        cur_group_ind:usize,
        before_tokens:TS,
    ) {
        // let Some(last)=self.stk.last_mut() else {return;};

        let group=&mut self.groups[cur_group_ind];

        if group.tokens.inds2().start==before_tokens.inds2().start {
            group.trim=!primitive.is_trimmable(); //may change multiple times as And's fail
        }

    }

    fn groups_on_success(&mut self,
        cur_group_ind:usize,
        cur_group_len:usize,
        cur_tokens:TS,
        // cur :&Work<'g,P,TS>,
        // before_tokens : Option<TS>,
        // cur_group_ind:usize,
        // cur_primitives:TokenIterContainer<'t>,
    ) {
        let Some(last)=self.stk.last_mut() else {return;};

        //


        // if cur.trim {
        //     if last.group_len!=cur.group_len { //cur group len is greater
        //         let group=&mut self.groups[cur.group_ind];

        //         if let Some(before_tokens)=before_tokens {

        //         }
        //         group.tokens.trim2();
        //         last.trim=false; //not necessary?
        //     } else { //no group being submitted to trim
        //         last.trim=true;
        //     }
        // }

        //
        if cur_group_ind!=last.group_ind { //group close
            let group=&mut self.groups[cur_group_ind];

            if group.trim {
                group.tokens.trim2();

            }
        }

        //
        last.group_len=cur_group_len;

        //
        // if self.debug {
        //     println!("==do_groups_primitives_clamp: cur_group_ind={}, last.group_ind={}",cur.group_ind,last.group_ind);
        // }


        //clamp groups tokens (for groups that have ended)
        let mut g=cur_group_ind;

        //
        while g>last.group_ind {
            let group=&mut self.groups[g];
            group.tokens.truncate2(group.tokens.len2()-cur_tokens.len2());
            g=group.parent;
        }

        //
        self.groups.truncate(last.group_len); //why? //same as cur.group_len

    }

    // fn do_non_term_visiteds(&mut self,
    //     t:&'g str,
    //     cur_primitives:TokenIterContainer<'t>,
    //     cur_visiteds: HashSet<(&'g str, usize)>,
    // ) -> Result<HashSet<(&'g str, usize)>,GrammarWalkError<'g>> {
    //     //
    //     if !self.non_term_recursive_check { return  Ok(Default::default()); }

    //     //
    //     let v=(t,cur_primitives.inds2().start);

    //     //
    //     if cur_visiteds.contains(&v) { return Err(GrammarWalkError::RecursiveNonTerm(t)); }

    //     //
    //     let mut visiteds=cur_visiteds;
    //     visiteds.insert(v);

    //     //
    //     Ok(visiteds)
    // }

    fn new_group(&mut self,cur:&Work<'g,P,TS>) -> (usize,usize) {
        let GrammarNode::Group(_,name)=cur.grammar.as_ref() else {panic!("");};
        let parent=cur.group_ind;
        let tokens=cur.tokens.clone();

        let new_group_ind=self.groups.len();
        self.groups.push(TempGroup { name, parent, tokens, trim:false,});
        (new_group_ind,self.groups.len())
    }

    fn update_tokens(&mut self,
        // cur:&Work<'g,P,TS>,
        tokens:TS,
        set_last_tokens:bool,
    ) {
        if self.stk.is_empty() {
            self.tokens_remaining=tokens.clone();
        } else if set_last_tokens {
            let Some(last)=self.stk.last_mut() else {panic!("");};
            last.tokens=tokens.clone();
        }

        if tokens.inds2().start > self.tokens_furthest.inds2().start {
            self.tokens_furthest=tokens.clone();
        }
    }

    fn handle_exit_last_many(&mut self,cur:&Work<'g,P,TS>) { //if not parsing anything, exit the many
        let Some(last)=self.stk.last_mut() else {return;};
        if !last.grammar.is_many() || last.tokens.len2()!=cur.tokens.len2() {return;}

        last.grammar=Rc::new(GrammarNode::Always); //self.always.clone();
    }


    pub fn last_loc2(&self) -> TS {

        //
        if self.use_expect2 {
            if self.expects2.is_empty() {
                // println!("-- here1");
                let mut t=self.tokens_remaining.clone();
                t.trim2();

                if t.is_empty2() {
                    t
                } else {
                    self.tokens_remaining.clone()
                }
            } else if self.expect_token_start2.is_empty2() {
                self.tokens_remaining.clone()
            } else {
                self.expect_token_start2.clone()
                // self.expect_token_start2.first().ok()
                //     // .or_else(||self.tokens_remaining.first().ok())
                //     .and_then(|x|x.prevs().rev().find(|x|!x.is_eol()))
                //     .map(|x|x.end_loc())
                //     // .map(|x|x.start_loc())
                //     .unwrap_or(self.tokens_remaining.start_loc())


            }
        } else if self.use_expect1 {
            if self.expects1.is_empty() {
                self.tokens_remaining.clone()
            } else {
                self.expected_tokens_remaining1.clone()
            }
        } else {
            self.tokens_remaining.clone()
        }

    }

    // pub fn last_loc(&self) -> Loc {
    //     // println!("l1 {:?} {:?} || {:?}",self.tokens_remaining.loc(),self.tokens_remaining.last_loc(),self.tokens_remaining);
    //     // println!("l2 {:?} {:?} || {:?}",self.expected_tokens_remaining.loc(),self.expected_tokens_remaining.last_loc(),self.expected_tokens_remaining);
    //     // println!("{:?}:{}:{}",self.top_tokens,self.top_tokens.loc(),self.top_tokens.last_loc());
    //     // println!("{:?}:{}:{}",self.tokens_remaining,self.tokens_remaining.loc(),self.tokens_remaining.last_loc());
    //     // println!("{:?}:{}:{}",self.expected_tokens_remaining,self.expected_tokens_remaining.loc(),self.expected_tokens_remaining.last_loc());

    //     // for t in self.top_tokens {
    //     //     println!("t {t:?} :: {} to {}",t.start_loc(),t.end_loc());
    //     // }

    //     //
    //     if self.use_expect2 {
    //         if self.expects2.is_empty() {
    //             // println!("-- here1");
    //             let mut t=self.tokens_remaining;
    //             t.trim2();

    //             t.find(|x|!x.is_eol()) //is_eol
    //                 .map(|x|x.start_loc())
    //                 .unwrap_or(self.tokens_remaining.start_loc())

    //             // self.tokens_remaining.loc()
    //         } else {
    //             // println!("-- here2");
    //             self.expect_token_start2.first().ok()
    //                 // .or_else(||self.tokens_remaining.first().ok())
    //                 .and_then(|x|x.prevs().rev().find(|x|!x.is_eol()))
    //                 .map(|x|x.end_loc())
    //                 // .map(|x|x.start_loc())
    //                 .unwrap_or(self.tokens_remaining.start_loc())


    //             // let x=self.expect_token_start2.first().unwrap().prevs().rev().find(|x|!x.is_eol()).map(|x|x.end_loc());

    //             // x.unwrap_or(self.expect_token_start2.loc())
    //             // self.expect_token_start2.loc()
    //             // self.tokens_furthest.last_loc()
    //             // self.tokens_furthest.first().map(|x|x.end_loc()).unwrap_or(self.tokens_furthest.loc())
    //             // self.expect_token_start2.last_loc()
    //         }
    //     } else if self.use_expect1 {
    //         if self.expects1.is_empty() {
    //             self.tokens_remaining.start_loc()
    //         } else {
    //             self.expected_tokens_remaining1.start_loc()
    //         }
    //     } else {
    //         self.tokens_remaining.start_loc()
    //     }

    //     // //

    //     // let out_loc=if self.expects1.is_empty() {
    //     //     self.tokens_remaining.loc()
    //     // } else {
    //     //     if self.use_expect2 {
    //     //         self.expect_token_start2.loc()
    //     //     } else if self.use_expect1 {
    //     //         self.expected_tokens_remaining1.loc()
    //     //     } else {
    //     //         self.tokens_remaining.loc()
    //     //     }
    //     // };

    //     // // println!("l3 {out_loc:?}");

    //     // out_loc
    // }

    pub fn expects_string(&self) -> String {
        if self.use_expect2 {
            self.expecteds_string2()
        } else if self.use_expect1 {
            self.expecteds_string1()
        } else {
             String::new()
        }
    }


    fn expecteds_string2(&self) -> String {
        if !self.use_expect2 {return String::new();}

        //
        self.expects2.iter().rev().map(|x|match &x.expect_type {
            // TempExpectType::NoExpect => "",
            TempExpectType::Expect(n) => n,
            TempExpectType::Int => "int",
            TempExpectType::Float => "float",
            TempExpectType::String => "string",
            TempExpectType::Identifier => "identifier",
            TempExpectType::Symbol(s) => s,
            TempExpectType::Keyword(s) => s,
            TempExpectType::Eol => "eol",
        })
        // .filter(|x|!x.is_empty())
        .collect::<Vec<_>>().join(", ")
    }

    //
    fn expecteds_string1(&self) -> String {
        if !self.use_expect1 {return String::new();}

        //
        self.expects1.iter().rev().map(|x|match &x.expect_type {
            TempExpectType::Expect(n) => n,
            TempExpectType::Int => "int",
            TempExpectType::Float => "float",
            TempExpectType::String => "string",
            TempExpectType::Identifier => "identifier",
            TempExpectType::Symbol(s) => s,
            TempExpectType::Keyword(s) => s,
            TempExpectType::Eol => "eol",
        })
        // .filter(|x|!x.is_empty())
        .collect::<Vec<_>>().join(", ")
    }

    //
    // fn trim_groups(&mut self) {
    //     for g in &mut self.groups {
    //         if g.trim {
    //             g.tokens.trim2();
    //         }
    //     }
    //     for (i,g) in self.groups.iter_mut().enumerate() {
    //         if i!=0 && g.trim {
    //             g.tokens.trim2();
    //         }
    //     }
    // }
    //
    pub fn get_walk(&self, in_betweens:bool) -> Walk<'g,TS> {
        //
        let mut groups_out: Vec<WalkGroup<'g,TS>>=Vec::new();//vec![WalkGroup{ name: "", children: 0..0, tokens: todo!() }];


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
                    println!("\t{i}: g{g}, p{p}, {:?}, {:?}, {:?}",group_infos[g].name,group_infos[g].tokens.inds2(),group_infos[g].tokens);
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
            groups_out.push(WalkGroup { name: Some(g.name),
                children: 0..0, // csum..csum+c
                tokens: g.tokens.clone(),
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

        //insert groups for ungrouped tokens
        if in_betweens {
            let mut groups_out2=vec![WalkGroup{
                name: groups_out[0].name.clone(),
                children: 0..0,
                tokens: groups_out[0].tokens.clone(),
            }];

            {
                let mut stk=vec![(0,0)]; //old_group_ind, new_group_ind

                while let Some((gind,gind_new))=stk.pop() {
                    let g=&groups_out[gind];
                    let group_outs2_start=groups_out2.len();
                    // for cind in g.children.clone() {

                    // }

                    let mut cur_tokens = g.tokens.clone();

                    let mut children_new_inds=Vec::new();

                    for cind in g.children.clone()
                    {
                        let child_group=&groups_out[cind];
                        let child_tokens=child_group.tokens.clone();
                        let between_tokens_len=child_tokens.inds2().start-cur_tokens.inds2().start;

                        if between_tokens_len!=0 {
                            let mut between_tokens=cur_tokens.clone();
                            between_tokens.truncate2(between_tokens_len);
                            groups_out2.push(WalkGroup { name: None, children: 0..0, tokens: between_tokens });
                        }

                        children_new_inds.push((cind,groups_out2.len()));

                        groups_out2.push(WalkGroup {
                            name: child_group.name,
                            children: 0..0,
                            tokens: child_group.tokens.clone(),
                        });

                        // cur_tokens.take2(between_tokens_len+child_tokens.len2());
                        cur_tokens.eat2(child_tokens.inds2().end-cur_tokens.clone().inds2().start);
                        // cur_tokens.take(n)
                    }

                    if cur_tokens.len2()!=0 &&g.children.len()!=0 {
                        groups_out2.push(WalkGroup { name: None, children: 0..0, tokens: cur_tokens });
                    }


                    stk.extend(children_new_inds.into_iter().rev());
                    {
                        let group_outs2_end=groups_out2.len();
                        let gnew=&mut groups_out2[gind_new];
                        gnew.children=group_outs2_start..group_outs2_end;

                    }
                }
            }

            groups_out=groups_out2;
        }

        //
        let walk=Walk{ groups: groups_out };
        walk
    }

    //
    pub fn run(&mut self,start_non_term:&'g str,) -> Result<(),GrammarWalkError<'g>> {
        //
        self.init(start_non_term)?;

        //
        let mut result: Result<(), GrammarWalkError<'g>>=Ok(());

        //
        while let Some(cur)=self.stk.pop() {

           if let Err(e)=self.step(cur) {


                //
                if self.debug {
                    // let err_loc=self.last_loc();

                    // match e {
                    //     GrammarWalkError::RecursiveNonTerm(t) => {
                    //         println!("Recursive NonTerm {t:?}, At {}",self.tokens_remaining.start_loc());
                    //     }
                    //     GrammarWalkError::MissingNonTerm(t) => {
                    //         println!("Missing NonTerm {t:?}, At {}",self.tokens_remaining.start_loc());
                    //     }
                    //     GrammarWalkError::FailedParse => {

                    //         println!("Failed parse, At {}, expected {:?}",self.last_loc(),"self.expecteds_string()");
                    //     }
                    //     GrammarWalkError::Unfinished =>{}
                    // }
                }

                result=Err(e);
                break;
           } else {
           }
        }

        //
        // if self.debug {
        //     println!("groups={:?}",self.groups);
        // }

        //trim eols
        self.tokens_remaining.trim2();

        //
        if !result.is_err() && !self.tokens_remaining.is_empty2() {
            if self.debug {
                // // println!("error, failed to parse all tokens {:?}",self.primitives_remaining);
                // println!("error, failed to parse all tokens, at {}",self.last_loc());
                // // println!("{:?}",self.expecteds); //self.expected.1 should be empty?
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
        if result.is_err() {

            self.expect_on_error2();
            self.expect_on_error1();
        }

        //
        // if self.use_expect1 && result.is_err() {
        //     if self.debug {
        //         println!("expects:");
        //         for (i,x) in self.expects1.iter().enumerate() {
        //             // println!("e {:?} || {:?} || {} => {} || {:?}",x.expected_type,x.tokens_start.inds2().start,x.tokens_start.loc(),x.tokens_start.last_loc(),x.tokens_start.inds());

        //             println!("    e{i}:p{}:t{} {:?} :: {:?}",
        //                 x.parent.map(|q|format!("{q}")).unwrap_or("_".to_string()),
        //                 x.tokens_start.inds2().start,
        //                 x.expect_type,
        //                 x.tokens_start,
        //             );

        //         }
        //     }


        // }

        //
        // if self.debug {
        //     println!("===a {}",self.tokens_remaining.is_empty());
        // }

        //
        // if self.debug {
        //     for (i,g) in self.groups.iter().enumerate() {
        //         println!("g{i}: {:?} {:?}",g.name,g.tokens);
        //     }
        // }

        // //
        // if self.debug {
        //     println!("top_primitives={:?}", self.top_tokens );
        // }

        //
        result
    }

    fn step(&mut self,cur:Work<'g,P,TS>) -> Result<(),GrammarWalkError<'g>> {
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
                    grammar, work_success_len: success_len, work_fail_len: fail_len, tokens,
                    group_ind, group_len,
                    // and_id,
                    grammar_ind,
                    first,
                    stow_new_len,
                    // hist_stows_stk_len,hist_ends_stk_len,
                    // hist_stows_ind,
                    stow_len,
                    // hist_stows_stk_len,
                    // hist_fails_len,
                    // hist_prevs_ind,
                    // hist_prevs_len,
                    // expected_news_len,expect_len,
                    in_expect,
                    expect_ind1,expect_len1,
                    expect_new_len2,expect_len2,
                    // was_start_ind,was_ind,was_len,
                    was_new_len,was_ind,
                    ..
                }=&cur;

                //
                let ps=tokens.inds2();
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
                let grammar=match grammar.as_ref() {
                    GrammarNode::And(gs, error_after ) => {
                        Rc::new(GrammarNode::And(Box::from(&gs[*grammar_ind..]),*error_after))
                    }
                    GrammarNode::Or(gs, ) => {
                        Rc::new(GrammarNode::Or(Box::from(&gs[*grammar_ind..]),))
                    }
                    _ => grammar.clone(),
                };

                //
                println!("");
                println!("=>{c:4}: {grammar:?}, ps={ps:?}, success={success_len}, fail={fail_len}, first={first}",);

                if false {
                    // println!("        and_id={and_id}, groups.len={groups_len2}, group_ind={group_ind}, group_len={group_len}, gs={temp_groups:?}",);
                    println!("        groups.len={groups_len2}, group_ind={group_ind}, group_len={group_len}, gs={temp_groups:?}",);


                    // println!("        first={is_first}, stow_new_len={stow_new_len}, hist_stows_stk_len={hist_stows_stk_len}:{}, hist_ends_stk_len={hist_ends_stk_len}:{}, ",
                    //     self.hist_stows_stk.last().map(|x|x.elements.len()).unwrap_or_default(),
                    //     self.hist_ends_stk.last().map(|x|x.elements.len()).unwrap_or_default(),
                    // );
                    // let stow_len=if *hist_stows_stk_len==0{None}else{
                    //     self.hist_stows_stk.get(hist_stows_stk_len-1).map(|x|x.elements.len())
                    // };
                    // println!("        first={first}, stow_new_len={stow_new_len}, stow_len={stow_len:?}, hist_prevs_ind={hist_prevs_ind}, hist_prevs_len={hist_prevs_len}",);
                    // println!("        first={first}, hist news_len={stow_new_len}, stows_len={stow_len:?}, prevs_ind={hist_prevs_ind}, fails_len={hist_fails_len},",);
                    // println!("        actual: hist news_len={}, stows_len={:?}, prevs_len={}, fails_len={}",
                    //     self.hist_news.len(),self.hist_stows.len(),self.hist_prevs.len(),self.hist_fails.len(),
                    // );
                    println!("        first={first}, hist news_len={stow_new_len} ({}), stows_len={stow_len:?} ({})",
                        self.stow_news.len(),
                        self.stows.len(),
                    );
                    // println!("        hist_stows_ind={hist_stows_ind}, stow_len={stow_len},",
                    //     self.stk.get(cur.)
                    // );

                }

                //
                if true {
                    println!("        expect2_new_len={expect_new_len2:?} ({}), expect_len2={expect_len2} ({})",self.expect_news2.len(),self.expects2.len(),);
                    println!("        in_expect={in_expect}");
                    println!("        expect_news2=[{}]",
                        self.expect_news2.iter().enumerate()
                            .map(|(i,x)|format!("e{i}:t{}:{:?}",
                                x.tokens_start.inds2().start,
                                x.expect_type,
                            ))
                            .collect::<Vec<_>>().join(", "),
                    );
                     println!("        expects2=[{}]",
                        self.expects2.iter().enumerate()
                            .map(|(i,x)|format!("e{i}:t{}:{:?}",
                                x.tokens_start.inds2().start,
                                x.expect_type,
                            ))
                            .collect::<Vec<_>>().join(", "),
                    );
                }

                //
                if true {
                    println!("        expect_ind1={expect_ind1:?}, expect_len1={expect_len1}, expecteds1.len={}",self.expects1.len());

                     println!("        expecteds1=[{}]",
                        self.expects1.iter().enumerate()
                            .map(|(i,x)|format!("e{i}:p{}:t{}:{:?}",
                                x.parent.map(|q|format!("{q}")).unwrap_or("_".to_string()),
                                x.tokens_start.inds2().start,
                                x.expect_type,
                            ))
                            .collect::<Vec<_>>().join(", "),
                    );
                } else if false {
                    println!("        expecteds1: ind={expect_ind1:?}, len={expect_len1} ({})",self.expects1.len());

                    for (i,x) in self.expects1.iter().enumerate() {
                        println!("            e{i}:p{}:t{}: {:?}",
                            x.parent.map(|q|format!("{q}")).unwrap_or("_".to_string()),
                            x.tokens_start.inds2().start,
                            x.expect_type,
                        );
                    }

                }

                //
                if true {
                    //

                     println!("        was_news: len={was_new_len} ({})",self.was_news.len());

                    for (i,w) in self.was_news.iter().enumerate().rev() {
                        println!("            {i}: {:?}",w.name);
                    }

                    //
                     println!("        wases: ind={was_ind}, len= ({})",self.wases.len());

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
                //     println!("        expected_news_len={expected_news_len}, expect_len={expect_len}");
                //     println!("        expected news={:?}",
                //         self.expected_news.iter().map(|x|&x.expected_type).collect::<Vec<_>>(),
                //     );
                //      println!("        expected={:?}",
                //         self.expecteds.iter().map(|x|&x.expected_type).collect::<Vec<_>>(),
                //     );

                // }

                //
                if true {
                    //
                    println!("        hist_news: len={stow_new_len} ({})",self.stow_news.len(),);

                    for (i,h) in self.stow_news.iter().enumerate() {
                        println!("            {i}:t{}: {:?}",h.tokens_start.inds2().start,h.grammar)
                    }

                    //
                    println!("        hist_stows {stow_len} ({})",self.stows.len());

                    for (i,x) in self.stows.iter().enumerate().rev() {
                        println!("            {i}:t{}: s:{} : f:{} ",x.tokens_start_ind,
                            x.success.as_ref().map(|v|format!("{:?}",v.grammar)).unwrap_or_else(||"_".to_string()),
                            x.fail.as_ref().map(|v|format!("{:?}",v.grammar)).unwrap_or_else(||"_".to_string()),
                        );

                    }


                    // if true {
                    //     for (i,x) in self.hist_stows.iter().enumerate().rev() {
                    //         if let Some(v)=&x.success_val {
                    //             println!("            s:{i}:{}",x.success_val.as_ref().map(|y|format!("{:?}",&y.grammar)).unwrap_or("_".to_string()));
                    //         }
                    //         if let Some(v)=&x.fail_val {
                    //             println!("            f:{i}:{}",x.fail_val.as_ref().map(|y|format!("{:?}",&y.grammar)).unwrap_or("_".to_string()));
                    //         }
                    //         // println!("            fs:{i}:{}",x.fail_vals.grammers.iter().map(|x|format!("{x:?}")).collect::<Vec<_>>().join(", "));
                    //     }

                    // } else
                    // if *stow_len!=0 {
                    //     let hist_stow=self.hist_stows.last().unwrap();

                    //     if let Some(hist_stow_val)=&hist_stow.success_val {
                    //         let hist_stow_groups=&self.hist_stows_groups[
                    //             hist_stow.stow_groups_start..hist_stow_val.stow_groups_end
                    //         ];
                    //         // let hist_stow_prev=&self.hist_stows_prevs[
                    //         //     hist_stow.stow_prevs_start..hist_stow_val.stow_prevs_end
                    //         // ];
                    //         println!("            s");
                    //         println!("            grammar={:?}",hist_stow_val.grammar);
                    //         println!("            groups={:?}",hist_stow_groups.iter().enumerate().map(|(i,g)|format!("{i}:p{}:{}",
                    //             g.parent,
                    //             g.name,
                    //         )).collect::<Vec<_>>());
                    //     }
                    //     if let Some(hist_stow_fail_val)=&hist_stow.fail_val {
                    //         println!("            f");
                    //         println!("            grammar={:?}",hist_stow_fail_val.grammar);

                    //     }
                    // }

                    // // //
                    // // println!("        hist_prevs_last {hist_prevs_ind}..{hist_prevs_len} : {}",
                    // //     self.hist_prevs.len(),
                    // // );

                    // // // for i in *hist_prevs_ind.. self.hist_prevs.len() //*hist_prevs_len
                    // // for (i,x) in self.hist_prevs[*hist_prevs_ind..].iter().enumerate()
                    // // {
                    // //     // let x=&self.hist_prevs[i];
                    // //     println!("            {i}:[{:?}]: {:?}",x.tokens_start_ind,x.grammar)
                    // // }

                    // //
                    // println!("        hist_fails {} : ({})",cur.hist_fails_len,self.hist_fails.len());

                    // if false {

                    //     for (i,x) in self.hist_fails.iter().enumerate().rev() {
                    //         println!("            {i}: {:?}",x.grammers);
                    //     }
                    // } else
                    // if cur.hist_fails_len!=0 {
                    //     let his_fails_last=&self.hist_fails[cur.hist_fails_len-1].grammers;

                    //     for (i,x) in his_fails_last.iter().enumerate() {
                    //         println!("            {i}: {x:?}",);
                    //     }

                    // }

                }

                //
                println!("        tokens {tokens:?}");
            }

            //
            if false {
                for (i,Work {grammar:g, work_success_len:s, work_fail_len:f, tokens,
                    group_ind, group_len,..}) in self.stk.iter().enumerate()
                {
                    // println!("    {i:3}: ps={:?}, success={s}, fail={f}, and_id={and_id}, group_ind={group_ind}, group_len={group_len}, {g:?},",tokens.inds());
                    println!("    {i:3}: ps={:?}, success={s}, fail={f}, group_ind={group_ind}, group_len={group_len}, {g:?},",tokens.inds2());
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
            GrammarNode::Expect(..) => {self.grammar_expect(cur);}
            // GrammarNode::Stow(..) => {self.grammar_stow(cur);}
            GrammarNode::Was(..) => {self.grammar_was(cur);}
            GrammarNode::Had(..) => {self.grammar_had(cur);}

            // GrammarNode::Prev(..) => {self.grammar_prev(cur);}
            GrammarNode::Group(..) => {self.grammar_group(cur);}
            GrammarNode::And(..) => {self.grammar_and(cur);}
            GrammarNode::Or(..) => {self.grammar_or(cur);}
            GrammarNode::Many(..) => {self.grammar_many(cur);}
            GrammarNode::NonTerm(..) => {self.grammar_non_term(cur)?;}
            GrammarNode::Error => {return Err(self.grammar_error(cur));}
            GrammarNode::Always => {self.grammar_always(cur);}

            // GrammarNode::String|GrammarNode::Identifier|GrammarNode::Int
            // |GrammarNode::Float|GrammarNode::Symbol(..)|GrammarNode::Keyword(..)
            // |GrammarNode::Eol
            GrammarNode::Primitive(..)=> { self.grammar_primitive(cur,); }

        }

        //
        Ok(())
    }

    //
    pub fn set_debug(&mut self,debug:bool) {
        self.debug=debug;
    }

    // pub fn set_hist_non_term_only(&mut self,hist_non_term_only:bool) {
    //     self.hist_non_term_only=hist_non_term_only;
    // }


    fn get_non_term(&mut self,n:&'g str) -> Result<Rc<GrammarNode<'g,P>>,GrammarWalkError<'g>> {
        if let Some(g)=self.non_term_cache.get(n) {
            Ok(g.clone())
        } else
        if let Some(g)=(self.grammar_func)(n) {
            self.non_term_cache.insert(n, g.clone());
            Ok(g)
        } else {
            Err(GrammarWalkError::MissingNonTerm(n))
        }
    }
    pub fn step_count(&self) -> usize {
        self.step_count
    }
}