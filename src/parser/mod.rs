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
//!               |  LParen EXPRESSION RParen
//!               |  BLOCK
//!               |  VALUE
//!               ;
//!
//! VALUE         := LITERAL
//!               |  FUNCTION_CALL
//!               |  Ident
//!               ;
//!
//! LITERAL       := Number
//!               |  True
//!               |  False
//!               ;
//! ```

use super::lexer::{wrapper::LexerWrapper, Token};
use logos::source::Source;
use std::iter::Peekable;
use std::ops::Range;

pub mod ast;
pub mod error;

#[cfg(test)]
mod test;

pub type Output<Out = ()> = Result<Out, error::ParserError>;

pub struct Parser<Source>
where
    Source: logos::source::Source<'static> + Copy,
{
    lexer: Peekable<LexerWrapper<Source>>,
    range: Range<usize>,
    slice: &'static str,
}

impl<Source> Parser<Source>
where
    Source: self::Source<'static> + Copy,
{
    pub fn new(lexer: LexerWrapper<Source>) -> Parser<Source> {
        Parser {
            lexer: lexer.peekable(),
            range: 0..1,
            slice: "",
        }
    }

    pub fn parse(&mut self) -> Output<ast::Statement> {
        if let Some(token_item) = self.lexer.peek() {
            match token_item.token {
                Token::Let
                | Token::LBrace
                | Token::LParen
                | Token::Minus
                | Token::Int
                | Token::True
                | Token::False => self.statement(),
                _ => Err(error::ParserError::error(
                    "Unsupported token.",
                    self.range.clone(),
                )),
            }
        } else {
            Err(error::ParserError::error(
                "Noop. Should be fixed",
                self.range.clone(),
            ))
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
                        self.range.clone(),
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
            | Token::True
            | Token::False => Ok(ast::Statement::Expression(self.expression(0)?)),
            Token::RBrace => Ok(ast::Statement::Empty),
            _ => Err(error::ParserError::error(
                "Unsupported token.",
                self.range.clone(),
            )),
        }
        //println!(
        //    "Finished parsing statement, next token: {:?}",
        //    self.peek_token()
        //);
    }

    fn identifier(&mut self) -> Output<String> {
        //println!("Parsing Identifier, next token: {:?}", self.peek_token());
        if let Token::Ident = self.next_token() {
            Ok(String::from(self.slice))
        } else {
            Err(error::ParserError::error(
                "Token is not an identifier",
                self.range.clone(),
            ))
        }
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

    fn next_token(&mut self) -> Token {
        if let Some(token_item) = self.lexer.next() {
            self.range = token_item.range();
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
            Err(error::ParserError::error(
                format!("Expected token: {:?}, found: {:?}", expected, token),
                self.range.clone(),
            ))
        }
    }
}
