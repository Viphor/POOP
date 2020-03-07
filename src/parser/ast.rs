use super::error::ParserError;
use super::Output;
use super::Parser;
use super::Source;
use super::{Token, Tokens};
use std::ops::Deref;
use std::str;

pub type ProgramContainer = Box<Program>;

#[derive(Debug, PartialEq)]
pub enum Program {
    Decl(Decl, ProgramContainer),
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum Decl {
    VarDecl(VarDecl),
    FuncDecl(FuncDecl),
}

#[derive(Debug, PartialEq)]
pub struct FuncDecl {
    pub name: String,
    pub args: Vec<ArgDecl>,
    pub return_type: Type,
    pub body: Block,
}

impl FuncDecl {
    pub fn new(name: &str, args: Vec<ArgDecl>, return_type: Type, body: Block) -> Self {
        Self {
            name: name.to_string(),
            args,
            return_type,
            body,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ArgDecl {
    pub name: String,
    pub arg_type: Type,
}

impl ArgDecl {
    pub fn new(name: &str, arg_type: Type) -> Self {
        Self {
            name: name.to_string(),
            arg_type,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Int,
    Float,
    Double,
    Boolean,
    String,
    Void,
    UserDefined(String),
}

impl<Source> From<&mut Parser<Source>> for Type
where
    Source: self::Source<'static> + Copy,
{
    fn from(parser: &mut Parser<Source>) -> Type {
        match parser.next_token() {
            Token::IntType => Type::Int,
            Token::FloatType => Type::Float,
            Token::DoubleType => Type::Double,
            Token::BooleanType => Type::Boolean,
            Token::VoidType => Type::Void,
            Token::Ident => Type::UserDefined(String::from(parser.slice)),
            token => panic!(
                "Expected {}, found {}",
                Tokens::from(vec![
                    Token::IntType,
                    Token::FloatType,
                    Token::DoubleType,
                    Token::BooleanType,
                    Token::VoidType,
                    Token::Ident
                ]),
                token
            ),
        }
    }
}

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
                Token::Ident
                | Token::Minus
                | Token::Int
                | Token::String
                | Token::True
                | Token::False => Ok(Expression::Value(parser.value()?)),
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
    FunctionCall(FunctionCall),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(Number),
    Boolean(bool),
    String(String),
}

//impl From<Token> for Literal {
//    fn from(token: Token) -> Self {
//        match token {
//            Token::Int | Token::Minus => Literal::Number(token.into()),
//            Token::True => Literal::Boolean(true),
//            Token::False => Literal::Boolean(false),
//            _ => panic!("Expected: number or boolean in parsing of Literal"),
//        }
//    }
//}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    pub fn new(name: &str, arguments: Vec<Expression>) -> Self {
        Self {
            name: String::from(name),
            arguments,
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
