/*
TODO
* could add tokens: Indent, Unindent
** do it python style, where the first indent, defines the size and type(space/tab) for sibling ones
** have option to enable it
** what about EOLs? keep, any indents come after
** ignore indents with comments or only whieespace
** have matching unindents for all indents

*/

mod error;
mod input;
mod data;
mod tokenize;

mod container;

pub use tokenize::tokenize;
pub use error::*;
pub use container::*;
pub use data::Tokenized;