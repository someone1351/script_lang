use std::{fmt::Display, ops::Deref, rc::Rc};
use crate::clike::{compiler::{tokeniser2::{MyKeyword, MySymbol}, tokens::Tokens}, grammar::{container::WalkGroupContainer, PrimitiveTrait} };

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

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum MyPrimitive {
    String,
    Identifier,
    Int,
    Float,
    Symbol(MySymbol),
    Keyword(MyKeyword),
    Eol,
}


// impl<'g> GrammarPrimitive<'g> {
//     fn to<NT>(self) -> GrammarNode<'g,NT,GrammarPrimitive<'g>> {
//         GrammarNode::Primitive(self)
//     }
// }

impl PrimitiveTrait for MyPrimitive {
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
    BitXor,
    BitAnd,
    BitOr,
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
    // SetSubOp,
    FactorOp,
    TermOp,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
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

pub fn get_non_term<'g>(n:MyNonTerm) -> Option<&'g GrammarNode<'g,MyNonTerm,MyPrimitive,MyGroup<'g>>> {
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
            &Primitive(Keyword(MyKeyword::Var)),
            &NonTerm(MyNonTerm::VarEntry),
            &Many(&And(&[ &Primitive(Symbol(MySymbol::Comma)), &NonTerm(MyNonTerm::VarEntry), ]),0,0),
            // Symbol(MySymbol::Comma).opt(),
        ]),

        MyNonTerm::VarEntry => &Group(&And(&[
            &Group(&Primitive(Identifier),MyGroup("name")),
            &Or(&[&And(&[&Primitive(Symbol(MySymbol::Set)), &NonTerm(MyNonTerm::Expr)]),&Always,]),
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
            &Primitive(Keyword(MyKeyword::Fn)),
            &Group(&Primitive(Identifier), MyGroup("name")),
            &NonTerm(MyNonTerm::FuncParams),
            &NonTerm(MyNonTerm::Block),
        ]),MyGroup("func")),

        MyNonTerm::Lambda => &Group(&And(&[
            &Primitive(Keyword(MyKeyword::Fn)),
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
            &Many(&And(&[ &Primitive(Symbol(MySymbol::Comma)), &Expect(&Group(&Primitive(Identifier),MyGroup("param")), "param"), ]),0,0),
        ]),MyGroup("params")),

        MyNonTerm::FuncVariadic => &Group(&And(&[&Primitive(Symbol(MySymbol::Dot)),&Primitive(Symbol(MySymbol::Dot)),&Primitive(Symbol(MySymbol::Dot)),]),MyGroup("variadic")),
        MyNonTerm::FuncNotVariadic => &Group(&Or(&[&Primitive(Symbol(MySymbol::Comma)),&Always]), MyGroup("not_variadic")),


        MyNonTerm::Format => &And(&[&Primitive(Keyword(MyKeyword::Format)),&NonTerm(MyNonTerm::FormatParams),]),
        MyNonTerm::Print => &Group(&And(&[&Primitive(Keyword(MyKeyword::Print)),&NonTerm(MyNonTerm::FormatParams),]), MyGroup("print")),
        MyNonTerm::Println => &Group(&And(&[&Primitive(Keyword(MyKeyword::Println)),&NonTerm(MyNonTerm::FormatParams),]), MyGroup("println")),

        MyNonTerm::FormatParams => &Group(&And(&[
            &NonTerm(MyNonTerm::LParen),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &Or(&[&Group(&Primitive(String), MyGroup("string")), &NonTerm(MyNonTerm::Expr),]),
                    &Many(&And(&[&Primitive(Symbol(MySymbol::Comma)), &NonTerm(MyNonTerm::Expr),]),0,0),
                    &Or(&[&Primitive(Symbol(MySymbol::Comma)), &Always]),
                ]), &Always]),
                &NonTerm(MyNonTerm::RParen),
            ]),"closing bracket"),
        ]),MyGroup("format")),

        MyNonTerm::If => &Group(&And(&[
            &Group(&And(&[&Primitive(Keyword(MyKeyword::If)), &NonTerm(MyNonTerm::Expr), &NonTerm(MyNonTerm::Block), ]),MyGroup("cond")),
            &Many(&Group(&And(&[&Primitive(Keyword(MyKeyword::Elif)), &NonTerm(MyNonTerm::Expr), &NonTerm(MyNonTerm::Block), ]),MyGroup("cond")),0,0),
            &Or(&[&Group(&And(&[&Primitive(Keyword(MyKeyword::Else)),&NonTerm(MyNonTerm::Block),]),MyGroup("else")), &Always]),
        ]),MyGroup("if")),

        MyNonTerm::While => &Group(&And(&[
            &Primitive(Keyword(MyKeyword::While)),
            &NonTerm(MyNonTerm::Expr),
            &NonTerm(MyNonTerm::Block),
        ]),MyGroup("while")),

        MyNonTerm::For => &Group(&And(&[
            &Primitive(Keyword(MyKeyword::For)),
            &Group(&Primitive(Identifier),MyGroup("name")),
            &Primitive(Keyword(MyKeyword::In)),
            &NonTerm(MyNonTerm::Expr),
            &NonTerm(MyNonTerm::ForOp),
            &NonTerm(MyNonTerm::Expr),
            &NonTerm(MyNonTerm::Block),
        ]),MyGroup("for")),

        MyNonTerm::Continue => &Group(&Primitive(Keyword(MyKeyword::Continue)),MyGroup("continue")),
        MyNonTerm::Break => &Group(&Primitive(Keyword(MyKeyword::Break)),MyGroup("break")),
        MyNonTerm::Return => &Group(&And(&[&Primitive(Keyword(MyKeyword::Return)), &Or(&[&NonTerm(MyNonTerm::Expr), &Always]),]),MyGroup("return")),

        MyNonTerm::Include => &And(&[&Primitive(Keyword(MyKeyword::Include)),&Group(&Primitive(String),MyGroup("include")),]),

        MyNonTerm::Expr => &Expect(&Group(&NonTerm(MyNonTerm::Or1),MyGroup("expr")),"expr"),
        // MyNonTerm::Expr => &Expect(&Group(&NonTerm(MyNonTerm::Factor),MyGroup("expr")),"expr"),
        // // MyNonTerm::Expr => &Expect(&Group(&NonTerm(MyNonTerm::Term),MyGroup("expr")),"expr"),

        MyNonTerm::Or1 => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::And1),
                &Many(&And(&[ &Primitive(Symbol(MySymbol::Or)), &NonTerm(MyNonTerm::And1), ]),1,0),
            ]), MyGroup("or")),
            &NonTerm(MyNonTerm::And1),
        ]),

        MyNonTerm::And1 => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::Compare),
                &Many(&And(&[ &Primitive(Symbol(MySymbol::And)), &NonTerm(MyNonTerm::Compare), ]),1,0),
            ]),MyGroup("and")),
            &NonTerm(MyNonTerm::Compare),
        ]),

        MyNonTerm::Compare => &Or(&[
            &Group(&And(&[&NonTerm(MyNonTerm::BitOr), &Primitive(Symbol(MySymbol::LessThan)), &NonTerm(MyNonTerm::BitOr),]),MyGroup("lt")),
            &Group(&And(&[&NonTerm(MyNonTerm::BitOr), &Primitive(Symbol(MySymbol::GreaterThan)), &NonTerm(MyNonTerm::BitOr),]),MyGroup("gt")),
            &Group(&And(&[&NonTerm(MyNonTerm::BitOr), &Primitive(Symbol(MySymbol::LessThanEqual)),&NonTerm(MyNonTerm::BitOr),]),MyGroup("le")),
            &Group(&And(&[&NonTerm(MyNonTerm::BitOr), &Primitive(Symbol(MySymbol::GreaterThanEqual)), &NonTerm(MyNonTerm::BitOr),]),MyGroup("ge")),
            &Group(&And(&[&NonTerm(MyNonTerm::BitOr), &Primitive(Symbol(MySymbol::EqualTo)), &NonTerm(MyNonTerm::BitOr),]),MyGroup("eq")),
            &Group(&And(&[&NonTerm(MyNonTerm::BitOr), &Primitive(Symbol(MySymbol::NotEqualTo)), &NonTerm(MyNonTerm::BitOr),]),MyGroup("ne")),
            &NonTerm(MyNonTerm::BitOr),
        ]),

        MyNonTerm::BitOr => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::BitXor),
                &Many(&And(&[ &Primitive(Symbol(MySymbol::BitOr)), &NonTerm(MyNonTerm::BitXor), ]),1,0),
            ]),MyGroup("xor")),
            &NonTerm(MyNonTerm::BitXor),
        ]),

        MyNonTerm::BitXor => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::BitAnd),
                &Many(&And(&[ &Primitive(Symbol(MySymbol::BitXor)), &NonTerm(MyNonTerm::BitAnd), ]),1,0),
            ]),MyGroup("xor")),
            &NonTerm(MyNonTerm::BitAnd),
        ]),

        MyNonTerm::BitAnd => &Or(&[
            &Group(&And(&[
                &NonTerm(MyNonTerm::Factor),
                &Many(&And(&[ &Primitive(Symbol(MySymbol::BitAnd)), &NonTerm(MyNonTerm::Factor), ]),1,0),
            ]),MyGroup("xor")),
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
                        &Primitive(Symbol(MySymbol::Add)),
                        &Group(&Primitive(Symbol(MySymbol::Sub)),MyGroup("neg")),
                        &Group(&Primitive(Symbol(MySymbol::Not)),MyGroup("not")),
                        &Group(&Primitive(Symbol(MySymbol::BitNot)),MyGroup("bit_not")),
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
            &NonTerm(MyNonTerm::LBracket),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &NonTerm(MyNonTerm::Expr),
                    &Many(&And(&[&Primitive(Symbol(MySymbol::Comma)),&NonTerm(MyNonTerm::Expr),]),0,0),
                    &Or(&[&Primitive(Symbol(MySymbol::Comma)), &Always]),
                ]), &Always]),
                &NonTerm(MyNonTerm::RBracket),
            ]),"closing square bracket"),
        ]),MyGroup("array")),

        MyNonTerm::Dict => &Group(&And(&[
            &NonTerm(MyNonTerm::LBrace),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &NonTerm(MyNonTerm::DictVal),
                    &Many(&And(&[&Primitive(Symbol(MySymbol::Comma)),&NonTerm(MyNonTerm::DictVal),]),0,0),
                    &Or(&[&Primitive(Symbol(MySymbol::Comma)), &Always]),
                ]), &Always]),
                &NonTerm(MyNonTerm::RBrace),
            ]),"closing brace"),
        ]),MyGroup("dict")),

        MyNonTerm::DictVal => &Group(&And(&[
            &Expect(&Or(&[
                &Group(&Primitive(Identifier),MyGroup("name")),

                &Group(&Or(&[
                    &Primitive(Int), &Primitive(String),
                    &Primitive(Keyword(MyKeyword::Nil)),
                    &Primitive(Keyword(MyKeyword::True)),
                    &Primitive(Keyword(MyKeyword::False)),
                ]),MyGroup("primitive")),

            ]),"key"),
            &Expect(&Primitive(Symbol(MySymbol::SemiColon)),"colon"),
            &Or(&[&NonTerm(MyNonTerm::Expr),&Error(""),]), //not needed unlike in val's field_index_calls, because that was optional, this is not //needed?

        ]),MyGroup("dict_val")),

        MyNonTerm::Block => &Group(&And(&[
            &Expect(&NonTerm(MyNonTerm::LBrace),"block"),
            &Expect(&And(&[ &NonTerm(MyNonTerm::Stmts), &NonTerm(MyNonTerm::RBrace), ]),"closing brace"),
        ]),MyGroup("block")),

        MyNonTerm::FieldIndexCall => &Or(&[
            &Group(&And(&[&NonTerm(MyNonTerm::Field),&NonTerm(MyNonTerm::Call),]),MyGroup("call_field")),
            &Group(&Has(&NonTerm(MyNonTerm::Field)),MyGroup("field")),
            &Group(&Has(&NonTerm(MyNonTerm::Index)),MyGroup("index")),
            &Group(&NonTerm(MyNonTerm::Call),MyGroup("call_val")),
        ]),

        MyNonTerm::Call => &Group(&And(&[
            &NonTerm(MyNonTerm::LParen),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &NonTerm(MyNonTerm::Expr),
                    &Many(&And(&[ &Primitive(Symbol(MySymbol::Comma)), &NonTerm(MyNonTerm::Expr), ]),0,0),
                    &Or(&[&Primitive(Symbol(MySymbol::Comma)), &Always]),
                ]), &Always]),
                &NonTerm(MyNonTerm::RParen),
            ]),"closing bracket"),
        ]),MyGroup("params")),

        MyNonTerm::Field => &And(&[
            &Primitive(Symbol(MySymbol::Dot)),
            &Expect(&Or(&[
                &Group(&Primitive(Int),MyGroup("field_index")),
                &Group(&Primitive(Identifier),MyGroup("field_name")),
                // Error,
            ]),"field"),
        ]),

        MyNonTerm::Index => &And(&[
            &NonTerm(MyNonTerm::LBracket),
            &NonTerm(MyNonTerm::Expr),
            &Expect(&NonTerm(MyNonTerm::RBracket),"closing square bracket"),
        ]),

        MyNonTerm::Primitive1 => &Group(&Or(&[
            &Primitive(Int),
            &Primitive(Float),
            &Primitive(String),
            &Primitive(Identifier),
            &Primitive(Keyword(MyKeyword::Nil)),
            &Primitive(Keyword(MyKeyword::Void)),
            &Primitive(Keyword(MyKeyword::True)),
            &Primitive(Keyword(MyKeyword::False)),
        ]),MyGroup("primitive")),

        MyNonTerm::End => &Expect(&Many(&Or(&[&Primitive(Symbol(MySymbol::SemiColon)),&Primitive(Eol)]),1,0),"semicolon"),

        MyNonTerm::ForOp => &Or(&[
            &Group(&And(&[&NonTerm(MyNonTerm::ForToOp),&Primitive(Symbol(MySymbol::Set)),]),MyGroup("to_eq")),
            &Group(&NonTerm(MyNonTerm::ForToOp),MyGroup("to")),
        ]),

        MyNonTerm::ForToOp => &And(&[&Primitive(Symbol(MySymbol::Dot)),&Primitive(Symbol(MySymbol::Dot)),]),

        MyNonTerm::SetOp => &Or(&[
            &Group(&Primitive(Symbol(MySymbol::Set)),MyGroup("eq")),

            &Or(&[
                &Group(&Primitive(Symbol(MySymbol::SetAdd)),MyGroup("add")),
                &Group(&Primitive(Symbol(MySymbol::SetSub)),MyGroup("sub")),
                &Group(&Primitive(Symbol(MySymbol::SetMul)),MyGroup("mul")),
                &Group(&Primitive(Symbol(MySymbol::SetDiv)),MyGroup("div")),

                &Group(&Primitive(Symbol(MySymbol::SetBitAnd)),MyGroup("bit_and")),
                &Group(&Primitive(Symbol(MySymbol::SetBitOr)),MyGroup("bit_or")),
                &Group(&Primitive(Symbol(MySymbol::SetBitXor)),MyGroup("bit_xor")),

                &Group(&Primitive(Symbol(MySymbol::SetAnd)),MyGroup("and")),
                &Group(&Primitive(Symbol(MySymbol::SetOr)),MyGroup("or")),

            ]),

        ]),



        MyNonTerm::FactorOp => &Or(&[
            &Group(&Primitive(Symbol(MySymbol::Add)),MyGroup("add")),
            &Group(&Primitive(Symbol(MySymbol::Sub)),MyGroup("sub")),
        ]),

        MyNonTerm::TermOp => &Or(&[
            &Group(&Primitive(Symbol(MySymbol::Mul)),MyGroup("mul")),
            &Group(&Primitive(Symbol(MySymbol::Div)),MyGroup("div")),
            &Group(&Primitive(Symbol(MySymbol::Mod)),MyGroup("mod")),
        ]),

        MyNonTerm::LBrace => &Primitive(Symbol(MySymbol::LBrace)),
        MyNonTerm::RBrace => &Primitive(Symbol(MySymbol::RBrace)),
        MyNonTerm::LBracket => &Primitive(Symbol(MySymbol::LBracket)),
        MyNonTerm::RBracket => &Primitive(Symbol(MySymbol::RBracket)),
        MyNonTerm::LParen => &Primitive(Symbol(MySymbol::LParen)),
        MyNonTerm::RParen => &Primitive(Symbol(MySymbol::RParen)),

        // _ => {return None;}
    })
}




