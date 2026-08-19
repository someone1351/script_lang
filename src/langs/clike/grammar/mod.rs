/*
TODO
* add Group(group_name,grammar_item), for output
** or instead add as method and(expr,expr).group("abc")

* output
** if something like and(expr, expr, expr (or stmt)) => [expr,expr,stmt]
** if something like and(group("abc",and(expr, expr)), expr (or stmt)) => [abc[expr,expr],stmt]
** if group not used, all output would be one single list of primitives
*/

// use std::{collections::{BTreeMap, HashMap, HashSet}, ops::Range};

// use crate::{build::Loc, };
// use super::grammar::walker::GrammarWalker;

// use super::tokenizer::{TokenContainer, TokenIterContainer, ValueContainer};

pub mod node;
pub mod walker;
pub mod container;
pub mod error;
pub mod data;

mod temp_data;
mod utils;


// use node::*;
pub use error::*;



/*
TODO
* add Expected grammar node eg
    "val" => [..].or().expect("val"),
    "block" => [..].and().expect("block"),
** add the expects after second eg
    val_field => [".", Identifier.expect("field") ].and()

    so that if you  have "if i", it willsay expecting "block" instead of infix/field/call/index etc
*/

/*

NOTE
* in work, has user to designate whether from input grammar or walker added grammar
* for AND and OR grammars, have index representing which element itis up to
* in work, could add grammar_imd, to replace work.user, and.ind, or.ind ?
** and use for MANY aswell eg increment for each loop

NOTE
    if the grammar is
        S => V* Eol*
        V => A (P B)?

    if the input is
        A Eol P

    where Eol is considered whitespace and potentially ignored
    then A is parsed, P is too, but fails for B, so it falls back to just parsing A
        and then Eol as specified in S

    P is cleared from expects, since S already parsed V* (ie A) and Eol*

    A solution is to make items failing in an AND after the first to throw an error
        so in V, P succeeding and B failing throws an error

        but sometimes you don't want that eg
            X => (A B) | (A C)

        so the solution is to either modify the expects system to retain the failing B,
            as it's token index is greater than the Eol's

        or to have specialised ANDs
            one that throws and error if any after first element fails
            another like above with X, that stows (stores result) the first item automatically,
                and either throw an error for any fail  after second element
                or just not throw any errors at all
            another that doesn't stow, and doesn't throw errors if an element fails (ie after first or second)

        or just manually use Error nodes eg
            V => A (P (B|Error))?

NOTE (same problem as above)
    if have input: A, Eol, B
    with grammar: (A (B C)? Eol)*

    it will parse A, B, fail on C, succeed on Eol
        with B left in input
        expecting A from start of MANY

    but want it to be expecting C instead

    can't just store expects with token_ind greater than cur parsed
        as expecting C, the expect token_ind would be from B's
        and Eol's token_ind would be same  as B's +1

    could keep all expects at current parse

NOTE
* for stowing, could have get_non_term return (grammar,bool), with bool denoting stow
** then inside the func have two sets of matches one for stowed and the other for not

*/