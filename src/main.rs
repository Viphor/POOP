use logos::Logos;
use poop::codegen::Codegen;
use poop::execution_engine::ExecutionEngine;
use poop::lexer::{wrapper::LexerWrapper, Token};
use poop::parser::Parser;

fn main() {
    let program = "
    fn calc() -> int {
        let x = 5 + 10;
        let z = {
            let y = x + 2;
            5
        };
        x + 4 * y + z
    }

    fn main() -> int {
        printf(\"Output: %d\n\", calc());
        0
    }";
    println!("Running the following program: {}", program);

    let lexer = LexerWrapper(Token::lexer(program));
    let mut parser = Parser::new(lexer);
    let program = parser.parse().expect("This should be able to parse");

    let mut codegen = Codegen::new("test");
    let main_fn = codegen.build_program(program);

    let mut ee = ExecutionEngine::new(codegen);
    println!("Return code: {}", ee.run_as_main(main_fn, &[]));

    //let ee_main = ee.get_function("expr");

    //ee_main();
}
