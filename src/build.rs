// build.rs

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro]
pub fn convert_to_asm(input: TokenStream) -> TokenStream {
    // Parse the input tokens
    let item_fn = parse_macro_input!(input as ItemFn);

    // Get function name and content
    let fn_name = &item_fn.sig.ident;
    let block = &item_fn.block;

    // Generate assembly code based on function content
    let asm_code = quote! {
        fn #fn_name() {
            // Simulated assembly code generation
            // Example: Turning `let a: i32 = 30;` into `let REG_1: i32 = 30;`
            #block
        }
    };

    // Convert to TokenStream and return
    asm_code.into()
}
