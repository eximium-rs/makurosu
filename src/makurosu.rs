mod codegen;
mod parse;

use proc_macro2::TokenStream;

pub use codegen::codegen;
pub use parse::parse;

pub fn expand(input: TokenStream) -> TokenStream {
    let item = parse(input);
    codegen(item)
}
