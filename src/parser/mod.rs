//! The syntax used in this parser is as follows:
//!
//! ``` EBNF
//! PROGRAM       := IMPORT PROGRAM
//!               |  DECL PROGRAM
//!               |  λ
//!               ;
//!
//! IMPORT        := Use IDENTIFIER Semicolon ;
//!
//! DECL          := FUNC_DECL
//!               |  CLASS_DECL
//!               |  VAR_DECL
//!               ;
//!
//! FUNC_DECL     := Fn Ident LParen ARG_DECL RParen RETURN_DECL BLOCK ;
//!
//! CLASS_DECL    := MODIFIER Class CLASS_BODY ;
//!
//! VAR_DECL      := Let Ident [ TYPE_DECL ] Equal EXPRESSION ;
//!
//! ARG_DECL      := ARG [ Comma ARG_DECL ]
//!               |  λ
//!               ;
//!
//! ARG           := Ident TYPE_DECL ;
//!
//! RETURN_DECL   := Arrow IDENTIFIER
//!               |  λ
//!               ;
//!
//! TYPE_DECL     := Colon IDENTIFIER ;
//!
//! BLOCK         := LBrace BLOCK_CONTENT RBrace ;
//!
//! BLOCK_CONTENT := STATEMENT [ Semicolon BLOCK_CONTENT ]
//!               |  λ
//!               ;
//!
//! STATEMENT     := EXPRESSION
//!               |  VAR_DECL
//!               ;
//!
//! (* Note: This is going to be evaluated using Pratt parsing
//!    therefore allowing left recursion *)
//! EXPRESSION    := EXPRESSION Plus EXPRESSION
//!               |  EXPRESSION Minus EXPRESSION
//!               |  EXPRESSION Star EXPRESSION
//!               |  EXPRESSION Slash EXPRESSION
//!               |  EXPRESSION Percent EXPRESSION
//!               |  EXPRESSION Equality EXPRESSION
//!               |  EXPRESSION NotEq EXPRESSION
//!               |  EXPRESSION LessThan EXPRESSION
//!               |  EXPRESSION GreaterThan EXPRESSION
//!               |  EXPRESSION LessEq EXPRESSION
//!               |  EXPRESSION GreaterEq EXPRESSION
//!               |  EXPRESSION And EXPRESSION
//!               |  EXPRESSION Or EXPRESSION
//!               |  Not EXPRESSION
//!               |  LParen EXPRESSION RParen
//!               |  IF_EXPRESSION
//!               |  BLOCK
//!               |  VALUE
//!               ;
//!
//! IF_EXPRESSION := If EXPRESSION BLOCK [ Else ( BLOCK | IF_EXPRESSION ) ]
//!               ;
//!
//! VALUE         := LITERAL
//!               |  Ident LParen ARG_LIST RParen
//!               |  Ident
//!               ;
//!
//! LITERAL       := Number
//!               |  True
//!               |  False
//!               ;
//!
//! ARG_LIST      := EXPRESSION [ Comma, ARG_LIST ]
//!               |  λ
//!               ;
//! ```

use super::lexer::{wrapper::LexerWrapper, RangeConverter, Token, Tokens};
use logos::source::Source;
use std::iter::Peekable;
//use std::ops::Range;
use std::str;

pub mod ast;
pub mod error;

#[cfg(test)]
mod test;

pub type Output<Out = ()> = Result<Out, error::ParserError>;

pub struct Parser<'source, Source>
where
    Source: logos::source::Source<'source> + Copy,
{
    lexer: Peekable<LexerWrapper<Source>>,
    range: (usize, usize),
    slice: &'source str,
    range_converter: RangeConverter,
}

