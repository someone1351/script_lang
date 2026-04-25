use super::parser::rules::grammar_decl;
use super::grammar::walk::GrammarWalker;
use super::tokenizer::TokenIterContainer;

pub mod rules;

pub fn parse<'a>( top_primitives:TokenIterContainer<'a>) {
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

    if walker.run("start") {

    } else {

    }

    // let walk=walker.get_walk();

    // println!("{}",walk.root());

    // let mut top_primitives=top_primitives;
    // println!("====");
    // println!("{top_primitives:?}",);

    // let a=top_primitives.pop_front_amount(10);

    // println!("=\n{top_primitives:?}\n=\n{a:?}");

    // let a=top_primitives.pop_front_amount(10);
    // println!("=\n{top_primitives:?}\n=\n{a:?}");

}
