

use super::error::*;
use super::temp_data::*;
use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;

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
            expected_news:Default::default(),
            expecteds:Default::default(),

            grammar_func,
            primitives_remaining:top_primitives.clone(),
            top_primitives,
            debug:false,
            non_term_recursive_check:true,

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
            and_id: 0,

            from_user:false,
            is_first:true,

            hist_news_len:0,
            hist_begins_stk_len:0,
            hist_ends_stk_len:1,

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
            is_first:true,

            hist_news_len:0,
            hist_begins_stk_len:0,
            hist_ends_stk_len:1,

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
                is_first:true,

                hist_news_len:0,
                hist_begins_stk_len:0,
                hist_ends_stk_len:1,

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
        self.hist_begins_stk.clear(); //don't need initial one because don't need to store begins before an Or exists
        // self.hist_ends_stk.clear();
        self.hist_ends_stk=vec![Default::default()]; //need an initial one because require ends regardless of an Or existing

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
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,

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
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,

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
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,

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
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,

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
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,

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
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,

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
        self.set_remaining_prims(&cur);

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
                is_first:false,

                hist_news_len,
                hist_begins_stk_len:cur.hist_begins_stk_len,
                hist_ends_stk_len:cur.hist_ends_stk_len,

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
            is_first:cur.is_first, //cur.from_user &&  //only want to know about grammars added by user, not the walker, could check from_user elsewhere,

            hist_news_len,
            hist_begins_stk_len:cur.hist_begins_stk_len,
            hist_ends_stk_len:cur.hist_ends_stk_len,

            expected_news_len:cur.expected_news_len,
        });
    }

    fn grammar_or(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Or(gs)=&cur.grammar else{panic!("");};

        //
        let Some(g_first)=gs.first().cloned() else { return; };

        //
        let hist_news_len=self.hist_news_add(&cur);
        let hist_begins_stk_len=self.hist_begins_stk_push(&cur);
        let hist_ends_stk_len=self.hist_ends_stk_push(&cur);

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
                is_first:cur.is_first,

                hist_news_len,
                hist_begins_stk_len,
                hist_ends_stk_len,

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
            is_first:cur.is_first,

            hist_news_len,
            hist_begins_stk_len,
            hist_ends_stk_len,

            expected_news_len:cur.expected_news_len,
        });
    }

    fn grammar_prev(&mut self,cur :Work<'t,'g>,) {
        let GrammarNode::Prev(g)=&cur.grammar else {panic!("");};
        //
        let _hist_news_len=self.hist_news_add(&cur);

        //
        if self.hist_ends_stk.last().unwrap().elements.contains_key(&g) {
            //
            self.stk.truncate(cur.success_len);
            self.handle_exit_last_many(&cur);

            //
            if let Some(last)=self.stk.last_mut() {
                last.tokens=cur.tokens;
                last.group_len=cur.group_len;
            }

            //
            // self.hist_news_truncate_to_last(); //why on success??
            self.do_groups_primitives_clamp(cur.group_ind,cur.tokens);
            self.hist_news_drain(&cur,true,false);
            self.expect_news_truncate_to_last();
            self.set_remaining_prims(&cur);
        } else {
            //
            self.stk.truncate(cur.fail_len);
            self.hist_news_truncate_to_last();
            let _expected_news_len=self.add_expected_new(&cur);
            self.expected_news_drain(&cur);
        }
    }

    fn grammar_always(&mut self,cur :Work<'t,'g>,) {
        //
        self.stk.truncate(cur.success_len);
        let _hist_news_len=self.hist_news_add(&cur);
        self.handle_exit_last_many(&cur);

        //
        if let Some(last)=self.stk.last_mut() {
            if self.debug {
                println!("---- last.group_len={}, cur.group_len={}, last.group_ind={}, cur.group_ind={}",
                    last.group_len,cur.group_len,
                    last.group_ind, cur.group_ind,
                );
            }

            //
            last.tokens=cur.tokens;
            last.group_len=cur.group_len; //done below //not anymore
        }

        //
        self.do_groups_primitives_clamp(cur.group_ind,cur.tokens); //here
        self.hist_news_drain(&cur,true,false);
        self.expect_news_truncate_to_last();
        self.set_remaining_prims(&cur);
        // self.clear_expected();
    }

    fn grammar_try_from_hist_begins(&mut self,cur :&Work<'t,'g>) -> bool {
        //
        if !cur.from_user || !cur.is_first {return false;} // !(cur.from_user && cur.is_first)

        //
        let Some(hist_begins)=self.hist_begins_stk.last() else {return false;};
        let Some(hist_begin)=hist_begins.elements.get(&cur.grammar) else {return false;};

        //
        self.stk.truncate(cur.success_len);

        //
        if let Some(last)=self.stk.last_mut() {
            //add groups
            for g in hist_begin.groups.iter() {
                self.groups.push(TempGroupInfo { parent: cur.group_ind+g.parent, ..g.clone()});
            }

            //
            last.group_len=self.groups.len(); //cur.group_len+or_element.groups;
            last.tokens=hist_begin.tokens_after; //cur.tokens
        }

        //
        self.do_groups_primitives_clamp(cur.group_ind,cur.tokens);
        self.hist_news_drain(&cur,false, false); //not needed? no.. if And(Z,Or(And(X,Y),X)), then will add that
        self.expect_news_truncate_to_last();
        self.set_remaining_prims(&cur);

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

        //
        match prim_func(&mut cur.tokens) {
            Ok(v) => {
                //
                self.stk.truncate(cur.success_len);

                //
                if let Some(last)=self.stk.last_mut() {
                    last.tokens=cur.tokens;
                    last.group_len=cur.group_len;
                }

                //
                self.do_groups_primitives_clamp(cur.group_ind,cur.tokens);
                self.hist_news_drain(&cur,true,true);
                self.expect_news_truncate_to_last();
                self.set_remaining_prims(&cur);

                //
                if self.debug {
                    println!("--- hmm stk={:?}",self.stk.iter().map(|x|x.grammar.clone()).collect::<Vec<_>>());

                    if let Some(last)=self.stk.last() {
                        let last_hist_ends=&self.hist_ends_stk[last.hist_ends_stk_len-1].elements;
                        println!("---last_hist_ends={:?}, len={}",last_hist_ends,last_hist_ends.len());
                    }
                }

                //
                Some(v)
            }
            Err(loc) => {
                //
                self.stk.truncate(cur.fail_len);
                self.hist_news_truncate_to_last();
                let _expected_news_len=self.add_expected_new(&cur);
                self.expected_news_drain(&cur);
                self.set_remaining_prims(&cur);

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

    fn expected_news_drain(&mut self, cur:&Work<'t,'g>,) {
        let Some(last)=self.stk.last_mut() else {panic!("");};
        let drained_expected_news=self.expected_news.drain(last.expected_news_len ..).collect::<Vec<_>>();

        //
        for drained_expected_new in drained_expected_news {
            self.expecteds.push(TempExpected { expect_type: drained_expected_new.expected_type });
        }
    }

    fn expect_news_truncate_to_last(&mut self) {
        let Some(last)=self.stk.last() else {panic!("");};
        self.expected_news.truncate(last.expected_news_len);
    }

    fn hist_news_drain(&mut self,
        cur:&Work<'t,'g>,
        gotten:bool, //what was this for again? something to do with not adding cur grammar to hist_begins? it was for not adding cur grammar to hist_new?
        hist_ends_remove_previous:bool,
    ) {
        //should always be some (due to init), use panic instead of ret?
        let Some(last)=self.stk.last_mut() else {return;};

        //
        last.hist_ends_stk_len=cur.hist_ends_stk_len;

        //why only use for primitives?
        if hist_ends_remove_previous {
            let last_tokens_start=last.tokens.inds().start;

            self.hist_ends_stk[last.hist_ends_stk_len-1].elements.retain(|_k,v|{
                v.tokens.inds().start>=last_tokens_start
                // last_tokens_start<v.tokens.inds().start
            });
        }

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

            //
            (  hist_new.grammar.clone(), TempHistEnd { tokens, } )
        }).collect::<Vec<_>>();

        //
        for i in 0..drained_hist_news.len() {
            let drained_hist_new=&drained_hist_news[i];

            // //
            // if self.debug {
            //     println!("---going i={i}, is_first={}, g={:?} self.or_stk.len={}",
            //         hist_new.is_first,hist_new.grammar,
            //         self.hist_begins_stk.len(),
            //     );
            // }

            //
            if !drained_hist_new.is_first { continue; }
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
            hist_begins.elements.entry(drained_hist_new.grammar.clone()).or_insert_with(||TempHistBegin {
                groups,
                hist_ends: HashMap::from_iter(added_hist_ends[i+1..].into_iter().cloned()),
                tokens_after: cur.tokens,
            });
        }

        //
        self.hist_ends_stk[last.hist_ends_stk_len-1].elements.extend(added_hist_ends.into_iter());
    }

    fn hist_news_truncate_to_last(&mut self) {
        let Some(last)=self.stk.last() else {panic!("");};
        self.hist_news.truncate(last.hist_news_len); //what is it for? clears failed takeable_starts?
    }

    fn hist_begins_stk_push(&mut self,cur:&Work<'t,'g>) -> usize {
        if  self.hist_begins_stk.is_empty() //reason? shouldn't it not be empty, as would be created by first? there was a reason ...
            || !cur.is_first //don't need to use cur.from_user, first is better, as could handle potential Or not added by user
        {
            self.hist_begins_stk.push(Default::default());
        }

        self.hist_begins_stk.len()

    }
    fn hist_ends_stk_push(&mut self,cur:&Work<'t,'g>) -> usize {
        if cur.is_first {
            self.hist_ends_stk.push(self.hist_ends_stk.last().cloned().unwrap());
        }

        self.hist_ends_stk.len()
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
                let n=group.tokens.len()-cur_primitives.len();
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

    fn set_remaining_prims(&mut self,cur:&Work<'t,'g>) {
        if !self.stk.is_empty() {return;}
        self.primitives_remaining=cur.tokens;
    }

    fn handle_exit_last_many(&mut self,cur:&Work<'t,'g>) { //if not parsing anything, exit the many
        let Some(last)=self.stk.last_mut() else {panic!("");};
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
            if self.hist_begins_stk.len() != cur.hist_begins_stk_len {
                println!("--- or_stk dif len, or_stk.len={}, cur.or_stk_len={}",self.hist_begins_stk.len(),cur.hist_begins_stk_len);
            }
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
                    group_ind, group_len,and_id,is_first,
                    hist_news_len,hist_begins_stk_len,hist_ends_stk_len,
                    ..
                }=&cur;

                //
                let ps=tokens.inds();
                let temp_groups=groups.iter().enumerate().map(|(i,x)|format!("g{i}:p{}:{}",x.parent,x.name)).collect::<Vec<_>>();
                let groups_len2=groups.len();

                //
                println!("=>{c:4}: {grammar:?}, ps={ps:?}, success={success_len}, fail={fail_len}, ",);
                println!("        and_id={and_id}, groups.len={groups_len2}, group_ind={group_ind}, group_len={group_len}, gs={temp_groups:?}",);
                println!("        first={is_first}, hist_news_len={hist_news_len}, hist_begins_stk_len={hist_begins_stk_len}:{}, hist_ends_stk_len={hist_ends_stk_len}:{}, ",
                    self.hist_begins_stk.last().map(|x|x.elements.len()).unwrap_or_default(),
                    self.hist_ends_stk.last().map(|x|x.elements.len()).unwrap_or_default(),
                );

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
}