impl<'source, Source> Parser<'source, Source>
where
    Source: self::Source<'source> + Copy,
{
    pub fn new(
        lexer: LexerWrapper<Source>,
        range_converter: RangeConverter,
    ) -> Parser<'source, Source> {
        Parser {
            lexer: lexer.peekable(),
            range: (0, 0),
            slice: "",
            range_converter,
        }
    }

    pub fn parse(&mut self) -> Output<ast::Program> {
        self.program()
        //if let Some(token_item) = self.lexer.peek() {
        //    match token_item.token {
        //        Token::Let
        //        | Token::LBrace
        //        | Token::LParen
        //        | Token::Minus
        //        | Token::Int
        //        | Token::True
        //        | Token::False => self.statement(),
        //        _ => Err(error::ParserError::error(
        //            "Unsupported token.",
        //            self.range.clone(),
        //        )),
        //    }
        //} else {
        //    Err(error::ParserError::error(
        //        "Noop. Should be fixed",
        //        self.range.clone(),
        //    ))
        //}
    }

    fn program(&mut self) -> Output<ast::Program> {
        match self.peek_token() {
            Token::Fn | Token::Let => Ok(ast::Program::Decl(
                self.decl()?,
                ast::ProgramContainer::new({ self.program()? }),
            )),
            Token::End => Ok(ast::Program::Empty),
            token => Err(error::ParserError::expected(
                vec![Token::Fn, Token::Let, Token::End],
                token,
                self.range,
            )),
        }
    }

    fn decl(&mut self) -> Output<ast::Decl> {
        match self.peek_token() {
            Token::Fn => Ok(ast::Decl::FuncDecl(self.func_decl()?)),
            Token::Let => {
                let res = ast::Decl::VarDecl(self.var_decl()?);
                self.expect_token(Token::Semicolon)?;
                Ok(res)
            }
            token => Err(error::ParserError::expected(
                vec![Token::Fn, Token::Let],
                token,
                self.range,
            )),
        }
    }

    fn func_decl(&mut self) -> Output<ast::FuncDecl> {
        self.expect_token(Token::Fn)?;
        self.expect_token(Token::Ident)?;
        let name = self.slice;
        self.expect_token(Token::LParen)?;
        let args = self.arg_decls()?;
        self.expect_token(Token::RParen)?;
        let return_type = if let Token::Arrow = self.peek_token() {
            self.next_token();
            self.type_decl()?
        } else {
            ast::Type::Void
        };
        let body = self.block()?;

        Ok(ast::FuncDecl::new(name, args, return_type, body))
    }

    fn arg_decls(&mut self) -> Output<Vec<ast::ArgDecl>> {
        let mut list = Vec::new();
        while let Token::Ident = self.peek_token() {
            self.next_token();
            let name = self.slice;
            self.expect_token(Token::Colon)?;
            let arg_type = self.type_decl()?;
            list.push(ast::ArgDecl::new(name, arg_type));
            if let Token::Comma = self.peek_token() {
                self.next_token();
            } else {
                break;
            }
        }

        Ok(list)
    }

    fn type_decl(&mut self) -> Output<ast::Type> {
        match self.next_token() {
            Token::IntType => Ok(ast::Type::Int),
            Token::FloatType => Ok(ast::Type::Float),
            Token::DoubleType => Ok(ast::Type::Double),
            Token::BooleanType => Ok(ast::Type::Boolean),
            Token::VoidType => Ok(ast::Type::Void),
            Token::Ident => Ok(ast::Type::UserDefined(String::from(self.slice))),
            token => Err(error::ParserError::expected(
                vec![
                    Token::IntType,
                    Token::FloatType,
                    Token::DoubleType,
                    Token::BooleanType,
                    Token::VoidType,
                    Token::Ident,
                ],
                token,
                self.range,
            )),
        }
    }

    fn block(&mut self) -> Output<ast::Block> {
        //println!("Parsing block, next token: {:?}", self.peek_token());
        self.expect_token(Token::LBrace)?;
        let mut statements = Vec::new();
        while {
            statements.push(self.statement()?);
            //println!(
            //    "Parsing next statement in block. Next token: {:?}",
            //    self.peek_token()
            //);
            match self.next_token() {
                Token::Semicolon => true,
                Token::RBrace => false,
                token => {
                    return Err(error::ParserError::error(
                        &format!("Expected ; or }}, found: {:?} ({})", token, self.slice),
                        self.range,
                    ))
                }
            }
        } {} // This a rust hack for do while.
        Ok(ast::Block::new(statements))
    }

    fn statement(&mut self) -> Output<ast::Statement> {
        //println!("Parsing statement, next token: {:?}", self.peek_token());
        match self.peek_token() {
            Token::Let => Ok(ast::Statement::VarDecl(self.var_decl()?)),
            Token::LBrace
            | Token::LParen
            | Token::Minus
            | Token::Ident
            | Token::Int
            | Token::String
            | Token::If
            | Token::Not
            | Token::True
            | Token::False => Ok(ast::Statement::Expression(self.expression(0)?)),
            Token::RBrace => Ok(ast::Statement::Empty),
            _ => Err(error::ParserError::error("Unsupported token.", self.range)),
        }
        //println!(
        //    "Finished parsing statement, next token: {:?}",
        //    self.peek_token()
        //);
    }

    fn identifier(&mut self) -> Output<String> {
        //println!("Parsing Identifier, next token: {:?}", self.peek_token());
        self.expect_token(Token::Ident)?;
        Ok(String::from(self.slice))
    }

    fn var_decl(&mut self) -> Output<ast::VarDecl> {
        //println!("Parsing VarDecl, next token: {:?}", self.peek_token());
        self.expect_token(Token::Let)?;
        let identifier = self.identifier()?;
        self.expect_token(Token::Equal)?;
        let expression = self.expression(0)?;
        Ok(ast::VarDecl::new(identifier, expression))
    }

    /// rbp in this context means right binding power
    fn expression(&mut self, rbp: usize) -> Output<ast::Expression> {
        //println!("Parsing expression, next token: {:?}", self.peek_token());
        let mut left = ast::Expression::nud(self)?;
        let mut token = self.peek_token();
        if token == Token::End {
            return Ok(left);
        }

        while ast::Expression::bp(token) > rbp {
            self.next_token();
            left = ast::Expression::led(left, token, self.expression(ast::Expression::bp(token))?)?;
            token = self.peek_token();
            if token == Token::End {
                return Ok(left);
            }
        }

        //println!(
        //    "Finished parsing expression, next token: {:?}",
        //    self.peek_token()
        //);
        Ok(left)
    }

    fn if_expression(&mut self) -> Output<ast::IfExpression> {
        self.expect_token(Token::If)?;
        let condition = self.expression(0)?;
        let body = self.block()?;

        let mut else_expression = ast::ElseExpression::None;
        if let Token::Else = self.peek_token() {
            self.expect_token(Token::Else)?;
            else_expression = match self.peek_token() {
                Token::LBrace => ast::ElseExpression::Block(self.block()?),
                Token::If => ast::ElseExpression::IfExpression(ast::IfExpressionContainer::new(
                    self.if_expression()?,
                )),
                token => {
                    return Err(error::ParserError::expected(
                        vec![Token::LBrace, Token::If],
                        token,
                        self.range,
                    ))
                }
            }
        }
        Ok(ast::IfExpression::new(condition, body, else_expression))
    }

    fn value(&mut self) -> Output<ast::Value> {
        match self.next_token() {
            token @ Token::Minus
            | token @ Token::Int
            | token @ Token::String
            | token @ Token::True
            | token @ Token::False => Ok(ast::Value::Literal(self.literal(token)?)),
            Token::Ident => {
                if let Token::LParen = self.peek_token() {
                    let identifier = self.slice;
                    self.expect_token(Token::LParen)?;
                    let mut arguments = Vec::new();
                    while match self.peek_token() {
                        Token::RParen => false,
                        Token::Comma => {
                            self.next_token();
                            true
                        }
                        _ => true,
                    } {
                        arguments.push(self.expression(0)?);
                    }
                    self.expect_token(Token::RParen)?;
                    Ok(ast::Value::FunctionCall(ast::FunctionCall::new(
                        identifier, arguments,
                    )))
                } else {
                    Ok(ast::Value::Variable(String::from(self.slice)))
                }
            }
            token => Err(error::ParserError::expected(
                vec![
                    Token::Minus,
                    Token::Int,
                    Token::True,
                    Token::False,
                    Token::Ident,
                ],
                token,
                self.range,
            )),
        }
    }

    fn literal(&mut self, mut token: Token) -> Output<ast::Literal> {
        let sign = if let Token::Minus = token {
            token = self.next_token();
            -1
        } else {
            1
        };
        match token {
            Token::True => Ok(ast::Literal::Boolean(true)),
            Token::False => Ok(ast::Literal::Boolean(false)),
            Token::String => Ok(ast::Literal::String(String::from(
                &self.slice[1..self.slice.len() - 1],
            ))),
            Token::Int => Ok(ast::Literal::Number(self.number(token, sign)?)),
            token => Err(error::ParserError::expected(
                vec![Token::True, Token::False, Token::Int],
                token,
                self.range,
            )),
        }
    }

    fn number(&mut self, token: Token, sign: isize) -> Output<ast::Number> {
        match token {
            Token::Int => Ok(ast::Number::Int(
                sign * str::from_utf8(self.slice.as_bytes())
                    .expect("Could not parse byte array")
                    .parse::<isize>()
                    .expect("Parsing of integer failed"),
            )),
            token => Err(error::ParserError::expected(
                vec![Token::Int],
                token,
                self.range,
            )),
        }
    }

    fn next_token(&mut self) -> Token {
        if let Some(token_item) = self.lexer.next() {
            self.range = self.range_converter.to_line_and_pos(token_item.range());
            self.slice = token_item.slice();
            //println!("Consuming token: {:?} ({})", token_item.token, self.slice);
            token_item.token
        } else {
            self.slice = "EOF";
            Token::End
        }
    }

    fn peek_token(&mut self) -> Token {
        if let Some(token_item) = self.lexer.peek() {
            token_item.token
        } else {
            Token::End
        }
    }

    fn expect_token(&mut self, expected: Token) -> Output {
        let token = self.next_token();
        if expected == token {
            Ok(())
        } else {
            Err(error::ParserError::expected(
                vec![expected],
                token,
                self.range,
            ))
        }
    }
}
