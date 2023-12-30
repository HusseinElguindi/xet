use super::{Node, Parser, Token};

#[derive(Debug)]
pub struct VerbatimNode(String);

impl VerbatimNode {
    /// Parses tokens, whose lexemes will be reflected verbatim in generated code. There must be at
    /// least one token in the parser's token iterator. The combine predicate should return true if
    /// the next token's lexeme should me combined into this node's body. It is passed the next
    /// (peeked) token.
    pub fn parse(parser: &mut Parser, combine: impl Fn(&Token) -> bool) -> Self {
        let mut body = parser.tokens.next().unwrap().lexeme;

        // Combine the next token if the combine predicate is true.
        while let Some(token) = parser.tokens.peek() {
            if combine(token) {
                body.push_str(&token.lexeme);
            } else {
                break;
            }
            parser.tokens.next();
        }

        Self(body)
    }
}

impl Node for VerbatimNode {
    fn codegen(&self) -> String {
        self.0.clone()
    }
}
