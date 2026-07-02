use super::super::grammar::error::GrammarWalkError;



#[derive(Clone,Debug,Hash,PartialEq,Eq)]
pub enum GrammarNode<'g> {
    Many(Box<GrammarNode<'g>>),
    And(Vec<GrammarNode<'g>>), //should store reversed?
    Or(Vec<GrammarNode<'g>>), //should store reversed?

    NonTerm(&'g str),
    Group(Box<GrammarNode<'g>>,&'g str,),
    Expected(Box<GrammarNode<'g>>, &'g str,),
    Prev(Box<GrammarNode<'g>>),

    String,
    Identifier,
    Int,
    Float,
    Symbol(&'g str),
    Keyword(&'g str),
    Eol,

    Always, //always succeeds
    Error(GrammarWalkError<'g>),
}

impl<'g> GrammarNode<'g> {
    pub fn many0(self) -> GrammarNode<'g> {
        Self::Many(self.into())
    }
    pub fn many1(self) -> GrammarNode<'g> {
        [self.clone(),self.many0(),].and()
    }
    pub fn opt(self) -> GrammarNode<'g> {
        [self.into(),Self::Always].or()
    }
    pub fn group(self,name: &'g str) -> GrammarNode<'g> {
        Self::Group(self.into(),name)
    }
    pub fn expected(self,name: &'g str,) -> GrammarNode<'g> {
        Self::Expected(self.into(),name)
    }
    pub fn prev(self) -> GrammarNode<'g> {
        Self::Prev(self.into())
    }
    pub fn is_many(&self) -> bool {
        if let GrammarNode::Many(_)=self {true} else {false}
    }
}

//todo have array stored in rev for or/and
pub trait GrammarArrayTrait<'g> {
    fn and(&self) -> GrammarNode<'g>;
    fn or(&self) -> GrammarNode<'g>;
}

impl<'a,const N: usize> GrammarArrayTrait <'a> for [GrammarNode<'a>; N] {
    fn and(&self) -> GrammarNode<'a> {
        GrammarNode::And(self.into())
    }
    fn or(&self) -> GrammarNode<'a> {
        GrammarNode::Or(self.into())
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

