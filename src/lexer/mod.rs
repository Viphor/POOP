pub use logos::Logos;

pub mod wrapper;

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    // Logos specific
    #[end]
    End,
    #[error]
    Error,

    // Comments
    #[regex = "//[^\n]*\n"]
    LineComment,

    // Special characters
    #[token = "("]
    LParen,
    #[token = ")"]
    RParen,
    #[token = "{"]
    LBrace,
    #[token = "}"]
    RBrace,
    #[token = "["]
    LBracket,
    #[token = "]"]
    RBracket,
    #[token = "."]
    Period,
    #[token = ","]
    Comma,
    #[token = ";"]
    Semicolon,
    #[token = ":"]
    Colon,
    #[token = "->"]
    Arrow,
    #[token = "+"]
    Plus,
    #[token = "-"]
    Minus,
    #[token = "*"]
    Star,
    #[token = "/"]
    Slash,
    #[token = "%"]
    Percent,
    #[token = "="]
    Equal,
    #[token = "=="]
    Equality,

    // Keywords
    #[token = "fn"]
    Fn,
    #[token = "let"]
    Let,

    // Literals
    #[regex = "[0-9][0-9_]*"]
    Int,
    #[token = "true"]
    True,
    #[token = "false"]
    False,

    // Identifier
    #[regex = "[a-zA-Z_][a-zA-Z0-9_]*"]
    Ident,
}
