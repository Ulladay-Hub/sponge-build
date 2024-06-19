mod decompose_tokens;
mod parse_tokens;
mod generate_asm;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("sponge-build")
        .version("0.1.0")
        .about("A powerful rust module to convert Rust to ASM")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .takes_value(true)
            .required(true)
            .about("Sets the input file"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .takes_value(true)
            .required(true)
            .about("Sets the output file"))
        .get_matches();

    let input = matches.value_of("input").expect("Input file is required");
    let output = matches.value_of("output").expect("Output file is required");

    println!("Input file: {}", input);
    println!("Output file: {}", output);

    let tokens = decompose_tokens::decompose(input).expect("Failed to decompose tokens");
    let parsed_tokens = parse_tokens::parse(tokens);
    let asm_code = generate_asm::generate(&parsed_tokens);

    std::fs::write(output, asm_code).expect("Failed to write to output file");
}
