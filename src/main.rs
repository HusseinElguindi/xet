mod parser;
mod reader;
mod scanner;
mod token;

use anyhow::Result;
use parser::Parser;
use reader::Reader;
use scanner::Scanner;
use std::fs::File;
use token::Token;

use crate::token::{LiteralType, TokenType};

fn main() -> Result<()> {
    // Scanner
    let mut file = File::open("./prog.xet")?;
    let reader = Reader(&mut file);
    let scanner = Scanner::new(reader);
    let tokens = scanner.scan();
    println!(
        "{:?}",
        tokens
            .iter()
            .filter(|t| !matches!(t.tpe, TokenType::COMMENT | TokenType::WHITESPACE(_)))
            .collect::<Vec<&Token>>()
    );
    // _reconstruct_tokens(&tokens);

    // Parser
    let mut parser = Parser::new(tokens);
    let root = parser.parse();
    println!("{}", root?.codegen());

    Ok(())
}

fn _reconstruct_tokens(tokens: &Vec<Token>) {
    tokens
        .iter()
        .map(|t| &t.lexeme)
        .for_each(|a| print!("{}", a));
}
