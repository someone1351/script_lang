// #![feature(mapped_lock_guards)]

// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #[allow(unused_parens)]

/*
TODO
* limit stack frame size
* limit heap memory
- eg size of collections, ie add checks to push/insert etc

* limit stack memory
* limit global decls
* limit vars in gc
* limit loops
* rust func recursion size

* limits in compiler
- string/symbol/path length
- func args num
- call params num

* in compiler fix src locs for instructions (used in error reporting)

* add struct and method decls in script
- store methods in globals, can be captured ie if there is a call, any methods with name are captured
- - need to allow methods decl'd in included files to be captured too, but how to do at compile time?
- need to be able to define names for rust types eg libscope.def_type_name::<Array>("array");

* need a strategy to run gc automatically instead of the current manual running of it

* for scripts stored in config files need to be able to report correct line/col/pos/row in config file for errors

* for errors in generated code, need it to report back to position of config file used to generate it

TODO2
* allow methods to have typed return types
- if method call has
- - multiple returntyped methods, err
- - dynamic return the dynamic
- there is no type inference
- can force select method with return type by decl var with type
- - what if var is uninitialised? force typed vars to be initialised?

TODO3
* remove func calls from within methods
- instead have "continues", where a method or func call is set to be run after the rust method call ends
- - the params for the method are left on the stack
- - after the nominated call is finished, run the original method again
- - the return value of the method doing the call, is accessible from when the original method is called again

TODO

- methods can only be declared at root, and are initialised before everything else
- - what about when declared in another file and included, they won't be avail til the include?
- - - could initialise the includes also first, so their declared methods are also init first
- - - - and then run the rest (decl vars etc) once the include is reached

TODO
* store methods used in build, then check at runtime if they exists? no as can't know param types

TODO
* allow garbage collecting on single val
** so when val is dropped, or vals are dropped, can try to gc them immediately, also can add any other vals dropped in the process to the gc'ing list

TODO
* for closures that ref each other, instead of storing as a managed (gc'd), store them together as a single unmanaged, and index into them for each func, so only all dropped after last func/closure is dropped
** could be a problem if too many funcs all refing each other in a giant graph, but unlikely to be a situation, could also detect that and optionally not do this
** problem if a global the func is being stored in is a ref type?
*** probably not, just store a pointer to the mega closure into that global ref
*** if megaclosure stores a ref to a global (or even a local) ref, and that global/local ref stores a ref to that func, then will be circular
**** could make that global/local ref a part of the megaclosure,
**** or just disallow the creation of a megaclosure under those circumstances
***** would be nice to be able to have an option of not using a gc though

** problem if accessing global declared by another script, as can't check them at commpile time for circular ref
*** could have only work with funcs stored as local vars
**** would then need to allow local vars to be declared at root level, which are currently are globals
**** then not do megaclosures for functions stored as globals (or if not using gc, then error)
**** would need special var decl for globals eg "global g" instead of "var g"
***** allow declaration inside blocks/funcs/etc?
*/


pub mod compiler;
pub mod common;
pub mod interpreter;
// pub mod sexpr_parser;
// pub mod sexpr_compiler;

// use std::path::{Path, PathBuf};


pub mod langs;
pub use common::*;
// pub use sexpr_lang::sexpr_parser::*;
// pub use sexpr_lang::sexpr_compiler::*;
pub use interpreter::*;

pub use compiler::*;


pub use langs::*;
