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
    End,

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

// impl<T: Default, const N: usize> Default for [T; N] {
//     fn default() -> [T; N] {
//         [T::default; N]
//     }
// }
impl<'a, const N: usize> From<[GrammarItem<'a>; N]> for  GrammarItem<'a> {
    fn from(value: [GrammarItem<'a>; N]) -> Self {
        Self::And(value.into())
    }
}

// impl<T> Default for [T; 0] {
//     fn default() -> [T; 0] {
//         []
//     }
// }
pub fn grammar_decl<'a>(n:&str) -> GrammarItem<'a> {
    use GrammarItem::*;
    match n {
        "start" => NonTerm("stmts"),

        "stmts" => And([NonTerm("stmt"), NonTerm("end"),].into()).many0(),
        "loop_stmts" => And([NonTerm("loop_stmt"), NonTerm("end"),].into()).many0(),
        "func_stmts" => And([NonTerm("func_stmt"), NonTerm("end"),].into()).many0(),

        "loop_stmt" => Or([NonTerm("stmt"), NonTerm("break"), NonTerm("continue"),].into()),
        "func_stmt" => Or([NonTerm("stmt"), NonTerm("return"),].into()),

        "stmt" => Or([
            NonTerm("expr"),NonTerm("var"),NonTerm("set"),NonTerm("while"),NonTerm("for"),
            NonTerm("include"),NonTerm("format"),NonTerm("print"),NonTerm("println"),
            NonTerm("if"),NonTerm("call"),
        ].into()),

        "continue" => Keyword("continue"),
        "break" => Keyword("break"),
        "return" => And([Keyword("return"), NonTerm("expr").opt(),].into()),

        "var_set_body" => And([NonTerm("idn"), NonTerm("eq"),NonTerm("expr")].into()),
        "var" => And([
            Keyword("var"), NonTerm("var_set_body"),
            And([NonTerm("comma"),NonTerm("var_set_body"),].into()).many0(),
        ].into()),

        "set" => And([
            NonTerm("idn"),
            Or([NonTerm("add"),NonTerm("sub"),NonTerm("mul"),NonTerm("div"),NonTerm("not")].into()),
            NonTerm("eq"),
            NonTerm("expr"),
        ].into()),

        "cond" => And([NonTerm("lparenth"),NonTerm("expr"),NonTerm("rparenth"),].into()),
        "block" => And([NonTerm("lcurly"),NonTerm("stmts"),NonTerm("rcurly"),].into()),
        "if" => And([
            Keyword("if"), NonTerm("cond"), NonTerm("block"),
            And([Keyword("elif"),NonTerm("cond"),NonTerm("block"),].into()).many0(),
            And([Keyword("else"),NonTerm("block"),].into()).opt(),
        ].into()),
        "while" => And([Keyword("while"), NonTerm("cond"), NonTerm("block"),].into()),
        "for_init" => Or([
            NonTerm("var"),
            And([NonTerm("set"), And([NonTerm("comma"),NonTerm("set")].into()).many0(),].into()),
        ].into()),
        "for_incr_stmt" => Or([NonTerm("set"),NonTerm("call"),].into()),
        "for_incr" => And([
            NonTerm("for_incr_stmt"),
            And([NonTerm("comma"),NonTerm("for_incr_stmt")].into()).many0(),
        ].into()),
        "for" => And([
            Keyword("for"),
            NonTerm("lparenth"),
            NonTerm("for_init"),
            NonTerm("semicolon"),
            NonTerm("expr"),
            NonTerm("semicolon"),
            NonTerm("for_incr"),
            NonTerm("rparenth"),
            NonTerm("block"),
        ].into()),
        "call_params" => And([
            NonTerm("lparenth"),
            And([ NonTerm("expr"), And([NonTerm("comma"),NonTerm("expr"),].into()).many0(), ].into()).opt(),
            NonTerm("rparenth"),
        ].into()),
        "call" => And([NonTerm("idn"),NonTerm("call_params"),].into()),
        "include" => And([Keyword("include"),String,].into()),

        "func_params" => And([
            NonTerm("lparenth"),
            And([NonTerm("idn"), And([NonTerm("comma"),NonTerm("idn"),].into()).many0(),].into()).opt(),
            NonTerm("ellipsis").opt(),
            NonTerm("comma").opt(),
            NonTerm("rparenth"),
        ].into()),
        "func_decl" => And([Keyword("fn"),Identifier,NonTerm("func_params"),NonTerm("lcurly"),NonTerm("func_stmts"),NonTerm("rcurly"),].into()),
        "func_lambda" => And([Keyword("fn"),NonTerm("func_params"),NonTerm("lcurly"),NonTerm("func_stmts"),NonTerm("rcurly"),].into()),

        "infix" => Or([
            Symbol("+"),Symbol("-"),Symbol("*"),Symbol("/"),
            Symbol(">"),Symbol("<"),
            And([Symbol("<"),Symbol("="),].into()),
            And([Symbol(">"),Symbol("="),].into()),
            And([Symbol("="),Symbol("="),].into()),
            And([Symbol("!"),Symbol("="),].into()),
            And([Symbol("&"),Symbol("&"),].into()),
            And([Symbol("|"),Symbol("|"),].into()),
        ].into()),
        "expr" => And([NonTerm("val"), And([NonTerm("infix"),NonTerm("val"),].into()).many0(),].into()),

        "prefix" => Or([Symbol("+"),Symbol("-"),Symbol("!"),].into()),

        "val" => And([
            NonTerm("prefix").many0(),
            Or([
                Int,Float,String,Identifier,NonTerm("call"),
                And([Symbol("lparenth"),NonTerm("expr"),Symbol("rparenth"),].into()),
            ].into()),
        ].into()),
        _ => Never,
    }

}
pub fn grammar_run() {

}

