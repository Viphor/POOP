use logos::Logos;
use poop::codegen::Codegen;
use poop::execution_engine::ExecutionEngine;
use poop::lexer::{wrapper::LexerWrapper, Token};
use poop::parser::Parser;

fn main() {
    let program = "{
        let x = 5 + 10;
        {
            let y = x + 2;
        };
        x + 4 * y
    }";
    println!("Running the following program: {}", program);

    let lexer = LexerWrapper(Token::lexer(program));
    let mut parser = Parser::new(lexer);
    let statement = parser.parse().expect("This should be able to parse");

    let mut codegen = Codegen::new("test");
    let main_fn = codegen.build_program(statement);

    let mut ee = ExecutionEngine::new(codegen);
    println!("Return code: {}", ee.run_as_main(main_fn, &[]));

    //let ee_main = ee.get_function("expr");

    //ee_main();
}
