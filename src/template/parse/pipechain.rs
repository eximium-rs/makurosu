use proc_macro2::{Ident, TokenTree};
use syn::{
    braced,
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token,
    Pat,
    Token
};

#[derive(Clone, Debug)]
pub enum PipechainFilterArg {
    Pattern(Pat),
    Verbatim(TokenTree)
}

impl Parse for PipechainFilterArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<Pat>() {
            Ok(pat) => Ok(Self::Pattern(pat)),
            Err(_) => Ok(Self::Verbatim(input.parse()?))
        }
    }
}

#[derive(Clone, Debug)]
pub struct PipechainFilterCall {
    pub name: Ident,
    pub paren_token: Option<token::Paren>,
    pub args: Option<Punctuated<PipechainFilterArg, Token!(,)>>
}

impl PipechainFilterCall {
    pub fn peek_start(input: ParseStream) -> bool {
        input.peek(Token!(|)) && input.peek2(syn::Ident)
    }
}

impl Parse for PipechainFilterCall {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token!(|) = input.parse()?;
        let name: Ident = input.parse()?;
        if !input.peek(token::Paren) {
            return Ok(Self { name, paren_token: None, args: None })
        }
        let content;
        let paren_token = Some(parenthesized!(content in input));
        let args = Some(content.parse_terminated(PipechainFilterArg::parse)?);
        Ok(Self { name, paren_token, args })
    }
}

#[derive(Clone, Debug)]
pub struct Pipechain {
    pub percent_token: Token!(%),
    pub brace_token: token::Brace,
    pub patterns: Vec<Pat>,
    pub filters: Vec<PipechainFilterCall>
}

impl Pipechain {
    pub fn peek_start(input: ParseStream) -> bool {
        input.peek(Token!(%)) && input.peek2(token::Brace)
    }
}

impl Parse for Pipechain {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let percent_token: Token!(%) = input.parse()?;
        let brace_token: token::Brace = braced!(content in input);

        let mut patterns: Vec<Pat> = Vec::new();
        while !content.is_empty() && !PipechainFilterCall::peek_start(&content)
        {
            patterns.push(content.parse()?);
        }

        let mut filters: Vec<PipechainFilterCall> = Vec::new();
        if PipechainFilterCall::peek_start(&content) {
            while !content.is_empty() {
                filters.push(content.parse()?);
            }
        }
        Ok(Self { percent_token, brace_token, patterns, filters })
    }
}

#[cfg(test)]
#[path = "pipechain_test.rs"]
mod pipechain_test;
