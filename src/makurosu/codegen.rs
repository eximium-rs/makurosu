use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::ItemMacro;

pub fn codegen(mut item: ItemMacro) -> TokenStream {
    let mut iterator = item.mac.tokens.into_iter();
    let mut tokens = TokenStream::default();

    loop {
        match iterator.next() {
            Some(TokenTree::Group(group))
                if matches!(group.delimiter(), Delimiter::Brace) =>
            {
                tokens.extend_one(quote! {{
                    makurosu::template! #group
                }});
            }
            Some(token) => {
                tokens.extend_one(token);
            }
            None => {
                item.mac.tokens = tokens;
                return item.into_token_stream()
            }
        }
    }
}

#[cfg(test)]
#[path = "codegen_test.rs"]
mod codegen_test;
