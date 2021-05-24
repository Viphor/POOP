#[cfg(feature = "type-system-debug")]
use backtrace::Backtrace;

use super::Type;
use std::fmt;

pub struct TypeSystemError {
    message: String,
    code: TypeSystemErrorCode,
    #[cfg(feature = "type-system-debug")]
    backtrace: Backtrace,
}

impl TypeSystemError {
    pub fn new<T: Into<String>>(message: T, code: TypeSystemErrorCode) -> Self {
        Self {
            message: message.into(),
            code,
            #[cfg(feature = "type-system-debug")]
            backtrace: Backtrace::new_unresolved(),
        }
    }

    pub fn error<T: Into<String>>(message: T) -> Self {
        Self::new(message, TypeSystemErrorCode::E9999)
    }

    pub fn type_mismatch(expected: Vec<Type>, found: Type) -> Self {
        Self::new(
            format!("Expected types: {:?}, found: {}", expected, found),
            TypeSystemErrorCode::E0001,
        )
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn code(&self) -> &TypeSystemErrorCode {
        &self.code
    }
}

impl fmt::Debug for TypeSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TypeSystem error {:?}: {}", self.code, self.message)?;
        #[cfg(feature = "type-system-debug")]
        {
            let mut backtrace = self.backtrace.clone();
            backtrace.resolve();
            write!(f, "\n{:?}", backtrace)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum TypeSystemErrorCode {
    /// Type mismatch
    E0001,
    /// Lazy developer
    E9999,
}
