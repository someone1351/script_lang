use std::{fmt::Display, ops::Deref, rc::Rc};
use crate::clike::{grammar::{container::WalkGroupContainer, PrimitiveTrait}, tokenizer::{TokenContainer, TokenIterContainer}};

/*
TODO
* don't allow field_ind (eg a.5) to be used for int/float/bool
* don't allow spaces between the decimal point and numbers in a float?
** allow floats like .5 ?
*/
use super::super::grammar::node::*;


#[derive(Copy,Clone,Hash,PartialEq,Eq,Debug)]
// pub enum MyGroup {}
pub struct MyGroup<'g>(pub &'g str);

#[derive(Clone,Hash,PartialEq,Eq,Debug)]
pub enum MyPrimitive<'g> {
    String,
    Identifier,
    Int,
    Float,
    Symbol(&'g str),
    Keyword(&'g str),
    Eol,
}


// impl<'g> GrammarPrimitive<'g> {
//     fn to<NT>(self) -> GrammarNode<'g,NT,GrammarPrimitive<'g>> {
//         GrammarNode::Primitive(self)
//     }
// }

impl<'g> PrimitiveTrait for MyPrimitive<'g> {
    fn is_trimmable(&self) -> bool {
        match self {
            MyPrimitive::Eol => true,
            _ => false,
        }
    }
}


// impl<'g> Into<GrammarNode<'g,GrammarPrimitive<'g>>> for GrammarPrimitive<'g> {
//     fn into(self) -> GrammarNode<'g,GrammarPrimitive<'g>> {
//         GrammarNode::Primitive(self)
//     }
// }

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
        // |"a"|"b"|"c"|"d"
        => true,
        _=>false,
    }
}

#[derive(Debug,Clone,Copy,Hash,PartialEq, Eq)]
pub enum MyNonTerm {
    Start,
    Stmts,
    Stmt,
    Var,
    VarEntry,
    Set,
    SetVar,
    SetField,
    SetIndex,
    Func,
    Lambda,
    FuncParams,
    FuncParams2,
    FuncVariadic,
    FuncNotVariadic,
    Format,
    Print,
    Println,
    FormatParams,
    If,
    While,
    For,
    Continue,
    Break,
    Return,
    Include,
    Expr,
    Or1,
    Xor,
    And1,
    Compare,
    Factor,
    Term,
    Prefixes,
    Postfixes,
    Val,
    Array,
    Dict,
    DictVal,
    Block,
    FieldIndexCall,
    Call,
    Field,
    Index,
    Primitive1,
    End,
    ForOp,
    ForToOp,
    SetOp,
    SetSubOp,
    FactorOp,
    TermOp,
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    LParen,
    RParen,
}

// impl AsRef<GrammarNode<'static,MyNonTerm,GrammarPrimitive<'static>>> for MyNonTerm {
//     fn as_ref(&self) -> &'static GrammarNode<'static,MyNonTerm,GrammarPrimitive<'static>> {
//         GrammarNode::NonTerm(self)
//     }
// }

// impl Into<GrammarNode<'static,MyNonTerm,GrammarPrimitive<'static>>> for MyNonTerm {
//     fn into(self) -> GrammarNode<'static,MyNonTerm,GrammarPrimitive<'static>> {
//         GrammarNode::NonTerm(self)
//     }
// }

// impl Deref for MyNonTerm {
//     type Target = GrammarNode<'static,MyNonTerm,GrammarPrimitive<'static>>;

//     fn deref(&self) -> &'static Self::Target {
//         GrammarNode::NonTerm(*self)
//     }
// }

