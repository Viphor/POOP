use super::ast::*;
use super::*;
use logos::Logos;

fn parser(program: &'static str) -> Parser<&'static str> {
    Parser::new(
        LexerWrapper(Token::lexer(program)),
        RangeConverter::new(program),
    )
}

#[test]
fn expression_precedence_multiplication_last_test() {
    let mut parser = parser("3 + 4 * 5");
    let expression = parser.expression(0);

    let expected = Expression::Addition(
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(3),
        )))),
        ExpressionContainer::new(Expression::Multiplication(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(4),
            )))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(5),
            )))),
        )),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_precedence_multiplication_first_test() {
    let mut parser = parser("3 * 4 + 5");
    let expression = parser.expression(0);

    let expected = Expression::Addition(
        ExpressionContainer::new(Expression::Multiplication(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(3),
            )))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(4),
            )))),
        )),
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(5),
        )))),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_precedence_parentheses_test() {
    let mut parser = parser("3 * (4 + 5)");
    let expression = parser.expression(0);

    let expected = Expression::Multiplication(
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(3),
        )))),
        ExpressionContainer::new(Expression::Addition(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(4),
            )))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(5),
            )))),
        )),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_precedence_double_parentheses_test() {
    let mut parser = parser("(3 * (4 + 5)) + 6");
    let expression = parser.expression(0);

    let expected = Expression::Addition(
        ExpressionContainer::new(Expression::Multiplication(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(3),
            )))),
            ExpressionContainer::new(Expression::Addition(
                ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                    Number::Int(4),
                )))),
                ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                    Number::Int(5),
                )))),
            )),
        )),
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(6),
        )))),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_multiplication_and_unary_minus_test() {
    let mut parser = parser("3 * -4");
    let expression = parser.expression(0);

    let expected = Expression::Multiplication(
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(3),
        )))),
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(-4),
        )))),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_unary_minus_first_then_multiplication_test() {
    let mut parser = parser("-3 * 4");
    let expression = parser.expression(0);

    let expected = Expression::Multiplication(
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(-3),
        )))),
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(4),
        )))),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_precedence_modulus_first_test() {
    let mut parser = parser("3 % 4 + 5");
    let expression = parser.expression(0);

    let expected = Expression::Modulus(
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(3),
        )))),
        ExpressionContainer::new(Expression::Addition(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(4),
            )))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(5),
            )))),
        )),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_precedence_modulus_last_test() {
    let mut parser = parser("3 + 4 % 5");
    let expression = parser.expression(0);

    let expected = Expression::Modulus(
        ExpressionContainer::new(Expression::Addition(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(3),
            )))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(4),
            )))),
        )),
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(5),
        )))),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_with_variable() {
    let mut parser = parser("x + 2");
    let expression = parser.expression(0);

    let expected = Expression::Addition(
        ExpressionContainer::new(Expression::Value(Value::Variable(String::from("x")))),
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(2),
        )))),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_with_function_call_without_arguments() {
    let mut parser = parser("x() + 2");
    let expression = parser.expression(0);

    let expected = Expression::Addition(
        ExpressionContainer::new(Expression::Value(Value::FunctionCall(FunctionCall::new(
            "x",
            Vec::new(),
        )))),
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(2),
        )))),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_with_function_call_with_single_argument() {
    let mut parser = parser("x(2 + 2) + 2");
    let expression = parser.expression(0);

    let expected = Expression::Addition(
        ExpressionContainer::new(Expression::Value(Value::FunctionCall(FunctionCall::new(
            "x",
            vec![Expression::Addition(
                ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                    Number::Int(2),
                )))),
                ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                    Number::Int(2),
                )))),
            )],
        )))),
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(2),
        )))),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn expression_with_function_call_with_multiple_arguments() {
    let mut parser = parser("x(2 + 2, y, z) + 2");
    let expression = parser.expression(0);

    let expected = Expression::Addition(
        ExpressionContainer::new(Expression::Value(Value::FunctionCall(FunctionCall::new(
            "x",
            vec![
                Expression::Addition(
                    ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                        Number::Int(2),
                    )))),
                    ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                        Number::Int(2),
                    )))),
                ),
                Expression::Value(Value::Variable(String::from("y"))),
                Expression::Value(Value::Variable(String::from("z"))),
            ],
        )))),
        ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
            Number::Int(2),
        )))),
    );

    println!("{:?}", expression);
    assert_eq!(expression.unwrap(), expected);
}

#[test]
fn var_decl_with_string() {
    let mut parser = parser("let x = \"hello, world!\"");
    let var_decl = parser.var_decl();

    let expected = VarDecl::new(
        "x".to_string(),
        Expression::Value(Value::Literal(Literal::String(String::from(
            "hello, world!",
        )))),
    );

    println!("{:?}", var_decl);
    assert_eq!(var_decl.unwrap(), expected);
}