/*
    start => stmts
    stmts => (stmt end)*
    loop_stmts => ((stmt | 'break' | 'continue') end)*
    func_stmts => ((stmt | return) end)*
    stmt => expr | var | if | set | while | for | call | include | format | print | println
    return => 'return' expr?
    var => 'var' idn '=' expr ' (comma idn '=' expr)*
    set => idn ((add | sub | mul | div | and | or | not )? '=') expr
    if => 'if' lparenth expr rparenth lcurly stmts rcurly ('elif' lparenth expr rparenth lcurly stmts rcurly)* (else lcurly stmts rcurly)?
    while => 'while' lparenth expr rparenth lcurly loop_stmts rcurly
    for => 'for' lparenth (var|set)? semicolon expr semicolon (var|set)? rparenth lcurly loop_stmts rcurly
    call_params => lparenth (expr | (comma expr)*)? rparenth
    call => idn call_params
    include => 'include' str

    func_params => lparenth (idn (comma idn)*)? comma? rparenth
    func_decl => 'fn' idn func_params lcurly func_stmts rcurly
    func_lambda => 'fn' func_params lcurly func_stmts rcurly

    expr => val (infix val)*

    infix => add | sub | mul | div | and | or | lt | le | gt | ge | eq | ne
    prefix => add | sub | not

    val => prefix* (int | float | string | idn | call | (lparen expr rparen))

    var_idn => idn (var_ind|var_field)*
    var_field => dot (idn|int) call_params?
    var_ind => '[' expr ']' call_params?

    format_params => lparenth (string | expr)? (comma expr)* comma? rparenth
    format => 'format' format_params
    print => 'print' format_params
    println => 'println' format_params

    end = semicolon | eol | eob

    set => '='
    add_set '+' '='
    eq => '=' '='
    ne => '!' '='
    gt => '>'
    lt => '<'
    ge => '>' '='
    le => '<' '='


    add => '+'
    sub => '-'
    mul => '*'
    div => '/'
    not => '!'
    and => '&' '&'
    or => '|' '|'
    comma => ','
    dot => '.'
    semicolon => ';'

    lsquare => '['
    rsquare => ']'
    lparenth => '('
    rparenth => ')'
    lcurly => '{'
    rcurly => '}'
    quote => '"'

====

ManyStar(GrammarItem)
ManyPlus(GrammarItem)
And(Vec<GrammarItem>) //stored reversed
Or(Vec<GrammarItem>) //stored reversed
Opt(GrammarItem)

String
Symbol
Identifier
Int,
Float,
End,

NonTerm(str)
Always, //always succeeds
===

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