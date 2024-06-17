pub mod decompose_tokens;
pub mod parse_tokens;
pub mod generate_asm;

use clap::{App, Arg};
use std::fs;

pub fn run() {
    let matches = App::new("sponge-build")
        .version("0.1.0")
        .author("bradinator <imnotamilkglass@gmail.com>")
        .about("Converts Rust code to ASM")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("Input Rust file"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Output ASM file"),
        )
        .get_matches();

    let input = matches.value_of("input").expect("Input file is required");
    let output = matches.value_of("output").expect("Output file is required");

    let rust_code = fs::read_to_string(input).expect("Unable to read input file");
    let tokens = decompose_tokens::decompose(&rust_code);
    fs::write("tokens.json", serde_json::to_string(&tokens).expect("Failed to serialize tokens"))
        .expect("Unable to write tokens.json");

    let parsed_tokens = parse_tokens::parse(tokens);
    let asm_code = generate_asm::generate(&parsed_tokens);
    fs::write(output, asm_code).expect("Unable to write output ASM file");
}
