mod block;
mod for_loop;
mod if_stmt;
mod match_stmt;
mod pipechain;

use proc_macro2::{TokenStream, TokenTree};
use proc_macro_error::abort;
use syn::{
    parse::{Parse, ParseStream},
    Result
};

use crate::template::parse::{
    for_loop::For,
    if_stmt::If,
    match_stmt::Match,
    pipechain::Pipechain
};

pub use block::Block;

#[derive(Debug, Clone)]
pub enum Node {
    Pipechain(Pipechain),
    If(If),
    For(For),
    Match(Match),
    Block(Block),
    Verbatim(TokenTree)
}

pub fn parse_nodes(input: ParseStream) -> Result<Vec<Node>> {
    let mut nodes: Vec<Node> = vec![];
    while !input.is_empty() {
        if If::peek_start(input) {
            nodes.push(Node::If(input.parse()?));
        } else if Match::peek_start(input) {
            nodes.push(Node::Match(input.parse()?));
        } else if For::peek_start(input) {
            nodes.push(Node::For(input.parse()?));
        } else if Pipechain::peek_start(input) {
            nodes.push(Node::Pipechain(input.parse()?));
        } else if Block::peek_start(input) {
            nodes.push(Node::Block(input.parse()?));
        } else {
            nodes.push(Node::Verbatim(input.parse()?));
        }
    }
    Ok(nodes)
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub nodes: Vec<Node>
}

impl Parse for Ast {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Ast { nodes: parse_nodes(input)? })
    }
}

pub fn parse(input: TokenStream) -> Ast {
    match syn::parse2::<Ast>(input) {
        Ok(ast) => ast,
        Err(err) => {
            abort!(err);
        }
    }
}
