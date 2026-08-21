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
        // |"a"|"b"|"c"|"d"
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
        // "start" => [
        //     NonTerm("val").many0(),
        //     NonTerm("end"),
        // ].and(),

        //
        "start" => NonTerm("stmts"),

        "stmts" => [
            NonTerm("stmt"),
            [
                NonTerm("end"),
                NonTerm("stmt"),
            ].and().many0(),
        ].and().opt(),

        "stmt" => [
            NonTerm("var"),
            NonTerm("set"),

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

            NonTerm("expr"),


            // NonTerm("val"),
        ].or() //.expect("stmt")
        ,

        "var" => [
            Keyword("var"),
            NonTerm("var_entry"),
            [ Symbol(","), NonTerm("var_entry"), ].and().many0(),
            Symbol(",").opt(),
        ].and(),

        "var_entry" => [
            Identifier.group("name"),
            [NonTerm("var_set_op"), NonTerm("expr")].and().opt(),
        ].and(),

        "set" => [
            NonTerm("set_val"),
            NonTerm("set_field"),
            NonTerm("set_index"),
        ].or(),

        "set_val" => [
            Identifier.group("name"),
            NonTerm("set_op"),
            NonTerm("expr"),
        ].and().group("set_val"),

        "set_field" => [
            NonTerm("val").stow().had("field"),
            NonTerm("set_op"),
            NonTerm("expr"),
        ].and().group("set_field"),

        "set_index" => [
            NonTerm("val").stow().had("index"),
            NonTerm("set_op"),
            NonTerm("expr"),
        ].and().group("set_field"),

        "func" => [
            Keyword("fn"),
            Identifier.group("name"),
            NonTerm("func_params"),
            NonTerm("block"),
        ].and().group("func"),

        "lambda" => [
            Keyword("fn"),
            NonTerm("func_params"),
            NonTerm("block"),
        ].and().group("lambda"),

        "func_params" => [
            NonTerm("lparen"),
            [
                [
                    Identifier.group("param"),
                    [ Symbol(","), Identifier.group("param"), ].and().many0(),
                    [ NonTerm("ellipsis"), Symbol(","), ].or().opt(),
                ].and().opt(),
                NonTerm("rparen"),
            ].and().expect("closing bracket"),
        ].and().group("params"),

        "ellipsis" => [Symbol("."),Symbol("."),Symbol("."),].and().group("ellipsis"),

        "format" => [Keyword("format"),NonTerm("format_params"),].and(),
        "print" => [Keyword("print"),NonTerm("format_params"),].and(),
        "println" => [Keyword("println"),NonTerm("format_params"),].and(),

        "format_params" => [
            NonTerm("lparen"),
            [
                NonTerm("rparen"),
            ].and(),
            [
                [String, NonTerm("expr"),].or(),
                [NonTerm("comma"), NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),

        ].and(),

        "if" => [
            [Keyword("if"), NonTerm("expr"), NonTerm("block"), ].and().group("cond"),
            [Keyword("elif"),NonTerm("expr"), NonTerm("block"), ].and().group("cond").many0(),
            [Keyword("else"),NonTerm("block"),].and().group("else").opt(),
        ].and().group("if"),

        "while" => [
            Keyword("while"),
            NonTerm("expr"),
            NonTerm("block"),
        ].and().group("while"),

        "for" => [
            Keyword("for"),
            Identifier.group("name"),
            Keyword("in"),
            NonTerm("expr"),
            NonTerm("for_op"),
            NonTerm("expr"),
            NonTerm("block"),
        ].and().group("for"),

        "continue" => Keyword("continue").group("continue"),
        "break" => Keyword("break").group("break"),
        "return" => [Keyword("return"), NonTerm("expr").opt(),].and().group("return"),

        "include" => [Keyword("include"),String.group("include"),].and(),

        "expr" => NonTerm("or").group("expr").expect("expr"),

        "or" => [
            [
                NonTerm("xor").stow(),
                [ NonTerm("or_op"), NonTerm("xor"), ].and().many1(),
            ].and().group("or"),
            NonTerm("xor"),
        ].or(),

        "xor" => [
            [
                NonTerm("and").stow(),
                [ NonTerm("xor_op"), NonTerm("and"), ].and().many1(),
            ].and().group("xor"),
            NonTerm("and"),
        ].or(),

        "and" => [
            [
                NonTerm("compare").stow(),
                [ NonTerm("and_op"), NonTerm("compare"), ].and().many1(),
            ].and().group("and"),
            NonTerm("compare"),
        ].or(),

        "compare" => [
            [ NonTerm("factor").stow(), NonTerm("compare_op"), NonTerm("factor"), ].and().group("compare"),
            NonTerm("factor"),
        ].or(),

        "factor" => [
            [
                NonTerm("term").stow(),
                [ NonTerm("factor_op"), NonTerm("term"), ].and().many1(),
            ].and().group("factor"),
            NonTerm("term"),
        ].or(),

        "term" => [
            [
                NonTerm("val").stow(),
                [NonTerm("term_op"),NonTerm("val"),].and().many1(),
            ].and().group("term"),
            NonTerm("val"),
        ].or(),

        "val" => [
            NonTerm("prefix_op").many1().group("prefixes").opt(),
            [
                [ Identifier.group("idn"), NonTerm("call"), ].and().group("call_func"),

                NonTerm("primitive"),
                NonTerm("bool"),
                NonTerm("nil"),
                NonTerm("void"),

                NonTerm("array"),
                NonTerm("dict"),

                NonTerm("if"),
                NonTerm("lambda"),
                NonTerm("block"),
                [ NonTerm("lparen"), NonTerm("expr"), NonTerm("rparen"), ].and(),
            ].or(),
            NonTerm("field_index_call").many0(),
        ].and().group("val"),

        "array" => [
            NonTerm("lsquare"),
            [
                [
                    NonTerm("expr"),
                    [Symbol(","),NonTerm("expr"),].and().many0(),
                    Symbol(",").opt(),
                ].and().opt(),
                NonTerm("rsquare"),
            ].and().expect("closing square bracket"),
        ].and().group("array"),

        "dict" => [
            NonTerm("lcurly"),
            [
                [
                    NonTerm("dict_val"),
                    [Symbol(","),NonTerm("dict_val"),].and().many0(),
                    Symbol(",").opt(),
                ].and().opt(),
                NonTerm("rcurly"),
            ].and().expect("closing brace"),
        ].and().group("dict"),

        "dict_val" => [
            [
                Identifier.group("name"),
                [
                    [ Int, String, ].or().group("primitive"),
                    NonTerm("bool"),
                    NonTerm("nil"),
                ].or().group("val"),
            ].or().expect("key"),
            Symbol(":").expect("colon"),
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
            [NonTerm("index").stow(),NonTerm("call"),].and().group("call_index"),
            NonTerm("index"),

            [NonTerm("field").stow(),NonTerm("call"),].and().group("call_field"),
            NonTerm("field"),

            NonTerm("call").group("call_val"),
        ].or(),

        "call" => [
            NonTerm("lparen"),
            [
                [
                    NonTerm("expr"),
                    [ Symbol(","), NonTerm("expr"), ].and().many0(),
                    Symbol(",").opt(),
                ].and().opt(),
                NonTerm("rparen"),
            ].and().expect("closing bracket"),
        ].and().group("params").expect("call"),

        "field" => [
            Symbol("."),
            [
                Int.group("field_index"),
                Identifier.group("field_name"),
                // Error,
            ].or().expect("field").was("field"),
        ].and().expect("field"),

        "index" => [
            NonTerm("lsquare"),
            [
                [
                    NonTerm("expr"),
                    NonTerm("rsquare"),
                ].and().expect("closing square bracket"),
            ].and().expect("index").was("index")
        ].and().expect("index"),

        "primitive" => [
            Int,
            Float,
            String,
            Identifier.group("idn"),
        ].or().group("primitive"),

        "bool" => [
            Keyword("true").group("true"),
            Keyword("false").group("false"),
        ].or(),

        "nil" => Keyword("nil").group("nil"),
        "void" => Keyword("void").group("void"),

        "end" => [Symbol(";"),Eol].or().expect("semicolon"),

        "for_op" => [
            [NonTerm("for_to_op").stow(),Symbol("=").opt(),].and().group("to_eq"),
            NonTerm("for_to_op").group("to"),
        ].or(),

        "for_to_op" => [Symbol("."),Symbol("."),].and(),

        "var_set_op" => Symbol("="),

        "set_op" => [
            Symbol("=").group("eq"),
            [ NonTerm("set_sub_op"), Symbol("="), ].and(),
        ].or().expect("set"),

        "set_sub_op" => [
            Symbol("+").group("add_eq"),
            Symbol("-").group("sub_eq"),
            Symbol("*").group("mul_eq"),
            Symbol("/").group("div_eq"),
            Symbol("!").group("not_eq"),

            [Symbol("&"),Symbol("&"),].and().group("and_eq"),
            [Symbol("|"),Symbol("|"),].and().group("or_eq"),

            Symbol("^").group("xor_eq"),
        ].or(),

        "prefix_op" => [
            Symbol("+"),
            Symbol("-").group("neg"),
            Symbol("!").group("not"),
        ].or().expect("prefix"),

        "xor_op" => Symbol("^").expect("xor"),
        "and_op" => [Symbol("&"),Symbol("&"),].and().expect("and"),
        "or_op" => [Symbol("|"),Symbol("|"),].and().expect("or"),

        "compare_op" => [
            Symbol("<").group("lt"),
            Symbol(">").group("gt"),
            [Symbol("<"),Symbol("=")].and().group("le"),
            [Symbol(">"),Symbol("=")].and().group("ge"),
            [Symbol("="),Symbol("=")].and().group("eq"),
            [Symbol("!"),Symbol("=")].and().group("ne"),
        ].or().expect("compare"),

        "factor_op" => [
            Symbol("+").group("add"),
            Symbol("-").group("sub"),
        ].or().expect("factor"),

        "term_op" => [
            Symbol("*").group("mul"),
            Symbol("/").group("div"),
            Symbol("%").group("mod"),
        ].or().expect("term"),

        "lcurly" => Symbol("{"),
        "rcurly" => Symbol("}"),
        "lsquare" => Symbol("["),
        "rsquare" => Symbol("]"),
        "lparen" => Symbol("("),
        "rparen" => Symbol(")"),

        _ => {return None;}
    }))
}

