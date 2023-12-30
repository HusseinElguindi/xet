use core::panic;

use crate::token::{LiteralType, Token, TokenType};

use super::{Node, Parser};

#[derive(Debug)]
pub struct EnvNode {
    name: String,
    body: Vec<Box<dyn Node>>,
}

impl EnvNode {
    pub fn parse(parser: &mut Parser) -> Self {
        // Consume/discard the ENV token
        parser.tokens.next();
        // Discard whitespace
        parser.skip_whitespace();

        let name = match parser.tokens.next().expect("expected an env name") {
            Token {
                tpe: TokenType::LITERAL(LiteralType::STRING),
                lexeme: name,
                line: _,
            } => name,
            _ => panic!("expected a string literal env name"),
        };
        parser.skip_whitespace();
        assert!(matches!(
            parser.tokens.next(),
            Some(Token {
                tpe: TokenType::LBRACE,
                lexeme: _,
                line: _
            })
        ));

        let body = parser._parse(Some(TokenType::RBRACE));
        Self { name, body }
    }
}

impl Node for EnvNode {
    fn codegen(&self) -> String {
        format!(
            "\\begin{{{}}}{}\\end{{{}}}",
            &self.name,
            self.body.iter().fold("".to_string(), |mut acc, node| {
                acc.push_str(&node.codegen());
                acc
            }),
            &self.name
        )
    }
}
