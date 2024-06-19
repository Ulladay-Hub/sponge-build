use crate::parse_tokens::ParsedToken;

pub fn generate(tokens: &[ParsedToken]) -> String {
    let mut asm_code = String::new();

    for token in tokens {
        asm_code.push_str(&format!("{} {}\n", token.kind, token.value));
    }

    asm_code
}
