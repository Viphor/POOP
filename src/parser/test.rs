use super::ast::{Expression, ExpressionContainer, Literal, Number, Value};
use super::*;
use logos::Logos;

#[test]
fn expression_precedence_multiplication_last_test() {
    let lexer = LexerWrapper(Token::lexer("3 + 4 * 5"));
    let mut parser = Parser::new(lexer);
    let expression = parser.expression(0);

    let expected = Expression::Plus(
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

    let expected = Expression::Plus(
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
        ExpressionContainer::new(Expression::Plus(
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
        ExpressionContainer::new(Expression::Plus(
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
        ExpressionContainer::new(Expression::Plus(
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
