// main.rs

use sponge_build::convert_to_asm;

// Use the procedural macro to convert Rust code to assembly-like code
convert_to_asm! {
    fn main() {
        let a: i32 = 30;
    }
}
