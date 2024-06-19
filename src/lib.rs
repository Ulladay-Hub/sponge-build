pub mod decompose_tokens;
pub mod parse_tokens;
pub mod generate_asm;

use clap::{Arg, Command};

pub fn run() {
    let matches = Command::new("sponge-build")
        .version("0.1.0")
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

    println!("Input file: {}", input);
    println!("Output file: {}", output);

    let tokens = decompose_tokens::decompose(input).expect("Failed to decompose tokens");
    let parsed_tokens = parse_tokens::parse(tokens);
    let asm_code = generate_asm::generate(&parsed_tokens);

    std::fs::write(output, asm_code).expect("Failed to write to output file");
}
