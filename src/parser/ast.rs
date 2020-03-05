use super::error::ParserError;
use super::Output;
use super::Parser;
use super::Source;
use super::Token;
use std::ops::Deref;
use std::str;

#[derive(Debug, PartialEq)]
pub enum Statement {
    VarDecl(VarDecl),
    Expression(Expression),
    Empty,
}

#[derive(Debug, PartialEq)]
pub struct Block(Vec<Statement>);

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self(statements)
    }
}

impl Deref for Block {
    type Target = Vec<Statement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct VarDecl {
    pub identifier: String,
    pub expression: Expression,
}

impl VarDecl {
    pub fn new(identifier: String, expression: Expression) -> Self {
        Self {
            identifier,
            expression,
        }
    }
}

pub type ExpressionContainer = Box<Expression>;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Addition(ExpressionContainer, ExpressionContainer),
    Subtraction(ExpressionContainer, ExpressionContainer),
    Multiplication(ExpressionContainer, ExpressionContainer),
    Division(ExpressionContainer, ExpressionContainer),
    Modulus(ExpressionContainer, ExpressionContainer),
    Equality(ExpressionContainer, ExpressionContainer),
    Block(Block),
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
                Token::Ident | Token::Minus | Token::Int | Token::True | Token::False => {
                    let res = Ok(Expression::Value(parser.into()));
                    parser.next_token();
                    res
                }
                Token::LParen => {
                    parser.expect_token(Token::LParen)?;
                    let expr = parser.expression(0);
                    parser.expect_token(Token::RParen)?;
                    expr
                }
                Token::LBrace => Ok(Expression::Block(parser.block()?)),
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
    Variable(String),
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
                Token::Ident => Value::Variable(String::from(token_item.slice())),
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
