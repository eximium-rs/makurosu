use proc_macro2::{Span, TokenStream};

use crate::template::filter::{Filter, FilterArg, FilterResult};

#[derive(Default)]
pub struct IdentFilter;

impl Filter for IdentFilter {
    fn transform(
        &self, input: TokenStream, _span: Span, _args: &[FilterArg]
    ) -> FilterResult {
        Ok(input)
    }
}
