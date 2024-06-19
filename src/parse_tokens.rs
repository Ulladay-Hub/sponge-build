use serde::{Serialize, Deserialize};
use crate::decompose_tokens::Token;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedToken {
    pub kind: String,
    pub value: String,
}

pub fn parse(tokens: Vec<Token>) -> Vec<ParsedToken> {
    tokens.into_iter().map(|token| ParsedToken {
        kind: token.kind,
        value: token.value,
    }).collect()
}
