use super::{Node, Parser};

#[derive(Debug)]
pub struct VerbatimNode {
    body: String,
}

// TODO: optional combine function, accepts peek and returns true if can combine the token. Optimization to reduce number of tree nodes
impl VerbatimNode {
    pub fn parse(parser: &mut Parser) -> Self {
        Self {
            body: parser.tokens.next().unwrap().lexeme,
        }

        // // Foundation for combine function behaviour
        // let mut body = "".to_string();
        // while let Some(token) = parser.tokens.peek() {
        //     // TODO: escaped characters, in conjuction with environments above
        //     // {} will need to be escaped to be part of a literal
        //     match token.tpe {
        //         TokenType::KEYWORD(_)
        //         | TokenType::COMMENT
        //         | TokenType::LBRACE
        //         | TokenType::RBRACE => break, // parse until a keyword or whitespace is reached
        //         _ => body.push_str(&token.lexeme),
        //     };
        //     parser.tokens.next();
        // }

        // Self { body }
    }
}

impl Node for VerbatimNode {
    fn codegen(&self) -> String {
        self.body.clone()
    }
}
