# script lang

Scripting language that is a cross between tcl, lisp and javascript.

## Features

### Methods

* currently only bindable from the rust interface
* type overloaded
* can access variables with temporary lifetimes

### Commands

* compile time macros
* used to add things like for/while loops, if statements etc

### Closures

* capture both local and global vars
* can capture variables declared after the function's decl

### Misc

* garbage collector (mark and sweep)
* can take script variables and store them in rust
