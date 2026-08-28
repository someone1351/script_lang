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
            Keyword("var"),
            NonTerm("var_entry"),
            [ Symbol(","), NonTerm("var_entry"), ].and().many0(),
            // Symbol(",").opt(),
        ].and(),

        "var_entry" => [
            Identifier.group("name"),
            [Symbol("="), NonTerm("expr")].and().opt(),
        ].and().group("var"),

        "set" => [
            NonTerm("set_var"),
            NonTerm("set_field"),
            NonTerm("set_index"),
        ].or(),

        "set_var" => [
            Identifier.group("name"),
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
                    [ NonTerm("func_params2"), NonTerm("func_variadic"), ].and(),
                    [ NonTerm("func_params2"), NonTerm("func_not_variadic"), ].and(),
                    [ Always.group("params"), Always.group("not_variadic"), ].and(),
                ].or(),
                NonTerm("rparen"),
            ].and().expect("closing bracket"),
        ].and(),

        "func_params2" => [
            Identifier.group("param").expect("param"),
            [ Symbol(","), Identifier.group("param").expect("param"), ].and().many0(),
        ].and().group("params"),

        "func_variadic" => [Symbol("."),Symbol("."),Symbol("."),].and().group("variadic"),
        "func_not_variadic" => Symbol(",").opt().group("not_variadic"),

        // "ellipsis" => [Symbol("."),Symbol("."),Symbol("."),].and().group("ellipsis"),

        "format" => [Keyword("format"),NonTerm("format_params"),].and(),
        "print" => [Keyword("print"),NonTerm("format_params"),].and(),
        "println" => [Keyword("println"),NonTerm("format_params"),].and(),

        "format_params" => [
            NonTerm("lparen"),
            [
                [
                    [String.group("string"), NonTerm("expr"),].or(),
                    [NonTerm("comma"), NonTerm("expr"),].and().many0(),
                    NonTerm("comma").opt(),
                ].and().opt(),
                NonTerm("rparen"),
            ].and().expect("closing bracket"),
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
                NonTerm("xor"),
                [
                    [Symbol("|"),Symbol("|").expect("or")].and(),
                    NonTerm("xor"),
                ].and().many1(),
            ].and().group("or"),
            NonTerm("xor"),
        ].or(),

        "xor" => [
            [
                NonTerm("and"),
                [ Symbol("^"), NonTerm("and"), ].and().many1(),
            ].and().group("xor"),
            NonTerm("and"),
        ].or(),

        "and" => [
            [
                NonTerm("compare"),
                [ Symbol("&"),Symbol("&").expect("and"), NonTerm("compare"), ].and().many1(),
            ].and().group("and"),
            NonTerm("compare"),
        ].or(),

        // "compare" => [
        //     [ NonTerm("factor"), NonTerm("compare_op"), NonTerm("factor"), ].and().group("compare"),
        //     NonTerm("factor"),
        // ].or(),

        "compare" => [
            [NonTerm("factor"), Symbol("<"), NonTerm("factor"),].and().group("lt"),
            [NonTerm("factor"), Symbol(">"), NonTerm("factor"),].and().group("gt"),
            [NonTerm("factor"), Symbol("<"),Symbol("="), NonTerm("factor"),].and().group("le"),
            [NonTerm("factor"), Symbol(">"),Symbol("="), NonTerm("factor"),].and().group("ge"),
            [NonTerm("factor"), Symbol("="),Symbol("=").expect("eq"), NonTerm("factor"),].and().group("eq"),
            [NonTerm("factor"), Symbol("!"),Symbol("=").expect("ne"), NonTerm("factor"),].and().group("ne"),
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
                    Symbol("+"),
                    Symbol("-").group("neg"),
                    Symbol("!").group("not"),
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
            [ Identifier.group("idn"), NonTerm("call"), ].and().group("call_func"),

            NonTerm("primitive"),
            // NonTerm("bool"),
            // NonTerm("nil"),
            // NonTerm("void"),

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
                    Int, String,
                    Keyword("nil"),
                    Keyword("true"),
                    Keyword("false"),
                ].or().group("primitive"),

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
            [NonTerm("index"),NonTerm("call"),].and().group("call_index"),
            NonTerm("index").group("index"),

            [NonTerm("field"),NonTerm("call"),].and().group("call_field"),
            NonTerm("field").group("field"),

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
        ].and().group("params"),

        "field" => [
            Symbol("."),
            [
                Int.group("field_index"),
                Identifier.group("field_name"),
                // Error,
            ].or().expect("field").was("field"),
        ].and(),

        "index" => [
            NonTerm("lsquare"),
            [
                [
                    NonTerm("expr"),
                    NonTerm("rsquare"),
                ].and().expect("closing square bracket"),
            ].and().expect("index").was("index")
        ].and(),

        "primitive" => [
            Int,
            Float,
            String,
            Identifier,
            Keyword("nil"),
            Keyword("void"),
            Keyword("true"),
            Keyword("false"),
        ].or().group("primitive"),

        // "bool" => [
        //     Keyword("true").group("true"),
        //     Keyword("false").group("false"),
        // ].or(),

        // "nil" => Keyword("nil").group("nil"),
        // "void" => Keyword("void").group("void"),

        "end" => [Symbol(";"),Eol].or().many1().expect("semicolon"),

        "for_op" => [
            [NonTerm("for_to_op"),Symbol("=").opt(),].and().group("to_eq"),
            NonTerm("for_to_op").group("to"),
        ].or(),

        "for_to_op" => [Symbol("."),Symbol("."),].and(),

        // "var_set_op" => Symbol("="),

        "set_op" => [
            Symbol("=").group("eq"),
            [ NonTerm("set_sub_op"), Symbol("="), ].and(),
        ].or(),

        "set_sub_op" => [
            Symbol("+").group("add"),
            Symbol("-").group("sub"),
            Symbol("*").group("mul"),
            Symbol("/").group("div"),
            Symbol("!").group("not"),

            [Symbol("&"),Symbol("&"),].and().group("and"),
            [Symbol("|"),Symbol("|"),].and().group("or"),

            Symbol("^").group("xor"),
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

        _ => {return None;}
    }))
}
