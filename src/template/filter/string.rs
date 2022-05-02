use proc_macro2::{Span, TokenStream};

use crate::template::filter::{Filter, FilterArg, FilterResult};

#[derive(Default)]
pub struct StringFilter;

impl Filter for StringFilter {
    fn transform(
        &self, input: TokenStream, _span: Span, _args: &[FilterArg]
    ) -> FilterResult {
        Ok(input)
    }
}
