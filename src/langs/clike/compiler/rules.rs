use std::{fmt::Display, rc::Rc};
use crate::clike::{grammar::{container::WalkGroupContainer, GrammarPrimitiveTrait}, tokenizer::{TokenContainer, TokenIterContainer}};

/*
TODO
* don't allow field_ind (eg a.5) to be used for int/float/bool
* don't allow spaces between the decimal point and numbers in a float?
** allow floats like .5 ?
*/
use super::super::grammar::node::*;

#[derive(Clone,Hash,PartialEq,Eq,Debug)]
pub enum GrammarPrimitive<'g> {
    String,
    Identifier,
    Int,
    Float,
    Symbol(&'g str),
    Keyword(&'g str),
    Eol,
}


impl<'g> GrammarPrimitive<'g> {
    fn to<NT>(self) -> GrammarNode<'g,NT,GrammarPrimitive<'g>> {
        GrammarNode::Primitive(self)
    }
}

impl<'g> GrammarPrimitiveTrait for GrammarPrimitive<'g> {
    fn is_trimmable(&self) -> bool {
        match self {
            GrammarPrimitive::Eol => true,
            _ => false,
        }
    }
}


// impl<'g> Into<GrammarNode<'g,GrammarPrimitive<'g>>> for GrammarPrimitive<'g> {
//     fn into(self) -> GrammarNode<'g,GrammarPrimitive<'g>> {
//         GrammarNode::Primitive(self)
//     }
// }

pub fn is_keyword(n:& str) -> bool {
    match n {
        "for"|"in"| //"to"|
        "while"|"continue"|"break"|
        "goto"|"label"|
        "include"|
        "true"|"false"|"nil"|"void"|
        "print"|"println"|"format"|
        "var"|"fn"|"return"|
        "if"|"elif"|"else"
        // |"a"|"b"|"c"|"d"
        => true,
        _=>false,
    }
}

#[derive(Debug,Clone,Copy,Hash,PartialEq, Eq)]
pub enum MyNonTerm {
    Start,
    Stmts,
    Stmt,
    Var,
    VarEntry,
    Set,
    SetVar,
    SetField,
    SetIndex,
    Func,
    Lambda,
    FuncParams,
    FuncParams2,
    FuncVariadic,
    FuncNotVariadic,
    Format,
    Print,
    Println,
    FormatParams,
    If,
    While,
    For,
    Continue,
    Break,
    Return,
    Include,
    Expr,
    Or1,
    Xor,
    And1,
    Compare,
    Factor,
    Term,
    Prefixes,
    Postfixes,
    Val,
    Array,
    Dict,
    DictVal,
    Block,
    FieldIndexCall,
    Call,
    Field,
    Index,
    Primitive1,
    End,
    ForOp,
    ForToOp,
    SetOp,
    SetSubOp,
    FactorOp,
    TermOp,
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    LParen,
    RParen,
}

pub fn get_non_term<'g>(n:MyNonTerm) -> Option<Rc<GrammarNode<'g,MyNonTerm,GrammarPrimitive<'g>>>> {
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
    use GrammarPrimitive::*;
    use GrammarNode::*;
    use MyNonTerm::*;


    Some(Rc::new(match n {
        // "start" => [
        //     NonTerm(Val).many0(),
        //     NonTerm("end"),
        // ].and(),

        // "mynum" => Primitive(Int).group("n"),
        // "start2" => [
        //     [NonTerm("mynum"),NonTerm("mynum")].and().group("a"),
        //     NonTerm("mynum").group("b").group("b"),
        // ].or(),
        //
        Start => NonTerm(Stmts),

        Stmts => [
            NonTerm(End).opt(),
            NonTerm(Stmt),
            [
                NonTerm(End),
                NonTerm(Stmt),
            ].and().many0(),
            NonTerm(End).opt(),
        ].and().opt(),

        Stmt => [
            NonTerm(Var),

            NonTerm(For),
            NonTerm(While),
            NonTerm(Func),

            NonTerm(Break),
            NonTerm(Continue),
            NonTerm(Return),

            NonTerm(Include),

            NonTerm(Format),
            NonTerm(Print),
            NonTerm(Println),

            NonTerm(Set),
            NonTerm(Expr),


            // NonTerm(Val),
        ].or() //.expect("stmt")
        ,

        Var => [
            Keyword("var").to(),
            NonTerm(VarEntry),
            [ Symbol(",").to(), NonTerm(VarEntry), ].and().many0(),
            // Symbol(",").opt(),
        ].and(),

        VarEntry => [
            Identifier.to().group("name"),
            [Symbol("=").to(), NonTerm(Expr)].and().opt(),
        ].and().group("var"),

        Set => [
            NonTerm(SetVar),
            NonTerm(SetField),
            NonTerm(SetIndex),
        ].or(),

        SetVar => [
            Identifier.to().group("name"),
            NonTerm(SetOp),
            NonTerm(Expr),
        ].and().group("set_var"),

        SetField => [
            NonTerm(Postfixes).had("field"),
            NonTerm(SetOp),
            NonTerm(Expr),
        ].and().group("set_field"),

        SetIndex => [
            NonTerm(Postfixes).had("index"),
            NonTerm(SetOp),
            NonTerm(Expr),
        ].and().group("set_field"),

        Func => [
            Keyword("fn").to(),
            Identifier.to().group("name"),
            NonTerm(FuncParams),
            NonTerm(Block),
        ].and().group("func"),

        Lambda => [
            Keyword("fn").to(),
            NonTerm(FuncParams),
            NonTerm(Block),
        ].and().group("lambda"),

        FuncParams => [
            NonTerm(LParen),
            [
                [
                    [ NonTerm(FuncParams2), NonTerm(FuncVariadic), ].and(),
                    [ NonTerm(FuncParams2), NonTerm(FuncNotVariadic), ].and(),
                    [ Always.group("params"), Always.group("not_variadic"), ].and(),
                ].or(),
                NonTerm(RParen),
            ].and().expect("closing bracket"),
        ].and(),

        FuncParams2 => [
            Identifier.to().group("param").expect("param"),
            [ Symbol(",").to(), Identifier.to().group("param").expect("param"), ].and().many0(),
        ].and().group("params"),

        FuncVariadic => [Symbol(".").to(),Symbol(".").to(),Symbol(".").to(),].and().group("variadic"),
        FuncNotVariadic => Symbol(",").to().opt().group("not_variadic"),

        // "ellipsis" => [Symbol("."),Symbol("."),Symbol("."),].and().group("ellipsis"),

        Format => [Keyword("format").to(),NonTerm(FormatParams),].and(),
        Print => [Keyword("print").to(),NonTerm(FormatParams),].and().group("print"),
        Println => [Keyword("println").to(),NonTerm(FormatParams),].and().group("println"),

        FormatParams => [
            NonTerm(LParen),
            [
                [
                    [String.to().group("string"), NonTerm(Expr),].or(),
                    [Symbol(",").to(), NonTerm(Expr),].and().many0(),
                    Symbol(",").to().opt(),
                ].and().opt(),
                NonTerm(RParen),
            ].and().expect("closing bracket"),
        ].and().group("format"),

        If => [
            [Keyword("if").to(), NonTerm(Expr), NonTerm(Block), ].and().group("cond"),
            [Keyword("elif").to(),NonTerm(Expr), NonTerm(Block), ].and().group("cond").many0(),
            [Keyword("else").to(),NonTerm(Block),].and().group("else").opt(),
        ].and().group("if"),

        While => [
            Keyword("while").to(),
            NonTerm(Expr),
            NonTerm(Block),
        ].and().group("while"),

        For => [
            Keyword("for").to(),
            Identifier.to().group("name"),
            Keyword("in").to(),
            NonTerm(Expr),
            NonTerm(ForOp),
            NonTerm(Expr),
            NonTerm(Block),
        ].and().group("for"),

        Continue => Keyword("continue").to().group("continue"),
        Break => Keyword("break").to().group("break"),
        Return => [Keyword("return").to(), NonTerm(Expr).opt(),].and().group("return"),

        Include => [Keyword("include").to(),String.to().group("include"),].and(),

        Expr => NonTerm(Or1).group("expr").expect("expr"),
        // "expr" => NonTerm(Prefixes).group("expr").expect("expr"),

        Or1 => [
            [
                NonTerm(Xor),
                [
                    [Symbol("|").to(),Symbol("|").to().expect("or")].and(),
                    NonTerm(Xor),
                ].and().many1(),
            ].and().group("or"),
            NonTerm(Xor),
        ].or(),

        Xor => [
            [
                NonTerm(And1),
                [ Symbol("^").to(), NonTerm(And1), ].and().many1(),
            ].and().group("xor"),
            NonTerm(And1),
        ].or(),

        And1 => [
            [
                NonTerm(Compare),
                [ Symbol("&").to(),Symbol("&").to().expect("and"), NonTerm(Compare), ].and().many1(),
            ].and().group("and"),
            NonTerm(Compare),
        ].or(),

        // "compare" => [
        //     [ NonTerm(Factor), NonTerm("compare_op"), NonTerm(Factor), ].and().group("compare"),
        //     NonTerm(Factor),
        // ].or(),

        Compare => [
            [NonTerm(Factor), Symbol("<").to(), NonTerm(Factor),].and().group("lt"),
            [NonTerm(Factor), Symbol(">").to(), NonTerm(Factor),].and().group("gt"),
            [NonTerm(Factor), Symbol("<").to(),Symbol("=").to(), NonTerm(Factor),].and().group("le"),
            [NonTerm(Factor), Symbol(">").to(),Symbol("=").to(), NonTerm(Factor),].and().group("ge"),
            [NonTerm(Factor), Symbol("=").to(),Symbol("=").to().expect("eq"), NonTerm(Factor),].and().group("eq"),
            [NonTerm(Factor), Symbol("!").to(),Symbol("=").to().expect("ne"), NonTerm(Factor),].and().group("ne"),
            NonTerm(Factor),
        ].or(),

        Factor => [
            [
                NonTerm(Term),
                [ NonTerm(FactorOp), NonTerm(Term), ].and().many1(),
            ].and().group("factor"),
            NonTerm(Term),
        ].or(),

        Term => [
            [
                NonTerm(Prefixes),
                [NonTerm(TermOp),NonTerm(Prefixes),].and().many1(),
            ].and().group("term"),
            NonTerm(Prefixes),
        ].or(),


        Prefixes => [
            [
                [
                    Symbol("+").to(),
                    Symbol("-").to().group("neg"),
                    Symbol("!").to().group("not"),
                ].or().many1().group("prefixes"),
                NonTerm(Postfixes),
            ].and().group("prefixes"),
            NonTerm(Postfixes),
        ].or(),

        Postfixes => [
            [
                NonTerm(Val),
                NonTerm(FieldIndexCall).many1().group("field_index_calls"),
            ].and().group("postfixes"),
            NonTerm(Val),
        ].or(),

        Val => [
            [ Identifier.to().group("idn"), NonTerm(Call), ].and().group("call_func"),

            NonTerm(Primitive1),
            NonTerm(Array),
            NonTerm(Dict),
            NonTerm(If),
            NonTerm(Lambda),
            NonTerm(Block),
            [ NonTerm(LParen), NonTerm(Expr), NonTerm(RParen), ].and(),
        ].or(),

        Array => [
            NonTerm(LSquare),
            [
                [
                    NonTerm(Expr),
                    [Symbol(",").to(),NonTerm(Expr),].and().many0(),
                    Symbol(",").to().opt(),
                ].and().opt(),
                NonTerm(RSquare),
            ].and().expect("closing square bracket"),
        ].and().group("array"),

        Dict => [
            NonTerm(LCurly),
            [
                [
                    NonTerm(DictVal),
                    [Symbol(",").to(),NonTerm(DictVal),].and().many0(),
                    Symbol(",").to().opt(),
                ].and().opt(),
                NonTerm(RCurly),
            ].and().expect("closing brace"),
        ].and().group("dict"),

        DictVal => [
            [
                Identifier.to().group("name"),

                [
                    Int.to(), String.to(),
                    Keyword("nil").to(),
                    Keyword("true").to(),
                    Keyword("false").to(),
                ].or().group("primitive"),

            ].or().expect("key"),
            Symbol(":").to().expect("colon"),
            [NonTerm(Expr),Error,].or(), //not needed unlike in val's field_index_calls, because that was optional, this is not
            // NonTerm(Expr),
            // [Int,Error,].or(),
            // Int,
        ].and().group("dict_val"),

        Block => [
            NonTerm(LCurly).expect("block"),
            [ NonTerm(Stmts), NonTerm(RCurly), ].and().expect("closing brace"),
        ].and().group("block"),

        FieldIndexCall => [
            [NonTerm(Field),NonTerm(Call),].and().group("call_field"),
            NonTerm(Field).group("field"),
            NonTerm(Index).group("index"),
            NonTerm(Call).group("call_val"),
        ].or(),

        Call => [
            NonTerm(LParen),
            [
                [
                    NonTerm(Expr),
                    [ Symbol(",").to(), NonTerm(Expr), ].and().many0(),
                    Symbol(",").to().opt(),
                ].and().opt(),
                NonTerm(RParen),
            ].and().expect("closing bracket"),
        ].and().group("params"),

        Field => [
            Symbol(".").to(),
            [
                Int.to().group("field_index"),
                Identifier.to().group("field_name"),
                // Error,
            ].or().expect("field"),
        ].and().was("field"),

        Index => [
            NonTerm(LSquare),
            NonTerm(Expr),
            NonTerm(RSquare).expect("closing square bracket"),
        ].and().was("index"),

        Primitive1 => [
            Int.to(),
            Float.to(),
            String.to(),
            Identifier.to(),
            Keyword("nil").to(),
            Keyword("void").to(),
            Keyword("true").to(),
            Keyword("false").to(),
        ].or().group("primitive"),

        End => [Symbol(";").to(),Eol.to()].or().many1().expect("semicolon"),

        ForOp => [
            [NonTerm(ForToOp),Symbol("=").to().opt(),].and().group("to_eq"),
            NonTerm(ForToOp).group("to"),
        ].or(),

        ForToOp => [Symbol(".").to(),Symbol(".").to(),].and(),

        // "var_set_op" => Symbol("="),

        SetOp => [
            Symbol("=").to().group("eq"),
            [ NonTerm(SetSubOp), Symbol("=").to(), ].and(),
        ].or(),

        SetSubOp => [
            Symbol("+").to().group("add"),
            Symbol("-").to().group("sub"),
            Symbol("*").to().group("mul"),
            Symbol("/").to().group("div"),
            Symbol("!").to().group("not"),

            [Symbol("&").to(),Symbol("&").to(),].and().group("and"),
            [Symbol("|").to(),Symbol("|").to(),].and().group("or"),

            Symbol("^").to().group("xor"),
        ].or(),


        FactorOp => [
            Symbol("+").to().group("add"),
            Symbol("-").to().group("sub"),
        ].or(),

        TermOp => [
            Symbol("*").to().group("mul"),
            Symbol("/").to().group("div"),
            Symbol("%").to().group("mod"),
        ].or(),

        LCurly => Symbol("{").to(),
        RCurly => Symbol("}").to(),
        LSquare => Symbol("[").to(),
        RSquare => Symbol("]").to(),
        LParen => Symbol("(").to(),
        RParen => Symbol(")").to(),

        _ => {return None;}
    }))
}


        // "bool" => [
        //     Keyword("true").group("true"),
        //     Keyword("false").group("false"),
        // ].or(),

        // "nil" => Keyword("nil").group("nil"),
        // "void" => Keyword("void").group("void"),


        // "factor" => [
        //     NonTerm(Term),
        //     [
        //         [ Symbol("+"), NonTerm(Term), ].and().group("add"),
        //         [ Symbol("-"), NonTerm(Term), ].and().group("sub"),
        //     ].or().many0()
        // ].and(),

        // "term" => [
        //     NonTerm(Val),
        //     [
        //         [ Symbol("*"), NonTerm(Val), ].and().group("mul"),
        //         [ Symbol("/"), NonTerm(Val), ].and().group("div"),
        //         [ Symbol("%"), NonTerm(Val), ].and().group("mod"),
        //     ].or().many0()
        // ].and(),

        // "prefix_op" => [
        //     Symbol("+"),
        //     Symbol("-").group("neg"),
        //     Symbol("!").group("not"),
        // ].or(),

        // "xor_op" => Symbol("^"),
        // "and_op" => [Symbol("&"),Symbol("&"),].and(),
        // "or_op" => [Symbol("|"),Symbol("|"),].and(),

        // "compare_op" => [
        //     Symbol("<").group("lt"),
        //     Symbol(">").group("gt"),
        //     [Symbol("<"),Symbol("=")].and().group("le"),
        //     [Symbol(">"),Symbol("=")].and().group("ge"),
        //     [Symbol("="),Symbol("=")].and().group("eq"),
        //     [Symbol("!"),Symbol("=")].and().group("ne"),
        // ].or(),


