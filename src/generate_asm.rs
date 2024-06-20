use crate::parse_tokens::ParsedToken;

pub fn generate(parsed_tokens: &[ParsedToken]) -> String {
    let mut asm_code = String::new();

    asm_code.push_str("section .data\n\n");
    asm_code.push_str("section .bss\n\n");
    asm_code.push_str("section .text\n");
    asm_code.push_str("    global _start\n\n");
    asm_code.push_str("_start:\n");

    for token in parsed_tokens {
        match token {
            ParsedToken::VariableAssignment { name, value } => {
                asm_code.push_str(&format!("    mov {}, {}\n", name, value));
            }
            _ => {}
        }
    }

    asm_code.push_str("\n    ; Exit the program\n");
    asm_code.push_str("    mov rax, 60        ; System call for exit\n");
    asm_code.push_str("    xor rdi, rdi       ; Exit code 0\n");
    asm_code.push_str("    syscall            ; Invoke the system call\n");

    asm_code
}
