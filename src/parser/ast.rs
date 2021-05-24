//! This module contains the structures needed to form the Abstract Syntax Tree (AST)
//! formed by the [`Parser`] (see for EBNF).

use super::error::ParserError;
use super::Output;
use super::Parser;
use super::Source;
use super::{Token, Tokens};
use std::fmt;
use std::ops::Deref;
use std::str;

/// Wrapper type to help get around recursive types
pub type ProgramContainer = Box<Program>;

/// The top level node in the AST.
#[derive(Debug, PartialEq)]
pub enum Program {
    /// Top level declaration, and compounded with the rest of the program
    Decl(Decl, ProgramContainer),
    /// Empty also represents the end of the program
    Empty,
}

/// Enum of all the types of top level declarations
#[derive(Debug, PartialEq)]
pub enum Decl {
    /// Variable Declaration
    VarDecl(VarDecl),
    /// Function Declaration
    FuncDecl(FuncDecl),
}

/// Function declaration
#[derive(Debug, PartialEq)]
pub struct FuncDecl {
    /// Name of the function
    pub name: String,
    /// Arguments of the function
    pub args: Vec<ArgDecl>,
    /// Return type of the function
    pub return_type: Type,
    /// Body of the function
    pub body: Block,
}

impl FuncDecl {
    /// Creates a new function declaration
    pub fn new(name: &str, args: Vec<ArgDecl>, return_type: Type, body: Block) -> Self {
        Self {
            name: name.to_string(),
            args,
            return_type,
            body,
        }
    }
}

/// Argument declaration
#[derive(Debug, PartialEq)]
pub struct ArgDecl {
    /// Name of the argument
    pub name: String,
    /// Type of the argument
    pub arg_type: Type,
}

impl ArgDecl {
    /// Creates a new argument declaration
    pub fn new(name: &str, arg_type: Type) -> Self {
        Self {
            name: name.to_string(),
            arg_type,
        }
    }
}

/// Enum of all the built in types.
/// This also allows for a user defined type
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    /// Integer (currently the size of [`isize`])
    Int,
    /// Float (currently the size of [`f32`]
    Float,
    /// Double (currently the size of [`f64`]
    Double,
    /// Boolean
    Boolean,
    /// String
    String,
    /// Void, representing nothing
    Void,
    /// User defined type. Uses a string to identify the name
    UserDefined(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Double => write!(f, "double"),
            Type::Boolean => write!(f, "boolean"),
            Type::String => write!(f, "string"),
            Type::Void => write!(f, "void"),
            Type::UserDefined(name) => write!(f, "{}", name),
        }
    }
}

