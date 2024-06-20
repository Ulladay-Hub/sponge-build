use crate::decompose_tokens::Token;

#[derive(Debug, Clone)]
pub enum ParsedToken {
    VariableAssignment { name: String, value: i32 },
}

pub fn parse(tokens: Vec<Token>) -> Vec<ParsedToken> {
    let mut parsed_tokens = Vec::new();
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        if let Token::Keyword(ref kw) = token {
            if kw == "let" {
                if let Some(Token::Identifier(name)) = iter.next() {
                    if let Some(Token::Symbol(':')) = iter.next() {
                        if let Some(Token::Keyword(typ)) = iter.next() {
                            if typ == "i32" {
                                if let Some(Token::Symbol('=')) = iter.next() {
                                    if let Some(Token::Literal(value)) = iter.next() {
                                        parsed_tokens.push(ParsedToken::VariableAssignment {
                                            name: name.clone(),
                                            value: *value,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    parsed_tokens
}
