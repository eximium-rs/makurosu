use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::{spanned::Spanned, Item, ItemMacro};

/// Parses a user input into a syntax tree.
pub fn parse(input: TokenStream) -> ItemMacro {
    match syn::parse2::<Item>(input) {
        Ok(Item::Macro(ast)) if ast.ident.is_some() => ast,
        Ok(item) => {
            abort! {
                item.span(),
                "Item is not a macro definition";
                help = "#[makurosu] can only be applied on `macro_rules!`."
            }
        }
        Err(err) => abort! { err.to_compile_error(), "Parsing failed." }
    }
}

#[cfg(test)]
#[path = "parse_test.rs"]
mod parse_test;
