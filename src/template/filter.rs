mod ident;
mod string;

use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};

pub fn register_builtins(registry: &mut FilterRegistry) {
    registry.register_filter("ident", ident::IdentFilter);
    registry.register_filter("string", string::StringFilter);
}

static REGISTRY: Lazy<Mutex<FilterRegistry>> = Lazy::new(|| {
    let mut registry = FilterRegistry::default();
    #[cfg(feature = "builtins")]
    register_builtins(&mut registry);
    Mutex::new(registry)
});

#[derive(Debug, Clone)]
pub enum FilterError {}

#[derive(Debug, Clone)]
pub enum FilterArg {
    Bool(bool),
    String(&'static str),
    Int(usize)
}

pub type FilterResult =
    core::result::Result<proc_macro2::TokenStream, FilterError>;

pub trait Filter: Sync + Send {
    fn transform(
        &self, input: proc_macro2::TokenStream, span: proc_macro2::Span,
        args: &[FilterArg]
    ) -> FilterResult;
}

#[derive(Default)]
pub struct FilterRegistry {
    pub filters: HashMap<String, Arc<dyn Filter + 'static>>
}

impl FilterRegistry {
    pub fn global() -> &'static Mutex<Self> {
        &REGISTRY
    }

    pub fn register_filter<F: Filter + 'static>(
        &mut self, name: &str, func: F
    ) {
        self.filters.insert(String::from(name), Arc::new(func));
    }

    pub fn get_filter(&self, name: &str) -> Option<&dyn Filter> {
        match self.filters.get(name) {
            Some(func) => Some(&**func),
            None => None
        }
    }

    pub fn has_filter(&self, name: &str) -> bool {
        self.filters.contains_key(name)
    }
}
