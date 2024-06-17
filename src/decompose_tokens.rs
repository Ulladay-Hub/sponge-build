use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub kind: String,
    pub value: String,
}

pub fn decompose(rust_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    // Example simple lexer logic to tokenize the Rust code
    let mut chars = rust_code.chars().peekable();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
        } else if ch.is_alphabetic() {
            let mut ident = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    ident.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let kind = match ident.as_str() {
                "fn" => "Keyword",
                _ => "Identifier",
            };
            tokens.push(Token { kind: kind.to_string(), value: ident });
        } else {
            chars.next();
        }
    }

    tokens
}
