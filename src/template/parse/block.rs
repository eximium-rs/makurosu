use syn::{
    braced,
    parse::{Parse, ParseStream},
    token,
    Result
};

use crate::template::parse::{parse_nodes, Node};

#[derive(Debug, Clone)]
pub struct Block {
    pub brace_token: token::Brace,
    pub nodes: Vec<Node>
}

impl Block {
    pub fn peek_start(input: ParseStream) -> bool {
        input.peek(token::Brace)
    }
}

impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Block {
            brace_token: braced!(content in input),
            nodes: parse_nodes(&content)?
        })
    }
}

#[cfg(test)]
#[path = "block_test.rs"]
mod block_test;
