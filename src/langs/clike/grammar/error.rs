
#[derive(Debug,Clone,Hash,PartialEq, Eq)]
pub enum GrammarWalkError<'a> {
    Unfinished,
    RecursiveNonTerm(&'a str),
    FailedParse,//((Loc,Vec<GrammarItem<'a>>,)),
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