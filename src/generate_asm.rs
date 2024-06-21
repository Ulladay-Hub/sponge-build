use crate::parse_tokens::ParsedToken;

pub fn generate(parsed_tokens: &[ParsedToken]) -> String {
    let mut asm_code = String::new();

    // Adding sections
    let mut data_section = String::new();
    let mut text_section = String::new();

    data_section.push_str("section .data\n");
    text_section.push_str("section .text\n    global _start\n\n_start:\n");

    for token in parsed_tokens {
        match token {
            ParsedToken::Variable { name, var_type, value } => {
                match var_type.as_str() {
                    "i32" => {
                        text_section.push_str(&format!("    mov {}, {}\n", name, value));
                    }
                    "i64" => {
                        text_section.push_str(&format!("    mov {}, {}\n", name, value));
                    }
                    "u32" => {
                        text_section.push_str(&format!("    mov {}, {}\n", name, value));
                    }
                    "&str" => {
                        data_section.push_str(&format!("    {} db '{}', 0x0A", name, value));
                    }
                    _ => {
                        // Add non supported variable type warning
                        text_section.push_str("; NOT SUPPORTED OR INVALID VARIABLE TYPE. WILL NOT BE IMPORTED")
                    }
                }
            }
            ParsedToken::Function { name, body } => {
                text_section.push_str(&format!("    ; Function {}\n", name));
                for statement in body {
                    text_section.push_str(&format!("    {}\n", statement));
                }
                text_section.push_str("    ; End Function\n");
            }
        }
    }

    // Adding exit code
    text_section.push_str("\n    ; Exit the program\n");
    text_section.push_str("    mov rax, 60        ; System call for exit\n");
    text_section.push_str("    xor rdi, rdi       ; Exit code 0\n");
    text_section.push_str("    syscall            ; Invoke the system call\n");

    asm_code.push_str(&data_section);
    asm_code.push_str("\nsection .bss\n");
    asm_code.push_str(&text_section);

    asm_code
}
