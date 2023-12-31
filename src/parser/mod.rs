mod env;
mod literal;
mod root;
mod verbatim;

use crate::token::{KeywordType, Token, TokenType};

use env::EnvNode;
use literal::LiteralNode;
use root::RootNode;
use verbatim::VerbatimNode;

use anyhow::{ensure, Context, Result};
use std::{fmt::Debug, iter::Peekable, vec};

pub trait Node: Debug {
    fn codegen(&self) -> String;
}

pub struct Parser {
    tokens: Peekable<vec::IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    // TODO: tex { } can be a "portal" to tex, everything inside is passed verbatim to tex

    pub fn parse(&mut self) -> Result<Box<dyn Node>> {
        let body = self.parse_until(None).with_context(|| {
            format!(
                "Parse error{}",
                self.tokens
                    .peek()
                    .map_or(" at EOF".to_string(), |token| format!(
                        " on line {}",
                        token.line
                    ))
            )
        })?;

        println!("{:#?}", body); // AST

        // TODO: Consider adding a "document" node, this will contain the nodes vec as the body
        Ok(Box::new(RootNode::new(body)))
        // todo!("return a single document token")
    }

    /// Parses and advances the token iterator until the specified token type is peeked, the
    /// terminating token is not consumned. If a terminating token is not specified, parsing
    /// continues until the iterator is empty.
    fn parse_until(&mut self, until_tpe: Option<TokenType>) -> Result<Vec<Box<dyn Node>>> {
        let mut nodes = vec![]; // this is basically the AST

        while let Some(token) = self.tokens.peek() {
            // Break if we match a token with the passed `until_tpe` type
            if let Some(ref until_tpe) = until_tpe {
                if *until_tpe == token.tpe {
                    // self.tokens.next();
                    return Ok(nodes);
                }
            }

            // Each node's parser is expected to advance the token iterator
            let node: Box<dyn Node> = match token.tpe {
                TokenType::KEYWORD(KeywordType::ENV) => Box::new(EnvNode::parse(self)?),
                TokenType::LITERAL(_) => Box::new(LiteralNode::parse(self)),
                TokenType::WHITESPACE(_) | TokenType::VERBATIM => {
                    Box::new(VerbatimNode::parse(self, |peek| {
                        matches!(peek.tpe, TokenType::WHITESPACE(_) | TokenType::VERBATIM)
                    }))
                }
                // TODO: More things
                _ => {
                    self.tokens.next();
                    continue;
                }
            };
            nodes.push(node);
        }

        // If the terminating token was matched (early return), then we do not reach here
        ensure!(
            until_tpe.is_none(),
            "unexpected token: expected {:?}",
            until_tpe.unwrap()
        );
        Ok(nodes)
    }

    /// Consumes the next token, which is expected to be of type `tpe`.
    fn consume(&mut self, expected_tpe: TokenType) -> Result<Token> {
        let token = self.tokens.next();
        ensure!(
            token.as_ref().is_some_and(|t| t.tpe == expected_tpe),
            "unexpected token: expected {:?}",
            expected_tpe
        );
        Ok(token.unwrap())
    }

    /// Advances the token iterator, discarding whitespace until a non-whitespace token is peeked.
    fn skip_whitespace(&mut self) {
        while let Some(token) = self.tokens.peek() {
            match token.tpe {
                TokenType::WHITESPACE(_) => _ = self.tokens.next(),
                _ => return,
            }
        }
    }
}
