use syn::{
    parse::{Parse, ParseStream},
    Expr,
    Pat,
    Token
};

use crate::template::parse::Block;

#[derive(Clone, Debug)]
pub struct For {
    pub percent_token: Token!(%),
    pub for_token: Token!(for),
    pub pat: Pat,
    pub in_token: Token!(in),
    pub expr: Box<Expr>,
    pub body: Block
}

impl For {
    pub fn peek_start(input: ParseStream) -> bool {
        input.peek(Token!(%)) && input.peek2(Token!(for))
    }
}

impl Parse for For {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            percent_token: input.parse()?,
            for_token: input.parse()?,
            pat: input.parse()?,
            in_token: input.parse()?,
            expr: input.parse()?,
            body: input.parse()?
        })
    }
}

#[cfg(test)]
#[path = "for_loop_test.rs"]
mod for_test;
