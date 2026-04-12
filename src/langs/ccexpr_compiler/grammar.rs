/*
TODO
* add Group(group_name,grammar_item), for output
** or instead add as method and(expr,expr).group("abc")

* output
** if something like and(expr, expr, expr (or stmt)) => [expr,expr,stmt]
** if something like and(group("abc",and(expr, expr)), expr (or stmt)) => [abc[expr,expr],stmt]
** if group not used, all output would be one single list of primitives
*/

use std::{collections::{BTreeMap, HashMap, HashSet}, ops::Range};

use crate::{ccexpr_parser::{PrimitiveContainer, PrimitiveIterContainer}, Loc};

#[derive(Clone,Debug,Hash,PartialEq,Eq)]
pub enum GrammarItem<'a> {
    Many(Box<GrammarItem<'a>>),
    // Many1(Box<GrammarItem<'a>>),
    And(Vec<GrammarItem<'a>>), //should store reversed?
    Or(Vec<GrammarItem<'a>>), //should store reversed?
    Opt(Box<GrammarItem<'a>>),
    Take(Box<GrammarItem<'a>>),
    Group(&'a str,Box<GrammarItem<'a>>),

    // List(Box<GrammarItem<'a>>,Box<GrammarItem<'a>>), //val,sep
    // ListNoTrail(Box<GrammarItem<'a>>,Box<GrammarItem<'a>>), //val,sep

    String,
    Identifier,
    Int,
    Float,
    Symbol(&'a str),
    Keyword(&'a str),
    Eol,

    NonTerm(&'a str),
    Always, //always succeeds
    // Never, //replace with Error ?
    Error,
    // Not(Box<GrammarItem<'a>>), //todo, needed? better to have NotIdentifier etc?
    Discard(Box<GrammarItem<'a>>), //todo, removes token from output (via just hding it, ie have hashmap of tokens to hide)
}

impl<'a> GrammarItem<'a> {
    pub fn many0(self) -> GrammarItem<'a> {
        Self::Many(self.into())
    }
    pub fn many1(self) -> GrammarItem<'a> {
        let x=self.clone();
        [x,self.many0(),].and()
        // Self::Many1(self.into())
    }
    pub fn opt(self) -> GrammarItem<'a> {
        Self::Opt(self.into())
    }
    pub fn group(self,name: &'a str) -> GrammarItem<'a> {
        Self::Group(name,self.into())
    }
    pub fn discard(self,) -> GrammarItem<'a> {
        Self::Discard(self.into())
    }
    pub fn take(self,) -> GrammarItem<'a> {
        Self::Take(self.into())
    }
    pub fn d(self,) -> GrammarItem<'a> {
        self.discard()
    }
    pub fn is_many(&self) -> bool {
        if let GrammarItem::Many(_)=self {
            true
        } else {
            false
        }
        // match self {
        //     GrammarItem::Many(_)|GrammarItem::Many1(_) => true,
        //     _ =>false,
        // }
    }
}

// impl<'a, const N: usize> From<[GrammarItem<'a>; N]> for  GrammarItem<'a> {
//     fn from(value: [GrammarItem<'a>; N]) -> Self {
//         Self::And(value.into())
//     }
// }

// #[macro_export]
// macro_rules! and {
//     ( $( $x:expr ),* $(,)? ) => {{
//         let mut v = Vec::new();
//         $( v.push($x); )*
//         GrammarItem::And(v.into())
//     }};
// }

// #[macro_export]
// macro_rules! or {
//     ( $( $x:expr ),* $(,)? ) => {{
//         let mut v = Vec::new();
//         $( v.push($x); )*
//         GrammarItem::And(v.into())
//     }};
// }

//todo have array stored in rev for or/and
trait GrammarArrayTrait<'a> {
    fn and(&self) -> GrammarItem<'a>;
    fn or(&self) -> GrammarItem<'a>;
}
impl<'a,const N: usize> GrammarArrayTrait <'a> for [GrammarItem<'a>; N] {
    fn and(&self) -> GrammarItem<'a> {
        GrammarItem::And(self.into())
    }
    fn or(&self) -> GrammarItem<'a> {
        GrammarItem::Or(self.into())

    }
}
pub fn grammar_decl<'a>(n:&str) -> GrammarItem<'a> {
    /*
    this:
        if(cond) {1} else {2}
        -5
    is same as: if(cond) {1} else {2}-5
    but that doesn't happen for things like for(..){}, while(..){}, might be better to treat those like exprs to be consistent, even though they aren't?

    should have checks for traversing recursively? or just let the user make the mistake?
        would need to keep a stk of hashsets containing nonterm names,
            store with rest of the work in the main stk

        only a problem when the recursive nonterm is used before any token is eaten


    if traversing same terminal and pos is the same, fail
    */
    use GrammarItem::*;
    match n {
        // "test" => [Int].and(),
        "test" => [Int,Float.opt()].and().many0(),
        "test2" => [Int,Float.opt()].and(),
        "test3" => [ Int.many0() ].and().opt(),

        "test4" => [[Int,String.opt(),].and(),Identifier,].or().opt(), //or(and(int,str?),idn)?
        "test5" =>  Int.many0(),
        "test6" =>  Int.many0().opt(),
        // "test7" =>  [ [ Int,String ].and(), Float, ].or(), //or(and(int,str),float)
        "test7" =>  [ Int.many0(), String, ].or().many0(), //.opt(), // many0(or(many0(int),str))

        "test8" => [
            Symbol("+"),
            Int,
        ].and(),

        "test9" => [
            Int.many0().group("a"),
            Float.many0().group("b"),
            String.many0().group("c"),
            // NonTerm("x").many0().group("a"),
            // NonTerm("x").take().group("b"),
            Eol.many0(),
        ].and(),
        "x" => Int,

        "start" => NonTerm("stmts"),

        "ending" => [NonTerm("semicolon"),Eol].or().many1().d(),
        "stmts" => [
            NonTerm("stmt"),
            [NonTerm("ending"), NonTerm("stmt"),].and().many0(),
            NonTerm("ending").many0(),
        ].and().opt(),

        "stmt" => [
            NonTerm("block"),
            NonTerm("var"),NonTerm("set"),
            NonTerm("func"),
            NonTerm("while"),NonTerm("for_in"),NonTerm("for_to"),
            NonTerm("break"), NonTerm("continue"),
            NonTerm("return"),
            NonTerm("include"),
            NonTerm("format"),NonTerm("print"),NonTerm("println"),
            NonTerm("expr"),
            // NonTerm("if"),
        ].or().group("stmt"),

        "continue" => Keyword("continue"),
        "break" => Keyword("break"),
        "return" => [Keyword("return"), NonTerm("expr").opt(),].and(),

        "var_set" => [Identifier, NonTerm("set_equal"),NonTerm("expr")].and(),
        "var" => [
            Keyword("var"), NonTerm("var_set"),
            [NonTerm("comma"),NonTerm("var_set"),].and().many0(),
        ].and(),

        "set" => [
            [
                [NonTerm("val"), NonTerm("val_field_index").take(),].and(),
                Identifier,
            ].or(),
            [NonTerm("add"),NonTerm("sub"),NonTerm("mul"),NonTerm("div"),NonTerm("not")].or().opt(),
            NonTerm("set_equal"),
            NonTerm("expr"),
        ].and(),

        "cond" => [NonTerm("lparen"),NonTerm("expr"),NonTerm("rparen"),].and(),
        "block" => [NonTerm("lcurly"),NonTerm("stmts"),NonTerm("rcurly"),].and(),
        "if" => [
            [Keyword("if").d(), NonTerm("cond"), NonTerm("block")].and().group(""),
            [Keyword("elif").d(),NonTerm("cond"),NonTerm("block"),].and().group("").many0(),
            [Keyword("else").d(),NonTerm("block"),].and().group("").opt(),
        ].and().group("if"),
        "while" => [Keyword("while").d(), NonTerm("cond"), NonTerm("block"),].and().group("while"),
        // "for_init" => [
        //     NonTerm("var"),
        //     [NonTerm("set"), [NonTerm("comma"),NonTerm("set")].and().many0(),].and(),
        // ].or(),
        // "for_incr_stmt" => [NonTerm("set"),NonTerm("call"),].or(),
        // "for_incr" => [
        //     NonTerm("for_incr_stmt"),
        //     [NonTerm("comma"),NonTerm("for_incr_stmt")].and().many0(),
        // ].and(),
        // "for" => [
        //     Keyword("for"),
        //     NonTerm("lparen"),
        //     NonTerm("for_init"),
        //     NonTerm("semicolon"),
        //     NonTerm("expr"),
        //     NonTerm("semicolon"),
        //     NonTerm("for_incr"),
        //     NonTerm("rparen"),
        //     NonTerm("block"),
        // ].and(),
        "for_in" => [
            Keyword("for"),
            NonTerm("lparen"),
            Identifier,
            Keyword("in").d(),
            [NonTerm("val"),NonTerm("call"),].and(),
            NonTerm("rparen"),
            NonTerm("block"),
            ].and(),
        "for_to" => [
            Keyword("for"),
            NonTerm("lparen"),
            Identifier,
            Keyword("in").d(),
            NonTerm("expr"),
            Keyword("to").d(),
            NonTerm("expr"),
            NonTerm("rparen"),
            NonTerm("block"),
        ].and(),
        "call" => [
            NonTerm("lparen"),
            [
                NonTerm("expr"),
                [NonTerm("comma").d(),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt().d(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),
        "include" => [Keyword("include").d(),String,].and().group("include"),

        "func_params" => [
            NonTerm("lparen"),
            [
                Identifier,
                [NonTerm("comma"),Identifier,].and().many0(),
                NonTerm("ellipsis").opt(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),
        "func" => [
            Keyword("fn"),
            [Identifier, NonTerm("val_field_index").many0()].and(),
            NonTerm("func_params"),
            NonTerm("lcurly"),
            NonTerm("stmts"),
            NonTerm("rcurly"),
        ].and(),
        "lambda" => [Keyword("fn"),NonTerm("func_params"),NonTerm("lcurly"),NonTerm("stmts"),NonTerm("rcurly"),].and(),

        "infix" => [
            NonTerm("add"),NonTerm("sub"),
            NonTerm("mul"),NonTerm("div"),
            NonTerm("lt"),NonTerm("gt"),
            NonTerm("le"),NonTerm("ge"),
            NonTerm("eq"),NonTerm("ne"),
            NonTerm("and"),NonTerm("or"),
        ].or(),

        "expr" => [
            NonTerm("val"),
            [NonTerm("infix"),NonTerm("val"),].and().many0(),
        ].and().group("expr"),

        "prefix" => [NonTerm("add"),NonTerm("sub"),NonTerm("not"),].or(),
        "val_field_index" => [ NonTerm("val_index"), NonTerm("val_field"), ].or(),

        "val_field" => [NonTerm("dot"),[Identifier,Int,].or(),].and(),
        "val_index" => [NonTerm("lsquare"),NonTerm("expr"),NonTerm("rsquare"),].and(),

        "val" => [
            NonTerm("prefix").many0(),
            [
                Int,
                Float,
                String,
                Keyword("void"),
                Keyword("nil"),
                Keyword("true"),
                Keyword("false"),
                NonTerm("if"),
                NonTerm("lambda"),
                Identifier,
                [NonTerm("lparen"),NonTerm("expr"),NonTerm("rparen"),].and(),
            ].or(),
            [NonTerm("val_field_index"),NonTerm("call"),].or().many0(),
        ].and(),

        "format_params" => [
            NonTerm("lparen"),
            String,
            [
                [String,NonTerm("expr"),].or(),
                [NonTerm("comma"),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),

        "format" => [Keyword("format"),NonTerm("format_params"),].and(),
        "print" => [Keyword("print"),NonTerm("format_params"),].and(),
        "println" => [Keyword("println"),NonTerm("format_params"),].and(),

        "lcurly" => Symbol("{").d(),
        "rcurly" => Symbol("}").d(),
        "lsquare" => Symbol("[").d(),
        "rsquare" => Symbol("]").d(),
        "lparen" => Symbol("(").d(),
        "rparen" => Symbol(")").d(),

        "semicolon" => Symbol(";"),
        "end" => [NonTerm("semicolon"),].or(),

        "dot" => Symbol("."),
        "ellipsis" => [NonTerm("dot"),NonTerm("dot"),NonTerm("dot"),].and(),
        "comma" => Symbol(","),

        "set_equal" => Symbol("="),

        "not" => Symbol("!").group("not"),
        "add" => Symbol("+"),
        "sub" => Symbol("-"),
        "mul" => Symbol("*"),
        "div" => Symbol("/"),

        "and" => [Symbol("&"),Symbol("&"),].and().group("and"),
        "or" => [Symbol("|"),Symbol("|"),].and().group("or"),

        "lt" => Symbol("<").group("lt"),
        "gt" => Symbol(">").group("gt"),
        "le" => [Symbol("<"),Symbol("="),].and().group("le"),
        "ge" => [Symbol(">"),Symbol("="),].and().group("ge"),
        "eq" => [Symbol("="),Symbol("="),].and().group("eq"),
        "ne" => [Symbol("!"),Symbol("="),].and().group("ne"),
        _ => Error,
    }

}

// #[derive(Clone,Debug)]
// enum GrammarOutput<'a> {
//     Group{name:&'a str,primitives:Range<usize>},
//     Primitive(PrimitiveContainer<'a>),

// }

// impl<'a> std::fmt::Debug for GrammarOutput<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Group { name, primitives } => {
//                 write!(f,"{name}:{primitives:?}")
//                 // f.debug_struct("Group").field("name", name).field("primitives", primitives).finish()
//             }
//             Self::Primitive(arg0) => {
//                 // f.debug_tuple("Primitive").field(arg0).finish()
//                 write!(f,"{arg0:?}")
//             }
//         }
//     }
// }


// #[derive(Clone)]
// enum TempOutput<'a> {
//     Group{name:&'a str,group:usize,parent_group:usize,},
//     Primitive{g:PrimitiveContainer<'a>,parent_group:usize,},

// }
//
pub fn grammar_run<'a>( top_primitives:PrimitiveIterContainer<'a>) {
    /*
    * need on_succes_ind, on_fail_len?
    * success/fail ind needs to be before or after? after for truncate?

    */

    /*
        abc|ab with "ab" => "" //abc will fail, but then tries ab, which succeeds
        ab|abc with "abc" => "c" //will consume ab, and then fail to consume c, there is no backtracking

     */
    // let mut temp_groups: Vec<Vec<GrammarOutput>> = vec![vec![]];
    // let mut temp_group_inds: Vec<(usize, usize)> = vec![];
    // // let mut cur_out=Vec::new();

    #[derive(Clone, Copy, Default, Debug)]
    struct PrimitiveInfo {
        // name:&'a str,
        // depth:usize,
        group:usize,
        discard:bool,
    }

    #[derive(Debug)]
    struct GroupInfo<'a> {
        name:&'a str,
        parent:usize, //group
        primitive_ind_start:usize,
    }

    // let mut temp_groups2: Vec<(&'a str,Range<usize>)> = vec![("",0..0)]; //start with root group, for simplicity ...
    // let mut temp_groups: BTreeMap<usize,PrimitiveInfo<'a>>=Default::default();

    // let mut temp_groups2:Vec<PrimitiveInfo> = vec![Default::default()];

    let mut temp_primtives : Vec<PrimitiveInfo> = Default::default();
    let mut temp_groups3 : Vec<GroupInfo<'a>> = vec![GroupInfo{
        name: "", parent: 0, primitive_ind_start:0,
    }];

    //not completely correct,
    //  if succeeds to a certain point, need to clear the expecteds,
    //  currently it just clears everything after any success
    //  could just add expects that are ==, and replace ones that are >
    //  on success, only clear if >= than loc
    let mut expected: (Loc,Vec<GrammarItem<'a>>,) = Default::default();
    // let mut temp_outpts:Vec<TempOutput<'a>> = vec![]

    struct Work<'a> {
        grammar:GrammarItem<'a>,
        success_len:usize,
        fail_len:usize,
        primitives:PrimitiveIterContainer<'a>,
        group_ind:usize,

        group_len:usize, //only used for removing unused groups ... but even then it is not required, mainly used for debugging
        output_len:usize,

        discard:bool,

        opt_hists:HashMap<&'a str,usize>, //[non_term]=output_ind

        visiteds:HashSet<(&'a str,usize)>,

        takeables:HashMap<GrammarItem<'a>,PrimitiveIterContainer<'a>>, //[non_term]
    }

    let mut stk=vec![
        Work{
            grammar:GrammarItem::Error,success_len:0,fail_len:0,primitives:top_primitives,
            group_ind: 0, group_len: 1, output_len: 0, discard:false,
            opt_hists:Default::default(),
            visiteds:Default::default(),
            takeables:Default::default(),
        },
        Work{
            grammar:grammar_decl("test9"),success_len:0,fail_len:1,primitives:top_primitives,
            group_ind: 0, group_len: 1, output_len: 0, discard:false,
            opt_hists:Default::default(),
            visiteds:Default::default(),
            takeables:Default::default(),
        },
    ];

    let mut primitives_remaining = top_primitives.clone();

    let mut c=0;
    while let Some(mut cur)=stk.pop() {
        c+=1;

        // // if c>30 {break;}
        // // println!(": {cur:?} || {} && {primitives:?}", stk.iter().rev().map(|x|format!("{:?}",x.0)).collect::<Vec<_>>().join(" << "), );
        {
            let Work { grammar, success_len, fail_len, primitives, group_ind, group_len, output_len, discard, opt_hists, visiteds, takeables: grammar_outputs, }=&cur;
            println!("{c:4}: {grammar:?}, ps={primitives:?}, success={success_len}, fail={fail_len}, group_ind={group_ind}, group_len={group_len}, output_len={output_len}, discard={discard}, opt_hists={opt_hists:?}, visiteds={visiteds:?}, {grammar_outputs:?}");
        }

        for (i,Work { grammar:g, success_len:s, fail_len:f, primitives:ps, group_ind, group_len, output_len, discard, opt_hists, visiteds, takeables: grammar_outputs }) in stk.iter()
            // .rev()
            .enumerate() {
            // println!("\t{i:3}: {g:?}\n\t   : {ps:?}\n\t   : success={s}, fail={f}",);
            println!("\t{i:3}: {g:?}, ps={ps:?},success={s}, fail={f}, group_ind={group_ind}, group_len={group_len}, output_len={output_len}, discard={discard}, opt_hists={opt_hists:?}, visiteds={visiteds:?}, {grammar_outputs:?}",);
        }

        match cur.grammar {
            GrammarItem::Group(name, g) => {
                // // outputs.push(GrammarOutput::Group { name, primitives: Vec::new() });
                // let new_group=temp_groups.len();
                // // temp_groups2.push((name,cur.primitives.inds().start..cur.primitives.inds().start));
                // temp_groups.push(value);
                let new_group_ind=temp_groups3.len();
                temp_groups3.push(GroupInfo {
                    name, parent: cur.group_ind,
                    primitive_ind_start: cur.primitives.inds().start,
                });
                let new_group_len=temp_groups3.len();

                stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,
                    // group: PrimitiveInfo { name, depth: cur.group.depth+1, group: cur.group_next_ind },
                    // group_next_ind:cur.group_next_ind+1,
                    group_ind: new_group_ind,
                    group_len: new_group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    opt_hists:cur.opt_hists,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                });


                // temp_group_inds.push((cur.group,temp_groups[cur.group].len()));
                // temp_groups[cur.group].push(GrammarOutput::Group { name, primitives: 0..0, });
                // temp_groups.push(Vec::new());
            }
            GrammarItem::Discard(g) => {

                stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:true,
                    opt_hists:cur.opt_hists,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                });


            }
            GrammarItem::And(gs) => {
                let Some(first)=gs.first().cloned() else {continue;};

                if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
                    stk.push(Work {
                        grammar: GrammarItem::And(rest.into()),
                        success_len: cur.success_len,
                        fail_len: cur.fail_len,

                        //not really necessary? since gets updated by always/primtitives
                        primitives: cur.primitives,

                        group_ind: cur.group_ind,
                        group_len: cur.group_len,
                        output_len: cur.output_len,
                        discard:cur.discard,

                        opt_hists:cur.opt_hists.clone(),
                        visiteds:cur.visiteds.clone(),
                        takeables:cur.takeables.clone(),

                    });
                }

                let success_len=if gs.len()>1 {stk.len()}else{cur.success_len};

                stk.push(Work {
                    grammar: first,
                    success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    opt_hists:cur.opt_hists,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                });
            }
            GrammarItem::Or(gs) => {
                let Some(first)=gs.first().cloned() else {continue;};

                if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
                    stk.push(Work {
                        grammar: GrammarItem::Or(rest.into()),
                        success_len: cur.success_len,
                        fail_len: cur.fail_len,
                        primitives: cur.primitives,

                        group_ind: cur.group_ind,
                        group_len: cur.group_len,
                        output_len: cur.output_len,
                        discard:cur.discard,
                        opt_hists:cur.opt_hists.clone(),
                        visiteds:cur.visiteds.clone(),
                        takeables:cur.takeables.clone(),
                    });
                }

                let fail_len=if gs.len()>1 {stk.len()}else{cur.fail_len};

                stk.push(Work {
                    grammar: first,
                    success_len: cur.success_len,
                    fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    opt_hists:cur.opt_hists,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                });
            }

            GrammarItem::Opt(g) => {
                stk.push(Work {
                    grammar: GrammarItem::Always,
                    success_len: cur.success_len,
                    fail_len: 0, //fail is not used
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    opt_hists:cur.opt_hists.clone(),
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                });

                let fail_len=stk.len();

                stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    opt_hists:cur.opt_hists,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                });
            }
            GrammarItem::Take(g) => {
                if let Some(x)=cur.takeables.get(&g).cloned() {
                    if let Some(p)=temp_primtives.get(x.inds().start) {
                        //clear unused groups
                        let mut g=p.group;

                        let mut gg=0;

                        while g!=0 {
                            let group=temp_groups3.get(g).unwrap();

                            if group.primitive_ind_start>=x.inds().start {
                                gg=g;
                            }

                            g=group.parent;
                        }

                        if gg!=0 {
                            temp_groups3.truncate(gg);
                        }
                    }

                    //clear outputs to start of taken
                    temp_primtives.truncate(x.inds().start);

                    //
                    stk.push(Work {
                        grammar: *g,
                        success_len: cur.success_len,
                        fail_len: cur.fail_len,

                        primitives: x.clone(),
                        output_len: x.inds().start,

                        group_ind: cur.group_ind,
                        group_len: temp_groups3.len(),

                        discard: cur.discard,
                        opt_hists: (),
                        visiteds: cur.visiteds, //
                        takeables: Default::default(),
                    });
                } else {
                    println!("no takeable {g:?}");
                    break;
                }
            }
            GrammarItem::Many(g) => {
                // let fail_len2=stk.len(); //only remove everything past here on fail
                stk.push(Work {
                    grammar: GrammarItem::Many(g.clone()),
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    opt_hists:cur.opt_hists.clone(),
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                });

                let success_len2=stk.len();

                stk.push(Work {
                    grammar: GrammarItem::Always,
                    success_len: cur.success_len,
                    fail_len: 0, //fail is not used
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    opt_hists:cur.opt_hists.clone(),
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                });

                let fail_len=stk.len();
                stk.push(Work {
                    grammar: *g,
                    success_len: success_len2,
                    fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    opt_hists:cur.opt_hists,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                });
            }
            // GrammarItem::Many1(g) => {
            //     stk.push((GrammarItem::Many(g.clone()),success_len,fail_len,primitives));
            //     let success_len=stk.len();
            //     stk.push((*g,success_len,fail_len,primitives));
            // }

            GrammarItem::NonTerm(t) => {
                let v=(t,cur.primitives.inds().start);

                if cur.visiteds.contains(&v) {
                    println!("err, circular nonterm {t}");
                    break;
                }

                let mut visiteds=cur.visiteds;
                visiteds.insert(v);

                stk.push(Work {
                    grammar: grammar_decl(t), //should return err on not found, instead of grammar never, should have error
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    opt_hists:cur.opt_hists,
                    visiteds,
                    takeables:cur.takeables,
                });
            }
            GrammarItem::Always => {
                stk.truncate(cur.success_len);

                if let Some(last)=stk.last_mut() {
                    if last.grammar.is_many() && last.primitives.len()==cur.primitives.len() { //if not parsing anything, exit the many
                        last.grammar=GrammarItem::Always;
                    }

                    last.primitives=cur.primitives;
                    // last.group_next_ind=cur.group_next_ind;


                    // temp_primtives.resize(cur.output_len+1, PrimitiveInfo{ group: cur.group_ind }); //discard:true,

                    // // last.group_ind=cur.group_ind;
                    let last_group_len= last.group_len;
                    last.group_len=cur.group_len;
                    last.output_len=cur.output_len;
                    last.opt_hists=cur.opt_hists;


                    println!("===www {last_group_len} {} {:?}",  cur.group_len, last.grammar);
                    for i in last_group_len .. cur.group_len {
                        let g=temp_groups3.get(i).unwrap();
                        println!("===hmmm {i}");

                        if g.primitive_ind_start==cur.primitives.inds().start {
                            temp_groups3.truncate(i); //removes this group and ones after
                            last.group_len=i;
                            println!("====== {i} {}",temp_groups3.len(), );
                            break;
                        }
                    }

                } else {
                    // top_primitives=cur.primitives;
                    primitives_remaining=cur.primitives;
                }

                // expected=Default::default();

                expected.0=Loc::zero();
                expected.1.clear();

            }
            // GrammarItem::Never => {
            //     stk.truncate(cur.fail_len);

            //     if stk.is_empty() {
            //         // top_primitives=cur.primitives;
            //     }
            // }
            GrammarItem::Error => {
                println!("====error {expected:?}");
                if !expected.1.is_empty() {
                    let ee=expected.1.iter().map(|g|match g {

                        GrammarItem::String => "string",
                        GrammarItem::Identifier => "identifier",
                        GrammarItem::Int => "int",
                        GrammarItem::Float => "float",
                        GrammarItem::Symbol(s) => *s,
                        GrammarItem::Keyword(s) => *s,
                        GrammarItem::Eol => todo!(),
                        _ =>"",
                    }).collect::<Vec<_>>().join(", ");

                    println!("At {}, expected {:?}",expected.0,ee);

                } else  {
                    let loc=cur.primitives.loc();

                    println!("At {loc}", );

                }

                break;

            }
            GrammarItem::String => {
                match cur.primitives.pop_string() {
                    Ok(v) => {
                        // expected=Default::default();
                        if v.primitive.start_loc() >= expected.0 {
                            expected.0=Loc::zero();
                            expected.1.clear();
                        }
                        println!("--- string {:?}",v.value);
                        stk.truncate(cur.success_len);

                        temp_primtives.resize(v.primitive.ind(), PrimitiveInfo{ group: cur.group_ind,discard:true, }); //discard:true,
                        temp_primtives.push(PrimitiveInfo{ group: cur.group_ind,discard:cur.discard,});

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                            // last.group_next_ind=cur.group_next_ind;

                            // last.group_ind=cur.group_ind;
                            last.group_len=cur.group_len;
                            last.output_len=temp_primtives.len();

                            last.takeables.clear();
                        }

                        // // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        // // temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                        // temp_groups.insert(v.primitive.ind(),cur.group);
                    }
                    Err(loc) => {
                        if loc==expected.0 {
                            expected.1.push(cur.grammar);
                        } else if loc>expected.0 {
                            expected.0=loc;
                            expected.1=vec![cur.grammar];
                        }

                        stk.truncate(cur.fail_len);


                        if let Some(last)=stk.last() {
                            temp_primtives.truncate(last.output_len);
                        }
                    }
                }
                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                    primitives_remaining=cur.primitives;
                }
            }
            GrammarItem::Identifier => {
                match cur.primitives.pop_identifier() {
                    Ok(v) => {
                        // expected=Default::default();
                        if v.primitive.start_loc() >= expected.0 {
                            expected.0=Loc::zero();
                            expected.1.clear();
                        }
                        println!("--- identifier {:?}",v.value);
                        stk.truncate(cur.success_len);

                        temp_primtives.resize(v.primitive.ind(), PrimitiveInfo{ group: cur.group_ind,discard:true, }); //discard:true,
                        temp_primtives.push(PrimitiveInfo{ group: cur.group_ind,discard:cur.discard,});

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                            // last.group_next_ind=cur.group_next_ind;


                            // last.group_ind=cur.group_ind;
                            last.group_len=cur.group_len;
                            last.output_len=temp_primtives.len();
                            last.takeables.clear();
                        }

                        // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        // temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                        // temp_groups.insert(v.primitive.ind(),cur.group);
                    }
                    Err(loc) => {
                        if loc==expected.0 {
                            expected.1.push(cur.grammar);
                        } else if loc>expected.0 {
                            expected.0=loc;
                            expected.1=vec![cur.grammar];
                        }

                        stk.truncate(cur.fail_len);


                        if let Some(last)=stk.last() {
                            temp_primtives.truncate(last.output_len);
                        }
                    }
                }
                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                    primitives_remaining=cur.primitives;
                }
            }
            GrammarItem::Int => {
                match cur.primitives.pop_int() {
                    Ok(v) => {
                        // expected=Default::default();
                        if v.primitive.start_loc() >= expected.0 {
                            expected.0=Loc::zero();
                            expected.1.clear();
                        }
                        println!("--- int {:?}",v.value);
                        stk.truncate(cur.success_len);

                        temp_primtives.resize(v.primitive.ind(), PrimitiveInfo{ group: cur.group_ind,discard:true, }); //discard:true,
                        temp_primtives.push(PrimitiveInfo{ group: cur.group_ind,discard:cur.discard,});

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                            // last.group_next_ind=cur.group_next_ind;

                            // last.group_ind=cur.group_ind;
                            last.group_len=cur.group_len;
                            last.output_len=temp_primtives.len();
                            last.takeables.clear();
                        }

                        // // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        // // temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                        // temp_groups.insert(v.primitive.ind(),cur.group);
                    }
                    Err(loc) => {
                        if loc==expected.0 {
                            expected.1.push(cur.grammar);
                        } else if loc>expected.0 {
                            expected.0=loc;
                            expected.1=vec![cur.grammar];
                        }

                        stk.truncate(cur.fail_len);

                        if let Some(last)=stk.last() {
                            temp_primtives.truncate(last.output_len);
                        }
                    }
                }

                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                    primitives_remaining=cur.primitives;
                }
            }
            GrammarItem::Float => {
                match cur.primitives.pop_float() {
                    Ok(v) => {
                        // expected=Default::default();
                        if v.primitive.start_loc() >= expected.0 {
                            expected.0=Loc::zero();
                            expected.1.clear();
                        }
                        println!("--- float {:?}",v.value);
                        stk.truncate(cur.success_len);

                        temp_primtives.resize(v.primitive.ind(), PrimitiveInfo{ group: cur.group_ind,discard:true, }); //discard:true,
                        temp_primtives.push(PrimitiveInfo{ group: cur.group_ind,discard:cur.discard,});

                        if let Some(last)=stk.last_mut() {
                        //     println!("{_g:?}");
                            last.primitives=cur.primitives;
                            // last.group_next_ind=cur.group_next_ind;

                            // last.group_ind=cur.group_ind;
                            last.group_len=cur.group_len;
                            last.output_len=temp_primtives.len();
                            last.takeables.clear();
                        }

                        // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        // temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                        // temp_groups.insert(v.primitive.ind(),cur.group);
                    }
                    Err(loc) => {
                        if loc==expected.0 {
                            expected.1.push(cur.grammar);
                        } else if loc>expected.0 {
                            expected.0=loc;
                            expected.1=vec![cur.grammar];
                        }

                        stk.truncate(cur.fail_len);

                        if let Some(last)=stk.last() {
                            temp_primtives.truncate(last.output_len);
                        }
                    }
                }

                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                    primitives_remaining=cur.primitives;
                }
            }
            GrammarItem::Symbol(s) => {
                match cur.primitives.pop_with_symbols([s]) {
                    Ok(v) => {
                        // expected=Default::default();
                        if v.primitive.start_loc() >= expected.0 {
                            expected.0=Loc::zero();
                            expected.1.clear();
                        }
                        println!("--- symbol {:?}",v.value);
                        stk.truncate(cur.success_len);

                        temp_primtives.resize(v.primitive.ind(), PrimitiveInfo{ group: cur.group_ind,discard:true, }); //discard:true,
                        temp_primtives.push(PrimitiveInfo{ group: cur.group_ind,discard:cur.discard,});

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                            // last.group_next_ind=cur.group_next_ind;

                            // last.group_ind=cur.group_ind;
                            last.group_len=cur.group_len;
                            last.output_len=temp_primtives.len();
                            last.takeables.clear();
                        }

                        // // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        // // temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                        // temp_groups.insert(v.primitive.ind(),cur.group);
                    }
                    Err(loc) => {
                        if loc==expected.0 {
                            expected.1.push(cur.grammar);
                        } else if loc>expected.0 {
                            expected.0=loc;
                            expected.1=vec![cur.grammar];
                        }

                        stk.truncate(cur.fail_len);
                        // println!("nos");

                        if let Some(last)=stk.last() {
                            temp_primtives.truncate(last.output_len);
                        }
                    }
                }
                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                    primitives_remaining=cur.primitives;
                }
            }
            GrammarItem::Keyword(s) => {
                match cur.primitives.pop_with_identifiers([s]) {
                    Ok(v) => {
                        // expected=Default::default();
                        if v.primitive.start_loc() >= expected.0 {
                            expected.0=Loc::zero();
                            expected.1.clear();
                        }
                        println!("--- keyword {:?}",v.value);
                        stk.truncate(cur.success_len);

                        temp_primtives.resize(v.primitive.ind(), PrimitiveInfo{ group: cur.group_ind,discard:true, }); //discard:true,
                        temp_primtives.push(PrimitiveInfo{ group: cur.group_ind,discard:cur.discard,});

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                            // last.group_next_ind=cur.group_next_ind;

                            // last.group_ind=cur.group_ind;
                            last.group_len=cur.group_len;
                            last.output_len=temp_primtives.len();
                            last.takeables.clear();
                        }

                        // // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        // // temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                        // temp_groups.insert(v.primitive.ind(),cur.group);
                    }
                    Err(loc) => {
                        if loc==expected.0 {
                            expected.1.push(cur.grammar);
                        } else if loc>expected.0 {
                            expected.0=loc;
                            expected.1=vec![cur.grammar];
                        }

                        stk.truncate(cur.fail_len);

                        if let Some(last)=stk.last() {
                            temp_primtives.truncate(last.output_len);
                        }
                    }
                }
                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                    primitives_remaining=cur.primitives;
                }
            }
            GrammarItem::Eol => {
                match cur.primitives.pop_eol() {
                    Ok(v) => {
                        if v.primitive.start_loc() >= expected.0 {
                            // expected.0=v.primitive.start_loc();
                            // expected=Default::default();
                            expected.0=Loc::zero();
                            expected.1.clear();
                        }
                        println!("eol");
                        stk.truncate(cur.success_len);

                        temp_primtives.resize(v.primitive.ind(), PrimitiveInfo{ group: cur.group_ind,discard:true, }); //discard:true,
                        temp_primtives.push(PrimitiveInfo{ group: cur.group_ind,discard:cur.discard,});

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                            // last.group_next_ind=cur.group_next_ind;

                            // last.group_ind=cur.group_ind;
                            last.group_len=cur.group_len;
                            last.output_len=temp_primtives.len();
                            last.takeables.clear();
                        }

                        // // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        // // temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                        // temp_groups.insert(v.primitive.ind(),cur.group);
                    }
                    Err(loc) => {
                        if loc==expected.0 {
                            expected.1.push(cur.grammar);
                        } else if loc>expected.0 {
                            expected.0=loc;
                            expected.1=vec![cur.grammar];
                        }

                        stk.truncate(cur.fail_len);

                        if let Some(last)=stk.last() {
                            temp_primtives.truncate(last.output_len);
                        }
                    }
                }
                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                    primitives_remaining=cur.primitives;
                }
            }
        }
    }

    // //
    // let mut temp_group_ends: Vec<usize> = Vec::new();

    // for (i,gs) in temp_groups.iter().enumerate() {
    //     let last_end=temp_group_ends.get(i).cloned().unwrap_or_default();
    //     temp_group_ends.push(last_end+gs.len());
    // }

    // //
    // for (i,(group,group_primitives_ind)) in temp_group_inds.into_iter().enumerate() {
    //     let i=i+1;
    //     let temp_group_primtives=&mut temp_groups[i];

    // }

    // //
    // let mut outputs: Vec<GrammarOutput> = Vec::new();

    // for gs in temp_groups {
    //     outputs.extend(gs);
    // }

    // {
    //     println!("===");

    //     let mut stk: Vec<(usize, usize)>=(0..temp_group_ends[0]).rev().map(|i|(i,0)).collect::<Vec<_>>();

    //     // let mut stk=vec![0..temp_group_ends.last().cloned().unwrap_or_default()];

    //     while let Some((ind,depth))=stk.pop() {
    //         let cur=&outputs[ind];

    //         let indent="  ".repeat(depth);

    //         match cur {
    //             GrammarOutput::Group { name, primitives } => {
    //                 stk.extend(primitives.clone().rev().map(|i|(i,depth+1)));
    //                 println!("{indent}group: {name}");
    //             }
    //             GrammarOutput::Primitive(p) => {
    //                 println!("{indent}{p:?}");
    //             }
    //         }
    //     }
    //     println!("===");
    // }

    println!("groups={temp_groups3:?}");
    println!("outputs={temp_primtives:?}");

    if !primitives_remaining.is_empty() {
        println!("error, failed to parse all tokens {primitives_remaining:?}");
    }

    println!("===");


    // let mut last_group:PrimitiveInfo=Default::default();

    let mut groups_visited: HashSet<usize>=HashSet::new();

    for p in top_primitives {
        let i=p.ind();
        let Some(output)=temp_primtives.get(i) else {
            break;
        };

        let mut g=output.group;
        let mut depth=0;
        let mut gs: Vec<usize>=Vec::new();
        while g!=0 {
            gs.push(g);
            let gg=&temp_groups3[g];

            depth+=1;

            g=gg.parent;

        }

        for (d,&g) in gs.iter().rev().enumerate() {
            let gg=&temp_groups3[g];

            if !groups_visited.contains(&g) {
                println!("{}{:?}",
                    "  ".repeat(d),
                    gg.name,
                );
                groups_visited.insert(g);
            }



        }

        println!("{}{}{p:?}",
            "  ".repeat(depth),
            if output.discard {"-"}else{""}
        );
        // let group=
        // let g=temp_groups.get(&i).cloned();//.unwrap_or_default();
        // let Some(g)=g else {
        //     // println!("~~~i=[{i}], p={p:?}");
        //     // break;
        //     continue;
        // };
        // // let prev_g= (i!=0).then(||temp_groups.get(&(i-1)).cloned().unwrap_or_default()).unwrap_or_default();

        // if g.depth>last_group.depth {
        //     println!("{}group: {}","    ".repeat(last_group.depth),g.name);
        // }


        // println!("{}{p:?}","    ".repeat(g.depth));
        // last_group=g;
    }
    println!("===");

    //
    println!("top_primitives={top_primitives:?}",  );
    // println!("output={outputs:?}",  );

}

/*

* use stk of grammer_items

* how handle manys,

* how handle ors
** need backtracking or lock in if any parse succeeds
** when coming across an OR, then mark ind into the stk, so can jump back to if fail, and if suceeds, then remove that OR and the mark
** can put ORs on second stk eg Vec<(Vec<GrammarItem>,usize)>

* for And, just push whole lot on stk?
** no simpler for handling Ors not to, or can depending

* for many0 and opt, push them on OR stk, but with Always as last option
** eg [OptionalItem,Always] or [Many0,Always]
*** Many0 on succeeds, ...
*/