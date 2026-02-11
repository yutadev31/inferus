use crate::{lexer::Lexer, parser::Parser, syntax::SyntaxNode};

pub mod ast;
mod lexer;
mod parser;
mod syntax;

pub fn parse(input: &str) -> SyntaxNode {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(input, &tokens);
    parser.parse();
    parser.build()
}
