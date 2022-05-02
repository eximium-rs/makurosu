use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token,
    Expr,
    Pat,
    PatOr,
    Token
};

use crate::template::parse::{pipechain::Pipechain, Block};

#[derive(Clone, Debug)]
pub struct MatchArm {
    pub pat: Pat,
    pub fat_arrow_token: Token!(=>),
    pub body: Box<Block>,
    pub comma: Option<Token!(,)>
}

impl MatchArm {
    pub fn peek_case(input: ParseStream) -> bool {
        input.peek(Token!(|)) &&
            !input.peek(Token!(||)) &&
            !input.peek(Token!(|=))
    }

    pub fn parse_multi_pat(input: ParseStream) -> syn::Result<Pat> {
        let pat: Pat = input.parse()?;
        if !Self::peek_case(input) {
            return Ok(pat)
        }
        let mut cases: Punctuated<Pat, Token!(|)> = {
            let mut cases = Punctuated::new();
            cases.push_value(pat);
            cases
        };
        while Self::peek_case(input) {
            cases.push_punct(input.parse::<Token!(|)>()?);
            cases.push_value(input.parse::<Pat>()?);
        }
        Ok(Pat::Or(PatOr { attrs: Vec::new(), leading_vert: None, cases }))
    }
}

impl Parse for MatchArm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: Self::parse_multi_pat(input)?,
            fat_arrow_token: input.parse()?,
            body: Box::new(input.parse()?),
            comma: None
        })
    }
}

#[derive(Clone, Debug)]
pub enum MatchExpr {
    Expr(Box<Expr>),
    Pipechain(Pipechain)
}

impl Parse for MatchExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<Pipechain>() {
            Ok(pipechain) => Ok(Self::Pipechain(pipechain)),
            Err(_) => Ok(Self::Expr(Box::new(Expr::parse_without_eager_brace(
                input
            )?)))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Match {
    pub percent_token: Token!(%),
    pub match_token: Token!(match),
    pub match_expr: MatchExpr,
    pub brace_token: token::Brace,
    pub arms: Vec<MatchArm>
}

impl Match {
    pub fn peek_start(input: ParseStream) -> bool {
        input.peek(Token!(%)) && input.peek2(Token!(match))
    }
}

impl Parse for Match {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let mut match_stmt = Self {
            percent_token: input.parse()?,
            match_token: input.parse()?,
            match_expr: input.parse()?,
            brace_token: braced!(content in input),
            arms: Vec::new()
        };

        while !content.is_empty() {
            match_stmt.arms.push(content.parse()?);
        }
        Ok(match_stmt)
    }
}

#[cfg(test)]
#[path = "match_stmt_test.rs"]
mod match_stmt_test;
