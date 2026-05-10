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

        "start" => [
            NonTerm("stmts"),
            NonTerm("ending").many0(),
        ].and(),

        "ending" => [NonTerm("semicolon"),Eol].or().many1(),
        "stmts" => [
            NonTerm("stmt"),
            [NonTerm("ending"), NonTerm("stmt"),].and().many0(),
            NonTerm("ending").many0(),
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
            // NonTerm("block"), //after expr, so dict can use the empty {} //put as expr or stmt?
            // NonTerm("if"),
        ].or(),

        "continue" => Keyword("continue"),
        "break" => Keyword("break"),
        "return" => [Keyword("return"), NonTerm("expr").opt(),].and(),

        "var_set" => [Identifier, NonTerm("equals"),NonTerm("expr")].and(),
        "var" => [
            Keyword("var"), NonTerm("var_set"),
            [NonTerm("comma"),NonTerm("var_set"),].and().many0(),
        ].and(),

        "set" => [
            [
                [Identifier,NonTerm("val_field_index_call").many0(), NonTerm("val_field_index").take(),].and(),
                Identifier,
            ].or(),
            [NonTerm("add"),NonTerm("sub"),NonTerm("mul"),NonTerm("div"),NonTerm("not")].or().opt(),
            NonTerm("equals"),
            NonTerm("expr"),
        ].and(),

        "cond" => [
            // NonTerm("lparen"),
            NonTerm("expr"),
            // NonTerm("rparen"),
        ].and(),

        "block" => [
            NonTerm("lcurly"),
            NonTerm("stmts").group("block"),
            NonTerm("rcurly"),
        ].and().expected("block"),

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

        "call" => [
            NonTerm("lparen"),
            [
                NonTerm("expr"),
                [
                    NonTerm("comma"),
                    NonTerm("expr"),
                ].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt().group("call"),
            NonTerm("rparen"),
        ].and(),

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
            [
                NonTerm("infix"),
                NonTerm("val"),
            ].and().many0(),
        ].and().group("expr").expected("expr"),

        "prefix" => [
            NonTerm("add"),
            NonTerm("sub"),
            NonTerm("not"),
        ].or(),

        "val_field_index" => [ NonTerm("val_index"), NonTerm("val_field"), ].or(),
        "val_field_index_call" => [ NonTerm("val_field_index").cede(), NonTerm("call"), ].or(),

        "val_field" => [
            NonTerm("dot"),
            [Identifier,Int,].or().group("field").expected("field"),
        ].and(),

        "val_index" => [
            NonTerm("lsquare"),
            NonTerm("expr").group("index").expected("index"),
            NonTerm("rsquare"),
        ].and(),

        "bool" => [
            Keyword("true"),
            Keyword("false"),
        ].or(),

        "nil" => Keyword("nil"),
        "void" => Keyword("void"),

        "val" => [
            NonTerm("prefix").many0(),
            [
                Int,
                Float,
                String,
                Identifier,

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
            NonTerm("val_field_index_call").many0(),
        ].and().group("val").expected("val"),

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
