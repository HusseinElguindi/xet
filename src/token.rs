#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum TokenType {
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,

    Eq,
    EqEq,
    LT,
    LE,
    GT,
    GE,

    PLUS,
    HYPHEN,
    STAR,
    SLASH,

    BAR,
    DOT,
    COLON,
    UNDERSCORE,

    COMMENT,

    WHITESPACE(WhiteSpaceType),

    KEYWORD(KeywordType),
    LITERAL(LiteralType),

    VERBATIM,

    EOF,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum WhiteSpaceType {
    SPACE,
    TAB,
    NEWLINE,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum LiteralType {
    STRING,
    NUMBER,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum KeywordType {
    ENV,
}

/// Accepts a lexeme and returns the type of keyword that was matched or None if not matched.
pub fn match_keyword(lexeme: &str) -> Option<KeywordType> {
    match lexeme {
        "env" => Some(KeywordType::ENV),
        _ => None,
    }
}

#[derive(Debug)]
pub struct Token {
    pub tpe: TokenType,
    pub lexeme: String,
    pub line: usize,
}
