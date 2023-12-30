use crate::token::Token;
use anyhow;
use std::error;

struct ParseError {
    token: Token,
    tpe: ErrorType,
}

#[derive(PartialEq)]
#[repr(u8)]
enum ErrorType {
    UnexpectedTokenError,
}

// impl error::Error for ParseError {}
