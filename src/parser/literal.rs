use crate::token::{LiteralType, Token, TokenType};

use super::{Node, Parser};

#[derive(Debug)]
pub struct LiteralNode(String);

// Parsing of literals in text mode

// TODO: add environments, text env, math env
// Literals should actual literals, e.g. "hi" and 2

impl LiteralNode {
    pub fn parse(parser: &mut Parser) -> Self {
        let mut body = "".to_string();
        while let Some(token) = parser.tokens.peek() {
            // TODO: escaped characters, in conjuction with environments above
            // {} will need to be escaped to be part of a literal
            match token.tpe {
                TokenType::KEYWORD(_)
                | TokenType::COMMENT
                | TokenType::LBRACE
                | TokenType::RBRACE => break, // parse until a keyword or whitespace is reached
                _ => body.push_str(&token.lexeme),
            };
            parser.tokens.next();
        }

        Self(body)
    }
}

impl Node for LiteralNode {
    fn codegen(&self) -> String {
        self.0.clone()
    }
}
