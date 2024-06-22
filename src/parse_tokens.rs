use crate::decompose_tokens::Token;

#[derive(Debug, Clone)]
pub enum ParsedToken {
    VariableAssignment { name: String, var_type: String, value: String },
    Function { name: String, body: Vec<String> },
}


pub fn parse(tokens: Vec<Token>) -> Vec<ParsedToken> {
    let mut parsed_tokens = Vec::new();

    let mut iter = tokens.into_iter();
    while let Some(token) = iter.next() {
        match token {
            Token::Keyword(keyword) if keyword == "fn" => {
                if let Some(Token::Identifier(name)) = iter.next() {
                    let mut body = Vec::new();
                    while let Some(body_token) = iter.next() {
                        if let Token::Symbol(sym) = &body_token {
                            if *sym == '}' {
                                break;
                            }
                        }
                        body.push(format!("{:?}", body_token));
                    }
                    parsed_tokens.push(ParsedToken::Function { name, body });
                }
            }
            Token::Identifier(name) => {
                if let Some(Token::Symbol(sym)) = iter.next() {
                    if sym == ':' {
                        if let Some(Token::Keyword(var_type)) = iter.next() {
                            if let Some(Token::Symbol(sym)) = iter.next() {
                                if sym == '=' {
                                    if let Some(Token::Literal(value)) = iter.next() {
                                        parsed_tokens.push(ParsedToken::VariableAssignment {
                                            name,
                                            var_type,
                                            value: value.to_string(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    parsed_tokens
}
