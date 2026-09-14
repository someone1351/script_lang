use std::fmt::Debug;

#[derive(Debug,Clone,Hash,PartialEq, Eq)]
pub enum GrammarWalkError<NT>
where
    NT:Debug,
{
    FailedParse,//((Loc,Vec<GrammarItem<'a>>,)),
    // Unfinished,
    RecursiveNonTerm(NT),
    MissingNonTerm(NT),
}

impl<NT> std::fmt::Display for GrammarWalkError<NT>
where
    NT:Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{self:?}",)
    }
}

impl<NT> std::error::Error for GrammarWalkError<NT>
where
    NT:Debug,
{
    fn description(&self) -> &str {
        "GrammarWalkError"
    }
}