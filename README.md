
# Sponge

Sponge is a powerful Rust module designed to convert Rust code into assembly language (ASM). It tokenizes Rust code, parses the tokens, and generates corresponding assembly code, making it easier to understand and optimize your Rust programs at the assembly level.

## Description

Sponge aims to bridge the gap between high-level Rust code and low-level assembly code. By converting Rust code into ASM, developers can gain insights into how their Rust code translates into machine instructions, which is valuable for performance optimization and learning purposes.

## Features

- **Tokenization**: Breaks down Rust code into tokens.
- **Parsing**: Converts tokens into a structured format.
- **ASM Generation**: Produces equivalent assembly code from parsed tokens.
- **Customizable Output**: Allows for tailored assembly code generation based on specific needs.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
sponge = "0.2.0"
```

Run `cargo build` to install the dependencies.

## Usage

Sponge can be used in multiple ways to suit your needs. You can use it to tokenize Rust code, parse the tokens, or generate ASM code.

### Tokenizing Rust Code

To tokenize a Rust file:

```rust
extern crate sponge;

use sponge::decompose_tokens::decompose;

fn main() {
    let filename = "script.rs";
    match decompose(filename) {
        Ok(tokens) => println!("{:?}", tokens),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Parsing Tokens

To parse the tokens into a structured format:

```rust
extern crate sponge;

use sponge::decompose_tokens::decompose;
use sponge::parse_tokens::parse;

fn main() {
    let filename = "script.rs";
    match decompose(filename) {
        Ok(tokens) => {
            let parsed_tokens = parse(tokens);
            println!("{:?}", parsed_tokens);
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Generating ASM Code

To generate ASM code from parsed tokens:

```rust
extern crate sponge;

use sponge::decompose_tokens::decompose;
use sponge::parse_tokens::parse;
use sponge::generate_asm::generate;

fn main() {
    let filename = "script.rs";
    match decompose(filename) {
        Ok(tokens) => {
            let parsed_tokens = parse(tokens);
            let asm_code = generate(&parsed_tokens);
            println!("{}", asm_code);
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Using the Command Line Interface

Sponge also provides a simple CLI to facilitate the conversion process. The available options are:

- `-i, --input <FILE>`: Specifies the input Rust file.
- `-o, --output <FILE>`: Specifies the output ASM file.

#### Example

1. Create a Rust file (`script.rs`) with the following content:

    ```rust
    fn main() {
        let REG_1: i32 = 50;
    }
    ```

2. Run the following command in your terminal to generate the ASM code:

    ```sh
    sponge build -i script.rs -o script.asm
    ```

    This will produce the `script.asm` file:

    ```asm
    section .data

    section .bss

    section .text
        global _start

    _start:
        mov r1, 50         ; Move the value 50 into register 1

        ; Exit the program
        mov rax, 60        ; System call for exit
        xor rdi, rdi       ; Exit code 0
        syscall            ; Invoke the system call
    ```

## Contributing

We welcome contributions to Sponge! Here’s how you can help:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Create a new Pull Request.

Please ensure your code follows the project’s coding standards and includes appropriate tests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For any inquiries or issues, please contact Bradinator at imnotamilkglass@gmail.com.

## Acknowledgements

Special thanks to all contributors and the Rust community for their support and contributions.
