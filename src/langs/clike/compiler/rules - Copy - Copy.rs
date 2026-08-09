

pub fn get_non_term<'a>(n:& str) -> Option<Rc<GrammarNode<'a>>> {


    Some(Rc::new(match n {


        "stmt" => [

            NonTerm("format"),
            NonTerm("print"),
            NonTerm("println"),
            NonTerm("expr"),
            // // NonTerm("block"), //after expr, so dict can use the empty {} //put as expr or stmt?
            // // NonTerm("if"),
        ].or(),




        "if_cond_block" => [NonTerm("expr"), NonTerm("block")].and(),


        "for_body" => [],
        "for" => [
            [
            NonTerm("for_body"),
                // [NonTerm("lparen"),NonTerm("for_body"),NonTerm("rparen"),].and(),
            ].or(),
        ].and().group("for"),



        "func" => [
            [Identifier, NonTerm("val_field_index").many0()].and(),

            // NonTerm("lcurly"),
            // NonTerm("stmts"),
            // NonTerm("rcurly"),
        ].and().group("func"),

        "lambda" => [

            // NonTerm("lcurly"),
            // NonTerm("stmts"),
            // NonTerm("rcurly"),
        ].and().group("lambda"),





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

