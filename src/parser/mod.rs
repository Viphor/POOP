//! The syntax used in this parser is as follows:
//!
//! ``` EBNF
//! PROGRAM      := IMPORT PROGRAM
//!              |  DECL PROGRAM
//!              |  λ
//!              ;
//!
//! IMPORT       := Use IDENTIFIER Semicolon ;
//!
//! DECL         := FUNC_DECL
//!              |  CLASS_DECL
//!              |  VAR_DECL
//!              ;
//!
//! FUNC_DECL    := Fn Ident LParen ARG_DECL RParen RETURN_DECL BODY ;
//!
//! CLASS_DECL   := MODIFIER Class CLASS_BODY ;
//!
//! VAR_DECL     := Let Ident [ TYPE_DECL ] Equal EXPRESSION ;
//!
//! ARG_DECL     := ARG [ Comma ARG_DECL ]
//!              |  λ
//!              ;
//!
//! ARG          := Ident TYPE_DECL ;
//!
//! RETURN_DECL  := Arrow IDENTIFIER
//!              |  λ
//!              ;
//!
//! TYPE_DECL    := Colon IDENTIFIER ;
//!
//! BODY         := LBrace BODY_CONTENT RBrace ;
//!
//! BODY_CONTENT := STATEMENT [ Semicolon BODY_CONTENT ]
//!              |  λ
//!              ;
//!
//! STATEMENT    := BODY
//!              |  EXPRESSION
//!              |  VAR_DECL
//!              ;
//!              
//! (* Note: This is going to be evaluated using Pratt parsing
//!    therefore allowing left recursion *)
//! EXPRESSION   := EXPRESSION Plus EXPRESSION
//!              |  EXPRESSION Minus EXPRESSION
//!              |  EXPRESSION Star EXPRESSION
//!              |  EXPRESSION Slash EXPRESSION
//!              |  EXPRESSION Percent EXPRESSION
//!              |  EXPRESSION Equality EXPRESSION
//!              |  LParen EXPRESSION RParen
//!              |  VALUE
//!              ;
//!
//! VALUE        := LITERAL
//!              |  FUNCTION_CALL
//!              |  VAR
//!              ;
//!
//! LITERAL      := Number
//!              |  True
//!              |  False
//!              ;
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
}

impl<Source> Parser<Source>
where
    Source: self::Source<'static> + Copy,
{
    pub fn new(lexer: LexerWrapper<Source>) -> Parser<Source> {
        Parser {
            lexer: lexer.peekable(),
            range: 0..1,
        }
    }

    pub fn parse(&mut self) -> Output<ast::Expression> {
        if let Some(token_item) = self.lexer.peek() {
            match token_item.token {
                Token::LParen | Token::Minus | Token::Int | Token::True | Token::False => {
                    self.expression(0)
                }
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

    /// rpb in this context means right binding power
    fn expression(&mut self, rbp: usize) -> Output<ast::Expression> {
        let mut left = ast::Expression::nud(self)?;
        self.next_token();
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

        Ok(left)
    }

    fn next_token(&mut self) -> Token {
        if let Some(token_item) = self.lexer.next() {
            self.range = token_item.range();
            token_item.token
        } else {
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
