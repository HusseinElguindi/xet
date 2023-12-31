use crate::{
    parser::{Node, Parser},
    token::{LiteralType, TokenType},
};

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct EnvNode {
    name: String,
    body: Vec<Box<dyn Node>>,
}

impl EnvNode {
    /// Parses an environment declaration. ENV LITERAL(STRING) LBRACE BODY RBRACE.
    pub fn parse(parser: &mut Parser) -> Result<Self> {
        // Discard the ENV token
        parser.tokens.next();

        parser.skip_whitespace();

        // Consume the environment name
        let name = parser
            .consume(TokenType::LITERAL(LiteralType::STRING))
            .context("expected a string literal environment name")?
            .lexeme;

        parser.skip_whitespace();

        // LBRACE
        parser
            .consume(TokenType::LBRACE)
            .context("expected '{{' after environment name")?;

        // BODY
        let body = parser.parse_until(Some(TokenType::RBRACE))?;

        // RBRACE
        parser
            .consume(TokenType::RBRACE)
            .context("expected '}}' after environment body declaration")?;

        Ok(Self { name, body })
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