impl<'t,'g> std::fmt::Debug for WalkGroupContainer<'g,TokenIterContainer<'t>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::{:?}", &self.group_ind,&self.name()))
    }
}

impl<'t,'g,> Display for WalkGroupContainer<'g,TokenIterContainer<'t>>

{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        enum Thing<'t,'g,> {
            Token(TokenContainer<'t>),
            Group(WalkGroupContainer<'g,TokenIterContainer<'t>>),
        }

        let mut stk = vec![(Thing::Group(*self),0)];

        while let Some((cur,depth))=stk.pop() {
            let indent="    ".repeat(depth);

            match cur {
                Thing::Group(cur) => {
                    writeln!(f,"{indent}group: {:?}",cur.name(),)?;
                    let mut cur_tokens = cur.tokens();

                    for child_group in cur.children().rev() {
                        let child_tokens=child_group.tokens();
                        let ps_len=cur_tokens.end-child_tokens.end;
                        let ps=cur_tokens.pop_back_amount(ps_len).unwrap();

                        stk.extend(ps.map(|t|(Thing::Token(t),depth+1)).rev());
                        stk.push((Thing::Group(child_group),depth+1));
                        cur_tokens.pop_back_amount(child_tokens.len()).unwrap();
                    }

                    //
                    stk.extend(cur_tokens.map(|t|(Thing::Token(t),depth+1)).rev());
                }
                Thing::Token(cur) => {
                    writeln!(f,"{indent}{cur:?}")?;
                }
            }
        }

        Ok(())
    }
}