#[test]
fn var_decl() {
    let mut parser = parser("let x = 5");
    let var_decl = parser.var_decl();

    let expected = VarDecl::new(
        "x".to_string(),
        Expression::Value(Value::Literal(Literal::Number(Number::Int(5)))),
    );

    println!("{:?}", var_decl);
    assert_eq!(var_decl.unwrap(), expected);
}

#[test]
fn block_single_statement_with_return() {
    let mut parser = parser("{ let x = 5 }");
    let block = parser.block();

    let expected = Block::new(vec![Statement::VarDecl(VarDecl::new(
        "x".to_string(),
        Expression::Value(Value::Literal(Literal::Number(Number::Int(5)))),
    ))]);

    println!("{:?}", block);
    assert_eq!(block.unwrap(), expected);
}

#[test]
fn block_single_statement_without_return() {
    let mut parser = parser("{ let x = 5; }");
    let block = parser.block();

    let expected = Block::new(vec![
        Statement::VarDecl(VarDecl::new(
            "x".to_string(),
            Expression::Value(Value::Literal(Literal::Number(Number::Int(5)))),
        )),
        Statement::Empty,
    ]);

    println!("{:?}", block);
    assert_eq!(block.unwrap(), expected);
}

#[test]
fn block_double_statement_with_return() {
    let mut parser = parser("{ let x = 5; x + 5 }");
    let block = parser.block();

    let expected = Block::new(vec![
        Statement::VarDecl(VarDecl::new(
            "x".to_string(),
            Expression::Value(Value::Literal(Literal::Number(Number::Int(5)))),
        )),
        Statement::Expression(Expression::Addition(
            Box::new(Expression::Value(Value::Variable(String::from("x")))),
            Box::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(5),
            )))),
        )),
    ]);

    println!("{:?}", block);
    assert_eq!(block.unwrap(), expected);
}

#[test]
fn block_with_inner_block() {
    let mut parser = parser("{ { let x = 5; x + 5 } }");
    let block = parser.block();

    let expected = Block::new(vec![Statement::Expression(Expression::Block(Block::new(
        vec![
            Statement::VarDecl(VarDecl::new(
                "x".to_string(),
                Expression::Value(Value::Literal(Literal::Number(Number::Int(5)))),
            )),
            Statement::Expression(Expression::Addition(
                Box::new(Expression::Value(Value::Variable(String::from("x")))),
                Box::new(Expression::Value(Value::Literal(Literal::Number(
                    Number::Int(5),
                )))),
            )),
        ],
    )))]);

    println!("{:?}", block);
    assert_eq!(block.unwrap(), expected);
}

#[test]
fn arg_decls_single_arg() {
    let mut parser = parser("first: int");
    let arg_decl = parser.arg_decls();

    let expected = vec![ArgDecl::new("first", Type::Int)];

    println!("{:?}", arg_decl);
    assert_eq!(arg_decl.unwrap(), expected);
}

#[test]
fn arg_decls_multiple_args() {
    let mut parser = parser("first: int, second: double, third: bool");
    let arg_decls = parser.arg_decls();

    let expected = vec![
        ArgDecl::new("first", Type::Int),
        ArgDecl::new("second", Type::Double),
        ArgDecl::new("third", Type::Boolean),
    ];

    println!("{:?}", arg_decls);
    assert_eq!(arg_decls.unwrap(), expected);
}

#[test]
fn func_decl_without_args_without_return_type() {
    let mut parser = parser("fn function() { 2 + 1 }");
    let function = parser.func_decl();

    let expected = FuncDecl::new(
        "function",
        vec![],
        Type::Void,
        Block::new(vec![Statement::Expression(Expression::Addition(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(2),
            )))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(1),
            )))),
        ))]),
    );

    println!("{:?}", function);
    assert_eq!(function.unwrap(), expected);
}

#[test]
fn func_decl_with_args_without_return_type() {
    let mut parser = parser("fn function(first: int, second: bool) { 2 + 1 }");
    let function = parser.func_decl();

    let expected = FuncDecl::new(
        "function",
        vec![
            ArgDecl::new("first", Type::Int),
            ArgDecl::new("second", Type::Boolean),
        ],
        Type::Void,
        Block::new(vec![Statement::Expression(Expression::Addition(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(2),
            )))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(1),
            )))),
        ))]),
    );

    println!("{:?}", function);
    assert_eq!(function.unwrap(), expected);
}

#[test]
fn func_decl_without_args_with_return_type() {
    let mut parser = parser("fn function() -> int { 2 + 1 }");
    let function = parser.func_decl();

    let expected = FuncDecl::new(
        "function",
        vec![],
        Type::Int,
        Block::new(vec![Statement::Expression(Expression::Addition(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(2),
            )))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(1),
            )))),
        ))]),
    );

    println!("{:?}", function);
    assert_eq!(function.unwrap(), expected);
}

#[test]
fn func_decl_with_args_with_return_type() {
    let mut parser = parser("fn function(first: int, second: bool) -> int { 2 + 1 }");
    let function = parser.func_decl();

    let expected = FuncDecl::new(
        "function",
        vec![
            ArgDecl::new("first", Type::Int),
            ArgDecl::new("second", Type::Boolean),
        ],
        Type::Int,
        Block::new(vec![Statement::Expression(Expression::Addition(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(2),
            )))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(1),
            )))),
        ))]),
    );

    println!("{:?}", function);
    assert_eq!(function.unwrap(), expected);
}

