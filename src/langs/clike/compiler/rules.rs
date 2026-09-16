use std::{fmt::Display, rc::Rc};
use crate::clike::{grammar::{container::WalkGroupContainer, GrammarPrimitiveTrait}, tokenizer::{TokenContainer, TokenIterContainer}};

/*
TODO
* don't allow field_ind (eg a.5) to be used for int/float/bool
* don't allow spaces between the decimal point and numbers in a float?
** allow floats like .5 ?
*/
use super::super::grammar::node::*;

#[derive(Clone,Hash,PartialEq,Eq,Debug)]
pub enum GrammarPrimitive<'g> {
    String,
    Identifier,
    Int,
    Float,
    Symbol(&'g str),
    Keyword(&'g str),
    Eol,
}


impl<'g> GrammarPrimitive<'g> {
    fn to<NT>(self) -> GrammarNode<'g,NT,GrammarPrimitive<'g>> {
        GrammarNode::Primitive(self)
    }
}

impl<'g> GrammarPrimitiveTrait for GrammarPrimitive<'g> {
    fn is_trimmable(&self) -> bool {
        match self {
            GrammarPrimitive::Eol => true,
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

pub fn get_non_term<'g>(n:MyNonTerm) -> Option<&'g GrammarNode<'g,MyNonTerm,GrammarPrimitive<'g>>> {
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
    use GrammarPrimitive::*;
    use GrammarNode::*;
    use MyNonTerm::*;


    Some(match n {
        // "start" => [
        //     NonTerm(Val).many0(),
        //     NonTerm("end"),
        // ].and(),

        // "mynum" => Primitive(Int).group("n"),
        // "start2" => [
        //     [NonTerm("mynum"),NonTerm("mynum")].and().group("a"),
        //     NonTerm("mynum").group("b").group("b"),
        // ].or(),
        //
        Start => &NonTerm(Stmts),

        Stmts => &Or(&[&And(&[
            &Or(&[&NonTerm(End),&Always]),
            &NonTerm(Stmt),
            &Many(&And(&[
                &NonTerm(End),
                &NonTerm(Stmt),
            ])),
            &Or(&[&NonTerm(End),&Always]),
        ]), &Always]),

        Stmt => &Or(&[
            &NonTerm(Var),

            &NonTerm(For),
            &NonTerm(While),
            &NonTerm(Func),

            &NonTerm(Break),
            &NonTerm(Continue),
            &NonTerm(Return),

            &NonTerm(Include),

            &NonTerm(Format),
            &NonTerm(Print),
            &NonTerm(Println),

            &NonTerm(Set),
            &NonTerm(Expr),

        ]),

        Var => &And(&[
            &Primitive(Keyword("var")),
            &NonTerm(VarEntry),
            &Many(&And(&[ &Primitive(Symbol(",")), &NonTerm(VarEntry), ])),
            // Symbol(",").opt(),
        ]),

        VarEntry => &Group(&And(&[
            &Group(&Primitive(Identifier),"name"),
            &Or(&[&And(&[&Primitive(Symbol("=")), &NonTerm(Expr)]),&Always,]),
        ]), "var"),

        Set => &Or(&[
            &NonTerm(SetVar),
            &NonTerm(SetField),
            &NonTerm(SetIndex),
        ]),

        SetVar => &Group(&And(&[
            &Group(&Primitive(Identifier),"name"),
            &NonTerm(SetOp),
            &NonTerm(Expr),
        ]), "set_var"),

        SetField => &Group(&And(&[
            &NonTerm(Postfixes),
            &Had("field"),
            &NonTerm(SetOp),
            &NonTerm(Expr),
        ]), "set_field"),

        SetIndex => &Group(&And(&[
            &NonTerm(Postfixes),
            &Had("index"),
            &NonTerm(SetOp),
            &NonTerm(Expr),
        ]), "set_index"),

        Func => &Group(&And(&[
            &Primitive(Keyword("fn")),
            &Group(&Primitive(Identifier), "name"),
            &NonTerm(FuncParams),
            &NonTerm(Block),
        ]),"func"),

        Lambda => &Group(&And(&[
            &Primitive(Keyword("fn")),
            &NonTerm(FuncParams),
            &NonTerm(Block),
        ]), "lambda"),

        FuncParams => &And(&[
            &NonTerm(LParen),
            &Expect(&And(&[
                &Or(&[
                    &And(&[ &NonTerm(FuncParams2), &NonTerm(FuncVariadic), ]),
                    &And(&[ &NonTerm(FuncParams2), &NonTerm(FuncNotVariadic), ]),
                    &And(&[ &Group(&Always,"params"), &Group(&Always,"not_variadic"), ]),
                ]),
                &NonTerm(RParen),
            ]), "closing bracket")
        ]),

        FuncParams2 => &Group(&And(&[
            &Expect(&Group(&Primitive(Identifier),"param"), "param"),
            &Many(&And(&[ &Primitive(Symbol(",")), &Expect(&Group(&Primitive(Identifier),"param"), "param"), ])),
        ]),"params"),

        FuncVariadic => &Group(&And(&[&Primitive(Symbol(".")),&Primitive(Symbol(".")),&Primitive(Symbol(".")),]),"variadic"),
        FuncNotVariadic => &Group(&Or(&[&Primitive(Symbol(",")),&Always]), "not_variadic"),


        Format => &And(&[&Primitive(Keyword("format")),&NonTerm(FormatParams),]),
        Print => &Group(&And(&[&Primitive(Keyword("print")),&NonTerm(FormatParams),]), "print"),
        Println => &Group(&And(&[&Primitive(Keyword("println")),&NonTerm(FormatParams),]), "println"),

        FormatParams => &Group(&And(&[
            &NonTerm(LParen),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &Or(&[&Group(&Primitive(String), "string"), &NonTerm(Expr),]),
                    &Many(&And(&[&Primitive(Symbol(",")), &NonTerm(Expr),])),
                    &Or(&[&Primitive(Symbol(",")), &Always]),
                ]), &Always]),
                &NonTerm(RParen),
            ]),"closing bracket"),
        ]),"format"),

        If => &Group(&And(&[
            &Group(&And(&[&Primitive(Keyword("if")), &NonTerm(Expr), &NonTerm(Block), ]),"cond"),
            &Many(&Group(&And(&[&Primitive(Keyword("elif")), &NonTerm(Expr), &NonTerm(Block), ]),"cond")),
            &Or(&[&Group(&And(&[&Primitive(Keyword("else")),&NonTerm(Block),]),"else"), &Always]),
        ]),"if"),

        While => &Group(&And(&[
            &Primitive(Keyword("while")),
            &NonTerm(Expr),
            &NonTerm(Block),
        ]),"while"),

        For => &Group(&And(&[
            &Primitive(Keyword("for")),
            &Group(&Primitive(Identifier),"name"),
            &Primitive(Keyword("in")),
            &NonTerm(Expr),
            &NonTerm(ForOp),
            &NonTerm(Expr),
            &NonTerm(Block),
        ]),"for"),

        Continue => &Group(&Primitive(Keyword("continue")),"continue"),
        Break => &Group(&Primitive(Keyword("break")),"break"),
        Return => &Group(&And(&[&Primitive(Keyword("return")), &Or(&[&NonTerm(Expr), &Always]),]),"return"),

        Include => &And(&[&Primitive(Keyword("include")),&Group(&Primitive(String),"include"),]),

        Expr => &Expect(&Group(&NonTerm(Or1),"expr"),"expr"),

        Or1 => &Or(&[
            &Group(&And(&[
                &NonTerm(Xor),
                &And(&[
                    &And(&[ &And(&[&Primitive(Symbol("|")),&Expect(&Primitive(Symbol("|")),"or")]), &NonTerm(Xor), ]),
                    &Many(&And(&[ &And(&[&Primitive(Symbol("|")),&Expect(&Primitive(Symbol("|")),"or")]), &NonTerm(Xor), ])),
                ]),
            ]), "or"),
            &NonTerm(Xor),
        ]),

        Xor => &Or(&[
            &Group(&And(&[
                &NonTerm(And1),
                &And(&[
                    &And(&[ &Primitive(Symbol("^")), &NonTerm(And1), ]),
                    &Many(&And(&[ &Primitive(Symbol("^")), &NonTerm(And1), ])),
                ]),
            ]),"xor"),
            &NonTerm(And1),
        ]),

        And1 => &Or(&[
            &Group(&And(&[
                &NonTerm(Compare),
                &And(&[
                    &And(&[ &Primitive(Symbol("&")),&Expect(&Primitive(Symbol("&")),"and"), &NonTerm(Compare), ]),
                    &Many(&And(&[ &Primitive(Symbol("&")),&Expect(&Primitive(Symbol("&")),"and"), &NonTerm(Compare), ]))
                ]),
            ]),"and"),
            &NonTerm(Compare),
        ]),


        Compare => &Or(&[
            &Group(&And(&[&NonTerm(Factor), &Primitive(Symbol("<")), &NonTerm(Factor),]),"lt"),
            &Group(&And(&[&NonTerm(Factor), &Primitive(Symbol(">")), &NonTerm(Factor),]),"gt"),
            &Group(&And(&[&NonTerm(Factor), &Primitive(Symbol("<")),&Primitive(Symbol("=")), &NonTerm(Factor),]),"le"),
            &Group(&And(&[&NonTerm(Factor), &Primitive(Symbol(">")),&Primitive(Symbol("=")), &NonTerm(Factor),]),"ge"),
            &Group(&And(&[&NonTerm(Factor), &Primitive(Symbol("=")),&Expect(&Primitive(Symbol("=")),"eq"), &NonTerm(Factor),]),"eq"),
            &Group(&And(&[&NonTerm(Factor), &Primitive(Symbol("!")),&Expect(&Primitive(Symbol("=")),"ne"), &NonTerm(Factor),]),"ne"),
            &NonTerm(Factor),
        ]),

        Factor => &Or(&[
            &Group(&And(&[
                &NonTerm(Term),
                &And(&[
                    &And(&[ &NonTerm(FactorOp), &NonTerm(Term), ]),
                    &Many(&And(&[ &NonTerm(FactorOp), &NonTerm(Term), ]))
                ]),
            ]),"factor"),
            &NonTerm(Term),
        ]),

        Term => &Or(&[
            &Group(&And(&[
                &NonTerm(Prefixes),
                &And(&[
                    &And(&[&NonTerm(TermOp),&NonTerm(Prefixes),]),
                    &Many(&And(&[&NonTerm(TermOp),&NonTerm(Prefixes),]))
                ]),
            ]),"term"),
            &NonTerm(Prefixes),
        ]),


        Prefixes => &Or(&[
            &Group(&And(&[
                &Group(&And(&[
                    &Or(&[
                        &Primitive(Symbol("+")),
                        &Group(&Primitive(Symbol("-")),"neg"),
                        &Group(&Primitive(Symbol("!")),"not"),
                    ]),
                    &Many(&Or(&[
                        &Primitive(Symbol("+")),
                        &Group(&Primitive(Symbol("-")),"neg"),
                        &Group(&Primitive(Symbol("!")),"not"),
                    ])),
                ]),"prefixes"),
                &NonTerm(Postfixes),
            ]),"prefixes"),
            &NonTerm(Postfixes),
        ]),

        Postfixes => &Or(&[
            &Group(&And(&[
                &NonTerm(Val),
                &Group(&And(&[
                    &NonTerm(FieldIndexCall),
                    &Many(&NonTerm(FieldIndexCall)),
                ]),"field_index_calls"),
            ]),"postfixes"),
            &NonTerm(Val),
        ]),

        Val => &Or(&[
            &Group(&And(&[ &Group(&Primitive(Identifier),"idn"), &NonTerm(Call), ]),"call_func"),

            &NonTerm(Primitive1),
            &NonTerm(Array),
            &NonTerm(Dict),
            &NonTerm(If),
            &NonTerm(Lambda),
            &NonTerm(Block),
            &And(&[ &NonTerm(LParen), &NonTerm(Expr), &NonTerm(RParen), ]),
        ]),

        Array => &Group(&And(&[
            &NonTerm(LSquare),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &NonTerm(Expr),
                    &Many(&And(&[&Primitive(Symbol(",")),&NonTerm(Expr),])),
                    &Or(&[&Primitive(Symbol(",")), &Always]),
                ]), &Always]),
                &NonTerm(RSquare),
            ]),"closing square bracket"),
        ]),"array"),

        Dict => &Group(&And(&[
            &NonTerm(LCurly),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &NonTerm(DictVal),
                    &Many(&And(&[&Primitive(Symbol(",")),&NonTerm(DictVal),])),
                    &Or(&[&Primitive(Symbol(",")), &Always]),
                ]), &Always]),
                &NonTerm(RCurly),
            ]),"closing brace"),
        ]),"dict"),

        DictVal => &Group(&And(&[
            &Expect(&Or(&[
                &Group(&Primitive(Identifier),"name"),

                &Group(&Or(&[
                    &Primitive(Int), &Primitive(String),
                    &Primitive(Keyword("nil")),
                    &Primitive(Keyword("true")),
                    &Primitive(Keyword("false")),
                ]),"primitive"),

            ]),"key"),
            &Expect(&Primitive(Symbol(":")),"colon"),
            &Or(&[&NonTerm(Expr),&Error,]), //not needed unlike in val's field_index_calls, because that was optional, this is not //needed?

        ]),"dict_val"),

        Block => &Group(&And(&[
            &Expect(&NonTerm(LCurly),"block"),
            &Expect(&And(&[ &NonTerm(Stmts), &NonTerm(RCurly), ]),"closing brace"),
        ]),"block"),

        FieldIndexCall => &Or(&[
            &Group(&And(&[&NonTerm(Field),&NonTerm(Call),]),"call_field"),
            &Group(&NonTerm(Field),"field"),
            &Group(&NonTerm(Index),"index"),
            &Group(&NonTerm(Call),"call_val"),
        ]),

        Call => &Group(&And(&[
            &NonTerm(LParen),
            &Expect(&And(&[
                &Or(&[&And(&[
                    &NonTerm(Expr),
                    &Many(&And(&[ &Primitive(Symbol(",")), &NonTerm(Expr), ])),
                    &Or(&[&Primitive(Symbol(",")), &Always]),
                ]), &Always]),
                &NonTerm(RParen),
            ]),"closing bracket"),
        ]),"params"),

        Field => &Was(&And(&[
            &Primitive(Symbol(".")),
            &Expect(&Or(&[
                &Group(&Primitive(Int),"field_index"),
                &Group(&Primitive(Identifier),"field_name"),
                // Error,
            ]),"field"),
        ]),"field"),

        Index => &Was(&And(&[
            &NonTerm(LSquare),
            &NonTerm(Expr),
            &Expect(&NonTerm(RSquare),"closing square bracket"),
        ]),"index"),

        Primitive1 => &Group(&Or(&[
            &Primitive(Int),
            &Primitive(Float),
            &Primitive(String),
            &Primitive(Identifier),
            &Primitive(Keyword("nil")),
            &Primitive(Keyword("void")),
            &Primitive(Keyword("true")),
            &Primitive(Keyword("false")),
        ]),"primitive"),

        End => &Expect(&And(&[
            &Or(&[&Primitive(Symbol(";")),&Primitive(Eol)]),
            &Many(&Or(&[&Primitive(Symbol(";")),&Primitive(Eol)])),
        ]),"semicolon"),

        ForOp => &Or(&[
            &Group(&And(&[&NonTerm(ForToOp),&Primitive(Symbol("=")),]),"to_eq"),
            &Group(&NonTerm(ForToOp),"to"),
        ]),

        ForToOp => &And(&[&Primitive(Symbol(".")),&Primitive(Symbol(".")),]),

        SetOp => &Or(&[
            &Group(&Primitive(Symbol("=")),"eq"),
            &And(&[ &NonTerm(SetSubOp), &Primitive(Symbol("=")), ]),
        ]),

        SetSubOp => &Or(&[
            &Group(&Primitive(Symbol("+")),"add"),
            &Group(&Primitive(Symbol("-")),"sub"),
            &Group(&Primitive(Symbol("*")),"mul"),
            &Group(&Primitive(Symbol("/")),"div"),
            &Group(&Primitive(Symbol("!")),"not"),

            &Group(&And(&[&Primitive(Symbol("&")),&Primitive(Symbol("&")),]),"and"),
            &Group(&And(&[&Primitive(Symbol("|")),&Primitive(Symbol("|")),]),"or"),

            &Group(&Primitive(Symbol("^")),"xor"),
        ]),


        FactorOp => &Or(&[
            &Group(&Primitive(Symbol("+")),"add"),
            &Group(&Primitive(Symbol("-")),"sub"),
        ]),

        TermOp => &Or(&[
            &Group(&Primitive(Symbol("*")),"mul"),
            &Group(&Primitive(Symbol("/")),"div"),
            &Group(&Primitive(Symbol("%")),"mod"),
        ]),

        LCurly => &Primitive(Symbol("{")),
        RCurly => &Primitive(Symbol("}")),
        LSquare => &Primitive(Symbol("[")),
        RSquare => &Primitive(Symbol("]")),
        LParen => &Primitive(Symbol("(")),
        RParen => &Primitive(Symbol(")")),

        // _ => {return None;}
    })
}




impl<'t,'g> std::fmt::Debug for WalkGroupContainer<'g,TokenIterContainer<'t>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}::{:?}", &self.group_ind,&self.name()))
    }
}

impl<'t,'g,> Display for WalkGroupContainer<'g,TokenIterContainer<'t>>

{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        enum Thing<'t,'g,> {
            Token(TokenContainer<'t>),
            Group(WalkGroupContainer<'g,TokenIterContainer<'t>>),
        }

        let mut stk = vec![(Thing::Group(*self),0)];

        while let Some((cur,depth))=stk.pop() {
            let indent="    ".repeat(depth);

            match cur {
                Thing::Group(cur) => {
                    writeln!(f,"{indent}group: {:?}",cur.name(),)?;
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