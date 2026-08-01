

pub fn get_non_term<'a>(n:& str) -> Option<Rc<GrammarNode<'a>>> {


    Some(Rc::new(match n {


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


        "cond" => [
            // NonTerm("lparen"),
            NonTerm("expr"),
            // NonTerm("rparen"),
        ].and(),

        "block" => [
            NonTerm("lcurly").expect("block"),
            NonTerm("stmts"),
            NonTerm("rcurly").expect("closing brace"),

        ].and(),

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
            // NonTerm("dot"),
            [
                NonTerm("field_name"),
                NonTerm("field_index"),
            ].or().expect("field")
        ].and(), //0

        "val_index" => [
            NonTerm("lsquare"),
            [
                NonTerm("expr").group("index").expect("index"), //0
                NonTerm("rsquare"),
            ].and().expect("index")
        ].and(),


        "val" => [
            [

                [
                    // [Identifier.group("name"),NonTerm("call_params")].and().group("mcall"),



                    NonTerm("array"),
                    NonTerm("dict"), //empty dict supercedes empty block

                    NonTerm("if"),
                    NonTerm("lambda"),
                    NonTerm("block").group("block"), //allow code blocks for  exprs?

                    [
                        NonTerm("lparen"),
                        NonTerm("expr"),
                        NonTerm("rparen"),
                    ].and(),
                ].or(),
            ].or(),
            // NonTerm("val_field_index_call").many0(),
            NonTerm("field_name" ),
        ].and().group("val").expect("val"), //0

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
            // [
            //     [
            //         NonTerm("dict_key_val"),
            //         [NonTerm("comma"),NonTerm("dict_key_val"),].and().many0(),
            //         NonTerm("comma").opt(),
            //     ].and().opt(),
            //     NonTerm("rcurly"),
            // ].and().expect("closing brace"),


            [
                NonTerm("dict_key_val"),
                [NonTerm("comma"),NonTerm("dict_key_val"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rcurly").expect("closing brace"),

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

    }))
}

