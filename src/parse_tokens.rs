use serde::{Serialize, Deserialize};
use crate::decompose_tokens::Token;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedToken {
    pub instruction: String,
    pub operand: Option<String>,
}

pub fn parse(tokens: Vec<Token>) -> Vec<ParsedToken> {
    let mut parsed_tokens = Vec::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.next() {
        match token.kind.as_str() {
            "Keyword" if token.value == "fn" => {
                // Parse function declaration
                if let Some(Token { kind: _, value }) = iter.next() {
                    parsed_tokens.push(ParsedToken {
                        instruction: "FUNCTION".to_string(),
                        operand: Some(value),
                    });
                }
            }
            "Identifier" if token.value.starts_with("REG_") => {
                // Parse variable declaration
                if let Some(Token { kind: _, value }) = iter.peek() {
                    if value == ":" {
                        iter.next(); // consume ':'
                        if let Some(Token { kind: _, value }) = iter.next() {
                            if value == "i32" {
                                parsed_tokens.push(ParsedToken {
                                    instruction: "DECLARE".to_string(),
                                    operand: Some(token.value),
                                });
                            }
                        }
                    }
                }
            }
            "Literal" => {
                // Parse literal assignment
                if let Some(ParsedToken { instruction, operand: Some(operand) }) = parsed_tokens.last() {
                    if instruction == "DECLARE" && operand.starts_with("REG_") {
                        parsed_tokens.push(ParsedToken {
                            instruction: "LOAD".to_string(),
                            operand: Some(format!("{} {}", operand, token.value)),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    parsed_tokens
}
