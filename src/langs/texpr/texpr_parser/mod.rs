//TODO
//add scanner to block container
//instead of just getting primitives eg "1*2" "+" "3" => "1" "*" "2" "+" "3"
//can generate start/end locs for them
//ScannedContainer {parsed,primitive_ind,start_loc,end_loc,chars:Range<usize>}
//block.scanner() -> ScannerContainer
//record.scanner() -> ScannerContainer


//start => (cmd | block | val | cmnt)*

// cmnt => [#] ([^#\n]|^([\r][\n]))* (eol|eof)

//block => [{] spc? start ws [}]

// cmd => idn (sep param)*

// param => block | val

//val => idn (ws (field|index))*
//field => "." idn
//index => "[" "]"

// int => ([+-]spcs?)?([1-9][0-9]*|[0-9])
// float => int([.][0-9]*)?
// bool => "true"|"false"
// str => ["] ([^"])* ["]
//===========================

/*
TODO1
* look at start/end loc for strings eg loc of start/end quotes and also of string value
* * how is it done for blocks?

TODO2
* add option for commas in records, similar to semicolons
* * two empty commas eg ",," equals 3 empty params? no just two params
* * have comma_loc func on param (like record has semicolon_loc), so can check in compiler to give error
* * if a comma at the end of the line, then any thing before any semi colon, is accepted as a param? no

* allow fields to come after new line? eg:
abc
    .def
    .ghi

* allow fields to have spaces? eg: abc .def .ghi

* could treat symbols as separate params eg abc+def => "abc", "+", "def"
* * for !%^&*-+=<>?/|
* * maybe not @ ~
* * maybe :`
* * not ,.[]{}()#$_\;"'
* * how to handle: expr 1 +2
* * * the expr command would want to know the number had a prefix, store as part of float/int primitive has prefix?
* * * or don't have prefix for numbers, but in compiler check if prev param in front is +/- ? no hassle
* * * also currently in 1 +2, the + is taken by the char_symbol, before it gets to number,
* * * * have to do number first? if so then this: +5+6 => 5 6

* for block, add get_bracket_type -> Bracket{Curly,Parentheses,Square,}, so can use for error checking if want to force one type


TODO
* allow fields for strings, numbers, bools ?
*/

mod input;
mod error;
mod container;
mod data;
mod temp_data;
mod parse;

pub use error::*;
pub use container::*;
pub use data::*;
pub use parse::parse;