impl<'source, Source> From<&mut Parser<'source, Source>> for Type
where
    Source: self::Source<'source> + Copy,
{
    fn from(parser: &mut Parser<'source, Source>) -> Type {
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

/// Enum representing Statements
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// Variable declaration
    VarDecl(VarDecl),
    /// Expression. Currently most things are expresssions
    Expression(Expression),
    /// Empty statement.
    /// The reason for this to exist is to enable having an empty last expression
    /// like you would find in Rust
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Addition(ExpressionContainer, ExpressionContainer),
    Subtraction(ExpressionContainer, ExpressionContainer),
    Multiplication(ExpressionContainer, ExpressionContainer),
    Division(ExpressionContainer, ExpressionContainer),
    Modulus(ExpressionContainer, ExpressionContainer),
    Equality(ExpressionContainer, ExpressionContainer),
    NotEq(ExpressionContainer, ExpressionContainer),
    LessThan(ExpressionContainer, ExpressionContainer),
    GreaterThan(ExpressionContainer, ExpressionContainer),
    LessEq(ExpressionContainer, ExpressionContainer),
    GreaterEq(ExpressionContainer, ExpressionContainer),
    And(ExpressionContainer, ExpressionContainer),
    Or(ExpressionContainer, ExpressionContainer),
    Not(ExpressionContainer),
    If(IfExpressionContainer),
    Block(Block),
    Value(Value),
}

impl Expression {
    /// NUD stands for `Null-Denotation` which means the operators with no left
    /// context.
    pub fn nud<'source, Source>(parser: &mut Parser<'source, Source>) -> Output<Expression>
    where
        Source: self::Source<'source> + Copy,
    {
        if let Some(token_item) = parser.lexer.peek() {
            match token_item.token {
                Token::Ident
                | Token::Minus
                | Token::Int
                | Token::String
                | Token::True
                | Token::False => Ok(Expression::Value(parser.value()?)),
                Token::Not => {
                    parser.next_token();
                    Ok(Expression::Not(ExpressionContainer::new(
                        parser.expression(0)?,
                    )))
                }
                Token::LParen => {
                    parser.expect_token(Token::LParen)?;
                    let expr = parser.expression(0);
                    parser.expect_token(Token::RParen)?;
                    expr
                }
                Token::LBrace => Ok(Expression::Block(parser.block()?)),
                Token::If => Ok(Expression::If(IfExpressionContainer::new(
                    parser.if_expression()?,
                ))),
                _ => Err(ParserError::error(
                    format!(
                        "Expected: number or boolean, found: {:?}",
                        token_item.slice()
                    ),
                    parser.range_converter.to_line_and_pos(token_item.range()),
                )),
            }
        } else {
            Err(ParserError::error("Expected: number or boolean", (0, 0)))
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
            Token::NotEq => Ok(Expression::NotEq(Box::new(left), Box::new(right))),
            Token::LessThan => Ok(Expression::LessThan(Box::new(left), Box::new(right))),
            Token::GreaterThan => Ok(Expression::GreaterThan(Box::new(left), Box::new(right))),
            Token::LessEq => Ok(Expression::LessEq(Box::new(left), Box::new(right))),
            Token::GreaterEq => Ok(Expression::GreaterEq(Box::new(left), Box::new(right))),
            Token::And => Ok(Expression::And(Box::new(left), Box::new(right))),
            Token::Or => Ok(Expression::Or(Box::new(left), Box::new(right))),
            token => Err(ParserError::expected(
                vec![
                    Token::Plus,
                    Token::Minus,
                    Token::Star,
                    Token::Slash,
                    Token::Percent,
                    Token::Equality,
                    Token::NotEq,
                    Token::LessThan,
                    Token::GreaterThan,
                    Token::LessEq,
                    Token::GreaterEq,
                    Token::And,
                    Token::Or,
                ],
                token,
                (0, 0),
            )),
        }
    }

    /// Function to determine binding power of an operator
    pub fn bp(token: Token) -> usize {
        match token {
            Token::Or => 10,
            Token::And => 20,
            Token::Equality
            | Token::NotEq
            | Token::LessThan
            | Token::GreaterThan
            | Token::LessEq
            | Token::GreaterEq => 30,
            Token::Percent => 40,
            Token::Plus | Token::Minus => 50,
            Token::Star | Token::Slash => 60,
            _ => usize::min_value(),
        }
    }
}

pub type IfExpressionContainer = Box<IfExpression>;

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression {
    pub condition: Expression,
    pub body: Block,
    pub else_expression: ElseExpression,
}

impl IfExpression {
    pub fn new(condition: Expression, body: Block, else_expression: ElseExpression) -> Self {
        Self {
            condition,
            body,
            else_expression,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElseExpression {
    Block(Block),
    IfExpression(IfExpressionContainer),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Literal(Literal),
    Variable(String),
    FunctionCall(FunctionCall),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(Number),
    Boolean(bool),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Int(isize),
    Float(f32),
    Double(f64),
}

impl<'source, Source> From<&mut Parser<'source, Source>> for Number
where
    Source: self::Source<'source> + Copy,
{
    fn from(parser: &mut Parser<'source, Source>) -> Self {
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
