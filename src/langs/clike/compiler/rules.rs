use std::rc::Rc;
/*
TODO
* don't allow field_ind (eg a.5) to be used for int/float/bool
* don't allow spaces between the decimal point and numbers in a float?
** allow floats like .5 ?
*/
use super::super::grammar::node::*;

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
        |"a"|"b"|"c"|"d"
        => true,
        _=>false,
    }
}

pub fn get_non_term<'a>(n:& str) -> Option<Rc<GrammarNode<'a>>> {
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
    use GrammarNode::*;

    Some(Rc::new(match n {

        "start" => NonTerm("stmts"),

        "stmts" => [
            NonTerm("end").many0(),
            [
                NonTerm("stmt"),
                [ NonTerm("end").many1(), NonTerm("stmt"), ].and().many0(),
            ].and().opt(),
            NonTerm("end").many0(),
        ].and(),

        "stmt" => [
            NonTerm("expr")
        ].or(),

        "expr" => NonTerm("or").group("expr").expect("expr"),

        "or" => [
            [
                NonTerm("xor"),
                [ NonTerm("or_op"), NonTerm("xor"), ].and().many1(),
            ].and().group("or"),
            NonTerm("xor"),
        ].or(),

        "xor" => [
            [
                NonTerm("and"),
                [ NonTerm("xor_op"), NonTerm("and"), ].and().many1(),
            ].and().group("xor"),
            NonTerm("and"),
        ].or(),

        "and" => [
            [
                NonTerm("compare"),
                [ NonTerm("and_op"), NonTerm("compare"), ].and().many1(),
            ].and().group("and"),
            NonTerm("compare"),
        ].or(),

        "compare" => [
            [
                NonTerm("factor"),
                NonTerm("compare_op"),
                NonTerm("factor"),
            ].and().group("compare"),
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
                NonTerm("val"),
                [NonTerm("term_op"),NonTerm("val"),].and().many1(),
            ].and().group("term"),
            NonTerm("val"),
        ].or(),


        "val" => [
            NonTerm("prefix_op").many0().group("prefixes"),
            [
                NonTerm("primitive"),
                NonTerm("bool"),
                NonTerm("nil"),
                NonTerm("void"),
            ].or(),
        ].and().group("val"),

        "primitive" => [
            Int,
            Float,
            String,
            Identifier.group("idn"),
        ].or().group("prim"),

        "bool" => [
            Keyword("true").group("true"),
            Keyword("false").group("false"),
        ].or(),

        "nil" => Keyword("nil").group("nil"),
        "void" => Keyword("void").group("void"),

        "end" => [Symbol(";").expect("semicolon"),Eol].or(),

        "prefix_op" => [
            Symbol("+"),
            Symbol("-").group("neg"),
            Symbol("!").group("not"),
        ].or(),

        "xor_op" => Symbol("^"),
        "and_op" => [Symbol("&"),Symbol("&"),].and(),
        "or_op" => [Symbol("|"),Symbol("|"),].and(),

        "compare_op" => [
            Symbol("<").group("lt"),
            Symbol(">").group("gt"),
            [Symbol("<"),Symbol("=")].and().group("le"),
            [Symbol(">"),Symbol("=")].and().group("ge"),
            [Symbol("="),Symbol("=")].and().group("eq"),
            [Symbol("!"),Symbol("=")].and().group("ne"),
        ].or(),

        "factor_op" => [
            Symbol("+").group("add"),
            Symbol("-").group("sub"),
        ].or(),

        "term_op" => [
            Symbol("*").group("mul"),
            Symbol("/").group("div"),
            Symbol("%").group("mod"),
        ].or(),

        "lcurly" => Symbol("{"),
        "rcurly" => Symbol("}"),
        "lsquare" => Symbol("["),
        "rsquare" => Symbol("]"),
        "lparen" => Symbol("("),
        "rparen" => Symbol(")"),

        "colon" => Symbol(":"),
        "dot" => Symbol("."),
        "ellipsis" => [NonTerm("dot"),NonTerm("dot"),NonTerm("dot"),].and(),
        "comma" => Symbol(","),



        // _ => Error(GrammarWalkError::MissingNonTerm(n)),
        _ => {return None;}
    }))
}
