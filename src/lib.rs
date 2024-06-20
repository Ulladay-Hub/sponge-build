pub mod decompose_tokens;
pub mod parse_tokens;
pub mod generate_asm;

use clap::{Arg, Command};
use std::fs;

pub fn run() {
    let matches = Command::new("sponge-build")
        .version("0.2.0")
        .about("A powerful rust module to convert Rust to ASM")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Sets the input file"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Sets the output file"))
        .get_matches();

    let input = matches.get_one::<String>("input").expect("Input file is required");
    let output = matches.get_one::<String>("output").expect("Output file is required");

    // Read the input file
    let input_content = fs::read_to_string(input).expect("Failed to read the input file");

    // Decompose the input content into tokens
    let tokens = decompose_tokens::decompose(&input_content).expect("Failed to decompose tokens");

    // Parse the tokens
    let parsed_tokens = parse_tokens::parse(tokens);

    // Generate ASM code
    let asm_code = generate_asm::generate(&parsed_tokens);

    // Write the ASM code to the output file
    fs::write(output, asm_code).expect("Failed to write the output file");
}
