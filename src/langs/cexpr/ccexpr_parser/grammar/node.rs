use super::super::super::ccexpr_parser::grammar::error::GrammarWalkError;



#[derive(Clone,Debug,Hash,PartialEq,Eq)]
pub enum GrammarNode<'a> {
    Many(Box<GrammarNode<'a>>),
    // Many1(Box<GrammarItem<'a>>),
    And(Vec<GrammarNode<'a>>), //should store reversed?
    Or(Vec<GrammarNode<'a>>), //should store reversed?
    Opt(Box<GrammarNode<'a>>),
    Cede(Box<GrammarNode<'a>>),
    Take(Box<GrammarNode<'a>>),
    Group(&'a str,Box<GrammarNode<'a>>),

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
    Discard(Box<GrammarNode<'a>>), //todo, removes token from output (via just hding it, ie have hashmap of tokens to hide)
}

impl<'a> GrammarNode<'a> {
    pub fn many0(self) -> GrammarNode<'a> {
        Self::Many(self.into())
    }
    pub fn many1(self) -> GrammarNode<'a> {
        let x=self.clone();
        [x,self.many0(),].and()
        // Self::Many1(self.into())
    }
    pub fn opt(self) -> GrammarNode<'a> {
        Self::Opt(self.into())
    }
    pub fn group(self,name: &'a str) -> GrammarNode<'a> {
        Self::Group(name,self.into())
    }
    pub fn discard(self,) -> GrammarNode<'a> {
        Self::Discard(self.into())
    }
    pub fn cede(self,) -> GrammarNode<'a> {
        Self::Cede(self.into())
    }
    pub fn take(self,) -> GrammarNode<'a> {
        Self::Take(self.into())
    }
    pub fn d(self,) -> GrammarNode<'a> {
        self.discard()
    }
    pub fn is_many(&self) -> bool {
        if let GrammarNode::Many(_)=self {
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
pub trait GrammarArrayTrait<'a> {
    fn and(&self) -> GrammarNode<'a>;
    fn or(&self) -> GrammarNode<'a>;
}
impl<'a,const N: usize> GrammarArrayTrait <'a> for [GrammarNode<'a>; N] {
    fn and(&self) -> GrammarNode<'a> {
        GrammarNode::And(self.into())
    }
    fn or(&self) -> GrammarNode<'a> {
        GrammarNode::Or(self.into())

    }
}