use super::ccexpr_parser::{grammar::walk::GrammarWalker, grammar_rules::grammar_decl, tokenizer::TokenIterContainer};


pub mod tokenizer;


pub mod grammar;
pub mod grammar_rules;

pub fn grammar_run<'a>( top_primitives:TokenIterContainer<'a>) {
    /*
    abc|ab with "ab" => "" //abc will fail, but then tries ab, which succeeds
    ab|abc with "abc" => "c" //will consume ab, and then fail to consume c, there is no backtracking
     */

    //expects not completely correct,
    //  if succeeds to a certain point, need to clear the expecteds,
    //  currently it just clears everything after any success
    //  could just add expects that are ==, and replace ones that are >
    //  on success, only clear if >= than loc

    let mut walker=GrammarWalker::new(top_primitives, grammar_decl);
    // walker.set_debug(true);
    walker.run("start");

    // walker.get_walk();

    // // println!("{}",walker.get_walk().root());

    // let mut top_primitives=top_primitives;
    // println!("====");
    // println!("{top_primitives:?}",);

    // let a=top_primitives.pop_front_amount(10);

    // println!("=\n{top_primitives:?}\n=\n{a:?}");

    // let a=top_primitives.pop_front_amount(10);
    // println!("=\n{top_primitives:?}\n=\n{a:?}");

}
