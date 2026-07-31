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
        "start" => NonTerm("expr_term"),

        // "stmt" => NonTerm("expr_term"),

        "block" => [
            NonTerm("lcurly"),
            [
                NonTerm("expr_term"),
                NonTerm("rcurly"),
            ].and().expect("closing brace"),
        ].and(),

        // "expr_factor" => [
        //     [
        //         NonTerm("expr_term"),
        //         [
        //             NonTerm("add"),
        //             NonTerm("expr_term"),
        //         ].and().many1(),
        //     ].and(),
        //     NonTerm("expr_term"),
        // ].or(),

        "expr_term" => [
            [
                NonTerm("block"),
                NonTerm("mul"),
                // [NonTerm("mul"),NonTerm("block"),].and().many1(),
            ].and(),
            NonTerm("block"),
        ].or(),


        // "val" => [
        //     // Int.opt(),
        //     // [

        //         NonTerm("block"),

        //     // ].or()
        // ].and(),

        "lcurly" => Symbol("{"),
        "rcurly" => Symbol("}"),

        "semicolon" => Symbol(";"),

        "add" => Symbol("+"),
        "sub" => Symbol("-"),
        "mul" => Symbol("*"),
        "div" => Symbol("/"),


        _ => {return None;}
    }))





}
