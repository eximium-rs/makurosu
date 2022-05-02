// mod filter;
mod parse;

use proc_macro2::TokenStream;

use parse::parse;

pub fn expand(input: TokenStream) -> TokenStream {
    let ast = parse(input);
    dbg!(ast);
    TokenStream::default()
}
