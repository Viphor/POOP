use super::ast::*;
use super::*;
use logos::Logos;

#[test]
fn expression_precedence_multiplication_last_test() {
    let lexer = LexerWrapper(Token::lexer("3 + 4 * 5"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("3 * 4 + 5"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("3 * (4 + 5)"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("(3 * (4 + 5)) + 6"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("3 * -4"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("-3 * 4"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("3 % 4 + 5"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("3 + 4 % 5"));
    let mut parser = Parser::new(lexer);
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
fn var_decl() {
    let lexer = LexerWrapper(Token::lexer("let x = 5"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("{ let x = 5 }"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("{ let x = 5; }"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("{ let x = 5; x + 5 }"));
    let mut parser = Parser::new(lexer);
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
    let lexer = LexerWrapper(Token::lexer("{ { let x = 5; x + 5 } }"));
    let mut parser = Parser::new(lexer);
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
