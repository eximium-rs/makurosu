#![feature(extend_one)]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

pub(crate) mod makurosu;
pub(crate) mod template;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn makurosu(_args: TokenStream, input: TokenStream) -> TokenStream {
    makurosu::expand(input.into()).into()
}

#[proc_macro_error]
#[proc_macro]
pub fn template(input: TokenStream) -> TokenStream {
    template::expand(input.into()).into()
}
