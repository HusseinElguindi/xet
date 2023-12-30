use crate::{
    parser::{Node, Parser},
    token::{LiteralType, Token, TokenType},
};

use anyhow::{anyhow, ensure, Context, Result};

#[derive(Debug)]
pub struct EnvNode {
    name: String,
    body: Vec<Box<dyn Node>>,
}

impl EnvNode {
    pub fn parse(parser: &mut Parser) -> Result<Self> {
        // Consume/discard the ENV token
        parser.tokens.next();
        // Discard whitespace
        parser.skip_whitespace();

        let name = match parser
            .tokens
            .next()
            .context("expected an environment identifier")?
        {
            Token {
                tpe: TokenType::LITERAL(LiteralType::STRING),
                lexeme: name,
                line: _,
            } => name,
            _ => return Err(anyhow!("environment name must be a string literal")),
        };
        parser.skip_whitespace();

        ensure!(
            matches!(
                parser.tokens.next(),
                Some(Token {
                    tpe: TokenType::LBRACE,
                    lexeme: _,
                    line: _
                })
            ),
            "expected '{{' after environment name"
        );

        let body = parser._parse(Some(TokenType::RBRACE))?;
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
