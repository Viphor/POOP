use logos::Logos;
use poop::codegen::Codegen;
use poop::lexer::{wrapper::LexerWrapper, Token};
use poop::parser::Parser;
use std::ffi::CString;

fn main() {
    let lexer = LexerWrapper(Token::lexer("3 + 4 + 5"));
    let mut parser = Parser::new(lexer);
    let expr = parser.parse().expect("This should be able to parse");

    let mut codegen = Codegen::new(CString::new("test").unwrap());
    codegen.build_program(expr);
}
