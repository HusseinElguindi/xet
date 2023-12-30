use std::iter::Peekable;

use crate::reader::{self, Reader};
use crate::token::{self, LiteralType, Token, TokenType, WhiteSpaceType};

use anyhow::{ensure, Context, Result};

pub struct Scanner<'a> {
    current: String,                   // The current lexeme
    line: usize,                       // The current line
    chars: Peekable<reader::Iter<'a>>, // Iterator of characters to scan
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(reader: Reader<'a>) -> Self {
        Scanner {
            current: String::new(),
            line: 1,
            chars: reader.into_iter().peekable(),
            tokens: Vec::default(),
        }
    }

    pub fn scan(mut self) -> Result<Vec<Token>> {
        while let Some(c) = self.chars.next() {
            self.current.push(c);

            let tpe = match c {
                // Keywords and string literals
                x if x.is_alphabetic() => {
                    self.advance_while_peek(|peek| peek.is_alphanumeric() || peek == '_');

                    token::match_keyword(&self.current)
                        .map_or(TokenType::LITERAL(LiteralType::STRING), |tpe| {
                            TokenType::KEYWORD(tpe)
                        })
                }

                // Numbers
                x if (x == '-' && self.chars.peek().is_some_and(|&peek| peek.is_ascii_digit()))
                    || x.is_ascii_digit() =>
                {
                    if x == '-' {
                        self.chars.next();
                    }

                    self.advance_while_peek(|peek| peek.is_ascii_digit());

                    // Optional fractional part
                    if self.chars.peek().is_some_and(|&peek| peek == '.') {
                        self.current.push('.');
                        self.chars.next();

                        self.match_char(|peek| peek.is_alphanumeric())?; // expect at least one more digit after a decimal point
                        self.advance_while_peek(|peek| peek.is_alphanumeric());
                    }

                    TokenType::LITERAL(LiteralType::NUMBER)
                }

                // Whitespace
                x if x.is_ascii_whitespace() => match x {
                    ' ' => TokenType::WHITESPACE(WhiteSpaceType::SPACE),
                    '\t' => TokenType::WHITESPACE(WhiteSpaceType::TAB),
                    '\n' => TokenType::WHITESPACE(WhiteSpaceType::NEWLINE),
                    _ => TokenType::VERBATIM,
                },

                '(' => TokenType::LPAREN,
                ')' => TokenType::RPAREN,
                '{' => TokenType::LBRACE,
                '}' => TokenType::RBRACE,

                '+' => TokenType::PLUS,
                '-' => TokenType::HYPHEN,
                '*' => TokenType::STAR,

                '|' => TokenType::BAR,
                '.' => TokenType::DOT,
                ':' => TokenType::COLON,
                '_' => TokenType::UNDERSCORE,

                '=' if matches!(self.chars.peek(), Some(x) if *x == '=') => TokenType::EqEq,
                '=' => TokenType::Eq,

                '<' if matches!(self.chars.peek(), Some(x) if *x == '=') => TokenType::LE,
                '<' => TokenType::LT,
                '>' if matches!(self.chars.peek(), Some(x) if *x == '=') => TokenType::GE,
                '>' => TokenType::GT,

                // Comments
                '/' if matches!(self.chars.peek(), Some(x) if *x == '/') => {
                    // Comments end at the first newline
                    self.advance_while_peek(|peek| peek != '\n');
                    TokenType::COMMENT
                }
                '/' => TokenType::SLASH,

                // Verbatim
                _ => {
                    self.advance_while_peek(|peek| !peek.is_ascii_whitespace());
                    TokenType::VERBATIM
                }
            };

            self.make_token(tpe.clone());
            if tpe == TokenType::WHITESPACE(WhiteSpaceType::NEWLINE) {
                self.line += 1;
            }
        }
        self.make_token(TokenType::EOF);
        Ok(self.tokens)
    }

    /// Match an expected character and advances.
    fn match_char(&mut self, pred: impl Fn(char) -> bool) -> Result<()> {
        let c = self.chars.next().context("unexpected character")?;
        ensure!(pred(c), "unexpected character");
        self.current.push(c);
        Ok(())
    }

    /// Advances the extent of the token as long the predicate is true. The predicated is passed
    /// the peeked character. The last character (fails pred) is not consumed.
    fn advance_while_peek(&mut self, pred: impl Fn(char) -> bool) {
        loop {
            if !self.chars.peek().is_some_and(|&peek| pred(peek)) {
                return;
            }
            let c = self.chars.next().unwrap(); // guaranteed to have some value
            self.current.push(c);
        }
    }

    fn make_token(&mut self, tpe: TokenType) {
        let token = Token {
            tpe,
            lexeme: self.current.clone(),
            line: self.line,
        };
        self.tokens.push(token);
        self.current.clear();
    }
}
