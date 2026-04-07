/*
TODO
* add Group(group_name,grammar_item), for output
** or instead add as method and(expr,expr).group("abc")

* output
** if something like and(expr, expr, expr (or stmt)) => [expr,expr,stmt]
** if something like and(group("abc",and(expr, expr)), expr (or stmt)) => [abc[expr,expr],stmt]
** if group not used, all output would be one single list of primitives
*/

use std::ops::Range;

use crate::ccexpr_parser::{PrimitiveContainer, PrimitiveIterContainer};

#[derive(Clone,Debug)]
pub enum GrammarItem<'a> {
    Many(Box<GrammarItem<'a>>),
    // Many1(Box<GrammarItem<'a>>),
    And(Vec<GrammarItem<'a>>), //should store reversed?
    Or(Vec<GrammarItem<'a>>), //should store reversed?
    Opt(Box<GrammarItem<'a>>),
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
    Never, //replace with Error ?
    // Not(Box<GrammarItem<'a>>), //todo, needed? better to have NotIdentifier etc?
    // Discard(Box<GrammarItem<'a>>), //todo, removes token from output (via just hding it, ie have hashmap of tokens to hide)
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

        "start" => NonTerm("stmts"),

        "ending" => [NonTerm("semicolon"),Eol].or().many1(),
        "stmts" => [
            NonTerm("stmt"),
            [NonTerm("ending"), NonTerm("stmt"),].and().many0(),
            NonTerm("ending").many0(),
        ].and().opt(),

        "stmt" => [
            NonTerm("block"),
            NonTerm("var"),NonTerm("set"),
            NonTerm("while"),NonTerm("for_in"),NonTerm("for_to"),
            NonTerm("break"), NonTerm("continue"),
            NonTerm("return"),
            NonTerm("include"),
            NonTerm("format"),NonTerm("print"),NonTerm("println"),
            NonTerm("expr"),
            // NonTerm("if"),
        ].or(),

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
                [NonTerm("val"), NonTerm("val_field_index"),].and(),
                Identifier,
            ].or(),
            [NonTerm("add"),NonTerm("sub"),NonTerm("mul"),NonTerm("div"),NonTerm("not")].or().opt(),
            NonTerm("set_equal"),
            NonTerm("expr"),
        ].and(),

        "cond" => [NonTerm("lparen"),NonTerm("expr"),NonTerm("rparen"),].and(),
        "block" => [NonTerm("lcurly"),NonTerm("stmts"),NonTerm("rcurly"),].and(),
        "if" => [
            Keyword("if"), NonTerm("cond"), NonTerm("block"),
            [Keyword("elif"),NonTerm("cond"),NonTerm("block"),].and().many0(),
            [Keyword("else"),NonTerm("block"),].and().opt(),
        ].and().group("if"),
        "while" => [Keyword("while"), NonTerm("cond"), NonTerm("block"),].and(),
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
            Keyword("in"),
            [NonTerm("val"),NonTerm("call"),].and(),
            NonTerm("rparen"),
            NonTerm("block"),
            ].and(),
        "for_to" => [
            Keyword("for"),
            NonTerm("lparen"),
            Identifier,
            Keyword("in"),
            NonTerm("expr"),
            Keyword("to"),
            NonTerm("expr"),
            NonTerm("rparen"),
            NonTerm("block"),
        ].and(),
        "call" => [
            NonTerm("lparen"),
            [
                NonTerm("expr"),
                [NonTerm("comma"),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),
        "include" => [Keyword("include"),String,].and(),

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
        "func_decl" => [
            Keyword("fn"),
            [Identifier, NonTerm("val_field_index").many0()].and(),
            NonTerm("func_params"),
            NonTerm("lcurly"),
            NonTerm("stmts"),
            NonTerm("rcurly"),
        ].and(),
        "func_lambda" => [Keyword("fn"),NonTerm("func_params"),NonTerm("lcurly"),NonTerm("stmts"),NonTerm("rcurly"),].and(),

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
        ].and(),

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

        "lcurly" => Symbol("{"),
        "rcurly" => Symbol("}"),
        "lsquare" => Symbol("["),
        "rsquare" => Symbol("]"),
        "lparen" => Symbol("("),
        "rparen" => Symbol(")"),

        "semicolon" => Symbol(";"),
        "end" => [NonTerm("semicolon"),].or(),

        "dot" => Symbol("."),
        "ellipsis" => [NonTerm("dot"),NonTerm("dot"),NonTerm("dot"),].and(),
        "comma" => Symbol(","),

        "set_equal" => Symbol("="),

        "not" => Symbol("!"),
        "add" => Symbol("+"),
        "sub" => Symbol("-"),
        "mul" => Symbol("*"),
        "div" => Symbol("/"),

        "and" => [Symbol("&"),Symbol("&"),].and(),
        "or" => [Symbol("|"),Symbol("|"),].and(),

        "lt" => Symbol("<"),
        "gt" => Symbol(">"),
        "le" => [Symbol("<"),Symbol("="),].and(),
        "ge" => [Symbol(">"),Symbol("="),].and(),
        "eq" => [Symbol("="),Symbol("="),].and(),
        "ne" => [Symbol("!"),Symbol("="),].and(),
        _ => Never,
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

//
pub fn grammar_run<'a>(mut top_primitives:PrimitiveIterContainer<'a>) {
    /*
    * need on_succes_ind, on_fail_len?
    * success/fail ind needs to be before or after? after for truncate?

    */
    // let mut temp_groups: Vec<Vec<GrammarOutput>> = vec![vec![]];
    // let mut temp_group_inds: Vec<(usize, usize)> = vec![];
    // // let mut cur_out=Vec::new();

    let mut temp_groups2: Vec<(&'a str,Range<usize>)> = vec![("",0..0)]; //start with root group, for simplicity ...

    struct Work<'a> {
        grammar:GrammarItem<'a>,
        success_len:usize,
        fail_len:usize,
        primitives:PrimitiveIterContainer<'a>,
        group:usize,
    }

    let mut stk=vec![
        Work{grammar:grammar_decl("start"),success_len:0,fail_len:0,primitives:top_primitives,group:0,}
    ];

    let mut c=0;
    while let Some(mut cur)=stk.pop() {
        c+=1;

        // if c>30 {break;}
        // println!(": {cur:?} || {} && {primitives:?}", stk.iter().rev().map(|x|format!("{:?}",x.0)).collect::<Vec<_>>().join(" << "), );
        {
            let Work { grammar, success_len, fail_len, primitives,group: group_start, }=&cur;
            println!("{c:4}: {grammar:?}, ps={primitives:?}, success={success_len}, fail={fail_len}, group_start={group_start}");
        }

        for (i,Work { grammar:g, success_len:s, fail_len:f, primitives:ps,group: group_start }) in stk.iter()
            // .rev()
            .enumerate() {
            // println!("\t{i:3}: {g:?}\n\t   : {ps:?}\n\t   : success={s}, fail={f}",);
            println!("\t{i:3}: {g:?}, ps={ps:?},success={s}, fail={f}, group_start={group_start}",);
        }

        match cur.grammar {
            GrammarItem::Group(name, g) => {
                // outputs.push(GrammarOutput::Group { name, primitives: Vec::new() });
                let new_group=temp_groups2.len();
                temp_groups2.push((name,cur.primitives.inds().start..cur.primitives.inds().start));

                stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,
                    group: new_group,
                });


                // temp_group_inds.push((cur.group,temp_groups[cur.group].len()));
                // temp_groups[cur.group].push(GrammarOutput::Group { name, primitives: 0..0, });
                // temp_groups.push(Vec::new());
            }
            GrammarItem::And(gs) => {
                let Some(first)=gs.first().cloned() else {continue;};

                if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
                    stk.push(Work {
                        grammar: GrammarItem::And(rest.into()),
                        success_len: cur.success_len,
                        fail_len: cur.fail_len,
                        primitives: cur.primitives,
                        group: cur.group,
                    });
                }

                let success_len=if gs.len()>1 {stk.len()}else{cur.success_len};

                stk.push(Work {
                    grammar: first,
                    success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,
                    group: cur.group,
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
                        group: cur.group,
                    });
                }

                let fail_len=if gs.len()>1 {stk.len()}else{cur.fail_len};

                stk.push(Work {
                    grammar: first,
                    success_len: cur.success_len,
                    fail_len,
                    primitives: cur.primitives,
                    group: cur.group,
                });
            }

            GrammarItem::Opt(g) => {
                stk.push(Work {
                    grammar: GrammarItem::Always,
                    success_len: cur.success_len,
                    fail_len: 0, //fail is not used
                    primitives: cur.primitives,
                    group: cur.group,
                });

                let fail_len=stk.len();

                stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len,
                    primitives: cur.primitives,
                    group: cur.group,
                });
            }
            GrammarItem::Many(g) => {
                // let fail_len2=stk.len(); //only remove everything past here on fail
                stk.push(Work {
                    grammar: GrammarItem::Many(g.clone()),
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,
                    group: cur.group,
                });

                let success_len2=stk.len();

                stk.push(Work {
                    grammar: GrammarItem::Always,
                    success_len: cur.success_len,
                    fail_len: 0, //fail is not used
                    primitives: cur.primitives,
                    group: cur.group,
                });

                let fail_len=stk.len();
                stk.push(Work {
                    grammar: *g,
                    success_len: success_len2,
                    fail_len,
                    primitives: cur.primitives,
                    group: cur.group,
                });
            }
            // GrammarItem::Many1(g) => {
            //     stk.push((GrammarItem::Many(g.clone()),success_len,fail_len,primitives));
            //     let success_len=stk.len();
            //     stk.push((*g,success_len,fail_len,primitives));
            // }
            GrammarItem::Always => {
                stk.truncate(cur.success_len);

                if let Some(last)=stk.last_mut() {
                    if last.grammar.is_many() && last.primitives.len()==cur.primitives.len() { //if not parsing anything, exit the many
                        last.grammar=GrammarItem::Always;
                    }

                    last.primitives=cur.primitives;
                } else {
                    // top_primitives=cur.primitives;
                }
            }
            GrammarItem::Never => {
                stk.truncate(cur.fail_len);

                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                }
            }
            GrammarItem::NonTerm(t) => {
                stk.push(Work {
                    grammar: grammar_decl(t), //should return err on not found, instead of grammar never, should have error
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,
                    group: cur.group,
                });
            }
            GrammarItem::String => {
                match cur.primitives.pop_string() {
                    Ok(v) => {
                        println!("--- string {:?}",v.value);
                        stk.truncate(cur.success_len);

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                        }

                        // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                    }
                    Err(_loc) => {
                        stk.truncate(cur.fail_len);
                    }
                }
                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                }
            }
            GrammarItem::Identifier => {
                match cur.primitives.pop_identifier() {
                    Ok(v) => {
                        println!("--- identifier {:?}",v.value);
                        stk.truncate(cur.success_len);

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                        }

                        // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                    }
                    Err(_loc) => {
                        stk.truncate(cur.fail_len);
                    }
                }
                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                }
            }
            GrammarItem::Int => {
                match cur.primitives.pop_int() {
                    Ok(v) => {
                        println!("--- int {:?}",v.value);
                        stk.truncate(cur.success_len);
                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                        }

                        // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                    }
                    Err(_loc) => {
                        stk.truncate(cur.fail_len);
                    }
                }

                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                }
            }
            GrammarItem::Float => {
                match cur.primitives.pop_float() {
                    Ok(v) => {
                        println!("--- float {:?}",v.value);
                        stk.truncate(cur.success_len);

                        if let Some(last)=stk.last_mut() {
                        //     println!("{_g:?}");
                            last.primitives=cur.primitives;
                        }

                        // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                    }
                    Err(_loc) => {
                        stk.truncate(cur.fail_len);
                    }
                }

                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                }
            }
            GrammarItem::Symbol(s) => {
                match cur.primitives.pop_with_symbols([s]) {
                    Ok(v) => {
                        println!("--- symbol {:?}",v.value);
                        stk.truncate(cur.success_len);

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                        }

                        // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                    }
                    Err(_loc) => {
                        stk.truncate(cur.fail_len);
                        // println!("nos");
                    }
                }
                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                }
            }
            GrammarItem::Keyword(s) => {
                match cur.primitives.pop_with_identifiers([s]) {
                    Ok(v) => {
                        println!("--- keyword {:?}",v.value);
                        stk.truncate(cur.success_len);

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                        }

                        // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                    }
                    Err(_loc) => {
                        stk.truncate(cur.fail_len);
                    }
                }
                if stk.is_empty() {
                    // top_primitives=cur.primitives;
                }
            }
            GrammarItem::Eol => {
                match cur.primitives.pop_eol() {
                    Ok(v) => {
                        println!("eol");
                        stk.truncate(cur.success_len);

                        if let Some(last)=stk.last_mut() {
                            last.primitives=cur.primitives;
                        }

                        // temp_groups[cur.group].push(GrammarOutput::Primitive(v.primitive));
                        temp_groups2[cur.group].1.end=v.primitive.ind()+1; //end+=1
                    }
                    Err(_loc) => {
                        stk.truncate(cur.fail_len);
                    }
                }
                if stk.is_empty() {
                    // top_primitives=cur.primitives;
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

    println!("groups={temp_groups2:?}");

    println!("===");

    for p in top_primitives {
        for (gi,(gn,gr)) in temp_groups2.iter().enumerate().rev() {
            if gr.start==p.ind() {
                println!("{}group: {gn}","    ".repeat(gi));
            }

            if gr.contains(&p.ind()) {
                println!("{}{p:?}","    ".repeat(gi));
                break;
            }
        }

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