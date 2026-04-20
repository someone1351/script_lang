/*
TODO
* add Group(group_name,grammar_item), for output
** or instead add as method and(expr,expr).group("abc")

* output
** if something like and(expr, expr, expr (or stmt)) => [expr,expr,stmt]
** if something like and(group("abc",and(expr, expr)), expr (or stmt)) => [abc[expr,expr],stmt]
** if group not used, all output would be one single list of primitives
*/

use std::{collections::{BTreeMap, HashMap, HashSet}, ops::Range};

use crate::{build::Loc, ccexpr_parser::grammar::walk::GrammarWalker};
use super::tokenizer::{PrimitiveContainer, PrimitiveIterContainer, ValueContainer};

pub mod node;
pub mod walk;
pub mod container;
pub mod error;
pub mod data;

mod temp_data;


// use node::*;
pub use error::*;



