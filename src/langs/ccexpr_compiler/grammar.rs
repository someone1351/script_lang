/*
TODO
* add Group(group_name,grammar_item), for output
** or instead add as method and(expr,expr).group("abc")

* output
** if something like and(expr, expr, expr (or stmt)) => [expr,expr,stmt]
** if something like and(group("abc",and(expr, expr)), expr (or stmt)) => [abc[expr,expr],stmt]
** if group not used, all output would be one single list of primitives
*/

use crate::ccexpr_parser::PrimitiveIterContainer;

#[derive(Clone,Debug)]
pub enum GrammarItem<'a> {
    Many0(Box<GrammarItem<'a>>),
    Many1(Box<GrammarItem<'a>>),
    And(Vec<GrammarItem<'a>>), //stored reversed
    Or(Vec<GrammarItem<'a>>), //stored reversed
    Opt(Box<GrammarItem<'a>>),

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
    Never
}

impl<'a> GrammarItem<'a> {
    pub fn many0(self) -> GrammarItem<'a> {
        Self::Many0(self.into())
    }
    pub fn many1(self) -> GrammarItem<'a> {
        Self::Many1(self.into())
    }
    pub fn opt(self) -> GrammarItem<'a> {
        Self::Opt(self.into())
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

        "start" => NonTerm("stmts"),

        "stmts" => [ NonTerm("stmt"), [[NonTerm("semicolon"),Eol].or(), NonTerm("stmt"),].and().many0(), ].and().opt(),

        "stmt" => [
            NonTerm("expr"),
            NonTerm("var"),NonTerm("set"),NonTerm("while"),NonTerm("for"),
            NonTerm("break"), NonTerm("continue"),
            NonTerm("return"),
            NonTerm("include"),
            NonTerm("format"),NonTerm("print"),NonTerm("println"),
            // NonTerm("if"),
        ].or(),

        "continue" => Keyword("continue"),
        "break" => Keyword("break"),
        "return" => [Keyword("return"), NonTerm("expr").opt(),].and(),

        "var_set" => [NonTerm("idn"), NonTerm("set_equal"),NonTerm("expr")].and(),
        "var" => [
            Keyword("var"), NonTerm("var_set"),
            [NonTerm("comma"),NonTerm("var_set"),].and().many0(),
        ].and(),

        "set" => [
            NonTerm("idn"),
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
        ].and(),
        "while" => [Keyword("while"), NonTerm("cond"), NonTerm("block"),].and(),
        "for_init" => [
            NonTerm("var"),
            [NonTerm("set"), [NonTerm("comma"),NonTerm("set")].and().many0(),].and(),
        ].or(),
        "for_incr_stmt" => [NonTerm("set"),NonTerm("call"),].or(),
        "for_incr" => [
            NonTerm("for_incr_stmt"),
            [NonTerm("comma"),NonTerm("for_incr_stmt")].and().many0(),
        ].and(),
        "for" => [
            Keyword("for"),
            NonTerm("lparen"),
            NonTerm("for_init"),
            NonTerm("semicolon"),
            NonTerm("expr"),
            NonTerm("semicolon"),
            NonTerm("for_incr"),
            NonTerm("rparen"),
            NonTerm("block"),
        ].and(),
        "call_params" => [
            NonTerm("lparen"),
            [
                NonTerm("expr"),
                [NonTerm("comma"),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),
        "call" => [NonTerm("idn"),NonTerm("call_params"),].and(),
        "include" => [Keyword("include"),String,].and(),

        "func_params" => [
            NonTerm("lparen"),
            [
                NonTerm("idn"),
                [NonTerm("comma"),NonTerm("idn"),].and().many0(),
                NonTerm("ellipsis").opt(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),
        "func_decl" => [Keyword("fn"),Identifier,NonTerm("func_params"),NonTerm("lcurly"),NonTerm("stmts"),NonTerm("rcurly"),].and(),
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

        "val" => [
            NonTerm("prefix").many0(),
            [
                Int,
                Float,
                String,
                Identifier,
                Keyword("void"),Keyword("nil"),
                Keyword("true"),Keyword("false"),
                NonTerm("call"),
                NonTerm("if"),
                [NonTerm("lparen"),NonTerm("expr"),NonTerm("rparen"),].and(),
            ].or(),
            [ NonTerm("val_index"), NonTerm("val_field"), ].or().many0(),
        ].and(),

        "val_field" => [NonTerm("dot"),[Identifier,Int,].or(),].and(),
        "val_index" => [NonTerm("lsquare"),NonTerm("expr"),NonTerm("rsquare"),].and(),

        "format_params" => [
            NonTerm("lparen"),
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
//
pub fn grammar_run<'a>(mut top_primitives:PrimitiveIterContainer<'a>) {
    /*
    * need on_succes_ind, on_fail_len?
    * success/fail ind needs to be before or after? after for truncate?

    */

    let mut stk: Vec<(GrammarItem<'_>, usize,usize,PrimitiveIterContainer<'a>)>=vec![
        (grammar_decl("test7"),0,0,top_primitives)
    ];

    let mut c=0;
    while let Some((cur, success_len,fail_len, mut primitives))=stk.pop() {
        c+=1;

        // if c>55 {break;}
        // println!(": {cur:?} || {} && {primitives:?}", stk.iter().rev().map(|x|format!("{:?}",x.0)).collect::<Vec<_>>().join(" << "), );
        println!(": {cur:?}, ps={primitives:?}, success={success_len}, fail={fail_len}");

        for (i,(g,s,f,ps)) in stk.iter()
            // .rev()
            .enumerate() {
            // println!("\t{i:3}: {g:?}\n\t   : {ps:?}\n\t   : success={s}, fail={f}",);
            println!("\t{i:3}: {g:?}, ps={ps:?},success={s}, fail={f}",);
        }

        match cur {
            GrammarItem::And(gs) => {
                let Some(first)=gs.first().cloned() else {continue;};
                // let mut success_len2=None;

                if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
                    stk.push((GrammarItem::And(rest.into()),success_len,fail_len,primitives));
                }

                // let success_len2=success_len2.unwrap_or(success_len);

                let success_len=if gs.len()>1 {stk.len()}else{success_len};
                stk.push((first,success_len,fail_len,primitives));
            }
            GrammarItem::Or(gs) => {
                let Some(first)=gs.first().cloned() else {continue;};

                // let mut fail_len2=None;

                if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
                    stk.push((GrammarItem::Or(rest.into()),success_len,fail_len,primitives));
                }

                // let fail_len2 = fail_len2.unwrap_or(fail_len);

                let fail_len=if gs.len()>1 {stk.len()}else{fail_len};
                stk.push((first,success_len,fail_len,primitives));
            }

            GrammarItem::Opt(g) => {
                stk.push((GrammarItem::Always,success_len,0,primitives)); //fail is not used
                let fail_len=stk.len();
                stk.push((*g,success_len,fail_len,primitives));
            }
            GrammarItem::Many0(g) => {
                // let fail_len2=stk.len(); //only remove everything past here on fail
                stk.push((GrammarItem::Many0(g.clone()),success_len,fail_len,primitives));
                let success_len2=stk.len();
                stk.push((GrammarItem::Always,success_len,0,primitives)); //fail is not used
                let fail_len=stk.len();
                stk.push((*g,success_len2,fail_len,primitives));
            }
            GrammarItem::Many1(g) => {
                stk.push((GrammarItem::Many0(g.clone()),success_len,fail_len,primitives));
                let success_len=stk.len();
                stk.push((*g,success_len,fail_len,primitives));
            }
            GrammarItem::Always => {
                stk.truncate(success_len);

                if let Some((g,_,_,ps))=stk.last_mut() {
                    let is_many=match g {
                        GrammarItem::Many0(_)|GrammarItem::Many1(_) => true,
                        _ =>false,
                    };

                    if is_many {
                        *g=GrammarItem::Always;
                    }

                    *ps=primitives;
                } else {
                    top_primitives=primitives;
                }
            }
            GrammarItem::Never => {
                stk.truncate(fail_len);

                if stk.is_empty() {
                    top_primitives=primitives;
                }
            }
            GrammarItem::String => {
                match primitives.pop_string() {
                    Ok(v) => {
                        println!("--- string {:?}",v.value);
                        stk.truncate(success_len);

                        if let Some((_g,_a,_b,ps))=stk.last_mut() {
                            *ps=primitives;
                        }
                    }
                    Err(_loc) => {
                        stk.truncate(fail_len);
                    }
                }
                if stk.is_empty() {
                    top_primitives=primitives;
                }
            }
            GrammarItem::Identifier => {
                match primitives.pop_identifier() {
                    Ok(v) => {
                        println!("--- identifier {:?}",v.value);
                        stk.truncate(success_len);

                        if let Some((_g,_a,_b,ps))=stk.last_mut() {
                            *ps=primitives;
                        }
                    }
                    Err(_loc) => {
                        stk.truncate(fail_len);
                    }
                }
                if stk.is_empty() {
                    top_primitives=primitives;
                }
            }
            GrammarItem::Int => {
                match primitives.pop_int() {
                    Ok(v) => {
                        println!("--- int {:?}",v.value);
                        stk.truncate(success_len);
                        if let Some((_g,_a,_b,ps))=stk.last_mut() {
                            *ps=primitives;
                        }
                    }
                    Err(_loc) => {
                        stk.truncate(fail_len);
                    }
                }

                if stk.is_empty() {
                    top_primitives=primitives;
                }
            }
            GrammarItem::Float => {
                match primitives.pop_float() {
                    Ok(v) => {
                        println!("--- float {:?}",v.value);
                        stk.truncate(success_len);

                        if let Some((_g,_a,_b,ps))=stk.last_mut() {
                        //     println!("{_g:?}");
                            *ps=primitives;
                        }
                    }
                    Err(_loc) => {
                        stk.truncate(fail_len);
                    }
                }

                if stk.is_empty() {
                    top_primitives=primitives;
                }
            }
            GrammarItem::Symbol(s) => {
                match primitives.pop_with_symbols([s]) {
                    Ok(v) => {
                        println!("--- symbol {:?}",v.value);
                        stk.truncate(success_len);

                        if let Some((_g,_a,_b,ps))=stk.last_mut() {
                            *ps=primitives;
                        }
                    }
                    Err(_loc) => {
                        stk.truncate(fail_len);
                    }
                }
                if stk.is_empty() {
                    top_primitives=primitives;
                }
            }
            GrammarItem::Keyword(s) => {
                match primitives.pop_with_identifiers([s]) {
                    Ok(v) => {
                        println!("--- keyword {:?}",v.value);
                        stk.truncate(success_len);

                        if let Some((_g,_a,_b,ps))=stk.last_mut() {
                            *ps=primitives;
                        }
                    }
                    Err(_loc) => {
                        stk.truncate(fail_len);
                    }
                }
                if stk.is_empty() {
                    top_primitives=primitives;
                }
            }
            GrammarItem::Eol => {
                match primitives.pop_eol() {
                    Ok(_) => {
                        println!("eol");
                        stk.truncate(success_len);

                        if let Some((_g,_a,_b,ps))=stk.last_mut() {
                            *ps=primitives;
                        }
                    }
                    Err(_loc) => {
                        stk.truncate(fail_len);
                    }
                }
                if stk.is_empty() {
                    top_primitives=primitives;
                }
            }
            GrammarItem::NonTerm(t) => {
                stk.push((grammar_decl(t),success_len,fail_len,primitives));
            }
        }
    }

    //
    println!("top_primitives={top_primitives:?}",  );

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