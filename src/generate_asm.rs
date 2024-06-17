use crate::decompose_tokens::Token;

pub fn generate(tokens: &[Token]) -> String {
    let mut asm_code = String::new();

    for token in tokens {
        match token.kind.as_str() {
            "Keyword" if token.value == "fn" => asm_code.push_str("; Function definition\n"),
            "Identifier" if token.value == "main" => asm_code.push_str("main:\n"),
            "Identifier" if token.value == "REG_1" => asm_code.push_str("mov REG_1, 30\n"),
            _ => {}
        }
    }

    asm_code
}
