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

    // Stow(Box<GrammarNode<'g>>),

    // Mark(Box<GrammarNode<'g>>),

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
    // pub fn stow(self) -> GrammarNode<'g> {
    //     Self::Stow(self.into())
    // }
    pub fn is_many(&self) -> bool {
        if let GrammarNode::Many(..)=self {true} else {false}
    }
    pub fn is_nonterm(&self) -> bool {
        if let GrammarNode::NonTerm(..)=self {true} else {false}
    }
    pub fn is_or(&self) -> bool {
        if let GrammarNode::Or(..)=self {true} else {false}
    }
    pub fn is_and(&self) -> bool {
        if let GrammarNode::And(..)=self {true} else {false}
    }
    pub fn get_non_term_name(&self) -> Option<&'g str> {
        if let Self::NonTerm(n)=self {
            Some(n)
        } else {
            None
        }
    }
    pub fn is_non_term(&self) -> bool {
        if let Self::NonTerm(..)=self {
            true
        } else {
            false
        }
    }
    pub fn is_always(&self) -> bool {
        if let GrammarNode::Always=self {
            true
        }else{
            false
        }
    }
    pub fn is_prev(&self) -> bool {
        if let GrammarNode::Prev(_)=self {
            true
        }else{
            false
        }
    }
    pub fn is_primtive(&self) -> bool {
        match self {
            // GrammarNode::Many(grammar_node) => todo!(),
            // GrammarNode::And(grammar_nodes) => todo!(),
            // GrammarNode::Or(grammar_nodes) => todo!(),
            // GrammarNode::NonTerm(_) => todo!(),
            // GrammarNode::Group(grammar_node, _) => todo!(),
            // GrammarNode::Expected(grammar_node, _) => todo!(),
            // GrammarNode::Prev(grammar_node) => todo!(),
            GrammarNode::String => true,
            GrammarNode::Identifier => true,
            GrammarNode::Int => true,
            GrammarNode::Float => true,
            GrammarNode::Symbol(_) => true,
            GrammarNode::Keyword(_) => true,
            GrammarNode::Eol => true,
            // GrammarNode::Always => todo!(),
            // GrammarNode::Error(grammar_walk_error) => todo!(),
            _ => false,
        }
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