pub fn get_non_term<'g>(n:MyNonTerm) -> Option<&'g GrammarNode<'g,MyNonTerm,MyPrimitive<'g>,MyGroup<'g>>> {
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
    use MyPrimitive::*;
    use GrammarNode::*;
    // use MyNonTerm::*;


    Some(match n {
        // "start" => [
        //     NonTerm(MyNonTerm::Val).many0(),
        //     NonTerm("end"),
        // ].and(),

        // "mynum" => Primitive(Int).group("n"),
        // "start2" => [
        //     [NonTerm("mynum"),NonTerm("mynum")].and().group("a"),
        //     NonTerm("mynum").group("b").group("b"),
        // ].or(),
        //
        // MyNonTerm::Start =>
        //     // &Expect(
        //     &Many(&Expect(&Primitive(Int),"y"),3,3)
        //     // ,"x")
        //     ,
        MyNonTerm::Start => &NonTerm(MyNonTerm::Stmts),

        MyNonTerm::Stmts => &Or(&[&And(&[
            &Or(&[&NonTerm(MyNonTerm::End),&Always]),
            &NonTerm(MyNonTerm::Stmt),
            &Many(&And(&[
                &NonTerm(MyNonTerm::End),
                &NonTerm(MyNonTerm::Stmt),
            ]),0,0),
            &Or(&[&NonTerm(MyNonTerm::End),&Always]),
        ]), &Always]),

        MyNonTerm::Stmt => &Or(&[
            &NonTerm(MyNonTerm::Var),

            &NonTerm(MyNonTerm::For),
            &NonTerm(MyNonTerm::While),
            &NonTerm(MyNonTerm::Func),

            &NonTerm(MyNonTerm::Break),
            &NonTerm(MyNonTerm::Continue),
            &NonTerm(MyNonTerm::Return),

            &NonTerm(MyNonTerm::Include),

            &NonTerm(MyNonTerm::Format),
            &NonTerm(MyNonTerm::Print),
            &NonTerm(MyNonTerm::Println),

            &NonTerm(MyNonTerm::Set),
            &NonTerm(MyNonTerm::Expr),

        ]),

        MyNonTerm::Var => &And(&[
            &Primitive(Keyword("var")),
            &NonTerm(MyNonTerm::VarEntry),
            &Many(&And(&[ &Primitive(Symbol(",")), &NonTerm(MyNonTerm::VarEntry), ]),0,0),
            // Symbol(",").opt(),
        ]),

        MyNonTerm::VarEntry => &Group(&And(&[
            &Group(&Primitive(Identifier),MyGroup("name")),
            &Or(&[&And(&[&Primitive(Symbol("=")), &NonTerm(MyNonTerm::Expr)]),&Always,]),
        ]), MyGroup("var")),

        MyNonTerm::Set => &Or(&[
            &NonTerm(MyNonTerm::SetVar),
            &NonTerm(MyNonTerm::SetField),
            &NonTerm(MyNonTerm::SetIndex),
        ]),

        MyNonTerm::SetVar => &Group(&And(&[
            &Group(&Primitive(Identifier),MyGroup("name")),
            &NonTerm(MyNonTerm::SetOp),
            &NonTerm(MyNonTerm::Expr),
        ]), MyGroup("set_var")),

        MyNonTerm::SetField => &Group(&And(&[
            &NonTerm(MyNonTerm::Postfixes),
            &Had(&NonTerm(MyNonTerm::Field)),
            &NonTerm(MyNonTerm::SetOp),
            &NonTerm(MyNonTerm::Expr),
        ]), MyGroup("set_field")),

        MyNonTerm::SetIndex => &Group(&And(&[
            &NonTerm(MyNonTerm::Postfixes),
            &Had(&NonTerm(MyNonTerm::Index)),
            &NonTerm(MyNonTerm::SetOp),
            &NonTerm(MyNonTerm::Expr),
        ]), MyGroup("set_index")),

        MyNonTerm::Func => &Group(&And(&[
            &Primitive(Keyword("fn")),
            &Group(&Primitive(Identifier), MyGroup("name")),
            &NonTerm(MyNonTerm::FuncParams),
            &NonTerm(MyNonTerm::Block),
        ]),MyGroup("func")),

        MyNonTerm::Lambda => &Group(&And(&[
            &Primitive(Keyword("fn")),
            &NonTerm(MyNonTerm::FuncParams),
            &NonTerm(MyNonTerm::Block),
        ]), MyGroup("lambda")),

        MyNonTerm::FuncParams => &And(&[
            &NonTerm(MyNonTerm::LParen),
            &Expect(&And(&[
                &Or(&[
                    &And(&[ &NonTerm(MyNonTerm::FuncParams2), &NonTerm(MyNonTerm::FuncVariadic), ]),
                    &And(&[ &NonTerm(MyNonTerm::FuncParams2), &NonTerm(MyNonTerm::FuncNotVariadic), ]),
                    &And(&[ &Group(&Always,MyGroup("params")), &Group(&Always,MyGroup("not_variadic")), ]),
                ]),
                &NonTerm(MyNonTerm::RParen),
            ]), "closing bracket")
        ]),

        MyNonTerm::FuncParams2 => &Group(&And(&[
            &Expect(&Group(&Primitive(Identifier),MyGroup("param")), "param"),
            &Many(&And(&[ &Primitive(Symbol(",")), &Expect(&Group(&Primitive(Identifier),MyGroup("param")), "param"), ]),0,0),
        ]),MyGroup("params")),

        MyNonTerm::FuncVariadic => &Group(&And(&[&Primitive(Symbol(".")),&Primitive(Symbol(".")),&Primitive(Symbol(".")),]),MyGroup("variadic")),
        MyNonTerm::FuncNotVariadic => &Group(&Or(&[&Primitive(Symbol(",")),&Always]), MyGroup("not_variadic")),


        MyNonTerm::Format => &And(&[&Primitive(Keyword("format")),&NonTerm(MyNonTerm::FormatParams),]),
        MyNonTerm::Print => &Group(&And(&[&Primitive(Keyword("print")),&NonTerm(MyNonTerm::FormatParams),]), MyGroup("print")),
        MyNonTerm::Println => &Group(&And(&[&Primitive(Keyword("println")),&NonTerm(MyNonTerm::FormatParams),]), MyGroup("println")),

        MyNonTerm::FormatParams => &Group(&And(&[
            &NonTerm(MyNonTerm::LParen),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &Or(&[&Group(&Primitive(String), MyGroup("string")), &NonTerm(MyNonTerm::Expr),]),
                    &Many(&And(&[&Primitive(Symbol(",")), &NonTerm(MyNonTerm::Expr),]),0,0),
                    &Or(&[&Primitive(Symbol(",")), &Always]),
                ]), &Always]),
                &NonTerm(MyNonTerm::RParen),
            ]),"closing bracket"),
        ]),MyGroup("format")),

        MyNonTerm::If => &Group(&And(&[
            &Group(&And(&[&Primitive(Keyword("if")), &NonTerm(MyNonTerm::Expr), &NonTerm(MyNonTerm::Block), ]),MyGroup("cond")),
            &Many(&Group(&And(&[&Primitive(Keyword("elif")), &NonTerm(MyNonTerm::Expr), &NonTerm(MyNonTerm::Block), ]),MyGroup("cond")),0,0),
            &Or(&[&Group(&And(&[&Primitive(Keyword("else")),&NonTerm(MyNonTerm::Block),]),MyGroup("else")), &Always]),
        ]),MyGroup("if")),

        MyNonTerm::While => &Group(&And(&[
            &Primitive(Keyword("while")),
            &NonTerm(MyNonTerm::Expr),
            &NonTerm(MyNonTerm::Block),
        ]),MyGroup("while")),

        MyNonTerm::For => &Group(&And(&[
            &Primitive(Keyword("for")),
            &Group(&Primitive(Identifier),MyGroup("name")),
            &Primitive(Keyword("in")),
            &NonTerm(MyNonTerm::Expr),
            &NonTerm(MyNonTerm::ForOp),
            &NonTerm(MyNonTerm::Expr),
            &NonTerm(MyNonTerm::Block),
        ]),MyGroup("for")),

        MyNonTerm::Continue => &Group(&Primitive(Keyword("continue")),MyGroup("continue")),
        MyNonTerm::Break => &Group(&Primitive(Keyword("break")),MyGroup("break")),
        MyNonTerm::Return => &Group(&And(&[&Primitive(Keyword("return")), &Or(&[&NonTerm(MyNonTerm::Expr), &Always]),]),MyGroup("return")),

        MyNonTerm::Include => &And(&[&Primitive(Keyword("include")),&Group(&Primitive(String),MyGroup("include")),]),

        MyNonTerm::Expr => &Expect(&Group(&NonTerm(MyNonTerm::Or1),MyGroup("expr")),"expr"),
        // MyNonTerm::Expr => &Expect(&Group(&NonTerm(MyNonTerm::Term),"expr"),"expr"),

        MyNonTerm::Or1 => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::Xor),
                &Many(&And(&[ &And(&[&Primitive(Symbol("|")),&Expect(&Primitive(Symbol("|")),"or")]), &NonTerm(MyNonTerm::Xor), ]),1,0),
            ]), MyGroup("or")),
            &NonTerm(MyNonTerm::Xor),
        ]),

        MyNonTerm::Xor => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::And1),
                &Many(&And(&[ &Primitive(Symbol("^")), &NonTerm(MyNonTerm::And1), ]),1,0),
            ]),MyGroup("xor")),
            &NonTerm(MyNonTerm::And1),
        ]),

        MyNonTerm::And1 => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::Compare),
                &Many(&And(&[ &Primitive(Symbol("&")),&Expect(&Primitive(Symbol("&")),"and"), &NonTerm(MyNonTerm::Compare), ]),1,0),
            ]),MyGroup("and")),
            &NonTerm(MyNonTerm::Compare),
        ]),


        MyNonTerm::Compare => &Or(&[
            &Group(&And(&[&NonTerm(MyNonTerm::Factor), &Primitive(Symbol("<")), &NonTerm(MyNonTerm::Factor),]),MyGroup("lt")),
            &Group(&And(&[&NonTerm(MyNonTerm::Factor), &Primitive(Symbol(">")), &NonTerm(MyNonTerm::Factor),]),MyGroup("gt")),
            &Group(&And(&[&NonTerm(MyNonTerm::Factor), &Primitive(Symbol("<")),&Primitive(Symbol("=")), &NonTerm(MyNonTerm::Factor),]),MyGroup("le")),
            &Group(&And(&[&NonTerm(MyNonTerm::Factor), &Primitive(Symbol(">")),&Primitive(Symbol("=")), &NonTerm(MyNonTerm::Factor),]),MyGroup("ge")),
            &Group(&And(&[&NonTerm(MyNonTerm::Factor), &Primitive(Symbol("=")),&Expect(&Primitive(Symbol("=")),"eq"), &NonTerm(MyNonTerm::Factor),]),MyGroup("eq")),
            &Group(&And(&[&NonTerm(MyNonTerm::Factor), &Primitive(Symbol("!")),&Expect(&Primitive(Symbol("=")),"ne"), &NonTerm(MyNonTerm::Factor),]),MyGroup("ne")),
            &NonTerm(MyNonTerm::Factor),
        ]),

        MyNonTerm::Factor => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::Term),
                &Many(&And(&[ &NonTerm(MyNonTerm::FactorOp), &NonTerm(MyNonTerm::Term), ]),1,0),
            ]),MyGroup("factor")),
            &NonTerm(MyNonTerm::Term),
        ]),

        MyNonTerm::Term => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::Prefixes),
                &Many(&And(&[&NonTerm(MyNonTerm::TermOp),&NonTerm(MyNonTerm::Prefixes),]),1,0),
            ]),MyGroup("term")),
            &NonTerm(MyNonTerm::Prefixes),
        ]),


        MyNonTerm::Prefixes => &Or(&[
            &Group(&And(&[
                &Group(
                    &Many(&Or(&[
                        &Primitive(Symbol("+")),
                        &Group(&Primitive(Symbol("-")),MyGroup("neg")),
                        &Group(&Primitive(Symbol("!")),MyGroup("not")),
                    ]),1,0),MyGroup("prefixes")),
                &NonTerm(MyNonTerm::Postfixes),
            ]),MyGroup("prefixes")),
            &NonTerm(MyNonTerm::Postfixes),
        ]),

        MyNonTerm::Postfixes => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::Val),
                &Group(&Many(&NonTerm(MyNonTerm::FieldIndexCall),1,0),MyGroup("field_index_calls")),
            ]),MyGroup("postfixes")),
            &NonTerm(MyNonTerm::Val),
        ]),

        MyNonTerm::Val => &Or(&[
            &Group(&And(&[ &Group(&Primitive(Identifier),MyGroup("idn")), &NonTerm(MyNonTerm::Call), ]),MyGroup("call_func")),

            &NonTerm(MyNonTerm::Primitive1),
            &NonTerm(MyNonTerm::Array),
            &NonTerm(MyNonTerm::Dict),
            &NonTerm(MyNonTerm::If),
            &NonTerm(MyNonTerm::Lambda),
            &NonTerm(MyNonTerm::Block),
            &And(&[ &NonTerm(MyNonTerm::LParen), &NonTerm(MyNonTerm::Expr), &NonTerm(MyNonTerm::RParen), ]),
        ]),

        MyNonTerm::Array => &Group(&And(&[
            &NonTerm(MyNonTerm::LSquare),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &NonTerm(MyNonTerm::Expr),
                    &Many(&And(&[&Primitive(Symbol(",")),&NonTerm(MyNonTerm::Expr),]),0,0),
                    &Or(&[&Primitive(Symbol(",")), &Always]),
                ]), &Always]),
                &NonTerm(MyNonTerm::RSquare),
            ]),"closing square bracket"),
        ]),MyGroup("array")),

        MyNonTerm::Dict => &Group(&And(&[
            &NonTerm(MyNonTerm::LCurly),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &NonTerm(MyNonTerm::DictVal),
                    &Many(&And(&[&Primitive(Symbol(",")),&NonTerm(MyNonTerm::DictVal),]),0,0),
                    &Or(&[&Primitive(Symbol(",")), &Always]),
                ]), &Always]),
                &NonTerm(MyNonTerm::RCurly),
            ]),"closing brace"),
        ]),MyGroup("dict")),

        MyNonTerm::DictVal => &Group(&And(&[
            &Expect(&Or(&[
                &Group(&Primitive(Identifier),MyGroup("name")),

                &Group(&Or(&[
                    &Primitive(Int), &Primitive(String),
                    &Primitive(Keyword("nil")),
                    &Primitive(Keyword("true")),
                    &Primitive(Keyword("false")),
                ]),MyGroup("primitive")),

            ]),"key"),
            &Expect(&Primitive(Symbol(":")),"colon"),
            &Or(&[&NonTerm(MyNonTerm::Expr),&Error,]), //not needed unlike in val's field_index_calls, because that was optional, this is not //needed?

        ]),MyGroup("dict_val")),

        MyNonTerm::Block => &Group(&And(&[
            &Expect(&NonTerm(MyNonTerm::LCurly),"block"),
            &Expect(&And(&[ &NonTerm(MyNonTerm::Stmts), &NonTerm(MyNonTerm::RCurly), ]),"closing brace"),
        ]),MyGroup("block")),

        MyNonTerm::FieldIndexCall => &Or(&[
            &Group(&And(&[&NonTerm(MyNonTerm::Field),&NonTerm(MyNonTerm::Call),]),MyGroup("call_field")),
            &Group(&Was(&NonTerm(MyNonTerm::Field)),MyGroup("field")),
            &Group(&Was(&NonTerm(MyNonTerm::Index)),MyGroup("index")),
            &Group(&NonTerm(MyNonTerm::Call),MyGroup("call_val")),
        ]),

        MyNonTerm::Call => &Group(&And(&[
            &NonTerm(MyNonTerm::LParen),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &NonTerm(MyNonTerm::Expr),
                    &Many(&And(&[ &Primitive(Symbol(",")), &NonTerm(MyNonTerm::Expr), ]),0,0),
                    &Or(&[&Primitive(Symbol(",")), &Always]),
                ]), &Always]),
                &NonTerm(MyNonTerm::RParen),
            ]),"closing bracket"),
        ]),MyGroup("params")),

        MyNonTerm::Field => &And(&[
            &Primitive(Symbol(".")),
            &Expect(&Or(&[
                &Group(&Primitive(Int),MyGroup("field_index")),
                &Group(&Primitive(Identifier),MyGroup("field_name")),
                // Error,
            ]),"field"),
        ]),

        MyNonTerm::Index => &And(&[
            &NonTerm(MyNonTerm::LSquare),
            &NonTerm(MyNonTerm::Expr),
            &Expect(&NonTerm(MyNonTerm::RSquare),"closing square bracket"),
        ]),

        MyNonTerm::Primitive1 => &Group(&Or(&[
            &Primitive(Int),
            &Primitive(Float),
            &Primitive(String),
            &Primitive(Identifier),
            &Primitive(Keyword("nil")),
            &Primitive(Keyword("void")),
            &Primitive(Keyword("true")),
            &Primitive(Keyword("false")),
        ]),MyGroup("primitive")),

        MyNonTerm::End => &Expect(&Many(&Or(&[&Primitive(Symbol(";")),&Primitive(Eol)]),1,0),"semicolon"),

        MyNonTerm::ForOp => &Or(&[
            &Group(&And(&[&NonTerm(MyNonTerm::ForToOp),&Primitive(Symbol("=")),]),MyGroup("to_eq")),
            &Group(&NonTerm(MyNonTerm::ForToOp),MyGroup("to")),
        ]),

        MyNonTerm::ForToOp => &And(&[&Primitive(Symbol(".")),&Primitive(Symbol(".")),]),

        MyNonTerm::SetOp => &Or(&[
            &Group(&Primitive(Symbol("=")),MyGroup("eq")),
            &And(&[ &NonTerm(MyNonTerm::SetSubOp), &Primitive(Symbol("=")), ]),
        ]),

        MyNonTerm::SetSubOp => &Or(&[
            &Group(&Primitive(Symbol("+")),MyGroup("add")),
            &Group(&Primitive(Symbol("-")),MyGroup("sub")),
            &Group(&Primitive(Symbol("*")),MyGroup("mul")),
            &Group(&Primitive(Symbol("/")),MyGroup("div")),
            &Group(&Primitive(Symbol("!")),MyGroup("not")),

            &Group(&And(&[&Primitive(Symbol("&")),&Primitive(Symbol("&")),]),MyGroup("and")),
            &Group(&And(&[&Primitive(Symbol("|")),&Primitive(Symbol("|")),]),MyGroup("or")),

            &Group(&Primitive(Symbol("^")),MyGroup("xor")),
        ]),


        MyNonTerm::FactorOp => &Or(&[
            &Group(&Primitive(Symbol("+")),MyGroup("add")),
            &Group(&Primitive(Symbol("-")),MyGroup("sub")),
        ]),

        MyNonTerm::TermOp => &Or(&[
            &Group(&Primitive(Symbol("*")),MyGroup("mul")),
            &Group(&Primitive(Symbol("/")),MyGroup("div")),
            &Group(&Primitive(Symbol("%")),MyGroup("mod")),
        ]),

        MyNonTerm::LCurly => &Primitive(Symbol("{")),
        MyNonTerm::RCurly => &Primitive(Symbol("}")),
        MyNonTerm::LSquare => &Primitive(Symbol("[")),
        MyNonTerm::RSquare => &Primitive(Symbol("]")),
        MyNonTerm::LParen => &Primitive(Symbol("(")),
        MyNonTerm::RParen => &Primitive(Symbol(")")),

        // _ => {return None;}
    })
}




