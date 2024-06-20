use std::error::Error;

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Symbol(char),
    Literal(i32),
}

pub fn decompose(content: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens = Vec::new();
    let words = content.split_whitespace();

    for word in words {
        match word {
            "fn" | "let" | "mut" | "i32" | "main" => tokens.push(Token::Keyword(word.to_string())),
            "{" | "}" | "(" | ")" | ":" | ";" | "=" => {
                for c in word.chars() {
                    tokens.push(Token::Symbol(c));
                }
            }
            _ => {
                if let Ok(literal) = word.parse::<i32>() {
                    tokens.push(Token::Literal(literal));
                } else {
                    tokens.push(Token::Identifier(word.to_string()));
                }
            }
        }
    }

    Ok(tokens)
}
