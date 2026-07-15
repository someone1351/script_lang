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

pub fn get_non_term<'a>(n:& str) -> Option<GrammarNode<'a>> {
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
    Some(match n {
        "test0" => [
            Int.opt(),
            String,
        ].and(),

        // "start" => [
        //     [Keyword("a"),Keyword("b"),].and(),
        //     [Keyword("a"),Keyword("b"),].and().prev(),
        // ].and(),

        // "start" => [
        //     [
        //         [[Keyword("a"),Keyword("c").group("c")].and(),Keyword("b"),].and(),
        //         [Keyword("a"),Keyword("c").group("c")].and(),
        //     ].or(),
        //     Keyword("d"),
        // ].and(),

        // "start" => [[Keyword("a"),Keyword("b")].or().many0(), Keyword("b").prev()].and(),
        // "start" => [
        //     NonTerm("stmts"),
        //     NonTerm("ending").many0(),
        // ].and(),
        // "start" => NonTerm("expr"),
        "start" => [
            [NonTerm("a0"),NonTerm("b"),].and(),
            [[NonTerm("a0"),NonTerm("c"),].and(),NonTerm("b")].and(),
            [NonTerm("a0"),NonTerm("c"),].and(),
            NonTerm("a1")
        ].or(),
        // "start" => [
        //     [NonTerm("a0"),Keyword("b")].and(),
        //     [NonTerm("a1"),Keyword("c")].and(),
        //     Keyword("a"),

        // ].or(),
        "a0" => Keyword("a").group("A"),
        "a1" => Keyword("a"),
        "b" => Keyword("b").group("B"),
        "c" => Keyword("c").group("C"),
        // "start" => NonTerm("factor"),
        "factor" => [
            [
                NonTerm("term"), //.stow(),
                [Symbol("+"),NonTerm("term")].and().many1(),
            ].and().group("factor"),
            NonTerm("term"),
        ].or(),
        // "start" => NonTerm("term"),
        "term" => [
            [
                NonTerm("num"),
                [Symbol("*"),NonTerm("num"),].and().many1(),
            ].and().group("term"),
            NonTerm("num"),
        ].or(),
        "num" => Int.group("num"),

        // "start" => [
        //     [NonTerm("a"),NonTerm("b"),NonTerm("c"),].and(),
        //     [NonTerm("a"),NonTerm("b"),].and(),
        // ].or(),
        // "a" => Keyword("a"),
        // "b" => Keyword("b"),
        // "c" => Keyword("c"),

        "ending" => [NonTerm("semicolon"),Eol].or().many1(),
        "stmts" => [
            NonTerm("stmt"),
            [NonTerm("ending"), NonTerm("stmt"),].and().many0(),
            // NonTerm("ending").many0(),
        ].and().opt(),

        "stmt" => [
            NonTerm("var"),
            NonTerm("set"),
            NonTerm("func"),
            NonTerm("while"),
            NonTerm("for"),
            NonTerm("break"),
            NonTerm("continue"),
            NonTerm("return"),
            NonTerm("include"),
            NonTerm("format"),
            NonTerm("print"),
            NonTerm("println"),
            NonTerm("expr"),
            // // NonTerm("block"), //after expr, so dict can use the empty {} //put as expr or stmt?
            // // NonTerm("if"),
        ].or(),

        "continue" => Keyword("continue"),
        "break" => Keyword("break"),
        "return" => [Keyword("return"), NonTerm("expr").opt(),].and(),

        "var_set" => [Identifier, NonTerm("equals"),NonTerm("expr")].and(),
        "var" => [
            Keyword("var"), NonTerm("var_set"),
            [NonTerm("comma"),NonTerm("var_set"),].and().many0(),
        ].and(),

        "set_eq" => [
            [
                [
                    NonTerm("add").group("add_eq"),
                    NonTerm("sub").group("sub_eq"),
                    NonTerm("mul").group("mul_eq"),
                    NonTerm("div").group("div_eq"),
                    NonTerm("not").group("not_eq"),
                    NonTerm("and").group("and_eq"),
                    NonTerm("or").group("or_eq"),
                    NonTerm("xor").group("xor_eq"),
                ].or(),
                NonTerm("equals"),
            ].and(),
            NonTerm("equals").group("eq"),
        ].or(),

        "set" => [
            [
                // [Identifier,NonTerm("val_field_index").cede().many0(), NonTerm("val_field_index").take(),].and(),
                // [NonTerm("val").ends_in(NonTerm("val_field_index"))].and(),
                [NonTerm("val"),NonTerm("val_field_index").prev()].and(),
                Identifier,
            ].or(),
            NonTerm("set_eq"),
            NonTerm("expr"),
        ].and().group("set"),

        "cond" => [
            // NonTerm("lparen"),
            NonTerm("expr"),
            // NonTerm("rparen"),
        ].and(),

        "block" => [
            NonTerm("lcurly").expected("block") //0
            ,
            NonTerm("stmts"), //.group("block"),
            NonTerm("rcurly").expected("closing brace") //1
            ,
        ].and() //.expected("block")
        ,

        "if_cond_block" => [NonTerm("cond"), NonTerm("block")].and(),

        "if" => [
            [Keyword("if"), NonTerm("if_cond_block"), ].and(),
            [Keyword("elif"),NonTerm("if_cond_block"), ].and().many0(),
            [Keyword("else"),NonTerm("block"),].and().opt(),
        ].and().group("if"),
        "while" => [
            Keyword("while"), NonTerm("cond"), NonTerm("block"),
        ].and().group("while"),

        "for_body" => [
            Identifier,
            Keyword("in"),
            NonTerm("expr"),
            // Keyword("to"),

            [NonTerm("dot"),NonTerm("dot"),NonTerm("equals").opt(),].and(),


            NonTerm("expr"),
        ].and(),
        "for" => [
            Keyword("for"),
            [
                NonTerm("for_body"),
                // [NonTerm("lparen"),NonTerm("for_body"),NonTerm("rparen"),].and(),
            ].or(),
            NonTerm("block"),
        ].and().group("for"),


        "include" => [Keyword("include"),String,].and(),

        "func_params" => [
            NonTerm("lparen"),
            [
                Identifier,
                [
                    NonTerm("comma"),
                    Identifier,
                ].and().many0(),
                NonTerm("ellipsis").opt(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),

        "func" => [
            Keyword("fn"),
            [Identifier, NonTerm("val_field_index").many0()].and(),
            NonTerm("func_params"),
            NonTerm("block"),
            // NonTerm("lcurly"),
            // NonTerm("stmts"),
            // NonTerm("rcurly"),
        ].and().group("func"),

        "lambda" => [
            Keyword("fn"),
            NonTerm("func_params"),
            NonTerm("block"),
            // NonTerm("lcurly"),
            // NonTerm("stmts"),
            // NonTerm("rcurly"),
        ].and().group("lambda"),

        // "compare_op" => [

        // ].or(),
        // "infix" => [
        //     NonTerm("add").group("add"),NonTerm("sub").group("sub"),
        //     NonTerm("mul").group("mul"),NonTerm("div").group("div"),
        //     NonTerm("percent").group("mod"),NonTerm("hat").group("pow"),

        //     NonTerm("and").group("and"),NonTerm("or").group("or"),
        // ].or(),

        "expr_or" => [
            [
                NonTerm("expr_xor"),
                [
                    NonTerm("or"),
                    NonTerm("expr_xor"),
                ].and().many1(),
            ].and().group("expr_or"),
            NonTerm("expr_xor"),
        ].or().expected("or"), //0

        "expr_xor" => [
            [
                NonTerm("expr_and"),
                [
                    NonTerm("xor"),
                    NonTerm("expr_and"),
                ].and().many1(),
            ].and().group("expr_xor"),
            NonTerm("expr_and"),
        ].or().expected("xor"), //0

        "expr_and" => [
            [
                NonTerm("expr_compare"),
                [
                    NonTerm("and"),
                    NonTerm("expr_compare"),
                ].and().many1(),
            ].and().group("expr_and"),
            NonTerm("expr_compare"),
        ].or().expected("and"), //0

        "expr_compare" => [
            [
                NonTerm("expr_factor"),
                [
                    NonTerm("lt").group("lt"),
                    NonTerm("gt").group("gt"),
                    NonTerm("le").group("le"),
                    NonTerm("ge").group("ge"),
                    NonTerm("eq").group("eq"),
                    NonTerm("ne").group("ne"),
                ].or(),
                NonTerm("expr_factor"),
            ].and().group("expr_compare"),
            NonTerm("expr_factor"),
        ].or().expected("compare"), //0

        "expr_factor" => [
            [
                NonTerm("expr_term"),
                [
                    [NonTerm("add").group("add"),NonTerm("sub").group("sub"),].or(),
                    NonTerm("expr_term"),
                ].and().many1(),
            ].and().group("expr_factor"),
            NonTerm("expr_term"),
        ].or().expected("factor"), //0

        "expr_term" => [
            [
                NonTerm("val"),
                [
                    [
                        NonTerm("mul").group("mul"),
                        NonTerm("div").group("div"),
                        NonTerm("mod").group("mod"),
                    ].or(),
                    NonTerm("val"),
                ].and().many1(),
            ].and().group("expr_term"),
            NonTerm("val"),
        ].or().expected("term"), //0

        // // "expr_term" => [
        // //     [NonTerm("val"),NonTerm("mul"),NonTerm("expr_term"),].and().group("mul"),
        // //     [NonTerm("val"),NonTerm("div"),NonTerm("expr_term"),].and().group("div"),
        // //     [NonTerm("val"),NonTerm("mod"),NonTerm("expr_term"),].and().group("mod"),
        // //     NonTerm("val"),
        // // ].or(),

        // // "expr" => [
        // //     NonTerm("val"),
        // //     [
        // //         NonTerm("infix"),
        // //         NonTerm("val"),
        // //     ].and().many0(),
        // // ].and().group("expr").expected0("expr"),

        "expr" => NonTerm("expr_or").group("expr").expected("expr"), //0
        // "expr" => NonTerm("val").group("expr").expected0("expr"),

        "prefixes" => [
            NonTerm("add").group("pos"),
            NonTerm("sub").group("neg"),
            NonTerm("not").group("not"),
        ].or().many1().group("prefixes"),

        "call_params" => [
            NonTerm("lparen"),
            [
                NonTerm("expr"),
                [
                    NonTerm("comma"),
                    NonTerm("expr"),
                ].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt().group("params"),
            NonTerm("rparen"),
        ].and(),


        "val_field_index" => [
            NonTerm("val_index"),
            NonTerm("val_field"),
        ].or(),

        "val_field_index_call" => [
            [
                [NonTerm("field_index"),NonTerm("call_params"),].and().group("call_field_index"),
                [NonTerm("field_name"),NonTerm("call_params"),].and().group("call_field_name"),
                NonTerm("call_params").group("call_val"),
                NonTerm("val_field_index"),
            ].or()
        ].or(),

        "field_name" => [NonTerm("dot"),Identifier.group("field_name")].and(),
        "field_index" => [NonTerm("dot"),Int.group("field_index")].and(),

        "val_field" => [
            NonTerm("field_name"),
            NonTerm("field_index"),
        ].or().expected("field"), //0

        "val_index" => [
            NonTerm("lsquare"),
            NonTerm("expr").group("index").expected("index"), //0
            NonTerm("rsquare"),
        ].and(),

        "bool" => [
            Keyword("true").group("true"),
            Keyword("false").group("false"),
        ].or(),

        "nil" => Keyword("nil").group("nil"),
        "void" => Keyword("void").group("void"),



        "val" => [
            NonTerm("prefixes").opt(),
            [
                [
                    Identifier.group("idn"),
                    NonTerm("call_params"),
                ].and().group("call_idn"),
                [
                    // // [Identifier.group("name"),NonTerm("call_params")].and().group("mcall"),
                    [
                        Int,
                        Float,
                        String,
                        Identifier.group("idn"),
                    ].or().group("primitive"),

                    NonTerm("bool"),
                    NonTerm("nil"),
                    NonTerm("void"),

                    NonTerm("array"),
                    NonTerm("dict"), //empty dict supercedes empty block

                    NonTerm("if"),
                    NonTerm("lambda"),
                    NonTerm("block"), //allow code blocks for  exprs?

                    [
                        NonTerm("lparen"),
                        NonTerm("expr"),
                        NonTerm("rparen"),
                    ].and(),
                ].or(),
            ].or().expected("val"), //0
            NonTerm("val_field_index_call").many0(),
        ].and().group("val").expected("val"), //0

        "dict_key_val" => [
            [
                Identifier,
                [NonTerm("sub").opt(),Int,].and(),
                String,
                NonTerm("bool"),
                Keyword("nil"),
            ].or(),
            NonTerm("colon"),
            NonTerm("expr"),
        ].and(),

        "dict" => [
            NonTerm("lcurly"),
            [
                NonTerm("dict_key_val"),
                [NonTerm("comma"),NonTerm("dict_key_val"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rcurly"),
        ].and(),

        "array" => [
            NonTerm("lsquare"),
            [
                NonTerm("expr"),
                [NonTerm("comma"),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rsquare"),
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

        "colon" => Symbol(":"),
        "semicolon" => Symbol(";"),

        "dot" => Symbol("."),
        "ellipsis" => [NonTerm("dot"),NonTerm("dot"),NonTerm("dot"),].and(),
        "comma" => Symbol(","),
        "equals" => Symbol("="),

        "not" => Symbol("!"),
        "add" => Symbol("+"),
        "sub" => Symbol("-"),
        "mul" => Symbol("*"),
        "div" => Symbol("/"),


        "xor" => Symbol("^"),
        "mod" => Symbol("%"),

        "and" => [Symbol("&"),Symbol("&"),].and(),
        "or" => [Symbol("|"),Symbol("|"),].and(),

        "lt" => Symbol("<"),
        "gt" => Symbol(">"),
        "le" => [Symbol("<"),Symbol("="),].and(),
        "ge" => [Symbol(">"),Symbol("="),].and(),
        "eq" => [Symbol("="),Symbol("="),].and(),
        "ne" => [Symbol("!"),Symbol("="),].and(),
        // _ => Error(GrammarWalkError::MissingNonTerm(n)),
        _ => {return None;}
    })

}
