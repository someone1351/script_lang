
use super::error::*;
use super::temp_data::*;
use std::collections::HashSet;

use crate::build::Loc;
use crate::ccexpr_parser::grammar::data::Walk;
use crate::ccexpr_parser::grammar::data::WalkGroup;
use super::super::tokenizer::{TokenIterContainer, ValueContainer};

use super::node::*;


// use data::*;
// use error::*;


pub struct GrammarWalker<'a,F> {
    top_primitives:TokenIterContainer<'a>,
    primitives_remaining: TokenIterContainer<'a>,

    primitive_infos : Vec<TempPrimitiveInfo>,
    group_infos : Vec<TempGroupInfo<'a>>,
    takeable_starts:Vec<(GrammarNode<'a>,TokenIterContainer<'a>)>, //[(g,output_ind_start)]
    grammar_func:F,

    stk: Vec<Work<'a>>,
    c:usize,
    expected: (Loc,Vec<GrammarNode<'a>>,),
    debug:bool,
}


impl<'a,F> GrammarWalker<'a,F>
where
    F: Fn(&'a str)->GrammarNode<'a>,
{

    pub fn new(top_primitives:TokenIterContainer<'a>, grammar_func:F) -> Self {
        Self {
            primitive_infos :  Default::default(),
            group_infos : Default::default(),
            takeable_starts: Default::default(),
            stk:Default::default(),
            c:Default::default(),
            expected:Default::default(),
            grammar_func,
            primitives_remaining:top_primitives.clone(),
            top_primitives,
            debug:false,
        }
    }

    fn init(&mut self,start_non_term:&'a str,) {
        self.stk.clear();

        self.stk.push(Work{
            grammar:GrammarNode::Error(GrammarWalkError::FailedParse),success_len:0,fail_len:0,primitives:self.top_primitives,
            group_ind: 0, group_len: 1, output_len: 0, discard:false,
            // takeable_starts:Default::default(),
            takeable_starts_len:0,
            visiteds:Default::default(),
            takeables:Default::default(),
            opt:false,
        });
        self.stk.push(Work{
            grammar:(self.grammar_func)(start_non_term),success_len:0,fail_len:1,primitives:self.top_primitives,
            group_ind: 0, group_len: 1, output_len: 0, discard:false,
            // takeable_starts:Default::default(),
            takeable_starts_len:0,
            visiteds:Default::default(),
            takeables:Default::default(),
            opt:false,
        });

        //
        self.primitive_infos.clear();
        self.group_infos=vec![TempGroupInfo{
            name: "",
            parent: 0,
            // primitive_ind_start:0,
            primitives:self.top_primitives,
        }];
        self.takeable_starts.clear();

        // self.primitives_remaining:top_primitives.clone(),
        // self.top_primitives,

        self.c=0;
        self.expected=Default::default();
    }

    pub fn run(&mut self,start_non_term:&'a str,) {
        self.init(start_non_term);

        while let Some(cur)=self.stk.pop() {
           if let Err(e)=self.step(cur) {
                if self.debug {
                    match e {
                        GrammarWalkError::RecursiveNonTerm(t) => {
                            println!("Recursive NonTerm {t:?}, At {}",self.expected.0);
                        }
                        GrammarWalkError::MissingNonTerm(t) => {
                            println!("Missing NonTerm {t:?}, At {}",self.expected.0);
                        }
                        GrammarWalkError::FailedParse => {
                            println!("Failed parse, At {}, expected {:?}",self.expected.0,self.expecteds_string());
                        }
                    }
                }

                break;
           }
        }


        //
        if self.debug {
            println!("groups={:?}",self.group_infos);
            println!("outputs={:?}",self.primitive_infos);
        }
            if !self.primitives_remaining.is_empty() {
                // println!("error, failed to parse all tokens {:?}",self.primitives_remaining);
                println!("error, failed to parse all tokens {}",self.expected.0);
            } else {
                println!("parsed ok");
            }

        // if self.debug {
            println!("===a {}",self.primitives_remaining.is_empty());
        // }

            let mut groups_visited: HashSet<usize>=HashSet::new();

            for p in self.top_primitives {
                let i=p.ind();
                let Some(output)=self.primitive_infos.get(i) else {
                    break;
                };

                let mut g=output.group;
                let mut depth=0;
                let mut gs: Vec<usize>=Vec::new();
                while g!=0 {
                    gs.push(g);
                    let gg=&self.group_infos[g];

                    depth+=1;

                    g=gg.parent;

                }

                for (d,&g) in gs.iter().rev().enumerate() {
                    let gg=&self.group_infos[g];

                    if !groups_visited.contains(&g) {
                        println!("{}{:?} : {:?}",
                            "  ".repeat(d),
                            gg.name,
                            gg.primitives.inds(),
                        );
                        groups_visited.insert(g);
                    }
                }

                println!("{}{}{p:?}",
                    "  ".repeat(depth),
                    if output.discard {"-"}else{""}
                );
            }
            println!("===");

        if self.debug {
            //
            println!("top_primitives={:?}", self.top_primitives );
            // println!("output={outputs:?}",  );
        }

    }

    fn step(&mut self,cur:Work<'a>) -> Result<(),GrammarWalkError<'a>> {
        if self.debug {
            self.c+=1;

            // // if c>30 {break;}
            // // println!(": {cur:?} || {} && {primitives:?}", self.stk.iter().rev().map(|x|format!("{:?}",x.0)).collect::<Vec<_>>().join(" << "), );
            {
                let c=self.c;
                let Work { grammar, success_len, fail_len, primitives, group_ind, group_len, output_len, discard, takeable_starts_len, visiteds, takeables, opt}=&cur;
                println!("{c:4}: {grammar:?}, ps={primitives:?}, success={success_len}, fail={fail_len}, group_ind={group_ind}, group_len={group_len}, output_len={output_len}, discard={discard}, takeable_starts_len={takeable_starts_len:?}, visiteds={visiteds:?}, opt={opt:?}, takeables={takeables:?}, ");
                println!("         -takeable_starts={:?}",self.takeable_starts);
                println!("         -temp_primtives={:?}",self.primitive_infos);
                println!("         -temp_groups3={:?}",self.group_infos);
            }

            for (i,Work { grammar:g, success_len:s, fail_len:f, primitives:ps, group_ind, group_len, output_len, discard, takeable_starts_len, visiteds, takeables, opt }) in self.stk.iter()
                // .rev()
                .enumerate() {
                // println!("\t{i:3}: {g:?}\n\t   : {ps:?}\n\t   : success={s}, fail={f}",);
                println!("\t{i:3}: {g:?}, ps={ps:?},success={s}, fail={f}, group_ind={group_ind}, group_len={group_len}, output_len={output_len}, discard={discard}, takeable_starts_len={takeable_starts_len:?}, visiteds={visiteds:?}, opt={opt:?}, takeables={takeables:?}",);
            }
        }

        //
        match cur.grammar {
            GrammarNode::Group(name, g) => {
                let new_group_ind=self.new_group(name, cur.group_ind, cur.primitives);
                let new_group_len=self.group_infos.len();

                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: new_group_ind,
                    group_len: new_group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:cur.opt,
                });
            }
            GrammarNode::Discard(g) => {

                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:true,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:cur.opt,
                });


            }
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
                        primitives: cur.primitives,

                        group_ind: cur.group_ind,
                        group_len: cur.group_len,
                        output_len: cur.output_len,
                        discard:cur.discard,

                        takeable_starts_len:cur.takeable_starts_len,
                        visiteds:cur.visiteds.clone(),
                        takeables:cur.takeables.clone(),
                        opt:false, //opt isnt passed to individual items in And

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
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:false, //opt isnt passed to individual items in And
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
                        primitives: cur.primitives,

                        group_ind: cur.group_ind,
                        group_len: cur.group_len,
                        output_len: cur.output_len,
                        discard:cur.discard,
                        takeable_starts_len:cur.takeable_starts_len,
                        visiteds:cur.visiteds.clone(),
                        takeables:cur.takeables.clone(),
                        opt:cur.opt,
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
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:cur.opt,
                });
            }

            GrammarNode::Opt(g) => {
                self.stk.push(Work {
                    grammar: GrammarNode::Always,
                    success_len: cur.success_len,
                    fail_len: 0, //fail is not used
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                    opt:false, //not used on always
                });

                let fail_len=self.stk.len();

                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:true,
                });
            }
            GrammarNode::Cede(g) => {
                //should return err if not giveable? ie not opt? or just ignore?
                //  or just don't rquire at all

                // if cur.opt {
                self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,
                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:self.takeable_starts.len(),
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:cur.opt,
                });
            }
            GrammarNode::Take(g) => {
                if let Some(taken_ps_start)=cur.takeables.get(&g).cloned() {

                    if self.debug {
                        println!("---the groups are {:?}",self.group_infos);
                    }
                    //how to remove no longer used groups, and fix inds of the used group that ccomes after the removed one?

                    let cur_group_ind=self.remove_groups_at_except(taken_ps_start,cur.group_ind,);


                    //clear outputs to start of taken
                    self.primitive_infos.truncate(taken_ps_start.inds().start);

                    //
                    // if cur.opt {
                    //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                    // }

                    //
                    self.stk.push(Work {
                        grammar: *g,
                        success_len: cur.success_len,
                        fail_len: cur.fail_len,

                        primitives: taken_ps_start.clone(),
                        output_len: taken_ps_start.inds().start,

                        group_ind: cur_group_ind,
                        group_len: self.group_infos.len(),

                        discard: cur.discard,
                        takeable_starts_len: cur.takeable_starts_len,//cur.takeable_starts_len, // ??
                        visiteds: cur.visiteds, //
                        takeables: Default::default(),
                        opt:cur.opt,
                    });
                } else {

                    //
                    self.stk.truncate(cur.fail_len);

                    //
                    if let Some(last)=self.stk.last() {
                        self.primitive_infos.truncate(last.output_len);
                        self.takeable_starts.truncate(last.takeable_starts_len);
                    }
                }
            }
            GrammarNode::Many(g) => {
                // let fail_len2=self.stk.len(); //only remove everything past here on fail
                self.stk.push(Work {
                    grammar: GrammarNode::Many(g.clone()),
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                    opt:true,
                });

                let success_len2=self.stk.len();

                self.stk.push(Work {
                    grammar: GrammarNode::Always,
                    success_len: cur.success_len,
                    fail_len: 0, //fail is not used
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                    opt:false, //not used
                });

                let fail_len=self.stk.len();


                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: success_len2,
                    fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:true,
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

                let visiteds=self.do_non_term_visiteds(t,cur.primitives,cur.visiteds)?;

                // let mut takeable_starts_len=cur.takeable_starts_len;
                // self.takeable_starts.insert((cur.grammar,cur.primitives.inds().start));
                // let g=(self.grammar_func)(t);

                // if cur.opt {
                //     self.takeable_starts.push((g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: (self.grammar_func)(t), //should return err on not found, instead of grammar never, should have error
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    visiteds,
                    takeables:cur.takeables,
                    takeable_starts_len:cur.takeable_starts_len,
                    opt:cur.opt,
                });
            }
            GrammarNode::Always => {
                self.stk.truncate(cur.success_len);

                //
                //
                if let Some(last)=self.stk.last_mut() {
                    if last.grammar.is_many() && last.primitives.len()==cur.primitives.len() { //if not parsing anything, exit the many
                        last.grammar=GrammarNode::Always;
                    }

                    last.primitives=cur.primitives;

                    // self.temp_primtives.resize(cur.output_len+1, PrimitiveInfo{ group: cur.group_ind }); //discard:true,

                    // // last.group_ind=cur.group_ind;
                    // let last_group_len= last.group_len;
                    last.group_len=cur.group_len; //done below //not anymore
                    last.output_len=cur.output_len;
                    // last.takeable_starts=cur.takeable_starts;

                    //


                    // //
                    // last.takeables.retain(|_k,v|{
                    //     v.inds().start >= last.primitives.inds().start
                    // });

                    //
                    last.takeables=cur.takeables;


                }

                //
                self.do_groups_primitives_clamp(cur.group_ind,cur.primitives);

                //
                self.last_remove_groups_at(cur.group_len,cur.primitives);

                //
                self.last_insert_start_takeables();

                self.set_remaining_prims(cur.primitives);

                self.clear_expected();
            }

            GrammarNode::Error(e) => {
                if self.debug {
                    println!("====error {:?}",self.expected);
                }
                //necesaary? any point to it?
                if
                    // self.expected.0.is_zero()
                    self.expected.1.is_empty()
                {
                    self.expected.0=cur.primitives.loc();
                }


                //
                self.set_remaining_prims(cur.primitives);

                // break;
                return Err(e);

            }
            GrammarNode::String => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_string()) {
                    if self.debug {
                        println!("--- string {v:?}");
                    }
                }
            }
            GrammarNode::Identifier => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_identifier()) {
                    if self.debug {
                        println!("--- identifier {v:?}");
                    }
                }
            }
            GrammarNode::Int => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_int()) {
                    if self.debug {
                        println!("--- int {v:?}");
                    }
                }
            }
            GrammarNode::Float => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_float()) {
                    if self.debug {
                        println!("--- float {v:?}");
                    }
                }
            }
            GrammarNode::Symbol(s) => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_with_symbols([s])) {
                    if self.debug {
                        println!("--- symbol {v:?}");
                    }
                }
            }
            GrammarNode::Keyword(s) => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_with_identifiers([s])) {
                    if self.debug {
                        println!("--- keyword {v:?}");
                    }
                }
            }
            GrammarNode::Eol => {
                if let Some(_)=self.do_primtive(cur,|ps|ps.pop_eol()) {
                    if self.debug {
                        println!("--- eol");
                    }
                }
            }
        }

        Ok(())
    }

    fn do_primtive<Q,P>(&mut self,mut cur:Work<'a>,prim_func:Q) -> Option<P>
    where
        Q:Fn(&mut TokenIterContainer<'a>)->Result<ValueContainer<'a,P>,Loc>,
    {
        match prim_func(&mut cur.primitives) {
            Ok(v) => {
                if v.primitive.start_loc() >= self.expected.0 {
                    self.clear_expected();
                }

                self.stk.truncate(cur.success_len);

                self.primitive_infos.resize(v.primitive.ind(), TempPrimitiveInfo{ group: cur.group_ind,discard:true, }); //discard:true,
                self.primitive_infos.push(TempPrimitiveInfo{ group: cur.group_ind,discard:cur.discard,});

                //
                if let Some(last)=self.stk.last_mut() {
                    last.primitives=cur.primitives;
                    last.group_len=cur.group_len;
                    last.output_len=self.primitive_infos.len();
                }

                //
                self.do_groups_primitives_clamp(cur.group_ind,cur.primitives);

                //
                self.last_remove_old_takeables();
                self.last_insert_start_takeables();

                //
                // if self.stk.is_empty() {
                //     self.primitives_remaining=cur.primitives;
                // }
                self.set_remaining_prims(cur.primitives);

                Some(v.value)
            }
            Err(loc) => {
                self.add_expected(loc,cur.grammar);

                //
                self.stk.truncate(cur.fail_len);

                //
                if let Some(last)=self.stk.last() {
                    self.primitive_infos.truncate(last.output_len);

                    self.takeable_starts.truncate(last.takeable_starts_len);
                }

                // if self.stk.is_empty() {
                //     self.primitives_remaining=cur.primitives;
                // }
                self.set_remaining_prims(cur.primitives);

                None
            }
        }

        //


    }

    fn do_groups_primitives_clamp(&mut self,
        cur_group_ind:usize,
        cur_primitives:TokenIterContainer<'a>,
    ) {
        if let Some(last)=self.stk.last_mut() {
            // let last_group_prim_len=last.primitives.len();

            let mut g=cur_group_ind;
            println!("--- cur_group_ind={g}, last.group_ind={}",last.group_ind);

            while g>last.group_ind {
                println!("g={g}");
                let group=&mut self.group_infos[g];
                // let mut last_primitives=group.primitives;

                println!("g={g} lg={} : {} {}",last.group_ind,group.primitives.len(),cur_primitives.len(),);
                let n=group.primitives.len()-cur_primitives.len();
                let group_prims=group.primitives.get_range(0..n).unwrap();

                group.primitives=group_prims;
                g=group.parent;
            }
        }
    }

    fn do_non_term_visiteds(&mut self,
        t:&'a str,
        cur_primitives:TokenIterContainer<'a>,
        cur_visiteds: HashSet<(&'a str, usize)>,
    ) -> Result<HashSet<(&'a str, usize)>,GrammarWalkError<'a>> {
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
        // mut last_takeables: HashMap<GrammarItem<'a>, PrimitiveIterContainer<'a>>,last_takeable_starts_len:usize,
    )
        // -> HashMap<GrammarItem<'a>, PrimitiveIterContainer<'a>>
    {
        if let Some(last)=self.stk.last_mut() {
            for (tg,tp_ind) in self.takeable_starts.drain(last.takeable_starts_len ..) {
                if self.debug {
                    println!("--- inserting takeable {tg:?} {tp_ind:?}",);
                }
                last.takeables.insert(tg, tp_ind);
            }
        }
        // last_takeables
    }

    fn last_remove_old_takeables(&mut self) {
        if let Some(last)=self.stk.last_mut() {
            last.takeables.retain(|_k,v|{
                v.inds().start >= last.primitives.inds().start
            });
        }
    }
    fn clear_expected(&mut self) {
        self.expected.0=Loc::zero();
        self.expected.1.clear();
    }
    fn add_expected(&mut self,loc:Loc,g:GrammarNode<'a>) {
        if loc==self.expected.0 {
            self.expected.1.push(g);
        } else if loc>self.expected.0 {
            self.expected.0=loc;
            self.expected.1=vec![g];
        }
    }
    fn new_group(&mut self,name : &'a str, parent:usize, ps:TokenIterContainer<'a>) -> usize {
        let new_group_ind=self.group_infos.len();

        self.group_infos.push(TempGroupInfo {
            name,
            parent,
            // primitive_ind_start: ps.inds().start,
            primitives:ps,
        });

        new_group_ind
    }

    fn set_remaining_prims(&mut self,cur_primitives:TokenIterContainer<'a>,) {
        if self.stk.is_empty() {
            self.primitives_remaining=cur_primitives;
        }
    }

    fn remove_groups_at_except(&mut self,
        taken_ps_start:TokenIterContainer<'a>,
        cur_group_ind:usize,
    ) -> usize{

        let mut cur_group_ind=cur_group_ind;


        // if let Some(temp_prim)=self.temp_primtives.get(taken_ps_start.inds().start) {


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

                let group=&self.group_infos[group_ind];
                group_ind=group.parent;

            }
        }

        //get first group ind with prim ind >= taken.inds().start
        let mut after_group_ind = self.group_infos.len();

        while after_group_ind > 0 {
            let group=self.group_infos.get(after_group_ind-1).unwrap();

            if
                // group.primitive_ind_start
                group.primitives.inds().start
                < taken_ps_start.inds().start {
                break;
            }

            after_group_ind-=1;
        }

        //get num of groups to remove
        let mut remove_groups_num=0;

        for i in after_group_ind..self.group_infos.len() {
            if cur_used_group_inds.contains(&i) {
                break;
            }

            remove_groups_num+=1;
        }

        //remove unused groups
        self.group_infos.drain(after_group_ind..after_group_ind+remove_groups_num);

        //
        for group in &mut self.group_infos[after_group_ind..] {
            if group.parent>=after_group_ind {
                group.parent-=remove_groups_num;
            }
        }

        //
        if cur_group_ind >=after_group_ind {
            cur_group_ind-=remove_groups_num;
        }

        //
        for prev in self.stk.iter_mut().rev() {
            if prev.group_ind >=after_group_ind {
                prev.group_ind-=remove_groups_num;
                prev.group_len-=remove_groups_num;
            }
        }

        cur_group_ind
    }

    fn last_remove_groups_at(&mut self,
        // last_group_len:usize ,
        cur_group_len:usize,cur_primitives:TokenIterContainer<'a>)
        // -> usize
    {

        if let Some(last)=self.stk.last_mut() {
            if self.debug {
                println!("===www {} {cur_group_len} ",  last.group_len ); //last.grammar
            }

            //
            for group_ind in last.group_len .. cur_group_len {
                let group=&self.group_infos[group_ind];

                if self.debug {
                    println!("===hmmm {group_ind}");
                }

                if
                    // group.primitive_ind_start
                    group.primitives.inds().start
                    ==cur_primitives.inds().start {
                    self.group_infos.truncate(group_ind); //removes this group and ones after
                    last.group_len=group_ind;

                    if self.debug {
                        println!("====== {group_ind} {}",self.group_infos.len(), );
                    }
                    break;
                    // return group_ind;
                }
            }

            // cur_group_len
        }
    }

    //
    pub fn set_debug(&mut self,debug:bool) {
        self.debug=debug;
    }
    pub fn expecteds_string(&self) -> String {
        self.expected.1.iter().map(|g|match g {
            GrammarNode::String => "string".to_string(),
            GrammarNode::Identifier => "identifier".to_string(),
            GrammarNode::Int => "int".to_string(),
            GrammarNode::Float => "float".to_string(),
            GrammarNode::Symbol(s) => format!("symbol({s})"),
            GrammarNode::Keyword(s) => format!("keyword({s})"),
            GrammarNode::Eol => "eol".to_string(),
            _ =>"".to_string(),
        }).collect::<Vec<_>>().join(", ")
    }
    pub fn last_loc(&self) -> Loc {
        self.expected.0
    }

    pub fn get_walk(&self) -> Walk<'a> {
        let mut groups: Vec<WalkGroup<'a>>=Vec::new();//vec![WalkGroup{ name: "", children: 0..0, tokens: todo!() }];
        // groups.resize_with(new_len, f);

        let mut group_infos2 = self.group_infos.iter().enumerate().map(|(i,g)|(i,g.parent,0)).collect::<Vec<_>>();

        //count children for each group
        for i in 1..group_infos2.len() {
            let p=group_infos2[i].1;
            group_infos2[p].2+=1;
        }

        //sort groups to breadth first
        group_infos2.sort_by(|&(g1,p1,_),&(g2,p2,_)|{
            match p1.cmp(&p2) {
                std::cmp::Ordering::Equal => g1.cmp(&g2),
                x=>x,
            }
        });

        //
        let mut csum=0;

        //
        for (_i,&(g,_p,c)) in group_infos2.iter().enumerate() {
            let gg=&self.group_infos[g];
            groups.push(WalkGroup { name: gg.name, children: csum..csum+c, tokens: gg.primitives });
            println!("{_i} name: {:?}, children: {:?}, tokens: {:?}",gg.name,csum..csum+c,gg.primitives.inds());
            csum+=c;
        }

        //
        let walk=Walk{ groups };
        walk
    }
}