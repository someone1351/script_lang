/*
TODO
* add Group(group_name,grammar_item), for output
** or instead add as method and(expr,expr).group("abc")

* output
** if something like and(expr, expr, expr (or stmt)) => [expr,expr,stmt]
** if something like and(group("abc",and(expr, expr)), expr (or stmt)) => [abc[expr,expr],stmt]
** if group not used, all output would be one single list of primitives
*/

#[derive(Clone)]
pub enum GrammarItem<'a> {
    Many0(Box<GrammarItem<'a>>),
    Many1(Box<GrammarItem<'a>>),
    And(Vec<GrammarItem<'a>>), //stored reversed
    Or(Vec<GrammarItem<'a>>), //stored reversed
    Opt(Box<GrammarItem<'a>>),

    List(Box<GrammarItem<'a>>,Box<GrammarItem<'a>>), //val,sep
    ListNoTrail(Box<GrammarItem<'a>>,Box<GrammarItem<'a>>), //val,sep

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
        "start" => NonTerm("stmts"),

        "stmts" => [
            NonTerm("stmt"),
            [[NonTerm("semicolon"),Eol].or(), NonTerm("stmt"),].and().many0(),
        ].and().opt(),

        "stmt" => [
            NonTerm("expr"),NonTerm("var"),NonTerm("set"),NonTerm("while"),NonTerm("for"),
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

        "cond" => [NonTerm("lparenth"),NonTerm("expr"),NonTerm("rparenth"),].and(),
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
            NonTerm("lparenth"),
            NonTerm("for_init"),
            NonTerm("semicolon"),
            NonTerm("expr"),
            NonTerm("semicolon"),
            NonTerm("for_incr"),
            NonTerm("rparenth"),
            NonTerm("block"),
        ].and(),
        "call_params" => [
            NonTerm("lparenth"),
            [
                NonTerm("expr"),
                [NonTerm("comma"),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparenth"),
        ].and(),
        "call" => [NonTerm("idn"),NonTerm("call_params"),].and(),
        "include" => [Keyword("include"),String,].and(),

        "func_params" => [
            NonTerm("lparenth"),
            [
                NonTerm("idn"),
                [NonTerm("comma"),NonTerm("idn"),].and().many0(),
                NonTerm("ellipsis").opt(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparenth"),
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
        "expr" => [NonTerm("val"), [NonTerm("infix"),NonTerm("val"),].and().many0(),].and(),

        "prefix" => [NonTerm("add"),NonTerm("sub"),NonTerm("not"),].or(),

        "val" => [
            NonTerm("prefix").many0(),
            [
                Int,Float,String,Identifier,NonTerm("idn"),
                NonTerm("call"),
                NonTerm("if"),
                [NonTerm("lparenth"),NonTerm("expr"),NonTerm("rparenth"),].and(),
            ].or(),
        ].and(),
        "idn_field" => [NonTerm("dot"),[Identifier,Int,].or(),].and(),
        "idn_index" => [NonTerm("lsquare"),NonTerm("expr"),NonTerm("rsquare"),].and(),
        "idn" => [
            Identifier,
            [NonTerm("idn_index"), NonTerm("idn_field"),].or().many0(),
        ].and(),
        "format_params" => [
            NonTerm("lparenth"),
            [
                [String,NonTerm("expr"),].or(),
                [NonTerm("comma"),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparenth"),
        ].and(),

        "format" => [Keyword("format"),NonTerm("format_params"),].and(),
        "print" => [Keyword("print"),NonTerm("format_params"),].and(),
        "println" => [Keyword("println"),NonTerm("format_params"),].and(),

        "lcurly" => Symbol("{"),
        "rcurly" => Symbol("}"),
        "lsquare" => Symbol("["),
        "rsquare" => Symbol("]"),
        "lparenth" => Symbol("("),
        "rparenth" => Symbol(")"),

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
pub fn grammar_run() {

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