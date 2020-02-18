use super::error::ParserError;
use super::Output;
use super::Parser;
use super::Source;
use super::Token;
use logos::source::Slice;
use std::str;

pub type ExpressionContainer = Box<Expression>;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Addition(ExpressionContainer, ExpressionContainer),
    Subtraction(ExpressionContainer, ExpressionContainer),
    Multiplication(ExpressionContainer, ExpressionContainer),
    Division(ExpressionContainer, ExpressionContainer),
    Modulus(ExpressionContainer, ExpressionContainer),
    Equality(ExpressionContainer, ExpressionContainer),
    Value(Value),
}

impl Expression {
    /// NUD stands for `Null-Denotation` which means the operators with no left
    /// context.
    pub fn nud<Source>(parser: &mut Parser<Source>) -> Output<Expression>
    where
        Source: self::Source<'static> + Copy,
    {
        if let Some(token_item) = parser.lexer.peek() {
            match token_item.token {
                Token::Minus | Token::Int | Token::True | Token::False => {
                    Ok(Expression::Value(parser.into()))
                }
                Token::LParen => {
                    parser.next_token();
                    let expr = parser.expression(0);
                    parser.expect_token(Token::RParen)?;
                    expr
                }
                _ => Err(ParserError::error(
                    format!(
                        "Expected: number or boolean, found: {:?}",
                        token_item.slice()
                    ),
                    token_item.range(),
                )),
            }
        } else {
            Err(ParserError::error("Expected: number or boolean", 0..1))
        }
    }

    /// LED stands for `Left-Denotation` which means operators that has a left
    /// context.
    pub fn led(left: Expression, token: Token, right: Expression) -> Output<Expression> {
        match token {
            Token::Plus => Ok(Expression::Addition(Box::new(left), Box::new(right))),
            Token::Minus => Ok(Expression::Subtraction(Box::new(left), Box::new(right))),
            Token::Star => Ok(Expression::Multiplication(Box::new(left), Box::new(right))),
            Token::Slash => Ok(Expression::Division(Box::new(left), Box::new(right))),
            Token::Percent => Ok(Expression::Modulus(Box::new(left), Box::new(right))),
            Token::Equality => Ok(Expression::Equality(Box::new(left), Box::new(right))),
            _ => Err(ParserError::error("Expected: +, -, *, /, %, or ==", 0..1)),
        }
    }

    /// Function to determine binding power of an operator
    pub fn bp(token: Token) -> usize {
        match token {
            Token::Equality => 10,
            Token::Percent => 20,
            Token::Plus | Token::Minus => 30,
            Token::Star | Token::Slash => 40,
            _ => usize::min_value(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Literal(Literal),
}

impl<Source> From<&mut Parser<Source>> for Value
where
    Source: self::Source<'static> + Copy,
{
    fn from(parser: &mut Parser<Source>) -> Self {
        if let Some(token_item) = parser.lexer.peek() {
            match token_item.token {
                Token::Minus | Token::Int | Token::True | Token::False => {
                    Value::Literal(parser.into())
                }
                _ => panic!("Expected: number or boolean in parsing of Value"),
            }
        } else {
            panic!("Expected: number or boolean in parsing of Value");
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(Number),
    Boolean(bool),
}

impl<Source> From<&mut Parser<Source>> for Literal
where
    Source: self::Source<'static> + Copy,
{
    fn from(parser: &mut Parser<Source>) -> Self {
        if let Some(token_item) = parser.lexer.peek() {
            match token_item.token {
                Token::Int | Token::Minus => Literal::Number(parser.into()),
                Token::True => Literal::Boolean(true),
                Token::False => Literal::Boolean(false),
                _ => panic!("Expected: number or boolean in parsing of Literal"),
            }
        } else {
            panic!("Expected: number or boolean in parsing of Literal");
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Int(isize),
    Float(f32),
    Double(f64),
}

impl<Source> From<&mut Parser<Source>> for Number
where
    Source: self::Source<'static> + Copy,
{
    fn from(parser: &mut Parser<Source>) -> Self {
        let sign = if Token::Minus == parser.peek_token() {
            parser.next_token();
            -1
        } else {
            1
        };
        if let Some(token_item) = parser.lexer.peek() {
            match token_item.token {
                Token::Int => Number::Int(
                    sign * str::from_utf8(token_item.slice().as_bytes())
                        .expect("Could not parse byte array")
                        .parse::<isize>()
                        .expect("Parsing of integer failed"),
                ),
                _ => panic!("Expected: number or boolean in parsing of Literal"),
            }
        } else {
            panic!("Expected: number or boolean in parsing of Literal, found EOF");
        }
    }
}

impl Number {
    pub fn to_int(&self) -> isize {
        match self {
            Self::Int(i) => *i,
            Self::Float(f) => *f as isize,
            Self::Double(d) => *d as isize,
        }
    }
}
