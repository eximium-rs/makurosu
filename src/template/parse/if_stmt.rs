use syn::{
    parse::{Parse, ParseStream},
    token,
    Expr,
    Token
};

use crate::template::parse::{pipechain::Pipechain, Block};

#[derive(Clone, Debug)]
pub enum IfExpr {
    Expr(Box<Expr>),
    Pipechain(Pipechain)
}

impl Parse for IfExpr {
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
pub struct If {
    pub percent_token: Token!(%),
    pub if_token: Token!(if),
    pub if_expr: IfExpr,
    pub body: Box<Block>,
    pub otherwise: Option<Otherwise>
}

impl If {
    pub fn peek_start(input: ParseStream) -> bool {
        input.peek(Token!(%)) && input.peek2(Token!(if))
    }
}

impl Parse for If {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            percent_token: input.parse()?,
            if_token: input.parse()?,
            if_expr: input.parse()?,
            body: Box::new(input.parse()?),
            otherwise: if Otherwise::peek(input) {
                Some(input.parse()?)
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct ElseIf {
    pub else_token: Token!(else),
    pub if_token: Token!(if),
    pub if_expr: IfExpr,
    pub body: Box<Block>,
    pub otherwise: Option<Otherwise>
}

impl ElseIf {
    pub fn peek_start(input: ParseStream) -> bool {
        input.peek(Token!(else)) && input.peek2(Token!(if))
    }
}

impl Parse for ElseIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            else_token: input.parse()?,
            if_token: input.parse()?,
            if_expr: input.parse()?,
            body: Box::new(input.parse()?),
            otherwise: if Otherwise::peek(input) {
                Some(input.parse()?)
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct Else {
    pub else_token: Token!(else),
    pub body: Box<Block>
}

impl Else {
    pub fn peek_start(input: ParseStream) -> bool {
        input.peek(Token!(else)) && input.peek2(token::Brace)
    }
}

impl Parse for Else {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { else_token: input.parse()?, body: input.parse()? })
    }
}

#[derive(Clone, Debug)]
pub enum Otherwise {
    ElseIf(Box<ElseIf>),
    Else(Box<Else>)
}

impl Otherwise {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token!(else))
    }
}

impl Parse for Otherwise {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if ElseIf::peek_start(input) {
            Ok(Otherwise::ElseIf(Box::new(input.parse()?)))
        } else if Else::peek_start(input) {
            Ok(Otherwise::Else(Box::new(input.parse()?)))
        } else {
            Err(input.lookahead1().error())
        }
    }
}

#[cfg(test)]
#[path = "if_stmt_test.rs"]
mod if_stmt_test;
