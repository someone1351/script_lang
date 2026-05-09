use super::super::grammar::error::GrammarWalkError;



#[derive(Clone,Debug,Hash,PartialEq,Eq)]
pub enum GrammarNode<'g> {
    Many(Box<GrammarNode<'g>>),
    // Many1(Box<GrammarItem<'a>>),
    And(Vec<GrammarNode<'g>>), //should store reversed?
    Or(Vec<GrammarNode<'g>>), //should store reversed?
    Opt(Box<GrammarNode<'g>>),
    Cede(Box<GrammarNode<'g>>),
    Take(Box<GrammarNode<'g>>),
    Group(&'g str,Box<GrammarNode<'g>>),
    Expect(&'g str,Box<GrammarNode<'g>>),

    // List(Box<GrammarItem<'a>>,Box<GrammarItem<'a>>), //val,sep
    // ListNoTrail(Box<GrammarItem<'a>>,Box<GrammarItem<'a>>), //val,sep

    String,
    Identifier,
    Int,
    Float,
    Symbol(&'g str),
    Keyword(&'g str),
    Eol,

    NonTerm(&'g str),
    Always, //always succeeds
    // Never, //replace with Error ?
    Error(GrammarWalkError<'g>),
    // Not(Box<GrammarItem<'a>>), //todo, needed? better to have NotIdentifier etc?

    //todo remove:
    // Discard(Box<GrammarNode<'g>>), //todo, removes token from output (via just hding it, ie have hashmap of tokens to hide)
}

impl<'g> GrammarNode<'g> {
    pub fn many0(self) -> GrammarNode<'g> {
        Self::Many(self.into())
    }
    pub fn many1(self) -> GrammarNode<'g> {
        let x=self.clone();
        [x,self.many0(),].and()
        // Self::Many1(self.into())
    }
    pub fn opt(self) -> GrammarNode<'g> {
        Self::Opt(self.into())
    }
    pub fn group(self,name: &'g str) -> GrammarNode<'g> {
        Self::Group(name,self.into())
    }
    pub fn expect(self,name: &'g str) -> GrammarNode<'g> {
        Self::Expect(name,self.into())
    }
    // pub fn discard(self,) -> GrammarNode<'g> {
    //     Self::Discard(self.into())
    // }
    pub fn cede(self,) -> GrammarNode<'g> {
        Self::Cede(self.into())
    }
    pub fn take(self,) -> GrammarNode<'g> {
        Self::Take(self.into())
    }
    // pub fn d(self,) -> GrammarNode<'g> {
    //     self.discard()
    // }
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