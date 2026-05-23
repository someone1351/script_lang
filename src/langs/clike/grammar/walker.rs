

use super::error::*;
use super::temp_data::*;
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

    // primitive_infos : Vec<TempPrimitiveInfo>,
    // group_infos : Vec<TempGroupInfo<'t,'g>>,
    groups_stk:Vec<TempGroupsElement<'t,'g>>,

    // takeable_starts:Vec<(GrammarNode<'g>,TokenIterContainer<'t>)>, //[(g,output_ind_start)]
    takeable_starts:Vec<TempTakeableStart<'t,'g>>,
    grammar_func:G,

    stk: Vec<Work<'t,'g>>,
    c:usize,
    expected_loc:Loc,
    expecteds:Vec<(u32,GrammarNode<'g>)>, //(priority,gramamr)//(u64,GrammarNode<'g>) //(id,grammar) //todo change grammar to &'g
    expected_count:u64,
    // expected_in_non_term:bool,
    // expected: (Loc,Vec<GrammarNode<'g>>,),
    debug:bool,

    grammar_debug_stk:Vec<TempGrammarNodeDebug<'t,'g>>,
    // keywords : HashSet<&'a str>,
    // keywords : &'a HashSet<&'a str>,
    // tokenized:Tokenized<'a>,

    non_term_recursive_check:bool,
}