impl<'t,'g> std::fmt::Debug for WalkGroupContainer<'g,MyGroup<'g>,TokenIterContainer<'t>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::{:?}", &self.group_ind,&self.name2().map(|x|x.0).unwrap_or("")))
    }
}

impl<'t,'g,> Display for WalkGroupContainer<'g,MyGroup<'g>,TokenIterContainer<'t>>

{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        enum Thing<'t,'g,> {
            Token(TokenContainer<'t>),
            Group(WalkGroupContainer<'g,MyGroup<'g>,TokenIterContainer<'t>>),
        }

        let mut stk = vec![(Thing::Group(self.clone()),0)];

        while let Some((cur,depth))=stk.pop() {
            let indent="    ".repeat(depth);

            match cur {
                Thing::Group(cur) => {
                    writeln!(f,"{indent}group: {:?}",cur.name2().map(|x|x.0).unwrap_or(""),)?;
                    let mut cur_tokens = cur.tokens();

                    for child_group in cur.children().rev() {
                        let child_tokens=child_group.tokens();
                        let ps_len=cur_tokens.end-child_tokens.end;
                        let ps=cur_tokens.pop_back_amount(ps_len).unwrap();

                        stk.extend(ps.map(|t|(Thing::Token(t),depth+1)).rev());
                        stk.push((Thing::Group(child_group),depth+1));
                        cur_tokens.pop_back_amount(child_tokens.len()).unwrap();
                    }

                    //
                    stk.extend(cur_tokens.map(|t|(Thing::Token(t),depth+1)).rev());
                }
                Thing::Token(cur) => {
                    writeln!(f,"{indent}{cur:?}")?;
                }
            }
        }

        Ok(())
    }
}