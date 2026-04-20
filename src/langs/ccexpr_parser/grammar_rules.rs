use crate::ccexpr_parser::grammar::GrammarWalkError;

// use super::grammar::*;
use super::grammar::node::*;

pub fn grammar_decl<'a>(n:&'a str) -> GrammarNode<'a> {
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

        "test9" => [
            // // Int.many0().group("a"),
            // // Float.many0().group("b"),
            // // // String.many0().group("c"),
            // NonTerm("x").many0(), //.group("a"),
            // NonTerm("x").take(), //.group("b"),
            Int.cede().many0().group("a"),
            Int.take().group("b"),
            // Eol.many0(),
        ].and(),
        // "x" => Int,

        "test10" => [
            [
                Identifier,
                // NonTerm("val_field_index").offer(),
                // // [ NonTerm("val_index"), NonTerm("val_field"), ].or(),
                // // NonTerm("val_field_index_call").many0(),
                // // [ NonTerm("val_field_index").offer(), NonTerm("call"), ].or(),
                // NonTerm("val_field_index").take(),

                NonTerm("val_field").cede().many0().group("a"), //.opt(),
                NonTerm("val_field").take().group("b"),
            ].and(),
            NonTerm("set_equal"),
            Int,
        ].and(),

        "start" => [
            NonTerm("stmts"),
            NonTerm("ending").many0(),
        ].and(),

        "ending" => [NonTerm("semicolon"),Eol].or().many1().d(),
        "stmts" => [
            NonTerm("stmt"),
            [NonTerm("ending"), NonTerm("stmt"),].and().many0(),
            NonTerm("ending").many0(),
        ].and().opt(),

        "stmt" => [
            // NonTerm("var"),
            NonTerm("set"),
            // NonTerm("func"),
            // NonTerm("while"),NonTerm("for_in"),NonTerm("for_to"),
            // NonTerm("break"), NonTerm("continue"),
            // NonTerm("return"),
            // NonTerm("include"),
            // NonTerm("format"),NonTerm("print"),NonTerm("println"),
            // NonTerm("expr"),
            // NonTerm("block"), //after expr, so dict can use the empty {}
            // // NonTerm("if"),
        ].or().group("stmt"),

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
                // NonTerm("val_field_index").offer().
                [Identifier,NonTerm("val_field_index_call").many0(), NonTerm("val_field_index").take(),].and(),
                Identifier,
            ].or(),
            // [NonTerm("add"),NonTerm("sub"),NonTerm("mul"),NonTerm("div"),NonTerm("not")].or().opt(),
            NonTerm("set_equal"),
            NonTerm("expr"),
        ].and(),

        "cond" => [NonTerm("lparen"),NonTerm("expr"),NonTerm("rparen"),].and(),
        "block" => [NonTerm("lcurly"),NonTerm("stmts"),NonTerm("rcurly"),].and(),
        "if" => [
            [Keyword("if").d(), NonTerm("cond"), NonTerm("block")].and().group(""),
            [Keyword("elif").d(),NonTerm("cond"),NonTerm("block"),].and().group("").many0(),
            [Keyword("else").d(),NonTerm("block"),].and().group("").opt(),
        ].and().group("if"),
        "while" => [Keyword("while").d(), NonTerm("cond"), NonTerm("block"),].and().group("while"),
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
            Keyword("in").d(),
            [NonTerm("val"),NonTerm("call"),].and(),
            NonTerm("rparen"),
            NonTerm("block"),
            ].and(),
        "for_to" => [
            Keyword("for"),
            NonTerm("lparen"),
            Identifier,
            Keyword("in").d(),
            NonTerm("expr"),
            Keyword("to").d(),
            NonTerm("expr"),
            NonTerm("rparen"),
            NonTerm("block"),
        ].and(),
        "call" => [
            NonTerm("lparen"),
            [
                NonTerm("expr"),
                [NonTerm("comma").d(),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt().d(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),
        "include" => [Keyword("include").d(),String,].and().group("include"),

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
        "func" => [
            Keyword("fn"),
            [Identifier, NonTerm("val_field_index").many0()].and(),
            NonTerm("func_params"),
            NonTerm("lcurly"),
            NonTerm("stmts"),
            NonTerm("rcurly"),
        ].and(),
        "lambda" => [Keyword("fn"),NonTerm("func_params"),NonTerm("lcurly"),NonTerm("stmts"),NonTerm("rcurly"),].and(),

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
            // [NonTerm("infix"),NonTerm("val"),].and().many0(),
        ].and().group("expr"),

        "prefix" => [NonTerm("add"),NonTerm("sub"),NonTerm("not"),].or(),
        "val_field_index" => [ NonTerm("val_index"), NonTerm("val_field"), ].or(),
        "val_field_index_call" => [ NonTerm("val_field_index").cede(), NonTerm("call"), ].or(),


        "val_field" => [NonTerm("dot"),[Identifier,Int,].or(),].and(),
        "val_index" => [NonTerm("lsquare"),NonTerm("expr"),NonTerm("rsquare"),].and(),
        "bool" => [Keyword("true"),Keyword("false"),].or(),
        "nil" => Keyword("nil"),
        "val" => [
            NonTerm("prefix").many0(),
            [
                Int,
                // Float,
                // String,
                // Keyword("bool"),
                // Keyword("nil"),
                // Keyword("void"),
                // NonTerm("if"),
                // NonTerm("lambda"),
                // NonTerm("array"),
                // NonTerm("dict"),
                // // NonTerm("block"), //allow code blocks?
                Identifier,
                // [NonTerm("lparen"),NonTerm("expr"),NonTerm("rparen"),].and(),
            ].or(),
            NonTerm("val_field_index_call").many0(),
            // [NonTerm("val_field_index").offer(),NonTerm("call"),].or().many0(),
        ].and(),

        "dict_key_val" => [
            [
                Identifier,
                [NonTerm("sub").opt(),Int,].and(),
                String,
                Keyword("bool"),
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

        "lcurly" => Symbol("{").d(),
        "rcurly" => Symbol("}").d(),
        "lsquare" => Symbol("[").d(),
        "rsquare" => Symbol("]").d(),
        "lparen" => Symbol("(").d(),
        "rparen" => Symbol(")").d(),

        "colon" => Symbol(":"),
        "semicolon" => Symbol(";"),
        "end" => [NonTerm("semicolon"),].or(),

        "dot" => Symbol("."),
        "ellipsis" => [NonTerm("dot"),NonTerm("dot"),NonTerm("dot"),].and(),
        "comma" => Symbol(","),

        "set_equal" => Symbol("="),

        "not" => Symbol("!").group("not"),
        "add" => Symbol("+"),
        "sub" => Symbol("-"),
        "mul" => Symbol("*"),
        "div" => Symbol("/"),

        "and" => [Symbol("&"),Symbol("&"),].and().group("and"),
        "or" => [Symbol("|"),Symbol("|"),].and().group("or"),

        "lt" => Symbol("<").group("lt"),
        "gt" => Symbol(">").group("gt"),
        "le" => [Symbol("<"),Symbol("="),].and().group("le"),
        "ge" => [Symbol(">"),Symbol("="),].and().group("ge"),
        "eq" => [Symbol("="),Symbol("="),].and().group("eq"),
        "ne" => [Symbol("!"),Symbol("="),].and().group("ne"),
        _ => Error(GrammarWalkError::MissingNonTerm(n)),
    }

}