impl<'t,'g,G> GrammarWalker<'t,'g,G>
where
    G: Fn(&str)->Option<GrammarNode<'g>>,
{

    pub fn new
        // <K>
        (top_primitives:TokenIterContainer<'t>, grammar_func:G,
        // keywords:K,
        // keywords:&'a HashSet<&'a str>,
    ) -> Self
    // where
    //     K:IntoIterator<Item = &'a str>,
    {
        Self {
            // primitive_infos :  Default::default(),
            // group_infos : Default::default(),
            groups_stk:Default::default(),
            takeable_starts: Default::default(),
            stk:Default::default(),
            c:Default::default(),
            // expected:Default::default(),
            expected_loc:Loc::zero(),
            expecteds:Vec::new(),
            // expected_in_non_term:false,
            grammar_func,
            primitives_remaining:top_primitives.clone(),
            top_primitives,
            debug:false,
            // keywords:HashSet::from_iter(keywords.into_iter()),
            // keywords,
            grammar_debug_stk:Vec::new(),
            non_term_recursive_check:true,
            expected_count: 0,
        }
    }

    pub fn set_non_term_recursive_check(&mut self,non_term_recursive_check:bool) {
        self.non_term_recursive_check=non_term_recursive_check;
    }

    fn init(&mut self,start_non_term:&'g str,) {
        self.stk.clear();

        self.stk.push(Work{
            grammar:GrammarNode::Error(GrammarWalkError::FailedParse),success_len:0,fail_len:0,tokens:self.top_primitives,
            group_ind: 0, group_len: 1,
            // output_len: 0,
            // discard:false,
            // takeable_starts:Default::default(),
            takeable_starts_len:0,
            visiteds:Default::default(),
            takeables:Default::default(),
            // opt:false,
            grammar_debug_len: 0,
            // grammar_debug_no_add: true,
            // expected:None,
            expected:Default::default(),
            groups_stk_ind: 0,

        });
        {
            let grammar=if let Some(g)=(self.grammar_func)(start_non_term) {
                g
            } else {
                GrammarNode::Error(GrammarWalkError::MissingNonTerm(start_non_term))
            };

            self.stk.push(Work{
                grammar, //:(self.grammar_func)(start_non_term),
                success_len:0,fail_len:1,tokens:self.top_primitives,
                group_ind: 0, group_len: 1,
                // output_len: 0,
                // discard:false,
                // takeable_starts:Default::default(),
                takeable_starts_len:0,
                visiteds:Default::default(),
                takeables:Default::default(),
                // opt:false,
                grammar_debug_len: 1,
                // grammar_debug_no_add: false,
                // expected:None,
                expected:Default::default(),
                groups_stk_ind: 0,
            });
        }

        //
        // self.primitive_infos.clear();
        // self.group_infos=vec![TempGroupInfo{
        //     name: "",
        //     parent: 0,
        //     // primitive_ind_start:0,
        //     primitives:self.top_primitives,
        // }];

        //
        self.groups_stk=vec![TempGroupsElement{
            groups: vec![TempGroupInfo{
                name: "",
                parent: 0,
                tokens:self.top_primitives,
            }],
            // token_groups: Vec::new(),
            tokens_start:0,
        }];

        //
        self.takeable_starts.clear();

        // self.primitives_remaining:top_primitives.clone(),
        // self.top_primitives,

        self.c=0;
        // self.expected=Default::default();
        self.expected_loc=Loc::zero();
        self.expecteds.clear();
        self.expected_count=0;
        // self.expected_in_non_term=false;

        self.grammar_debug_stk.clear();
    }

    pub fn run(&mut self,start_non_term:&'g str,) -> Result<(),GrammarWalkError<'g>> {
        self.init(start_non_term);

        let mut result: Result<(), GrammarWalkError<'g>>=Ok(());

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
                // return Err(e);
           }
        }


        //
        if self.debug {
            let group_infos=&self.groups_stk.last().unwrap().groups;
            // let primitive_infos=&self.groups_stk.last().unwrap().token_groups;

            println!("groups={:?}",group_infos);
            // println!("outputs={:?}",primitive_infos);
        }

        if !result.is_err() && !self.primitives_remaining.is_empty() {
            // self.expected_loc=self.primitives_remaining.loc();

            if self.debug {
                // println!("error, failed to parse all tokens {:?}",self.primitives_remaining);
                println!("error, failed to parse all tokens, at {}",self.expected_loc);
                println!("{:?}",self.expecteds); //self.expected.1 should be empty?
            }

            // return Err(GrammarWalkError::Unfinished);

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

        if self.debug {
            println!("===a {}",self.primitives_remaining.is_empty());
        }


        //
        if self.debug {
            let group_infos=&self.groups_stk.last().unwrap().groups;

            for (i,g) in group_infos.iter().enumerate() {
                println!("g{i}: {:?} {:?}",g.name,g.tokens);
            }
        }


        //
        // if self.debug {
        //     let mut groups_visited: HashSet<usize>=HashSet::new();

        //     for p in self.top_primitives {
        //         let i=p.ind();
        //         let Some(output)=self.primitive_infos.get(i) else {
        //             break;
        //         };

        //         let mut g=output.group;
        //         let mut depth=0;
        //         let mut gs: Vec<usize>=Vec::new();

        //         while g!=0 {
        //             gs.push(g);
        //             let gg=&self.group_infos[g];

        //             depth+=1;

        //             g=gg.parent;

        //         }

        //         for (d,&g) in gs.iter().rev().enumerate() {
        //             let gg=&self.group_infos[g];

        //             if !groups_visited.contains(&g) {
        //                 println!("{}{:?} : {:?}",
        //                     "  ".repeat(d),
        //                     gg.name,
        //                     gg.primitives.inds(),
        //                 );
        //                 groups_visited.insert(g);
        //             }
        //         }

        //         println!("{}{}{p:?}",
        //             "  ".repeat(depth),
        //             if output.discard {"-----"}else{""}
        //         );
        //     }
        //     println!("===");
        // }

        if self.debug {
            //
            println!("top_primitives={:?}", self.top_primitives );
            // println!("output={outputs:?}",  );
        }
        // if result.is_err() {
        //     return result;
        // }
        // Ok(())
        result

    }

    fn step(&mut self,cur:Work<'t,'g>) -> Result<(),GrammarWalkError<'g>> {

        {

            let group_infos=&mut self.groups_stk.last_mut().unwrap().groups;
            group_infos.truncate(cur.group_len);
        }
        //

        if self.debug {
            if
                // !cur.grammar_debug_no_add
                cur.grammar_debug_len> self.grammar_debug_stk.len()
            {
                let x=match cur.grammar {
                    GrammarNode::Many(_) => TempGrammarNodeDebug::Many(vec![]),
                    GrammarNode::And(_) => TempGrammarNodeDebug::And(vec![]),
                    GrammarNode::Or(_) => TempGrammarNodeDebug::Or(vec![]),
                    GrammarNode::Opt(_) => TempGrammarNodeDebug::Opt(None),
                    GrammarNode::Cede(_) => TempGrammarNodeDebug::Cede(None),
                    GrammarNode::Take(_) => TempGrammarNodeDebug::Take(None),
                    GrammarNode::Group(g, _) => TempGrammarNodeDebug::Group(g,None),
                    GrammarNode::Expected(p,g, _) => TempGrammarNodeDebug::Expected(p,g,None),
                    GrammarNode::String => TempGrammarNodeDebug::String(None),
                    GrammarNode::Identifier => TempGrammarNodeDebug::Identifier(None),
                    GrammarNode::Int => TempGrammarNodeDebug::Int(None),
                    GrammarNode::Float => TempGrammarNodeDebug::Float(None),
                    GrammarNode::Symbol(_) => TempGrammarNodeDebug::Symbol(None),
                    GrammarNode::Keyword(_) => TempGrammarNodeDebug::Keyword(None),
                    GrammarNode::Eol => TempGrammarNodeDebug::Eol(None),
                    GrammarNode::NonTerm(t) => TempGrammarNodeDebug::NonTerm(t,None),
                    GrammarNode::Always => TempGrammarNodeDebug::Always,
                    GrammarNode::Error(_) => TempGrammarNodeDebug::Error,
                    // GrammarNode::Discard(_) => TempGrammarNodeDebug::Discard(None),
                };
                // println!("===x={x}");
                self.grammar_debug_stk.push(x);
            } else {
                // println!("===no-x");

            }
        }


        //
        if self.debug {
            self.c+=1;

            // // if c>30 {break;}
            // // println!(": {cur:?} || {} && {primitives:?}", self.stk.iter().rev().map(|x|format!("{:?}",x.0)).collect::<Vec<_>>().join(" << "), );
            {

            let group_infos=&self.groups_stk.last().unwrap().groups;

                let c=self.c;
                let Work { grammar, success_len, fail_len, tokens, group_ind, group_len, takeable_starts_len, visiteds, takeables, grammar_debug_len, expected, groups_stk_ind }=&cur;
                // println!("=>{c:4}: {grammar:?}, ps={primitives:?}, success={success_len}, fail={fail_len}, group_ind={group_ind}, group_len={group_len}, output_len={output_len}, discard={discard}, takeable_starts_len={takeable_starts_len:?}, visiteds={visiteds:?}, opt={opt:?}, takeables={takeables:?}, ");
                // println!("         -takeable_starts={:?}",self.takeable_starts);
                // println!("         -temp_primtives={:?}",self.primitive_infos);
                let ps=tokens.inds();
                let expected=if expected.id==0 {"None".to_string()}else{format!("{}:{}",expected.id,expected.name)};
                //  println!("=>{c:4}: {grammar:?}, ps={ps:?}, success={success_len}, fail={fail_len}, expected={expected},  ",);
                println!("=>{c:4}: {grammar:?}, ps={ps:?}, success={success_len}, fail={fail_len}, ",);
                println!("        takeable_starts_len={takeable_starts_len:?}, takeables={:?}, ", takeables.iter().map(|t|(t.0,t.1.tokens)).collect::<Vec<_>>());
                println!("        groups_stk_ind={groups_stk_ind}, group_ind={group_ind}, group_len={group_len}, temp_groups3={:?}",group_infos.iter().map(|x|x.name).collect::<Vec<_>>());

                //
                // println!("        expecteds {} : = {}", self.expected_loc,self.expecteds_string());
                println!("        tokens {tokens:?}");
            }

            if false {

                let mut grammar_debug_stk=self.grammar_debug_stk.clone();
                for _i in (1 .. grammar_debug_stk.len()).rev() {
                    let x=grammar_debug_stk.pop().unwrap();
                    // println!("=={:?}",grammar_debug_stk.last().unwrap());
                    match grammar_debug_stk.last_mut().unwrap() {
                        TempGrammarNodeDebug::Many(gs)
                        |TempGrammarNodeDebug::And(gs)
                        |TempGrammarNodeDebug::Or(gs)
                        => {gs.push(x);}

                        TempGrammarNodeDebug::Opt(g)
                        |TempGrammarNodeDebug::Cede(g)
                        |TempGrammarNodeDebug::Take(g)
                        |TempGrammarNodeDebug::Group(_, g)
                        |TempGrammarNodeDebug::NonTerm(_, g)
                        |TempGrammarNodeDebug::Expected(_,_, g)
                        // |TempGrammarNodeDebug::Discard(g)
                        => {*g=Some(x.into())}

                        _=>{panic!("");}
                    }
                }

                if let Some(x)=grammar_debug_stk.first() {

                    println!("      {x}",);
                } else {
                    println!("      _",);

                }

                //
                println!("        [{}]",
                    self.grammar_debug_stk.iter().enumerate().map(|(i,d)|format!("{i}:{d}")).collect::<Vec<_>>().join(", ")
                );



            }

            //
            if false {
                for (i,Work { grammar:g, success_len:s, fail_len:f, tokens, group_ind, group_len,  takeable_starts_len, visiteds, takeables, grammar_debug_len, expected, groups_stk_ind  }) in self.stk.iter()
                    // .rev()
                    .enumerate() {
                    // println!("\t{i:3}: {g:?}\n\t   : {ps:?}\n\t   : success={s}, fail={f}",);
                    // println!("\t{i:3}: {g:?}, ps={primitives:?},success={s}, fail={f}, group_ind={group_ind}, group_len={group_len}, output_len={output_len}, discard={discard}, takeable_starts_len={takeable_starts_len:?}, visiteds={visiteds:?}, opt={opt:?}, takeables={takeables:?}",);
                    println!("    {i:3}: ps={:?}, success={s}, fail={f}, groups_stk_ind={groups_stk_ind}, group_ind={group_ind}, group_len={group_len}, {g:?},",tokens.inds()); //

                }
            }
        }

        //
        match cur.grammar {
            GrammarNode::Expected(priority, name, g) => {
                self.expected_count+=1;
                let expected=if cur.expected.id==0 {
                    // (self.expected_count,name)
                    let priority=priority+1; //so primitives/tokens are at priority 0, expected(s) are 1+
                    WorkExpected{ id: self.expected_count, priority, name }
                } else {
                    cur.expected
                };

                //TODO
                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    tokens: cur.tokens,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    // opt:cur.opt,
                    grammar_debug_len: cur.grammar_debug_len+1,
                    // expected_non_term:cur.expected_non_term,
                    // expected:Some(name),
                    // expected:(self.expected_count,name),
                    expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });
            }
            GrammarNode::Group(name, g) => {
                let new_group_ind=self.new_group(name, cur.group_ind, cur.tokens);

                let group_infos=&self.groups_stk.last().unwrap().groups;
                let new_group_len=group_infos.len();

                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    tokens: cur.tokens,

                    group_ind: new_group_ind,
                    group_len: new_group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    // opt:cur.opt,
                    grammar_debug_len: cur.grammar_debug_len+1,
                    // grammar_debug_no_add: false,
                    expected:cur.expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });
            }
            // GrammarNode::Discard(g) => {

            //     // if cur.opt {
            //     //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
            //     // }

            //     self.stk.push(Work {
            //         grammar: *g,
            //         success_len: cur.success_len,
            //         fail_len: cur.fail_len,
            //         tokens: cur.tokens,

            //         group_ind: cur.group_ind,
            //         group_len: cur.group_len,
            //         output_len: cur.output_len,
            //         discard:true,
            //         takeable_starts_len:cur.takeable_starts_len,
            //         visiteds:cur.visiteds,
            //         takeables:cur.takeables,
            //         opt:cur.opt,
            //         grammar_debug_len: cur.grammar_debug_len+1,
            //         // grammar_debug_no_add: false,
            //         expected:cur.expected,
            //     });


            // }
            GrammarNode::And(gs) => {
                let Some(first)=gs.first().cloned() else {
                    // continue;
                    return Ok(());
                };

                if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
                    self.stk.push(Work {
                        grammar: GrammarNode::And(rest.into()),
                        success_len: cur.success_len,
                        fail_len: cur.fail_len,

                        //not really necessary? since gets updated by always/primtitives
                        tokens: cur.tokens,

                        group_ind: cur.group_ind,
                        group_len: cur.group_len,
                        // output_len: cur.output_len,
                        // discard:cur.discard,

                        takeable_starts_len:cur.takeable_starts_len,
                        visiteds:cur.visiteds.clone(),
                        takeables:cur.takeables.clone(),
                        // opt:false, //opt isnt passed to individual items in And

                        grammar_debug_len: cur.grammar_debug_len,
                        // grammar_debug_no_add: true,
                        expected:cur.expected,
                        groups_stk_ind: cur.groups_stk_ind,

                    });
                }

                let success_len=if gs.len()>1 {self.stk.len()}else{cur.success_len};

                // if cur.opt {
                //     self.takeable_starts.push((first.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: first,
                    success_len,
                    fail_len: cur.fail_len,
                    tokens: cur.tokens,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    // opt:false, //opt isnt passed to individual items in And

                    grammar_debug_len: cur.grammar_debug_len+1,
                    // grammar_debug_no_add: false,
                    expected:cur.expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });
            }
            GrammarNode::Or(gs) => {
                let Some(first)=gs.first().cloned() else {
                    // continue;
                    return Ok(())
                };

                if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
                    self.stk.push(Work {
                        grammar: GrammarNode::Or(rest.into()),
                        success_len: cur.success_len,
                        fail_len: cur.fail_len,
                        tokens: cur.tokens,

                        group_ind: cur.group_ind,
                        group_len: cur.group_len,
                        // output_len: cur.output_len,
                        // discard:cur.discard,
                        takeable_starts_len:cur.takeable_starts_len,
                        visiteds:cur.visiteds.clone(),
                        takeables:cur.takeables.clone(),
                        // opt:cur.opt,

                        grammar_debug_len: cur.grammar_debug_len,
                        // grammar_debug_no_add: true,
                        expected:cur.expected,
                        groups_stk_ind: cur.groups_stk_ind,
                    });
                }

                let fail_len=if gs.len()>1 {self.stk.len()}else{cur.fail_len};

                // if cur.opt {
                //     self.takeable_starts.push((first.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: first,
                    success_len: cur.success_len,
                    fail_len,
                    tokens: cur.tokens,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    // opt:cur.opt,

                    grammar_debug_len: cur.grammar_debug_len+1,
                    // grammar_debug_no_add: false,
                    expected:cur.expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });
            }

            GrammarNode::Opt(g) => {
                self.stk.push(Work {
                    grammar: GrammarNode::Always,
                    success_len: cur.success_len,
                    fail_len: 0, //fail is not used
                    tokens: cur.tokens,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                    // opt:false, //not used on always
                    grammar_debug_len: cur.grammar_debug_len,
                    // grammar_debug_no_add: true,
                    expected:cur.expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });

                let fail_len=self.stk.len();

                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len,
                    tokens: cur.tokens,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    // opt:true,
                    grammar_debug_len: cur.grammar_debug_len+1,
                    // grammar_debug_no_add: false,
                    expected:cur.expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });
            }
            GrammarNode::Cede(g) => {
                //should return err if not giveable? ie not opt? or just ignore?
                //  or just don't rquire at all

                // if cur.opt {
                // self.takeable_starts.push((*g.clone(),cur.tokens.clone()));
                self.takeable_starts.push(TempTakeableStart { grammar: *g.clone(), tokens_start: cur.tokens.clone(), group_ind: cur.group_ind });
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    tokens: cur.tokens,
                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    takeable_starts_len:self.takeable_starts.len(),
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    // opt:cur.opt,
                    grammar_debug_len: cur.grammar_debug_len+1,
                    // grammar_debug_no_add: false,
                    expected:cur.expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });
            }
            GrammarNode::Take(g) => {
                if let Some(takeable)=cur.takeables.get(&g).cloned() {

                    if self.debug {
                        let group_infos=&self.groups_stk.last().unwrap().groups;

                        println!("---the groups are {:?}",group_infos);
                    }

                    //
                    let taken_ancestor_groups=self.get_cur_groups(takeable.group_ind);
                    let cur_ancestor_groups=self.get_cur_groups(cur.group_ind);
                    let dif_ancestor_groups=taken_ancestor_groups.difference(&cur_ancestor_groups).cloned().collect::<Vec<_>>();

                    //
                    let take_tokens_len=takeable.tokens.len();

                    //
                    self.groups_stk.push(self.groups_stk.last().unwrap().clone());
                    let last_groups=self.groups_stk.last_mut().unwrap();

                    //clamp dif_ancestor_groups to taken.start
                    for &g in &dif_ancestor_groups {
                        let group=&mut last_groups.groups[g];
                        println!("g={g}, \ngroup.tokens.len={:?} \ncur.tokens.len={:?} \ntaken_ps_start={:?}",
                            group.tokens,cur.tokens,
                            takeable.tokens,
                        );
                        // let n=group.tokens.len()-cur.tokens.len();
                        // let group_prims=group.tokens.get_amount(n).unwrap();
                        // group.tokens=group_prims;
                        group.tokens.pop_back_amount(take_tokens_len).unwrap();
                    }

                    //
                    for &g in cur_ancestor_groups.iter() {
                        let group=&mut last_groups.groups[g];
                        if group.tokens.inds().start > takeable.tokens_start.inds().start {
                            group.tokens=takeable.tokens_start;
                        }

                    }

                    //change parent of taken children groups to cur_group_ind
                    for g in takeable.inner_groups {
                        let group=&mut last_groups.groups[g];

                        if group.parent == takeable.group_ind {
                            group.parent=cur.group_ind;
                        }
                    }

                    //change parent of output tokens who's parent in dif_ancestor_groups
                    // for x in takeable.tokens.inds() {
                    //     let g=&mut last_groups.token_groups[x];

                    //     println!("=== x={x}, g={g}, cur.group_ind={}, contians={}",cur.group_ind,dif_ancestor_groups.contains(g));

                    //     if dif_ancestor_groups.contains(g)
                    //     {
                    //         *g=cur.group_ind;
                    //     }
                    // }



                    //
                    // last_groups.tokens_start=taken_ps_start.inds().start;
                    last_groups.tokens_start=takeable.tokens.inds().start;


                    //how to remove no longer used groups, and fix inds of the used group that ccomes after the removed one?

                    // let cur_group_ind=self.remove_groups_at_except(taken_ps_start,cur.group_ind,);
                    let cur_group_ind=cur.group_ind;


                    // //clear outputs to start of taken
                    // {

                    //     let primitive_infos=&mut self.groups_stk.last_mut().unwrap().token_groups;


                    //     // primitive_infos.truncate(taken_ps_start.inds().start);
                    //     primitive_infos.truncate(taken_ps_start.tokens_start.inds().start);

                    // }

                    //
                    // if cur.opt {
                    //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                    // }


                    let group_infos=&self.groups_stk.last().unwrap().groups;






                    //
                    self.stk.truncate(cur.success_len);

                    //
                    if let Some(last)=self.stk.last_mut() {
                        if last.grammar.is_many() && last.tokens.len()==cur.tokens.len() { //if not parsing anything, exit the many
                            last.grammar=GrammarNode::Always;
                        }
                        last.tokens=cur.tokens;
                        last.group_len=cur.group_len;
                        // last.output_len=cur.output_len;
                        last.takeables=cur.takeables;
                        // let primitive_infos=&mut self.groups_stk.last_mut().unwrap().token_groups;

                        if cur.expected.id!=last.expected.id {
                            last.expected=Default::default();
                        }

                        self.takeable_starts.truncate(last.takeable_starts_len);

                        if self.debug {
                            self.grammar_debug_stk.truncate(last.grammar_debug_len);
                        }
                    }


                    //
                    if self.debug {
                        self.consolidate_grammar_debug_stk();
                    }

                    //
                    self.do_groups_primitives_clamp(cur.group_ind,cur.tokens);
                    self.last_insert_start_takeables(cur.tokens);

                    self.set_remaining_prims(cur.tokens);

                } else {

                    //
                    self.stk.truncate(cur.fail_len);

                    //
                    if let Some(last)=self.stk.last() {

                        // let primitive_infos=&mut self.groups_stk.last_mut().unwrap().token_groups;


                        // primitive_infos.truncate(last.output_len);
                        self.takeable_starts.truncate(last.takeable_starts_len);

                        if self.debug {
                            self.grammar_debug_stk.truncate(last.grammar_debug_len);
                        }
                    }
                }
            }
            GrammarNode::Many(g) => {
                // let fail_len2=self.stk.len(); //only remove everything past here on fail
                self.stk.push(Work {
                    grammar: GrammarNode::Many(g.clone()),
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    tokens: cur.tokens,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                    // opt:true,
                    grammar_debug_len: cur.grammar_debug_len,
                    // grammar_debug_no_add: true,
                    expected:cur.expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });

                let success_len2=self.stk.len();

                self.stk.push(Work {
                    grammar: GrammarNode::Always,
                    success_len: cur.success_len,
                    fail_len: 0, //fail is not used
                    tokens: cur.tokens,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                    // opt:false, //not used
                    grammar_debug_len: cur.grammar_debug_len,
                    // grammar_debug_no_add: true,
                    expected:cur.expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });

                let fail_len=self.stk.len();


                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: success_len2,
                    fail_len,
                    tokens: cur.tokens,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    // opt:true,
                    grammar_debug_len: cur.grammar_debug_len+1,
                    // grammar_debug_no_add: false,
                    expected:cur.expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });
            }

            GrammarNode::NonTerm(t) => {
                // let v=(t,cur.primitives.inds().start);

                // if cur.visiteds.contains(&v) {
                //     println!("err, circular nonterm {t}");
                //     break;
                // }

                // let mut visiteds=cur.visiteds;
                // visiteds.insert(v);

                let visiteds=self.do_non_term_visiteds(t,cur.tokens,cur.visiteds)?;

                // let mut takeable_starts_len=cur.takeable_starts_len;
                // self.takeable_starts.insert((cur.grammar,cur.primitives.inds().start));

                // if cur.opt {
                //     self.takeable_starts.push((g.clone(),cur.primitives.clone()));
                // }

                // self.add_expected(cur.tokens.loc(), cur.grammar);

                let grammar=if let Some(g)=(self.grammar_func)(t) {
                    g
                } else {
                    GrammarNode::Error(GrammarWalkError::MissingNonTerm(t))
                };

                self.stk.push(Work {
                    grammar, //: (self.grammar_func)(t), //should return err on not found, instead of grammar never, should have error
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    tokens: cur.tokens,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    // output_len: cur.output_len,
                    // discard:cur.discard,
                    visiteds,
                    takeables:cur.takeables,
                    takeable_starts_len:cur.takeable_starts_len,
                    // opt:cur.opt,

                    grammar_debug_len: cur.grammar_debug_len+1,
                    // grammar_debug_no_add: false,
                    // expected_non_term:cur.expected_non_term,
                    // expected:cur.expected.or(Some(t)),
                    expected:cur.expected,
                    groups_stk_ind: cur.groups_stk_ind,
                });
            }
            GrammarNode::Always => {
                self.stk.truncate(cur.success_len);

                //
                //
                if let Some(last)=self.stk.last_mut() {
                    if last.grammar.is_many() && last.tokens.len()==cur.tokens.len() { //if not parsing anything, exit the many
                        last.grammar=GrammarNode::Always;
                    }

                    last.tokens=cur.tokens;

                    // self.temp_primtives.resize(cur.output_len+1, PrimitiveInfo{ group: cur.group_ind }); //discard:true,

                    // // last.group_ind=cur.group_ind;
                    // let last_group_len= last.group_len;
                    last.group_len=cur.group_len; //done below //not anymore
                    // last.output_len=cur.output_len;
                    // last.takeable_starts=cur.takeable_starts;

                    //


                    // //
                    // last.takeables.retain(|_k,v|{
                    //     v.inds().start >= last.primitives.inds().start
                    // });

                    //
                    last.takeables=cur.takeables;

                    // let grammar_debug_len_dif=cur.grammar_debug_len-last.grammar_debug_len;


                    if cur.expected.id!=last.expected.id {
                        last.expected=Default::default();
                    }
                    // last.expected_non_term=None;



                }

                //
                if self.debug {
                    self.consolidate_grammar_debug_stk();
                }

                //
                self.do_groups_primitives_clamp(cur.group_ind,cur.tokens);

                //
                // self.last_remove_groups_at(cur.group_len,cur.primitives);

                //
                self.last_insert_start_takeables(cur.tokens);

                self.set_remaining_prims(cur.tokens);

                // self.clear_expected();

                // self.group_infos.truncate(cur.group_len);
            }

            GrammarNode::Error(e) => {
                if self.debug {
                    println!("====error {:?} {:?}",self.expected_loc,self.expecteds,);
                }
                //necesaary? any point to it?
                // if
                //     // self.expected.0.is_zero()
                //     self.expecteds.is_empty()
                // {
                //     self.expected_loc=cur.primitives.loc();
                // }


                //
                self.set_remaining_prims(cur.tokens);


                // self.group_infos.truncate(cur.group_len);

                // break;
                return Err(e);

            }
            GrammarNode::String => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_string(),|v,self2|{
                    if self2.debug {
                        let Some(TempGrammarNodeDebug::String(x))=self2.grammar_debug_stk.last_mut() else {panic!("");};
                        *x=Some(v);
                    }
                }) {
                    if self.debug {
                        println!("--- string {v:?}");
                    }
                }
            }
            GrammarNode::Identifier => {

                // println!("--- try identifier {:?}",cur.primitives.first());

                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_identifier(),|v,self2|{
                    if self2.debug {
                        let Some(TempGrammarNodeDebug::Identifier(x))=self2.grammar_debug_stk.last_mut() else {panic!("");};
                        *x=Some(v);
                    }
                }) {
                    if self.debug {
                        println!("--- identifier {v:?}");
                    }
                    // println!("==={}",self.grammar_debug_stk.last().map(|x|format!("{x}")).unwrap_or("None".to_string()));
                }
            }
            GrammarNode::Int => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_int(),|v,self2|{
                    if self2.debug {
                        let Some(TempGrammarNodeDebug::Int(x))=self2.grammar_debug_stk.last_mut() else {panic!("");};
                        *x=Some(v);
                    }
                }) {
                    if self.debug {
                        println!("--- int {v:?}");
                    }
                }
            }
            GrammarNode::Float => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_float(),|v,self2|{
                    if self2.debug {
                        let Some(TempGrammarNodeDebug::Float(x))=self2.grammar_debug_stk.last_mut() else {panic!("");};
                        *x=Some(v);
                    }
                }) {
                    if self.debug {
                        println!("--- float {v:?}");
                    }
                }
            }
            GrammarNode::Symbol(s) => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_with_symbol(s),|v,self2|{
                    if self2.debug {
                        println!("=={:?}",self2.grammar_debug_stk.last());
                        let Some(TempGrammarNodeDebug::Symbol(x))=self2.grammar_debug_stk.last_mut() else {panic!("");}; //{s:?}
                        *x=Some(v);
                    }
                }) {
                    if self.debug {
                        println!("--- symbol {v:?}");
                    }
                }
            }
            GrammarNode::Keyword(s) => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_with_keyword(s),|v,self2|{
                    if self2.debug {
                        let Some(TempGrammarNodeDebug::Keyword(x))=self2.grammar_debug_stk.last_mut() else {panic!("");};
                        *x=Some(v);
                    }
                }) {
                    if self.debug {
                        println!("--- keyword {v:?}");
                    }

                }
            }
            GrammarNode::Eol => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_eol(),|v,self2|{
                    if self2.debug {
                        let Some(TempGrammarNodeDebug::Eol(x))=self2.grammar_debug_stk.last_mut() else {panic!("");};
                        *x=Some(v);
                    }
                }) {
                    if self.debug {
                        println!("--- eol");
                    }
                }
            }
        }

        Ok(())
    }

    fn do_primtive<Q,P,K>(&mut self,mut cur:Work<'t,'g>,prim_func:Q,on_ok:K) -> Option<ValueContainer<'t,P>>
    where
        P:Clone,
        Q:Fn(&mut TokenIterContainer<'t>)->Result<ValueContainer<'t,P>,Loc>,
        K: Fn(ValueContainer<'t,P>,&mut Self),
    {
        match prim_func(&mut cur.tokens) {
            Ok(v) => {
                let vprim=v.token;
                on_ok(v.clone(),self);
                // match cur.grammar {

                //     GrammarNode::String => todo!(),
                //     GrammarNode::Identifier => todo!(),
                //     GrammarNode::Int => todo!(),
                //     GrammarNode::Float => todo!(),
                //     GrammarNode::Symbol(_) => todo!(),
                //     GrammarNode::Keyword(_) => todo!(),
                //     GrammarNode::Eol => todo!(),
                //     _=>panic!("")
                // }
                // let Some(TempGrammarNodeDebug::Identifier(x))=self.grammar_debug_stk.last_mut() else {panic!("");};

                if vprim.start_loc() >= self.expected_loc {
                    self.clear_expected();
                }

                self.stk.truncate(cur.success_len);


                let group_infos=&mut self.groups_stk.last_mut().unwrap().groups;
                // let primitive_infos=&mut self.groups_stk.last_mut().unwrap().token_groups;


                // // primitive_infos.resize(vprim.ind(), TempPrimitiveInfo{ group: cur.group_ind,discard:true, }); //discard:true,
                // // primitive_infos.push(TempPrimitiveInfo{ group: cur.group_ind,discard:cur.discard,});

                // primitive_infos.resize(vprim.ind(), cur.group_ind);
                // primitive_infos.push(cur.group_ind);

                //
                if let Some(last)=self.stk.last_mut() {
                    last.tokens=cur.tokens;
                    last.group_len=cur.group_len;
                    // last.output_len=primitive_infos.len();

                    // last.expected=None;
                    last.expected=Default::default();
                }

                //
                if self.debug {
                    self.consolidate_grammar_debug_stk();
                }
                //
                self.do_groups_primitives_clamp(cur.group_ind,cur.tokens);

                //
                self.last_remove_old_takeables();
                self.last_insert_start_takeables(cur.tokens);

                //
                // if self.stk.is_empty() {
                //     self.primitives_remaining=cur.primitives;
                // }
                self.set_remaining_prims(cur.tokens);


                // self.group_infos.truncate(cur.group_len);

                Some(v)
            }
            Err(loc) => {
                // if self.stk.last().map(|last|!last.expected_non_term.is_none() ).unwrap_or_default()
                if cur.expected.id==0{
                    self.add_expected(loc,0,cur.grammar);
                }

                //
                self.stk.truncate(cur.fail_len);

                //
                if let Some(last)=self.stk.last_mut() {

                    // let primitive_infos=&mut self.groups_stk.last_mut().unwrap().token_groups;


                    // primitive_infos.truncate(last.output_len);

                    self.takeable_starts.truncate(last.takeable_starts_len);

                    if self.debug {
                        // println!("===---==--- gdb_stk_len cur={}, last={}",cur.grammar_debug_len,last.grammar_debug_len);
                        // println!("\tcur={:?}",self.grammar_debug_stk);
                        self.grammar_debug_stk.truncate(last.grammar_debug_len);
                        // println!("\tlast={:?}",self.grammar_debug_stk);
                    }

                    // if let Some(x)=cur.expected_non_term {
                    //     if last.expected_non_term.is_none() {
                    //         self.add_expected(loc, GrammarNode::NonTerm(x));

                    //     }
                    // //     last.expected_non_term=None;
                    // }

                    if cur.expected.id!=last.expected.id && cur.expected.id!=0 {
                        last.expected=Default::default();
                        self.add_expected(loc, cur.expected.priority,GrammarNode::NonTerm(cur.expected.name));
                    }
                }

                // if self.stk.is_empty() {
                //     self.primitives_remaining=cur.primitives;
                // }
                self.set_remaining_prims(cur.tokens);


                // self.group_infos.truncate(cur.group_len);


                None
            }
        }

        //


    }

    fn consolidate_grammar_debug_stk(&mut self, ) { //cur_grammar_debug_len:usize

        if let Some(last)=self.stk.last_mut() {
            // let grammar_debug_len=self.grammar_debug_stk.len();

            // if grammar_debug_len!=last.grammar_debug_len {
            for _ in (last.grammar_debug_len..self.grammar_debug_stk.len()).rev() {
                let last_gd= self.grammar_debug_stk.pop().unwrap();

                match self.grammar_debug_stk.last_mut().unwrap() {
                    TempGrammarNodeDebug::Many(gs)
                    |TempGrammarNodeDebug::And(gs)
                    |TempGrammarNodeDebug::Or(gs)
                    => {gs.push(last_gd);}

                    TempGrammarNodeDebug::Opt(g)
                    |TempGrammarNodeDebug::Cede(g)
                    |TempGrammarNodeDebug::Take(g)
                    |TempGrammarNodeDebug::Group(_, g)
                    |TempGrammarNodeDebug::Expected(_,_, g)
                    |TempGrammarNodeDebug::NonTerm(_, g)
                    // |TempGrammarNodeDebug::Discard(g)
                    => {*g=Some(last_gd.into())}

                    _=>{panic!("");}
                }

                // if i==last.grammar_debug_len {
                //     break;
                // }

                // last_gd=self.grammar_debug_stk.pop().unwrap();
            }
            // }
        }
    }

    fn do_groups_primitives_clamp(&mut self,
        cur_group_ind:usize,
        cur_primitives:TokenIterContainer<'t>,
    ) {
        if let Some(last)=self.stk.last_mut() {


            let group_infos=&mut self.groups_stk.last_mut().unwrap().groups;

            // let last_group_prim_len=last.primitives.len();

            let mut g=cur_group_ind;
            // println!("--- cur_group_ind={g}, last.group_ind={}",last.group_ind);

            // println!("---g={g} to lg={}",last.group_ind);
            while g>last.group_ind {
                let group=&mut group_infos[g];
                // let mut last_primitives=group.primitives;

                // println!("\tg={g} lg={} : {} {}",last.group_ind,group.primitives.len(),cur_primitives.len(),);
                let n=group.tokens.len()-cur_primitives.len();
                // println!("\tn={n} {:?}",group.primitives.get_range(0..n).unwrap());
                // println!("\t{:?}",group.primitives);
                // let group_prims=group.primitives.get_range(0..n).unwrap();
                let group_prims=group.tokens.get_amount(n).unwrap();

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
        if !self.non_term_recursive_check {
            return  Ok(Default::default());
        }

        let v=(t,cur_primitives.inds().start);

        if cur_visiteds.contains(&v) {
            // break;
            return Err(GrammarWalkError::RecursiveNonTerm(t));
        }

        let mut visiteds=cur_visiteds;
        visiteds.insert(v);

        Ok(visiteds)
    }

    fn last_insert_start_takeables(&mut self,
        cur_tokens:TokenIterContainer<'t>,
        // mut last_takeables: HashMap<GrammarItem<'a>, PrimitiveIterContainer<'a>>,last_takeable_starts_len:usize,
    )
        // -> HashMap<GrammarItem<'a>, PrimitiveIterContainer<'a>>
    {


        let group_infos=&self.groups_stk.last().unwrap().groups;
        let groups_len=group_infos.len();

        if let Some(last)=self.stk.last_mut() {
            for TempTakeableStart { grammar:tg, tokens_start, group_ind }// (tg,tp_ind)
                in self.takeable_starts.drain(last.takeable_starts_len ..)
            {

                let tokens_len=tokens_start.len()-cur_tokens.len();
                let tokens=tokens_start.get_amount(tokens_len).unwrap();

                if self.debug {
                    println!("--- inserting takeable {tg:?} {tokens:?}",);
                }
                // last.takeables.insert(tg, tp_ind);


                last.takeables.insert(tg, WorkTakeable {
                    tokens_start, tokens, group_ind,
                    inner_groups:group_ind+1 .. groups_len,
                });
            }
        }
        // last_takeables
    }

    fn last_remove_old_takeables(&mut self) {
        if let Some(last)=self.stk.last_mut() {
            last.takeables.retain(|_k,v|{
                // v.inds().start
                v.tokens.inds().start
                    >= last.tokens.inds().start
            });
        }
    }
    fn clear_expected(&mut self) {
        // println!("-------==== expected cleared, {}",self.expected_loc);

        self.expected_loc=Loc::zero();
        self.expecteds.clear();
    }
    fn add_expected(&mut self,loc:Loc,p:u32,g:GrammarNode<'g>) {

        if loc==self.expected_loc {
            self.expecteds.push((p,g.clone()));
            // println!("-------==== expected added {g:?}, {loc}=={}",self.expected_loc);
        } else if loc>self.expected_loc  { //|| self.expecteds.is_empty()
            self.expected_loc=loc;
            self.expecteds=vec![(p,g.clone())];
            // println!("-------==== expected new {g:?}, {loc}=={}",self.expected_loc);
        } else {
            // println!("-------==== expected not added {g:?}, {loc}=={}",self.expected_loc);
        }
    }
    fn new_group(&mut self,name : &'g str, parent:usize, ps:TokenIterContainer<'t>) -> usize {

        let group_infos=&mut self.groups_stk.last_mut().unwrap().groups;

        let new_group_ind=group_infos.len();

        group_infos.push(TempGroupInfo {
            name,
            parent,
            // primitive_ind_start: ps.inds().start,
            tokens:ps,
        });

        new_group_ind
    }

    fn set_remaining_prims(&mut self,cur_primitives:TokenIterContainer<'t>,) {
        if self.stk.is_empty() {
            self.primitives_remaining=cur_primitives;
        }
    }

    fn get_cur_groups(&self,cur_group_ind:usize,) -> HashSet<usize> {

        let group_infos=&self.groups_stk.last().unwrap().groups;
        //collect cur groups
        let mut cur_used_group_inds: HashSet<usize>=HashSet::new();

        //
        {
            let mut group_ind=cur_group_ind;

            loop {
                cur_used_group_inds.insert(group_ind);

                if group_ind==0 {
                    break;
                }

                let group=&group_infos[group_ind];
                group_ind=group.parent;

            }
        }

        cur_used_group_inds
    }


    //
    pub fn set_debug(&mut self,debug:bool) {
        self.debug=debug;
    }
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

    pub fn get_walk(&self) -> Walk<'t,'g> {
        let mut groups: Vec<WalkGroup<'t,'g>>=Vec::new();//vec![WalkGroup{ name: "", children: 0..0, tokens: todo!() }];
        // groups.resize_with(new_len, f);

        let group_infos=&self.groups_stk.last().unwrap().groups;
        // let primitive_infos=&self.groups_stk.last().unwrap().token_groups;


        let mut group_infos2 = group_infos.iter().enumerate()
            .map(|(i,g)|(i,g.parent,))
            .collect::<Vec<_>>(); //(grouo_ind,parent_ind,child_num)

        // //count children for each group
        // for i in 1..group_infos2.len() {
        //     let p=group_infos2[i].1;
        //     group_infos2[p].2+=1;
        // }

        //sort groups to breadth first
        group_infos2[1..].sort_by(|&(g1,p1,),&(g2,p2,)|{
            match p1.cmp(&p2) {
                std::cmp::Ordering::Equal => g1.cmp(&g2),
                x=>x,
            }
        });

        println!("groups2 {:?}",group_infos2.iter().enumerate().collect::<Vec<_>>());
        for (i,&(g,p,)) in group_infos2.iter().enumerate() {
            let group_infos=&self.groups_stk.last().unwrap().groups;
            println!("\t{i}: {g}, {p}, {:?}",group_infos[g].name);
        }
        //
        // let mut csum=1;
        let ind_map: HashMap<usize, usize> = HashMap::from_iter(group_infos2.iter().enumerate().map(|(i,&(g,_p,))|(g,i)));
        //
        for (i,&(gind,p,)) in group_infos2.iter().enumerate() {
            let group_infos=&self.groups_stk.last().unwrap().groups;
            let g=&group_infos[gind];
            // groups.push(WalkGroup { name: gg.name, children: csum..csum+c, tokens: gg.primitives.inds() });
            groups.push(WalkGroup { name: g.name,
                children:
                    // csum..csum+c
                    0..0
                    ,
                tokens: g.tokens });
            // println!("{_i} name: {:?}, children: {:?}, tokens: {:?} {:?}",g.name,csum..csum+c,g.primitives.inds(),g.primitives);
            // println!("{i} name: {:?}, c={c}, children: {:?}, ",g.name,csum..csum+c,);
            // csum+=c;

            if i!=0 { //as root's parent is 0, ie itself, which is incorrect
                let ind=ind_map.get(&p).cloned().unwrap();
                let c= &mut groups[ind].children;
                if c.start==0 {c.start=i;}
                c.start=c.start.min(i);
                c.end=c.end.max(i+1);
            }
        }

        //
        let walk=Walk{ groups };
        walk
    }
}