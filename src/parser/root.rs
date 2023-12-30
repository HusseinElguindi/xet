use crate::token::{Token, TokenType};

use super::{Node, Parser};

#[derive(Debug)]
pub struct RootNode {
    body: Vec<Box<dyn Node>>,
}

impl RootNode {
    pub fn new(body: Vec<Box<dyn Node>>) -> Self {
        Self { body }
    }
}

impl Node for RootNode {
    fn codegen(&self) -> String {
        self.body.iter().fold("".to_string(), |mut acc, node| {
            acc.push_str(&node.codegen());
            acc
        })
    }
}
