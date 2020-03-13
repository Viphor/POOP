use logos::Logos;
use poop::codegen::Codegen;
use poop::execution_engine::ExecutionEngine;
use poop::lexer::{wrapper::LexerWrapper, RangeConverter, Token};
use poop::parser::Parser;
use std::fs;

fn main() {
    let program = fs::read_to_string("test.poop").unwrap();
    println!("Running the following program: {}", program);

    let range_converter = RangeConverter::new(&program);
    let lexer = LexerWrapper(Token::lexer(&program as &str));
    let mut parser = Parser::new(lexer, range_converter);
    let program = parser.parse().expect("This should be able to parse");

    let mut codegen = Codegen::new("test");
    let main_fn = codegen.build_program(program);

    let mut ee = ExecutionEngine::new(codegen);
    println!("Return code: {}", ee.run_as_main(main_fn, &[]));

    //let ee_main = ee.get_function("expr");

    //ee_main();
}
