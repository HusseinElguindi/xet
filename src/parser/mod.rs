mod env;
mod literal;
mod root;
mod verbatim;

use crate::{
    parser::root::RootNode,
    token::{KeywordType, LiteralType, Token, TokenType},
};

use std::{iter::Peekable, vec::IntoIter};

use env::EnvNode;
use literal::LiteralNode;
use std::fmt::Debug;

use self::verbatim::VerbatimNode;

pub trait Node: Debug {
    fn codegen(&self) -> String;
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    // TODO: tex { } can be a "portal" to tex, everything inside is passed verbatim to tex

    pub fn parse(&mut self) -> Box<dyn Node> {
        let body = self._parse(None);
        println!("{:#?}", body);
        // TODO: Consider adding a "document" node, this will contain the nodes vec as the body
        Box::new(RootNode::new(body))
        // todo!("return a single document token")
    }

    // Parses and advances the token iterable the optionally specified token type is specified; if encountered, that token is also consumned.
    fn _parse(&mut self, until_tpe: Option<TokenType>) -> Vec<Box<dyn Node>> {
        let mut nodes = vec![]; // this is basically the AST

        while let Some(token) = self.tokens.peek() {
            // Break if we match a token with the passed `until_tpe` type
            if until_tpe
                .as_ref()
                .is_some_and(|until_tpe| *until_tpe == token.tpe)
            {
                self.tokens.next();
                break;
            }

            // Each node's parser is expected to advance the token iterator
            let node: Box<dyn Node> = match token.tpe {
                TokenType::KEYWORD(KeywordType::ENV) => Box::new(EnvNode::parse(self)),
                TokenType::LITERAL(_) => Box::new(LiteralNode::parse(self)),
                TokenType::WHITESPACE(_) | TokenType::VERBATIM => {
                    Box::new(VerbatimNode::parse(self))
                }
                // TODO: More things
                _ => {
                    self.tokens.next();
                    continue;
                }
            };
            nodes.push(node);
        }
        nodes
    }

    fn skip_whitespace(&mut self) {
        while let Some(token) = self.tokens.peek() {
            match token.tpe {
                TokenType::WHITESPACE(_) => _ = self.tokens.next(),
                _ => return,
            }
        }
    }
}