impl<'t,'g> std::fmt::Debug for WalkGroupContainer<'g,MyGroup<'g>,Tokens<'t>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::{:?}", &self.group_ind,&self.name2().map(|x|x.0).unwrap_or("")))

        // match self {
        //     WalkGroupContainer::Group { walk, group_ind } => {
        //         f.write_fmt(format_args!("g{}::{:?}", &group_ind,&self.name2().map(|x|x.0).unwrap_or("")))
        //     }
        //     WalkGroupContainer::Between { walk, parent_ind, between_ind } => {
        //         f.write_fmt(format_args!("b{between_ind}::p{parent_ind}", ))

        //     }
        // }
    }
}

// impl<'t,'g,> Display for WalkGroupContainer<'g,MyGroup<'g>,TokenIterContainer<'t>>

// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         enum Thing<'t,'g,> {
//             Token(TokenContainer<'t>),
//             Group(WalkGroupContainer<'g,MyGroup<'g>,TokenIterContainer<'t>>),
//         }

//         let mut stk = vec![(Thing::Group(self.clone()),0)];

//         while let Some((cur,depth))=stk.pop() {
//             let indent="    ".repeat(depth);

//             match cur {
//                 Thing::Group(cur) => {
//                     writeln!(f,"{indent}group: {:?}",cur.name2().map(|x|x.0).unwrap_or(""),)?;
//                     let mut cur_tokens = cur.tokens();

//                     for child_group in cur.children().rev() {
//                         let child_tokens=child_group.tokens();
//                         let ps_len=cur_tokens.end-child_tokens.end;
//                         let ps=cur_tokens.pop_back_amount(ps_len).unwrap();

//                         stk.extend(ps.map(|t|(Thing::Token(t),depth+1)).rev());
//                         stk.push((Thing::Group(child_group),depth+1));
//                         cur_tokens.pop_back_amount(child_tokens.len()).unwrap();
//                     }

//                     //
//                     stk.extend(cur_tokens.map(|t|(Thing::Token(t),depth+1)).rev());
//                 }
//                 Thing::Token(cur) => {
//                     writeln!(f,"{indent}{cur:?}")?;
//                 }
//             }
//         }

//         Ok(())
//     }
// }