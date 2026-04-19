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

use crate::{ccexpr_parser::{PrimitiveContainer, PrimitiveIterContainer, ValueContainer}, Loc};

#[derive(Clone,Debug,Hash,PartialEq,Eq)]
pub enum GrammarItem<'a> {
    Many(Box<GrammarItem<'a>>),
    // Many1(Box<GrammarItem<'a>>),
    And(Vec<GrammarItem<'a>>), //should store reversed?
    Or(Vec<GrammarItem<'a>>), //should store reversed?
    Opt(Box<GrammarItem<'a>>),
    Offer(Box<GrammarItem<'a>>),
    Take(Box<GrammarItem<'a>>),
    Group(&'a str,Box<GrammarItem<'a>>),

    // List(Box<GrammarItem<'a>>,Box<GrammarItem<'a>>), //val,sep
    // ListNoTrail(Box<GrammarItem<'a>>,Box<GrammarItem<'a>>), //val,sep

    String,
    Identifier,
    Int,
    Float,
    Symbol(&'a str),
    Keyword(&'a str),
    Eol,

    NonTerm(&'a str),
    Always, //always succeeds
    // Never, //replace with Error ?
    Error(GrammarWalkError<'a>),
    // Not(Box<GrammarItem<'a>>), //todo, needed? better to have NotIdentifier etc?
    Discard(Box<GrammarItem<'a>>), //todo, removes token from output (via just hding it, ie have hashmap of tokens to hide)
}

impl<'a> GrammarItem<'a> {
    pub fn many0(self) -> GrammarItem<'a> {
        Self::Many(self.into())
    }
    pub fn many1(self) -> GrammarItem<'a> {
        let x=self.clone();
        [x,self.many0(),].and()
        // Self::Many1(self.into())
    }
    pub fn opt(self) -> GrammarItem<'a> {
        Self::Opt(self.into())
    }
    pub fn group(self,name: &'a str) -> GrammarItem<'a> {
        Self::Group(name,self.into())
    }
    pub fn discard(self,) -> GrammarItem<'a> {
        Self::Discard(self.into())
    }
    pub fn offer(self,) -> GrammarItem<'a> {
        Self::Offer(self.into())
    }
    pub fn take(self,) -> GrammarItem<'a> {
        Self::Take(self.into())
    }
    pub fn d(self,) -> GrammarItem<'a> {
        self.discard()
    }
    pub fn is_many(&self) -> bool {
        if let GrammarItem::Many(_)=self {
            true
        } else {
            false
        }
        // match self {
        //     GrammarItem::Many(_)|GrammarItem::Many1(_) => true,
        //     _ =>false,
        // }
    }
}

// impl<'a, const N: usize> From<[GrammarItem<'a>; N]> for  GrammarItem<'a> {
//     fn from(value: [GrammarItem<'a>; N]) -> Self {
//         Self::And(value.into())
//     }
// }

// #[macro_export]
// macro_rules! and {
//     ( $( $x:expr ),* $(,)? ) => {{
//         let mut v = Vec::new();
//         $( v.push($x); )*
//         GrammarItem::And(v.into())
//     }};
// }

// #[macro_export]
// macro_rules! or {
//     ( $( $x:expr ),* $(,)? ) => {{
//         let mut v = Vec::new();
//         $( v.push($x); )*
//         GrammarItem::And(v.into())
//     }};
// }

//todo have array stored in rev for or/and
trait GrammarArrayTrait<'a> {
    fn and(&self) -> GrammarItem<'a>;
    fn or(&self) -> GrammarItem<'a>;
}
impl<'a,const N: usize> GrammarArrayTrait <'a> for [GrammarItem<'a>; N] {
    fn and(&self) -> GrammarItem<'a> {
        GrammarItem::And(self.into())
    }
    fn or(&self) -> GrammarItem<'a> {
        GrammarItem::Or(self.into())

    }
}
pub fn grammar_decl<'a>(n:&'a str) -> GrammarItem<'a> {
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
    use GrammarItem::*;
    match n {
        // "test" => [Int].and(),
        "test" => [Int,Float.opt()].and().many0(),
        "test2" => [Int,Float.opt()].and(),
        "test3" => [ Int.many0() ].and().opt(),

        "test4" => [[Int,String.opt(),].and(),Identifier,].or().opt(), //or(and(int,str?),idn)?
        "test5" =>  Int.many0(),
        "test6" =>  Int.many0().opt(),
        // "test7" =>  [ [ Int,String ].and(), Float, ].or(), //or(and(int,str),float)
        "test7" =>  [ Int.many0(), String, ].or().many0(), //.opt(), // many0(or(many0(int),str))

        "test8" => [
            Symbol("+"),
            Int,
        ].and(),

        "test9" => [
            // // Int.many0().group("a"),
            // // Float.many0().group("b"),
            // // // String.many0().group("c"),
            // NonTerm("x").many0(), //.group("a"),
            // NonTerm("x").take(), //.group("b"),
            Int.offer().many0().group("a"),
            Int.take().group("b"),
            // Eol.many0(),
        ].and(),
        // "x" => Int,

        "test10" => [
            [
                Identifier,
                // NonTerm("val_field_index").offer(),
                // // [ NonTerm("val_index"), NonTerm("val_field"), ].or(),
                // // NonTerm("val_field_index_call").many0(),
                // // [ NonTerm("val_field_index").offer(), NonTerm("call"), ].or(),
                // NonTerm("val_field_index").take(),

                NonTerm("val_field").offer(), //.opt(),
                NonTerm("val_field").take(),
            ].and(),
            NonTerm("set_equal"),
            NonTerm("int"),
        ].and(),

        "start" => [
            NonTerm("stmts"),
            NonTerm("ending").many0(),
        ].and(),

        "ending" => [NonTerm("semicolon"),Eol].or().many1().d(),
        "stmts" => [
            NonTerm("stmt"),
            [NonTerm("ending"), NonTerm("stmt"),].and().many0(),
            NonTerm("ending").many0(),
        ].and().opt(),

        "stmt" => [
            // NonTerm("var"),
            NonTerm("set"),
            // NonTerm("func"),
            // NonTerm("while"),NonTerm("for_in"),NonTerm("for_to"),
            // NonTerm("break"), NonTerm("continue"),
            // NonTerm("return"),
            // NonTerm("include"),
            // NonTerm("format"),NonTerm("print"),NonTerm("println"),
            // NonTerm("expr"),
            // NonTerm("block"), //after expr, so dict can use the empty {}
            // // NonTerm("if"),
        ].or().group("stmt"),

        "continue" => Keyword("continue"),
        "break" => Keyword("break"),
        "return" => [Keyword("return"), NonTerm("expr").opt(),].and(),

        "var_set" => [Identifier, NonTerm("set_equal"),NonTerm("expr")].and(),
        "var" => [
            Keyword("var"), NonTerm("var_set"),
            [NonTerm("comma"),NonTerm("var_set"),].and().many0(),
        ].and(),

        "set" => [
            [
                // NonTerm("val_field_index").offer().
                [Identifier,NonTerm("val_field_index_call").many0(), NonTerm("val_field_index").take(),].and(),
                Identifier,
            ].or(),
            // [NonTerm("add"),NonTerm("sub"),NonTerm("mul"),NonTerm("div"),NonTerm("not")].or().opt(),
            NonTerm("set_equal"),
            NonTerm("expr"),
        ].and(),

        "cond" => [NonTerm("lparen"),NonTerm("expr"),NonTerm("rparen"),].and(),
        "block" => [NonTerm("lcurly"),NonTerm("stmts"),NonTerm("rcurly"),].and(),
        "if" => [
            [Keyword("if").d(), NonTerm("cond"), NonTerm("block")].and().group(""),
            [Keyword("elif").d(),NonTerm("cond"),NonTerm("block"),].and().group("").many0(),
            [Keyword("else").d(),NonTerm("block"),].and().group("").opt(),
        ].and().group("if"),
        "while" => [Keyword("while").d(), NonTerm("cond"), NonTerm("block"),].and().group("while"),
        // "for_init" => [
        //     NonTerm("var"),
        //     [NonTerm("set"), [NonTerm("comma"),NonTerm("set")].and().many0(),].and(),
        // ].or(),
        // "for_incr_stmt" => [NonTerm("set"),NonTerm("call"),].or(),
        // "for_incr" => [
        //     NonTerm("for_incr_stmt"),
        //     [NonTerm("comma"),NonTerm("for_incr_stmt")].and().many0(),
        // ].and(),
        // "for" => [
        //     Keyword("for"),
        //     NonTerm("lparen"),
        //     NonTerm("for_init"),
        //     NonTerm("semicolon"),
        //     NonTerm("expr"),
        //     NonTerm("semicolon"),
        //     NonTerm("for_incr"),
        //     NonTerm("rparen"),
        //     NonTerm("block"),
        // ].and(),
        "for_in" => [
            Keyword("for"),
            NonTerm("lparen"),
            Identifier,
            Keyword("in").d(),
            [NonTerm("val"),NonTerm("call"),].and(),
            NonTerm("rparen"),
            NonTerm("block"),
            ].and(),
        "for_to" => [
            Keyword("for"),
            NonTerm("lparen"),
            Identifier,
            Keyword("in").d(),
            NonTerm("expr"),
            Keyword("to").d(),
            NonTerm("expr"),
            NonTerm("rparen"),
            NonTerm("block"),
        ].and(),
        "call" => [
            NonTerm("lparen"),
            [
                NonTerm("expr"),
                [NonTerm("comma").d(),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt().d(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),
        "include" => [Keyword("include").d(),String,].and().group("include"),

        "func_params" => [
            NonTerm("lparen"),
            [
                Identifier,
                [NonTerm("comma"),Identifier,].and().many0(),
                NonTerm("ellipsis").opt(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),
        "func" => [
            Keyword("fn"),
            [Identifier, NonTerm("val_field_index").many0()].and(),
            NonTerm("func_params"),
            NonTerm("lcurly"),
            NonTerm("stmts"),
            NonTerm("rcurly"),
        ].and(),
        "lambda" => [Keyword("fn"),NonTerm("func_params"),NonTerm("lcurly"),NonTerm("stmts"),NonTerm("rcurly"),].and(),

        "infix" => [
            NonTerm("add"),NonTerm("sub"),
            NonTerm("mul"),NonTerm("div"),
            NonTerm("lt"),NonTerm("gt"),
            NonTerm("le"),NonTerm("ge"),
            NonTerm("eq"),NonTerm("ne"),
            NonTerm("and"),NonTerm("or"),
        ].or(),

        "expr" => [
            NonTerm("val"),
            // [NonTerm("infix"),NonTerm("val"),].and().many0(),
        ].and().group("expr"),

        "prefix" => [NonTerm("add"),NonTerm("sub"),NonTerm("not"),].or(),
        "val_field_index" => [ NonTerm("val_index"), NonTerm("val_field"), ].or(),
        "val_field_index_call" => [ NonTerm("val_field_index").offer(), NonTerm("call"), ].or(),


        "val_field" => [NonTerm("dot"),[Identifier,Int,].or(),].and(),
        "val_index" => [NonTerm("lsquare"),NonTerm("expr"),NonTerm("rsquare"),].and(),
        "bool" => [Keyword("true"),Keyword("false"),].or(),
        "nil" => Keyword("nil"),
        "val" => [
            NonTerm("prefix").many0(),
            [
                Int,
                // Float,
                // String,
                // Keyword("bool"),
                // Keyword("nil"),
                // Keyword("void"),
                // NonTerm("if"),
                // NonTerm("lambda"),
                // NonTerm("array"),
                // NonTerm("dict"),
                // // NonTerm("block"), //allow code blocks?
                Identifier,
                // [NonTerm("lparen"),NonTerm("expr"),NonTerm("rparen"),].and(),
            ].or(),
            NonTerm("val_field_index_call").many0(),
            // [NonTerm("val_field_index").offer(),NonTerm("call"),].or().many0(),
        ].and(),

        "dict_key_val" => [
            [
                Identifier,
                [NonTerm("sub").opt(),Int,].and(),
                String,
                Keyword("bool"),
                Keyword("nil"),
            ].or(),
            NonTerm("colon"),
            NonTerm("expr"),
        ].and(),
        "dict" => [
            NonTerm("lcurly"),
            [
                NonTerm("dict_key_val"),
                [NonTerm("comma"),NonTerm("dict_key_val"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rcurly"),
        ].and(),
        "array" => [
            NonTerm("lsquare"),
            [
                NonTerm("expr"),
                [NonTerm("comma"),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rsquare"),
        ].and(),

        "format_params" => [
            NonTerm("lparen"),
            String,
            [
                [String,NonTerm("expr"),].or(),
                [NonTerm("comma"),NonTerm("expr"),].and().many0(),
                NonTerm("comma").opt(),
            ].and().opt(),
            NonTerm("rparen"),
        ].and(),

        "format" => [Keyword("format"),NonTerm("format_params"),].and(),
        "print" => [Keyword("print"),NonTerm("format_params"),].and(),
        "println" => [Keyword("println"),NonTerm("format_params"),].and(),

        "lcurly" => Symbol("{").d(),
        "rcurly" => Symbol("}").d(),
        "lsquare" => Symbol("[").d(),
        "rsquare" => Symbol("]").d(),
        "lparen" => Symbol("(").d(),
        "rparen" => Symbol(")").d(),

        "colon" => Symbol(":"),
        "semicolon" => Symbol(";"),
        "end" => [NonTerm("semicolon"),].or(),

        "dot" => Symbol("."),
        "ellipsis" => [NonTerm("dot"),NonTerm("dot"),NonTerm("dot"),].and(),
        "comma" => Symbol(","),

        "set_equal" => Symbol("="),

        "not" => Symbol("!").group("not"),
        "add" => Symbol("+"),
        "sub" => Symbol("-"),
        "mul" => Symbol("*"),
        "div" => Symbol("/"),

        "and" => [Symbol("&"),Symbol("&"),].and().group("and"),
        "or" => [Symbol("|"),Symbol("|"),].and().group("or"),

        "lt" => Symbol("<").group("lt"),
        "gt" => Symbol(">").group("gt"),
        "le" => [Symbol("<"),Symbol("="),].and().group("le"),
        "ge" => [Symbol(">"),Symbol("="),].and().group("ge"),
        "eq" => [Symbol("="),Symbol("="),].and().group("eq"),
        "ne" => [Symbol("!"),Symbol("="),].and().group("ne"),
        _ => Error(GrammarWalkError::MissingNonTerm(n)),
    }

}

// #[derive(Clone,Debug)]
// enum GrammarOutput<'a> {
//     Group{name:&'a str,primitives:Range<usize>},
//     Primitive(PrimitiveContainer<'a>),

// }

// impl<'a> std::fmt::Debug for GrammarOutput<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Group { name, primitives } => {
//                 write!(f,"{name}:{primitives:?}")
//                 // f.debug_struct("Group").field("name", name).field("primitives", primitives).finish()
//             }
//             Self::Primitive(arg0) => {
//                 // f.debug_tuple("Primitive").field(arg0).finish()
//                 write!(f,"{arg0:?}")
//             }
//         }
//     }
// }


// #[derive(Clone)]
// enum TempOutput<'a> {
//     Group{name:&'a str,group:usize,parent_group:usize,},
//     Primitive{g:PrimitiveContainer<'a>,parent_group:usize,},

// }
//


    #[derive(Clone, Copy, Default, Debug)]
    struct PrimitiveInfo {
        // name:&'a str,
        // depth:usize,
        group:usize,
        discard:bool,
    }

    #[derive(Debug)]
    struct GroupInfo<'a> {
        name:&'a str,
        parent:usize, //group
        primitive_ind_start:usize,
    }

    struct Work<'a> {
        grammar:GrammarItem<'a>,
        success_len:usize,
        fail_len:usize,
        primitives:PrimitiveIterContainer<'a>,
        group_ind:usize,

        group_len:usize, //only used for removing unused groups ... but even then it is not required, mainly used for debugging
        output_len:usize,

        discard:bool,

        // takeable_starts:HashSet<(GrammarItem<'a>,usize)>, //[(g,output_ind_start)]
        takeable_starts_len:usize,
        opt:bool,

        visiteds:HashSet<(&'a str,usize)>, //used for checking recursive nonterms

        takeables:HashMap<GrammarItem<'a>,PrimitiveIterContainer<'a>>, //[non_term]
    }

#[derive(Debug,Clone,Hash,PartialEq, Eq)]
pub enum GrammarWalkError<'a> {
    RecursiveNonTerm(&'a str),
    InvalidSyntax,//((Loc,Vec<GrammarItem<'a>>,)),
    MissingNonTerm(&'a str),
}

impl<'a> std::fmt::Display for GrammarWalkError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{self:?}",)
    }
}

impl<'a> std::error::Error for GrammarWalkError<'a> {
    fn description(&self) -> &str {
        "GrammarWalkError"
    }
}

pub struct GrammarWalker<'a,F> {
    top_primitives:PrimitiveIterContainer<'a>,
    temp_primtives : Vec<PrimitiveInfo>,
    temp_groups3 : Vec<GroupInfo<'a>>,
    takeable_starts:Vec<(GrammarItem<'a>,PrimitiveIterContainer<'a>)>, //[(g,output_ind_start)]
    grammar_func:F,

    primitives_remaining: PrimitiveIterContainer<'a>,
    stk: Vec<Work<'a>>,
    c:usize,
    expected: (Loc,Vec<GrammarItem<'a>>,),
}

impl<'a,F> GrammarWalker<'a,F>
where
    F: Fn(&'a str)->GrammarItem<'a>,
{
    pub fn new(top_primitives:PrimitiveIterContainer<'a>, grammar_func:F) -> Self {
        Self {
            temp_primtives :  Default::default(),
            temp_groups3 : vec![GroupInfo{ name: "", parent: 0, primitive_ind_start:0, }],
            takeable_starts: Default::default(),
            grammar_func,
            primitives_remaining:top_primitives.clone(),
            top_primitives,
            stk:vec![
                Work{
                    grammar:GrammarItem::Error(GrammarWalkError::InvalidSyntax),success_len:0,fail_len:0,primitives:top_primitives,
                    group_ind: 0, group_len: 1, output_len: 0, discard:false,
                    // takeable_starts:Default::default(),
                    takeable_starts_len:0,
                    visiteds:Default::default(),
                    takeables:Default::default(),
                    opt:false,
                },
                Work{
                    grammar:grammar_decl("test10"),success_len:0,fail_len:1,primitives:top_primitives,
                    group_ind: 0, group_len: 1, output_len: 0, discard:false,
                    // takeable_starts:Default::default(),
                    takeable_starts_len:0,
                    visiteds:Default::default(),
                    takeables:Default::default(),
                    opt:false,
                },
            ],
            c:0,
            expected:Default::default(),
        }
    }

    pub fn run(&mut self) {
        while let Some(cur)=self.stk.pop() {
            self.step(cur);
        }

                //        if !self.expected.1.is_empty() {
                //     let ee=self.expected.1.iter().map(|g|match g {

                //         GrammarItem::String => "string",
                //         GrammarItem::Identifier => "identifier",
                //         GrammarItem::Int => "int",
                //         GrammarItem::Float => "float",
                //         GrammarItem::Symbol(s) => *s,
                //         GrammarItem::Keyword(s) => *s,
                //         GrammarItem::Eol => todo!(),
                //         _ =>"",
                //     }).collect::<Vec<_>>().join(", ");

                //     println!("At {}, expected {:?}",self.expected.0,ee);

                // } else  {
                //     let loc=cur.primitives.loc();

                //     println!("At {loc}", );

                // }

        //
        println!("groups={:?}",self.temp_groups3);
        println!("outputs={:?}",self.temp_primtives);

        if !self.primitives_remaining.is_empty() {
            println!("error, failed to parse all tokens {:?}",self.primitives_remaining);
        }

        println!("===");


        let mut groups_visited: HashSet<usize>=HashSet::new();

        for p in self.top_primitives {
            let i=p.ind();
            let Some(output)=self.temp_primtives.get(i) else {
                break;
            };

            let mut g=output.group;
            let mut depth=0;
            let mut gs: Vec<usize>=Vec::new();
            while g!=0 {
                gs.push(g);
                let gg=&self.temp_groups3[g];

                depth+=1;

                g=gg.parent;

            }

            for (d,&g) in gs.iter().rev().enumerate() {
                let gg=&self.temp_groups3[g];

                if !groups_visited.contains(&g) {
                    println!("{}{:?}",
                        "  ".repeat(d),
                        gg.name,
                    );
                    groups_visited.insert(g);
                }



            }

            println!("{}{}{p:?}",
                "  ".repeat(depth),
                if output.discard {"-"}else{""}
            );

        }
        println!("===");

        //
        println!("top_primitives={:?}", self.top_primitives );
        // println!("output={outputs:?}",  );

    }

    fn step(&mut self,cur:Work<'a>) -> Result<(),GrammarWalkError<'a>> {
        self.c+=1;

        // // if c>30 {break;}
        // // println!(": {cur:?} || {} && {primitives:?}", self.stk.iter().rev().map(|x|format!("{:?}",x.0)).collect::<Vec<_>>().join(" << "), );
        {
            let c=self.c;
            let Work { grammar, success_len, fail_len, primitives, group_ind, group_len, output_len, discard, takeable_starts_len, visiteds, takeables, opt}=&cur;
            println!("{c:4}: {grammar:?}, ps={primitives:?}, success={success_len}, fail={fail_len}, group_ind={group_ind}, group_len={group_len}, output_len={output_len}, discard={discard}, takeable_starts_len={takeable_starts_len:?}, visiteds={visiteds:?}, opt={opt:?}, takeables={takeables:?}, ");
            println!("         -takeable_starts={:?}",self.takeable_starts);
            println!("         -temp_primtives={:?}",self.temp_primtives);
            println!("         -temp_groups3={:?}",self.temp_groups3);
        }

        for (i,Work { grammar:g, success_len:s, fail_len:f, primitives:ps, group_ind, group_len, output_len, discard, takeable_starts_len, visiteds, takeables, opt }) in self.stk.iter()
            // .rev()
            .enumerate() {
            // println!("\t{i:3}: {g:?}\n\t   : {ps:?}\n\t   : success={s}, fail={f}",);
            println!("\t{i:3}: {g:?}, ps={ps:?},success={s}, fail={f}, group_ind={group_ind}, group_len={group_len}, output_len={output_len}, discard={discard}, takeable_starts_len={takeable_starts_len:?}, visiteds={visiteds:?}, opt={opt:?}, takeables={takeables:?}",);
        }

        // match cur.grammar {
        //     GrammarItem::NonTerm(x) => {

        //     }
        //     _=> {}
        // }


        match cur.grammar {
            GrammarItem::Group(name, g) => {
                let new_group_ind=self.new_group(name, cur.group_ind, cur.primitives);
                let new_group_len=self.temp_groups3.len();

                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: new_group_ind,
                    group_len: new_group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:cur.opt,
                });
            }
            GrammarItem::Discard(g) => {

                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:true,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:cur.opt,
                });


            }
            GrammarItem::And(gs) => {
                let Some(first)=gs.first().cloned() else {
                    // continue;
                    return Ok(());
                };

                if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
                    self.stk.push(Work {
                        grammar: GrammarItem::And(rest.into()),
                        success_len: cur.success_len,
                        fail_len: cur.fail_len,

                        //not really necessary? since gets updated by always/primtitives
                        primitives: cur.primitives,

                        group_ind: cur.group_ind,
                        group_len: cur.group_len,
                        output_len: cur.output_len,
                        discard:cur.discard,

                        takeable_starts_len:cur.takeable_starts_len,
                        visiteds:cur.visiteds.clone(),
                        takeables:cur.takeables.clone(),
                        opt:false, //opt isnt passed to individual items in And

                    });
                }

                let success_len=if gs.len()>1 {self.stk.len()}else{cur.success_len};

                // if cur.opt {
                //     self.takeable_starts.push((first.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: first,
                    success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:false, //opt isnt passed to individual items in And
                });
            }
            GrammarItem::Or(gs) => {
                let Some(first)=gs.first().cloned() else {
                    // continue;
                    return Ok(())
                };

                if let Some(rest)=gs.get(1..).and_then(|r|(!r.is_empty()).then_some(r)) {
                    self.stk.push(Work {
                        grammar: GrammarItem::Or(rest.into()),
                        success_len: cur.success_len,
                        fail_len: cur.fail_len,
                        primitives: cur.primitives,

                        group_ind: cur.group_ind,
                        group_len: cur.group_len,
                        output_len: cur.output_len,
                        discard:cur.discard,
                        takeable_starts_len:cur.takeable_starts_len,
                        visiteds:cur.visiteds.clone(),
                        takeables:cur.takeables.clone(),
                        opt:cur.opt,
                    });
                }

                let fail_len=if gs.len()>1 {self.stk.len()}else{cur.fail_len};

                // if cur.opt {
                //     self.takeable_starts.push((first.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: first,
                    success_len: cur.success_len,
                    fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:cur.opt,
                });
            }

            GrammarItem::Opt(g) => {
                self.stk.push(Work {
                    grammar: GrammarItem::Always,
                    success_len: cur.success_len,
                    fail_len: 0, //fail is not used
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                    opt:false, //not used on always
                });

                let fail_len=self.stk.len();

                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:true,
                });
            }
            GrammarItem::Offer(g) => {
                //should return err if not giveable? ie not opt? or just ignore?
                //  or just don't rquire at all

                // if cur.opt {
                self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,
                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:self.takeable_starts.len(),
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:cur.opt,
                });
            }
            GrammarItem::Take(g) => {
                if let Some(taken_ps_start)=cur.takeables.get(&g).cloned() {
                    println!("---the groups are {:?}",self.temp_groups3);
                    //how to remove no longer used groups, and fix inds of the used group that ccomes after the removed one?

                    let cur_group_ind=self.remove_groups_at_except(taken_ps_start,cur.group_ind,);


                    //clear outputs to start of taken
                    self.temp_primtives.truncate(taken_ps_start.inds().start);

                    //
                    // if cur.opt {
                    //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                    // }

                    //
                    self.stk.push(Work {
                        grammar: *g,
                        success_len: cur.success_len,
                        fail_len: cur.fail_len,

                        primitives: taken_ps_start.clone(),
                        output_len: taken_ps_start.inds().start,

                        group_ind: cur_group_ind,
                        group_len: self.temp_groups3.len(),

                        discard: cur.discard,
                        takeable_starts_len: cur.takeable_starts_len,//cur.takeable_starts_len, // ??
                        visiteds: cur.visiteds, //
                        takeables: Default::default(),
                        opt:cur.opt,
                    });
                } else {

                    //
                    self.stk.truncate(cur.fail_len);

                    //
                    if let Some(last)=self.stk.last() {
                        self.temp_primtives.truncate(last.output_len);
                        self.takeable_starts.truncate(last.takeable_starts_len);
                    }
                }
            }
            GrammarItem::Many(g) => {
                // let fail_len2=self.stk.len(); //only remove everything past here on fail
                self.stk.push(Work {
                    grammar: GrammarItem::Many(g.clone()),
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                    opt:true,
                });

                let success_len2=self.stk.len();

                self.stk.push(Work {
                    grammar: GrammarItem::Always,
                    success_len: cur.success_len,
                    fail_len: 0, //fail is not used
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds.clone(),
                    takeables:cur.takeables.clone(),
                    opt:false, //not used
                });

                let fail_len=self.stk.len();


                // if cur.opt {
                //     self.takeable_starts.push((*g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: *g,
                    success_len: success_len2,
                    fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    takeable_starts_len:cur.takeable_starts_len,
                    visiteds:cur.visiteds,
                    takeables:cur.takeables,
                    opt:true,
                });
            }

            GrammarItem::NonTerm(t) => {
                // let v=(t,cur.primitives.inds().start);

                // if cur.visiteds.contains(&v) {
                //     println!("err, circular nonterm {t}");
                //     break;
                // }

                // let mut visiteds=cur.visiteds;
                // visiteds.insert(v);

                let visiteds=self.do_non_term_visiteds(t,cur.primitives,cur.visiteds)?;

                // let mut takeable_starts_len=cur.takeable_starts_len;
                // self.takeable_starts.insert((cur.grammar,cur.primitives.inds().start));
                // let g=(self.grammar_func)(t);

                // if cur.opt {
                //     self.takeable_starts.push((g.clone(),cur.primitives.clone()));
                // }

                self.stk.push(Work {
                    grammar: (self.grammar_func)(t), //should return err on not found, instead of grammar never, should have error
                    success_len: cur.success_len,
                    fail_len: cur.fail_len,
                    primitives: cur.primitives,

                    group_ind: cur.group_ind,
                    group_len: cur.group_len,
                    output_len: cur.output_len,
                    discard:cur.discard,
                    visiteds,
                    takeables:cur.takeables,
                    takeable_starts_len:cur.takeable_starts_len,
                    opt:cur.opt,
                });
            }
            GrammarItem::Always => {
                self.stk.truncate(cur.success_len);

                if let Some(last)=self.stk.last_mut() {
                    if last.grammar.is_many() && last.primitives.len()==cur.primitives.len() { //if not parsing anything, exit the many
                        last.grammar=GrammarItem::Always;
                    }

                    last.primitives=cur.primitives;

                    // self.temp_primtives.resize(cur.output_len+1, PrimitiveInfo{ group: cur.group_ind }); //discard:true,

                    // // last.group_ind=cur.group_ind;
                    // let last_group_len= last.group_len;
                    last.group_len=cur.group_len; //done below //not anymore
                    last.output_len=cur.output_len;
                    // last.takeable_starts=cur.takeable_starts;

                    //


                    // //
                    // last.takeables.retain(|_k,v|{
                    //     v.inds().start >= last.primitives.inds().start
                    // });

                    //
                    last.takeables=cur.takeables;


                }

                //
                // last.group_len=
                self.last_remove_groups_at(cur.group_len,cur.primitives);


                //
                self.last_insert_start_takeables();

                //
                // if self.stk.is_empty() {
                //     self.primitives_remaining=cur.primitives;
                // }

                self.set_remaining_prims(cur.primitives);

                // expected=Default::default();

                self.clear_expected();

            }

            GrammarItem::Error(e) => {
                println!("====error {:?}",self.expected);

                //necesaary? any point to it?
                if
                    // self.expected.0.is_zero()
                    self.expected.1.is_empty()
                {
                    self.expected.0=cur.primitives.loc();
                }


                //
                self.set_remaining_prims(cur.primitives);

                // break;
                return Err(e);

            }
            GrammarItem::String => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_string()) {
                    println!("--- string {v:?}");
                }
            }
            GrammarItem::Identifier => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_identifier()) {
                    println!("--- identifier {v:?}");
                }
            }
            GrammarItem::Int => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_int()) {
                    println!("--- int {v:?}");
                }
            }
            GrammarItem::Float => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_float()) {
                    println!("--- float {v:?}");
                }
            }
            GrammarItem::Symbol(s) => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_with_symbols([s])) {
                    println!("--- symbol {v:?}");
                }
            }
            GrammarItem::Keyword(s) => {
                if let Some(v)=self.do_primtive(cur,|ps|ps.pop_with_identifiers([s])) {
                    println!("--- keyword {v:?}");
                }
            }
            GrammarItem::Eol => {
                if let Some(_)=self.do_primtive(cur,|ps|ps.pop_eol()) {
                    println!("--- eol");
                }
            }
        }

        Ok(())
    }

    fn do_primtive<Q,P>(&mut self,mut cur:Work<'a>,prim_func:Q) -> Option<P>
    where
        Q:Fn(&mut PrimitiveIterContainer<'a>)->Result<ValueContainer<'a,P>,Loc>,
    {
        match prim_func(&mut cur.primitives) {
            Ok(v) => {
                if v.primitive.start_loc() >= self.expected.0 {
                    self.clear_expected();
                }

                self.stk.truncate(cur.success_len);

                self.temp_primtives.resize(v.primitive.ind(), PrimitiveInfo{ group: cur.group_ind,discard:true, }); //discard:true,
                self.temp_primtives.push(PrimitiveInfo{ group: cur.group_ind,discard:cur.discard,});

                if let Some(last)=self.stk.last_mut() {
                    last.primitives=cur.primitives;
                    last.group_len=cur.group_len;
                    last.output_len=self.temp_primtives.len();
                }

                //
                self.last_remove_old_takeables();
                self.last_insert_start_takeables();

                //
                // if self.stk.is_empty() {
                //     self.primitives_remaining=cur.primitives;
                // }
                self.set_remaining_prims(cur.primitives);

                Some(v.value)
            }
            Err(loc) => {
                self.add_expected(loc,cur.grammar);

                //
                self.stk.truncate(cur.fail_len);

                //
                if let Some(last)=self.stk.last() {
                    self.temp_primtives.truncate(last.output_len);

                    self.takeable_starts.truncate(last.takeable_starts_len);
                }

                // if self.stk.is_empty() {
                //     self.primitives_remaining=cur.primitives;
                // }
                self.set_remaining_prims(cur.primitives);

                None
            }
        }

        //


    }

    fn do_non_term_visiteds(&mut self,
        t:&'a str,
        cur_primitives:PrimitiveIterContainer<'a>,
        cur_visiteds: HashSet<(&'a str, usize)>,
    ) -> Result<HashSet<(&'a str, usize)>,GrammarWalkError<'a>> {
        let v=(t,cur_primitives.inds().start);

        if cur_visiteds.contains(&v) {
            println!("err, circular nonterm {t}");
            // break;
            return Err(GrammarWalkError::RecursiveNonTerm(t));
        }

        let mut visiteds=cur_visiteds;
        visiteds.insert(v);

        Ok(visiteds)
    }

    fn last_insert_start_takeables(&mut self,
        // mut last_takeables: HashMap<GrammarItem<'a>, PrimitiveIterContainer<'a>>,last_takeable_starts_len:usize,
    )
        // -> HashMap<GrammarItem<'a>, PrimitiveIterContainer<'a>>
    {
        if let Some(last)=self.stk.last_mut() {
            for (tg,tp_ind) in self.takeable_starts.drain(last.takeable_starts_len ..) {
                println!("--- inserting takeable {tg:?} {tp_ind:?}",);
                last.takeables.insert(tg, tp_ind);
            }
        }
        // last_takeables
    }

    fn last_remove_old_takeables(&mut self) {
        if let Some(last)=self.stk.last_mut() {
            last.takeables.retain(|_k,v|{
                v.inds().start >= last.primitives.inds().start
            });
        }
    }
    fn clear_expected(&mut self) {
        self.expected.0=Loc::zero();
        self.expected.1.clear();
    }
    fn add_expected(&mut self,loc:Loc,g:GrammarItem<'a>) {
        if loc==self.expected.0 {
            self.expected.1.push(g);
        } else if loc>self.expected.0 {
            self.expected.0=loc;
            self.expected.1=vec![g];
        }
    }
    fn new_group(&mut self,name : &'a str, parent:usize, ps:PrimitiveIterContainer<'a>) -> usize {
        let new_group_ind=self.temp_groups3.len();

        self.temp_groups3.push(GroupInfo {
            name,
            parent,
            primitive_ind_start: ps.inds().start,
        });

        new_group_ind
    }

    fn set_remaining_prims(&mut self,cur_primitives:PrimitiveIterContainer<'a>,) {
        if self.stk.is_empty() {
            self.primitives_remaining=cur_primitives;
        }
    }

    fn remove_groups_at_except(&mut self,
        taken_ps_start:PrimitiveIterContainer<'a>,
        cur_group_ind:usize,
    ) -> usize{

        let mut cur_group_ind=cur_group_ind;


        // if let Some(temp_prim)=self.temp_primtives.get(taken_ps_start.inds().start) {


        //collect cur groups
        let mut cur_used_group_inds: HashSet<usize>=HashSet::new();

        //
        {
            let mut group_ind=cur_group_ind;

            loop {
                cur_used_group_inds.insert(group_ind);

                if group_ind==0 {
                    break;
                }

                let group=&self.temp_groups3[group_ind];
                group_ind=group.parent;

            }
        }

        //get first group ind with prim ind >= taken.inds().start
        let mut after_group_ind = self.temp_groups3.len();

        while after_group_ind > 0 {
            let group=self.temp_groups3.get(after_group_ind-1).unwrap();

            if group.primitive_ind_start < taken_ps_start.inds().start {
                break;
            }

            after_group_ind-=1;
        }

        //get num of groups to remove
        let mut remove_groups_num=0;

        for i in after_group_ind..self.temp_groups3.len() {
            if cur_used_group_inds.contains(&i) {
                break;
            }

            remove_groups_num+=1;
        }

        //remove unused groups
        self.temp_groups3.drain(after_group_ind..after_group_ind+remove_groups_num);

        //
        for group in &mut self.temp_groups3[after_group_ind..] {
            if group.parent>=after_group_ind {
                group.parent-=remove_groups_num;
            }
        }

        //
        if cur_group_ind >=after_group_ind {
            cur_group_ind-=remove_groups_num;
        }

        //
        for prev in self.stk.iter_mut().rev() {
            if prev.group_ind >=after_group_ind {
                prev.group_ind-=remove_groups_num;
                prev.group_len-=remove_groups_num;
            }
        }

        cur_group_ind
    }

    fn last_remove_groups_at(&mut self,
        // last_group_len:usize ,
        cur_group_len:usize,cur_primitives:PrimitiveIterContainer<'a>)
        // -> usize
    {

        if let Some(last)=self.stk.last_mut() {
            println!("===www {} {cur_group_len} ",  last.group_len ); //last.grammar

            //
            for group_ind in last.group_len .. cur_group_len {
                let group=&self.temp_groups3[group_ind];
                println!("===hmmm {group_ind}");

                if group.primitive_ind_start==cur_primitives.inds().start {
                    self.temp_groups3.truncate(group_ind); //removes this group and ones after
                    last.group_len=group_ind;

                    println!("====== {group_ind} {}",self.temp_groups3.len(), );
                    break;
                    // return group_ind;
                }
            }

            // cur_group_len
        }
    }

}

pub fn grammar_run<'a>( top_primitives:PrimitiveIterContainer<'a>) {
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

    walker.run();
}