#[test]
fn decl_func_decl() {
    let mut parser = parser("fn function() { 2 + 1 }");
    let decl = parser.decl();

    let expected = Decl::FuncDecl(FuncDecl::new(
        "function",
        vec![],
        Type::Void,
        Block::new(vec![Statement::Expression(Expression::Addition(
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(2),
            )))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(1),
            )))),
        ))]),
    ));

    println!("{:?}", decl);
    assert_eq!(decl.unwrap(), expected);
}

#[test]
fn decl_var_decl() {
    let mut parser = parser("let x = 5;");
    let decl = parser.decl();

    let expected = Decl::VarDecl(VarDecl::new(
        String::from("x"),
        Expression::Value(Value::Literal(Literal::Number(Number::Int(5)))),
    ));
    println!("{:?}", decl);
    assert_eq!(decl.unwrap(), expected);
}

#[test]
fn program_with_single_function() {
    let mut parser = parser("fn main() { 2 + 1 }");
    let program = parser.program();

    let expected = Program::Decl(
        Decl::FuncDecl(FuncDecl::new(
            "main",
            vec![],
            Type::Void,
            Block::new(vec![Statement::Expression(Expression::Addition(
                ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                    Number::Int(2),
                )))),
                ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                    Number::Int(1),
                )))),
            ))]),
        )),
        ProgramContainer::new(Program::Empty),
    );

    println!("{:?}", program);
    assert_eq!(program.unwrap(), expected);
}

#[test]
fn if_expression_no_else() {
    let mut parser = parser("if x < 3 { 5 }");
    let if_expression = parser.if_expression();

    let expected = IfExpression::new(
        Expression::LessThan(
            ExpressionContainer::new(Expression::Value(Value::Variable(String::from("x")))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(3),
            )))),
        ),
        Block::new(vec![Statement::Expression(Expression::Value(
            Value::Literal(Literal::Number(Number::Int(5))),
        ))]),
        ElseExpression::None,
    );

    println!("{:?}", if_expression);
    assert_eq!(if_expression.unwrap(), expected);
}

#[test]
fn if_expression_with_else_block() {
    let mut parser = parser("if x < 3 { 5 } else { 2 }");
    let if_expression = parser.if_expression();

    let expected = IfExpression::new(
        Expression::LessThan(
            ExpressionContainer::new(Expression::Value(Value::Variable(String::from("x")))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(3),
            )))),
        ),
        Block::new(vec![Statement::Expression(Expression::Value(
            Value::Literal(Literal::Number(Number::Int(5))),
        ))]),
        ElseExpression::Block(Block::new(vec![Statement::Expression(Expression::Value(
            Value::Literal(Literal::Number(Number::Int(2))),
        ))])),
    );

    println!("{:?}", if_expression);
    assert_eq!(if_expression.unwrap(), expected);
}

#[test]
fn if_expression_with_else_if() {
    let mut parser = parser("if x < 3 { 5 } else if x > 2 { 2 }");
    let if_expression = parser.if_expression();

    let expected = IfExpression::new(
        Expression::LessThan(
            ExpressionContainer::new(Expression::Value(Value::Variable(String::from("x")))),
            ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                Number::Int(3),
            )))),
        ),
        Block::new(vec![Statement::Expression(Expression::Value(
            Value::Literal(Literal::Number(Number::Int(5))),
        ))]),
        ElseExpression::IfExpression(IfExpressionContainer::new(IfExpression::new(
            Expression::GreaterThan(
                ExpressionContainer::new(Expression::Value(Value::Variable(String::from("x")))),
                ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                    Number::Int(2),
                )))),
            ),
            Block::new(vec![Statement::Expression(Expression::Value(
                Value::Literal(Literal::Number(Number::Int(2))),
            ))]),
            ElseExpression::None,
        ))),
    );

    println!("{:?}", if_expression);
    assert_eq!(if_expression.unwrap(), expected);
}

#[test]
fn statement_with_if_expression_no_else() {
    let mut parser = parser("let x = if x < 3 { 5 }");
    let if_expression = parser.statement();

    let expected = Statement::VarDecl(VarDecl::new(
        String::from("x"),
        Expression::If(IfExpressionContainer::new(IfExpression::new(
            Expression::LessThan(
                ExpressionContainer::new(Expression::Value(Value::Variable(String::from("x")))),
                ExpressionContainer::new(Expression::Value(Value::Literal(Literal::Number(
                    Number::Int(3),
                )))),
            ),
            Block::new(vec![Statement::Expression(Expression::Value(
                Value::Literal(Literal::Number(Number::Int(5))),
            ))]),
            ElseExpression::None,
        ))),
    ));

    println!("{:?}", if_expression);
    assert_eq!(if_expression.unwrap(), expected);
}
