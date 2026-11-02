use std::fmt::Debug;

#[derive(Debug,Clone,Hash,PartialEq, Eq)]
pub enum GrammarWalkError<'g,NT>
where
    NT:Debug,
{
    Error(&'g str),
    FailedParse,//((Loc,Vec<GrammarItem<'a>>,)),
    // Unfinished,
    RecursiveNonTerm(NT),
    MissingNonTerm(NT),
}

impl<'g,NT> std::fmt::Display for GrammarWalkError<'g,NT>
where
    NT:Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{self:?}",)
    }
}

impl<'g,NT> std::error::Error for GrammarWalkError<'g,NT>
where
    NT:Debug,
{
    fn description(&self) -> &str {
        "GrammarWalkError"
    }
}