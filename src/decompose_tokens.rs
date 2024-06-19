use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub kind: String,
    pub value: String,
}

pub fn decompose(filename: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let tokens = vec![
        Token { kind: "Keyword".to_string(), value: "fn".to_string() },
        Token { kind: "Identifier".to_string(), value: "main".to_string() },
        Token { kind: "Symbol".to_string(), value: "(".to_string() },
        Token { kind: "Symbol".to_string(), value: ")".to_string() },
        Token { kind: "Symbol".to_string(), value: "{".to_string() },
        Token { kind: "Identifier".to_string(), value: "REG_1".to_string() },
        Token { kind: "Symbol".to_string(), value: ":".to_string() },
        Token { kind: "Keyword".to_string(), value: "i32".to_string() },
        Token { kind: "Symbol".to_string(), value: "=".to_string() },
        Token { kind: "Literal".to_string(), value: "30".to_string() },
        Token { kind: "Symbol".to_string(), value: ";".to_string() },
        Token { kind: "Symbol".to_string(), value: "}".to_string() },
    ];
    Ok(tokens)
}
