
use crate::clike::tokenizer::data::TokenType;
use super::super::super::tokenizer::data::Tokenized;

use super::*;

#[derive(Copy,Clone)]
pub struct FilteredTokenIterContainer<'a> {
    pub start : usize, //if 0, then 0 hasnt been traversed yet
    pub end : usize, //if last_ind then last_ind has been traversed
    pub parsed :&'a Tokenized,
}

impl<'a> FilteredTokenIterContainer<'a> {
    pub fn first(&self) -> Option<TokenContainer<'a>> {
        self.clone().next()
    }
    pub fn to_vec(&self) -> Vec<TokenContainer<'a>> {
        self.collect::<Vec<_>>()
    }
}

impl<'a> Iterator for FilteredTokenIterContainer<'a> {
    type Item = TokenContainer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.start >= self.end {
                break;
            }

            if let TokenType::Eol = self.parsed.primitives[self.start].token_type {
                self.start+=1;
                continue;
            }

            let x=TokenContainer {token_ind: self.start,parsed: self.parsed,};
            self.start+=1;
            return Some(x);
        }

        None
    }
}

impl<'a> DoubleEndedIterator for FilteredTokenIterContainer<'a> {
    fn next_back(&mut self) -> Option<TokenContainer<'a>> {
        loop {
            if self.start >= self.end {
                break;
            }

            self.end-=1;

            if let TokenType::Eol = self.parsed.primitives[self.start].token_type {
                continue;
            }

            return Some(TokenContainer {token_ind:self.end,parsed: self.parsed,});
        }

        None
    }
}

impl<'a> std::fmt::Debug for FilteredTokenIterContainer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        f.write_fmt(format_args!("[{}]", self.clone().map(|p|format!("{p:?}")).collect::<Vec<String>>().join(", ")))

    }
}