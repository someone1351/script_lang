
#[derive(Debug,Clone,Hash,PartialEq, Eq)]
pub enum GrammarWalkError<'g> {
    Unfinished,
    RecursiveNonTerm(&'g str),
    FailedParse,//((Loc,Vec<GrammarItem<'a>>,)),
    MissingNonTerm(&'g str),
}

impl<'g> std::fmt::Display for GrammarWalkError<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{self:?}",)
    }
}

impl<'g> std::error::Error for GrammarWalkError<'g> {
    fn description(&self) -> &str {
        "GrammarWalkError"
    }
}