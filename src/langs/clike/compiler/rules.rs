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
    fn to(self) -> GrammarNode<'g,GrammarPrimitive<'g>> {
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

pub fn get_non_term<'g>(n:& str) -> Option<Rc<GrammarNode<'g,GrammarPrimitive<'g>>>> {
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


    Some(Rc::new(match n {
        // "start" => [
        //     NonTerm("val").many0(),
        //     NonTerm("end"),
        // ].and(),

        "mynum" => Primitive(Int),
        "start" => [
            [NonTerm("mynum"),NonTerm("mynum")].and().group("a"),
            NonTerm("mynum").group("b"),
        ].or(),
        //
        "start2" => NonTerm("stmts"),

        "stmts" => [
            NonTerm("end").opt(),
            NonTerm("stmt"),
            [
                NonTerm("end"),
                NonTerm("stmt"),
            ].and().many0(),
            NonTerm("end").opt(),
        ].and().opt(),

        "stmt" => [
            NonTerm("var"),

            NonTerm("for"),
            NonTerm("while"),
            NonTerm("func"),

            NonTerm("break"),
            NonTerm("continue"),
            NonTerm("return"),

            NonTerm("include"),

            NonTerm("format"),
            NonTerm("print"),
            NonTerm("println"),

            NonTerm("set"),
            NonTerm("expr"),


            // NonTerm("val"),
        ].or() //.expect("stmt")
        ,

        "var" => [
            Keyword("var").to(),
            NonTerm("var_entry"),
            [ Symbol(",").to(), NonTerm("var_entry"), ].and().many0(),
            // Symbol(",").opt(),
        ].and(),

        "var_entry" => [
            Identifier.to().group("name"),
            [Symbol("=").to(), NonTerm("expr")].and().opt(),
        ].and().group("var"),

        "set" => [
            NonTerm("set_var"),
            NonTerm("set_field"),
            NonTerm("set_index"),
        ].or(),

        "set_var" => [
            Identifier.to().group("name"),
            NonTerm("set_op"),
            NonTerm("expr"),
        ].and().group("set_var"),

        "set_field" => [
            NonTerm("postfixes").had("field"),
            NonTerm("set_op"),
            NonTerm("expr"),
        ].and().group("set_field"),

        "set_index" => [
            NonTerm("postfixes").had("index"),
            NonTerm("set_op"),
            NonTerm("expr"),
        ].and().group("set_field"),

        "func" => [
            Keyword("fn").to(),
            Identifier.to().group("name"),
            NonTerm("func_params"),
            NonTerm("block"),
        ].and().group("func"),

        "lambda" => [
            Keyword("fn").to(),
            NonTerm("func_params"),
            NonTerm("block"),
        ].and().group("lambda"),

        "func_params" => [
            NonTerm("lparen"),
            [
                [
                    [ NonTerm("func_params2"), NonTerm("func_variadic"), ].and(),
                    [ NonTerm("func_params2"), NonTerm("func_not_variadic"), ].and(),
                    [ Always.group("params"), Always.group("not_variadic"), ].and(),
                ].or(),
                NonTerm("rparen"),
            ].and().expect("closing bracket"),
        ].and(),

        "func_params2" => [
            Identifier.to().group("param").expect("param"),
            [ Symbol(",").to(), Identifier.to().group("param").expect("param"), ].and().many0(),
        ].and().group("params"),

        "func_variadic" => [Symbol(".").to(),Symbol(".").to(),Symbol(".").to(),].and().group("variadic"),
        "func_not_variadic" => Symbol(",").to().opt().group("not_variadic"),

        // "ellipsis" => [Symbol("."),Symbol("."),Symbol("."),].and().group("ellipsis"),

        "format" => [Keyword("format").to(),NonTerm("format_params"),].and(),
        "print" => [Keyword("print").to(),NonTerm("format_params"),].and().group("print"),
        "println" => [Keyword("println").to(),NonTerm("format_params"),].and().group("println"),

        "format_params" => [
            NonTerm("lparen"),
            [
                [
                    [String.to().group("string"), NonTerm("expr"),].or(),
                    [Symbol(",").to(), NonTerm("expr"),].and().many0(),
                    Symbol(",").to().opt(),
                ].and().opt(),
                NonTerm("rparen"),
            ].and().expect("closing bracket"),
        ].and().group("format"),

        "if" => [
            [Keyword("if").to(), NonTerm("expr"), NonTerm("block"), ].and().group("cond"),
            [Keyword("elif").to(),NonTerm("expr"), NonTerm("block"), ].and().group("cond").many0(),
            [Keyword("else").to(),NonTerm("block"),].and().group("else").opt(),
        ].and().group("if"),

        "while" => [
            Keyword("while").to(),
            NonTerm("expr"),
            NonTerm("block"),
        ].and().group("while"),

        "for" => [
            Keyword("for").to(),
            Identifier.to().group("name"),
            Keyword("in").to(),
            NonTerm("expr"),
            NonTerm("for_op"),
            NonTerm("expr"),
            NonTerm("block"),
        ].and().group("for"),

        "continue" => Keyword("continue").to().group("continue"),
        "break" => Keyword("break").to().group("break"),
        "return" => [Keyword("return").to(), NonTerm("expr").opt(),].and().group("return"),

        "include" => [Keyword("include").to(),String.to().group("include"),].and(),

        "expr" => NonTerm("or").group("expr").expect("expr"),

        "or" => [
            [
                NonTerm("xor"),
                [
                    [Symbol("|").to(),Symbol("|").to().expect("or")].and(),
                    NonTerm("xor"),
                ].and().many1(),
            ].and().group("or"),
            NonTerm("xor"),
        ].or(),

        "xor" => [
            [
                NonTerm("and"),
                [ Symbol("^").to(), NonTerm("and"), ].and().many1(),
            ].and().group("xor"),
            NonTerm("and"),
        ].or(),

        "and" => [
            [
                NonTerm("compare"),
                [ Symbol("&").to(),Symbol("&").to().expect("and"), NonTerm("compare"), ].and().many1(),
            ].and().group("and"),
            NonTerm("compare"),
        ].or(),

        // "compare" => [
        //     [ NonTerm("factor"), NonTerm("compare_op"), NonTerm("factor"), ].and().group("compare"),
        //     NonTerm("factor"),
        // ].or(),

        "compare" => [
            [NonTerm("factor"), Symbol("<").to(), NonTerm("factor"),].and().group("lt"),
            [NonTerm("factor"), Symbol(">").to(), NonTerm("factor"),].and().group("gt"),
            [NonTerm("factor"), Symbol("<").to(),Symbol("=").to(), NonTerm("factor"),].and().group("le"),
            [NonTerm("factor"), Symbol(">").to(),Symbol("=").to(), NonTerm("factor"),].and().group("ge"),
            [NonTerm("factor"), Symbol("=").to(),Symbol("=").to().expect("eq"), NonTerm("factor"),].and().group("eq"),
            [NonTerm("factor"), Symbol("!").to(),Symbol("=").to().expect("ne"), NonTerm("factor"),].and().group("ne"),
            NonTerm("factor"),
        ].or(),

        "factor" => [
            [
                NonTerm("term"),
                [ NonTerm("factor_op"), NonTerm("term"), ].and().many1(),
            ].and().group("factor"),
            NonTerm("term"),
        ].or(),

        "term" => [
            [
                NonTerm("prefixes"),
                [NonTerm("term_op"),NonTerm("prefixes"),].and().many1(),
            ].and().group("term"),
            NonTerm("prefixes"),
        ].or(),

        // "factor" => [
        //     NonTerm("term"),
        //     [
        //         [ Symbol("+"), NonTerm("term"), ].and().group("add"),
        //         [ Symbol("-"), NonTerm("term"), ].and().group("sub"),
        //     ].or().many0()
        // ].and(),

        // "term" => [
        //     NonTerm("val"),
        //     [
        //         [ Symbol("*"), NonTerm("val"), ].and().group("mul"),
        //         [ Symbol("/"), NonTerm("val"), ].and().group("div"),
        //         [ Symbol("%"), NonTerm("val"), ].and().group("mod"),
        //     ].or().many0()
        // ].and(),

        "prefixes" => [
            [
                [
                    Symbol("+").to(),
                    Symbol("-").to().group("neg"),
                    Symbol("!").to().group("not"),
                ].or().many1().group("prefixes"),
                NonTerm("postfixes"),
            ].and().group("prefixes"),
            NonTerm("postfixes"),
        ].or(),

        "postfixes" => [
            [
                NonTerm("val"),
                NonTerm("field_index_call").many1().group("field_index_calls"),
            ].and().group("postfixes"),
            NonTerm("val"),
        ].or(),

        "val" => [
            [ Identifier.to().group("idn"), NonTerm("call"), ].and().group("call_func"),

            NonTerm("primitive"),
            NonTerm("array"),
            NonTerm("dict"),
            NonTerm("if"),
            NonTerm("lambda"),
            NonTerm("block"),
            [ NonTerm("lparen"), NonTerm("expr"), NonTerm("rparen"), ].and(),
        ].or(),

        "array" => [
            NonTerm("lsquare"),
            [
                [
                    NonTerm("expr"),
                    [Symbol(",").to(),NonTerm("expr"),].and().many0(),
                    Symbol(",").to().opt(),
                ].and().opt(),
                NonTerm("rsquare"),
            ].and().expect("closing square bracket"),
        ].and().group("array"),

        "dict" => [
            NonTerm("lcurly"),
            [
                [
                    NonTerm("dict_val"),
                    [Symbol(",").to(),NonTerm("dict_val"),].and().many0(),
                    Symbol(",").to().opt(),
                ].and().opt(),
                NonTerm("rcurly"),
            ].and().expect("closing brace"),
        ].and().group("dict"),

        "dict_val" => [
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
            [NonTerm("expr"),Error,].or(), //not needed unlike in val's field_index_calls, because that was optional, this is not
            // NonTerm("expr"),
            // [Int,Error,].or(),
            // Int,
        ].and().group("dict_val"),

        "block" => [
            NonTerm("lcurly").expect("block"),
            [ NonTerm("stmts"), NonTerm("rcurly"), ].and().expect("closing brace"),
        ].and().group("block"),

        "field_index_call" => [
            [NonTerm("field"),NonTerm("call"),].and().group("call_field"),
            NonTerm("field").group("field"),
            NonTerm("index").group("index"),
            NonTerm("call").group("call_val"),
        ].or(),

        "call" => [
            NonTerm("lparen"),
            [
                [
                    NonTerm("expr"),
                    [ Symbol(",").to(), NonTerm("expr"), ].and().many0(),
                    Symbol(",").to().opt(),
                ].and().opt(),
                NonTerm("rparen"),
            ].and().expect("closing bracket"),
        ].and().group("params"),

        "field" => [
            Symbol(".").to(),
            [
                Int.to().group("field_index"),
                Identifier.to().group("field_name"),
                // Error,
            ].or().expect("field"),
        ].and().was("field"),

        "index" => [
            NonTerm("lsquare"),
            NonTerm("expr"),
            NonTerm("rsquare").expect("closing square bracket"),
        ].and().was("index"),

        "primitive" => [
            Int.to(),
            Float.to(),
            String.to(),
            Identifier.to(),
            Keyword("nil").to(),
            Keyword("void").to(),
            Keyword("true").to(),
            Keyword("false").to(),
        ].or().group("primitive"),

        // "bool" => [
        //     Keyword("true").group("true"),
        //     Keyword("false").group("false"),
        // ].or(),

        // "nil" => Keyword("nil").group("nil"),
        // "void" => Keyword("void").group("void"),

        "end" => [Symbol(";").to(),Eol.to()].or().many1().expect("semicolon"),

        "for_op" => [
            [NonTerm("for_to_op"),Symbol("=").to().opt(),].and().group("to_eq"),
            NonTerm("for_to_op").group("to"),
        ].or(),

        "for_to_op" => [Symbol(".").to(),Symbol(".").to(),].and(),

        // "var_set_op" => Symbol("="),

        "set_op" => [
            Symbol("=").to().group("eq"),
            [ NonTerm("set_sub_op"), Symbol("=").to(), ].and(),
        ].or(),

        "set_sub_op" => [
            Symbol("+").to().group("add"),
            Symbol("-").to().group("sub"),
            Symbol("*").to().group("mul"),
            Symbol("/").to().group("div"),
            Symbol("!").to().group("not"),

            [Symbol("&").to(),Symbol("&").to(),].and().group("and"),
            [Symbol("|").to(),Symbol("|").to(),].and().group("or"),

            Symbol("^").to().group("xor"),
        ].or(),

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

        "factor_op" => [
            Symbol("+").to().group("add"),
            Symbol("-").to().group("sub"),
        ].or(),

        "term_op" => [
            Symbol("*").to().group("mul"),
            Symbol("/").to().group("div"),
            Symbol("%").to().group("mod"),
        ].or(),

        "lcurly" => Symbol("{").to(),
        "rcurly" => Symbol("}").to(),
        "lsquare" => Symbol("[").to(),
        "rsquare" => Symbol("]").to(),
        "lparen" => Symbol("(").to(),
        "rparen" => Symbol(")").to(),

        _ => {return None;}
    }))
}




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