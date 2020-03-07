#[cfg(feature = "parser-debug")]
use backtrace::Backtrace;

use super::{Token, Tokens};
use std::fmt;
use std::ops::Range;

pub struct ParserError {
    message: String,
    position: Range<usize>,
    code: ParserErrorCode,
    #[cfg(feature = "parser-debug")]
    backtrace: Backtrace,
}

impl ParserError {
    pub fn new<T: Into<String>>(
        message: T,
        code: ParserErrorCode,
        position: Range<usize>,
    ) -> ParserError {
        ParserError {
            message: message.into(),
            code,
            position,
            #[cfg(feature = "parser-debug")]
            backtrace: Backtrace::new_unresolved(),
        }
    }

    pub fn error<T: Into<String>>(message: T, position: Range<usize>) -> ParserError {
        ParserError::new(message, ParserErrorCode::E9999, position)
    }

    pub fn expected(expected: Vec<Token>, found: Token, position: Range<usize>) -> ParserError {
        ParserError::new(
            format!("Expected {}, found: {}", Tokens::from(expected), found),
            ParserErrorCode::E0002,
            position,
        )
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn code(&self) -> &ParserErrorCode {
        &self.code
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parser error {:?}: {} ({:?})",
            self.code, self.message, self.position
        )?;
        #[cfg(feature = "parser-debug")]
        {
            let mut backtrace = self.backtrace.clone();
            backtrace.resolve();
            write!(f, "\n{:?}", backtrace)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ParserErrorCode {
    /// Unused
    E0001,
    /// Expected another symbol
    E0002,
    /// Unspecified error (i.e. lazy developer)
    E9999,
}
