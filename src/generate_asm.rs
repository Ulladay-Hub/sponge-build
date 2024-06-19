use crate::parse_tokens::ParsedToken;

pub fn generate(tokens: &[ParsedToken]) -> String {
    let mut asm_code = String::new();

    for token in tokens {
        match token.instruction.as_str() {
            "FUNCTION" => {
                asm_code.push_str(&format!("; Function: {}\n", token.operand.as_ref().unwrap()));
            }
            "DECLARE" => {
                asm_code.push_str(&format!("; Declare variable: {}\n", token.operand.as_ref().unwrap()));
            }
            "LOAD" => {
                asm_code.push_str(&format!("MOV {}, {}\n", token.operand.as_ref().unwrap(), token.operand.as_ref().unwrap()));
            }
            _ => {}
        }
    }

    asm_code
}
